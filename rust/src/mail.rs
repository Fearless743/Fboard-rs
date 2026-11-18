use std::collections::HashMap;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde_json::{json, Value};
use sqlx::MySqlPool;

use crate::settings::Settings;
use crate::util;

pub struct TemplateMeta {
    pub name: &'static str,
    pub label: &'static str,
    pub required_vars: &'static [&'static str],
    pub optional_vars: &'static [&'static str],
}

/// Mirrors `App\Models\MailTemplate::TEMPLATES`.
pub const TEMPLATES: &[TemplateMeta] = &[
    TemplateMeta {
        name: "verify",
        label: "邮箱验证码",
        required_vars: &["code"],
        optional_vars: &["name", "url"],
    },
    TemplateMeta {
        name: "notify",
        label: "站点通知",
        required_vars: &["content"],
        optional_vars: &["name", "url"],
    },
    TemplateMeta {
        name: "remindExpire",
        label: "到期提醒",
        required_vars: &[],
        optional_vars: &["name", "url"],
    },
    TemplateMeta {
        name: "remindTraffic",
        label: "流量提醒",
        required_vars: &[],
        optional_vars: &["name", "url"],
    },
    TemplateMeta {
        name: "mailLogin",
        label: "邮件登录",
        required_vars: &["link"],
        optional_vars: &["name", "url"],
    },
];

pub fn template_meta(name: &str) -> Option<&'static TemplateMeta> {
    TEMPLATES.iter().find(|t| t.name == name)
}

static TEMPLATE_ESCAPED: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{\{\s*\$?([a-zA-Z0-9_.-]+)\s*\}\}").unwrap());
static TEMPLATE_NL2BR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{!!\s*nl2br\(\s*\$([a-zA-Z0-9_.-]+)\s*\)\s*!!\}").unwrap());
static TEMPLATE_RAW: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{!!\s*\$([a-zA-Z0-9_.-]+)\s*!!\}").unwrap());
static PLACEHOLDER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{\{\s*([a-zA-Z0-9_.-]+)(?:\|([^}]*))?\s*\}\}").unwrap());

/// Convert the bundled templates to the admin-editable `{{placeholder}}` syntax.
pub fn normalize_placeholders(template: &str) -> String {
    let out = TEMPLATE_NL2BR.replace_all(template, "{{$1}}");
    let out = TEMPLATE_RAW.replace_all(&out, "{{$1}}");
    TEMPLATE_ESCAPED.replace_all(&out, "{{$1}}").into_owned()
}

/// Replace `{{key}}` / `{{key|default}}` placeholders. Values are inserted
/// verbatim; callers escape user supplied text before passing it in.
pub fn render(template: &str, vars: &HashMap<String, String>) -> String {
    PLACEHOLDER
        .replace_all(template, |caps: &Captures<'_>| {
            let key = &caps[1];
            let default = caps.get(2).map(|m| m.as_str().trim());
            match vars.get(key) {
                Some(value) if !value.is_empty() => value.clone(),
                _ => default
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| caps[0].to_string()),
            }
        })
        .into_owned()
}

pub fn default_content(name: &str) -> String {
    let template = match name {
        "verify" => include_str!("../../resources/views/mail/default/verify.html"),
        "notify" => include_str!("../../resources/views/mail/default/notify.html"),
        "remindExpire" => include_str!("../../resources/views/mail/default/remindExpire.html"),
        "remindTraffic" => {
            include_str!("../../resources/views/mail/default/remindTraffic.html")
        }
        "mailLogin" => include_str!("../../resources/views/mail/default/mailLogin.html"),
        _ => "",
    };
    if template.is_empty() {
        return String::new();
    }
    normalize_placeholders(template)
}

pub fn default_subject(name: &str, app_name: &str) -> String {
    match name {
        "verify" => format!("{app_name} - 邮箱验证码"),
        "notify" => format!("{app_name} - 站点通知"),
        "remindExpire" => format!("{app_name} - 服务即将到期"),
        "remindTraffic" => format!("{app_name} - 流量使用提醒"),
        "mailLogin" => format!("{app_name} - 邮件登录"),
        _ => app_name.to_string(),
    }
}

pub fn escape_html(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#039;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Escape plain text and keep line breaks.
pub fn text_to_html(value: &str) -> String {
    escape_html(value)
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace('\n', "<br />\n")
}

fn config_str(settings: &Settings, keys: &[&str], default: &str) -> String {
    for key in keys {
        let value = settings.str(key, "");
        if !value.is_empty() {
            return value;
        }
    }
    default.to_string()
}

fn from_address(settings: &Settings) -> String {
    let address = config_str(settings, &["email_from_address", "mail_from_address"], "");
    if address.is_empty() {
        config_str(settings, &["email_username", "mail_username"], "")
    } else {
        address
    }
}

fn from_name(settings: &Settings) -> String {
    let name = config_str(settings, &["email_from_name", "mail_from_name"], "");
    if name.is_empty() {
        settings.str("app_name", "Fboard")
    } else {
        name
    }
}

fn mail_configured(settings: &Settings) -> bool {
    !config_str(settings, &["email_host", "mail_host"], "").is_empty()
}

/// Deliver one already rendered HTML mail and record it in `v2_mail_log`.
pub async fn send_raw(
    db: &MySqlPool,
    settings: &Settings,
    to: &str,
    subject: &str,
    html: &str,
    template_name: &str,
) -> anyhow::Result<()> {
    let result = deliver(settings, to, subject, html).await;
    let error = result.as_ref().err().map(|e| e.to_string());
    let now = util::now();
    let _ = sqlx::query(
        "INSERT INTO v2_mail_log (email, subject, template_name, error, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(to)
    .bind(subject)
    .bind(template_name)
    .bind(error.clone())
    .bind(now)
    .bind(now)
    .execute(db)
    .await;
    match result {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}

/// Render and deliver one of the named templates. A database override in
/// `v2_mail_templates` takes precedence over the bundled HTML template; this
/// mirrors `MailService::sendEmail()`.
pub async fn send_template(
    db: &MySqlPool,
    settings: &Settings,
    to: &str,
    template_name: &str,
    subject: &str,
    vars: &HashMap<String, String>,
) -> anyhow::Result<()> {
    let db_template: Option<(String, String)> =
        sqlx::query_as("SELECT subject, content FROM v2_mail_templates WHERE name = ? LIMIT 1")
            .bind(template_name)
            .fetch_optional(db)
            .await
            .unwrap_or(None);

    let app_name = settings.str("app_name", "Fboard");
    let (subject, html, log_name) = match db_template {
        Some((db_subject, db_content)) => {
            let rendered_subject = render(&db_subject, vars);
            let rendered_subject = if rendered_subject.trim().is_empty() {
                subject.to_string()
            } else {
                rendered_subject
            };
            (
                rendered_subject,
                render(&db_content, vars),
                format!("db:{template_name}"),
            )
        }
        None => {
            let view = default_content(template_name);
            let html = if view.is_empty() {
                render(
                    "<p>{{content}}</p><p><a href=\"{{url}}\">{{url}}</a></p>",
                    vars,
                )
            } else {
                render(&view, vars)
            };
            let subject = if subject.trim().is_empty() {
                default_subject(template_name, &app_name)
            } else {
                subject.to_string()
            };
            (subject, html, format!("mail.default.{template_name}"))
        }
    };

    send_raw(db, settings, to, &subject, &html, &log_name).await
}

async fn deliver(settings: &Settings, to: &str, subject: &str, html: &str) -> anyhow::Result<()> {
    if !mail_configured(settings) {
        anyhow::bail!("邮件服务未配置 (email_host)");
    }
    let host: String = config_str(settings, &["email_host", "mail_host"], "");
    let port: u16 = config_str(settings, &["email_port", "mail_port"], "465")
        .parse()
        .unwrap_or(465);
    let username = config_str(settings, &["email_username", "mail_username"], "");
    let password = config_str(settings, &["email_password", "mail_password"], "");
    let encryption = config_str(settings, &["email_encryption", "mail_encryption"], "ssl");
    let from_address = from_address(settings);
    let from_name = from_name(settings);
    if from_address.is_empty() {
        anyhow::bail!("发件人地址未配置 (email_from_address)");
    }

    let email = Message::builder()
        .from(format!("{from_name} <{from_address}>").parse()?)
        .to(to.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(html.to_string())?;

    let builder = match encryption.as_str() {
        "tls" | "starttls" => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&host)?,
        "none" | "" => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&host),
        _ => AsyncSmtpTransport::<Tokio1Executor>::relay(&host)?,
    };
    let mut builder = builder.port(port);
    if !username.is_empty() {
        builder = builder.credentials(Credentials::new(username, password));
    }
    builder.build().send(email).await?;
    Ok(())
}

/// Variables used when previewing/testing a template from the admin panel.
pub fn test_vars(name: &str, app_name: &str, app_url: &str) -> HashMap<String, String> {
    let mut vars = HashMap::new();
    vars.insert("name".into(), app_name.to_string());
    vars.insert("url".into(), app_url.to_string());
    match name {
        "verify" => {
            vars.insert("code".into(), "123456".into());
        }
        "notify" => {
            vars.insert("content".into(), "这是一封测试通知邮件。".into());
        }
        "mailLogin" => {
            vars.insert(
                "link".into(),
                format!(
                    "{}/#/login?verify=test-token",
                    app_url.trim_end_matches('/')
                ),
            );
        }
        _ => {}
    }
    vars
}

pub fn test_subject(name: &str, app_name: &str) -> String {
    match name {
        "verify" => format!("{app_name} - 验证码测试"),
        "notify" => format!("{app_name} - 通知测试"),
        "remindExpire" => format!("{app_name} - 到期提醒测试"),
        "remindTraffic" => format!("{app_name} - 流量提醒测试"),
        "mailLogin" => format!("{app_name} - 登录链接测试"),
        _ => format!("{app_name} - 邮件测试"),
    }
}

pub fn validate_content(name: &str, content: &str) -> Vec<String> {
    let Some(meta) = template_meta(name) else {
        return vec![format!("Unknown template: {name}")];
    };
    meta.required_vars
        .iter()
        .filter(|var| {
            !content.contains(&format!("{{{{{var}}}}}"))
                && !content.contains(&format!("{{{{ {var} }}}}"))
        })
        .map(|var| format!("缺少必要占位符: {{{{{var}}}}}"))
        .collect()
}

/// Build SMTP-related public config (used by the admin test endpoint).
pub fn smtp_snapshot(settings: &Settings) -> Value {
    json!({
        "host": config_str(settings, &["email_host", "mail_host"], ""),
        "port": config_str(settings, &["email_port", "mail_port"], "465"),
        "encryption": config_str(settings, &["email_encryption", "mail_encryption"], "ssl"),
        "from_address": from_address(settings),
        "from_name": from_name(settings),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars() -> HashMap<String, String> {
        let mut vars = HashMap::new();
        vars.insert("name".into(), "Fboard".into());
        vars.insert("code".into(), "123456".into());
        vars.insert("link".into(), "https://example.com/login".into());
        vars.insert("content".into(), "line1<br />\nline2".into());
        vars.insert("url".into(), "https://example.com".into());
        vars
    }

    #[test]
    fn renders_placeholders_and_defaults() {
        let out = render("Hi {{name}}, code {{code}}, {{missing|fallback}}", &vars());
        assert_eq!(out, "Hi Fboard, code 123456, fallback");
        assert_eq!(render("{{missing}}", &vars()), "{{missing}}");
    }

    #[test]
    fn normalizes_bundled_mail_templates() {
        let view = default_content("verify");
        assert!(view.contains("{{name}}"));
        assert!(view.contains("{{code}}"));
        assert!(!view.contains("{{$"));
        let html = render(&view, &vars());
        assert!(html.contains("123456"));
        assert!(html.contains("Fboard"));
    }

    #[test]
    fn escapes_and_keeps_line_breaks() {
        assert_eq!(
            text_to_html("<b>x</b>\nnext"),
            "&lt;b&gt;x&lt;/b&gt;<br />\nnext"
        );
    }

    #[test]
    fn validates_required_vars() {
        assert!(validate_content("verify", "<p>{{code}}</p>").is_empty());
        assert_eq!(
            validate_content("verify", "<p>no code</p>"),
            vec!["缺少必要占位符: {{code}}".to_string()]
        );
    }

    #[test]
    fn parse_smtp_port_and_encryption() {
        let settings = Settings::default();
        let snapshot = smtp_snapshot(&settings);
        assert_eq!(snapshot["port"], "465");
        assert_eq!(snapshot["encryption"], "ssl");
    }
}

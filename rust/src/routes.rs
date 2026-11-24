use axum::extract::{Path, State};
use axum::http::Uri;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{any, get, post};
use axum::Router;
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::api::{admin, admin_content, admin_user, auth, guest, order, server, subscribe, user};
use crate::state::SharedState;

pub fn build(state: SharedState) -> Router {
    let secure_path = state.secure_path();
    let subscribe_path = state.subscribe_path();
    let admin = Router::new().nest(&format!("/api/v2/{secure_path}"), admin_router());

    let admin_root = format!("/{secure_path}");
    let mut app = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/metrics", get(metrics))
        // ---- passport (v1 + v2 are aliases) ----
        .route("/api/v1/passport/auth/register", post(auth::register))
        .route("/api/v1/passport/auth/login", post(auth::login))
        .route("/api/v1/passport/auth/forget", post(auth::forget))
        .route(
            "/api/v1/passport/auth/getQuickLoginUrl",
            post(auth::quick_login_url),
        )
        .route(
            "/api/v1/passport/auth/loginWithMailLink",
            post(auth::mail_link),
        )
        .route("/api/v1/passport/auth/token2Login", get(auth::token2login))
        .route(
            "/api/v1/passport/comm/sendEmailVerify",
            post(auth::send_email_verify),
        )
        .route("/api/v1/passport/comm/pv", post(auth::pv))
        .route("/api/v2/passport/auth/register", post(auth::register))
        .route("/api/v2/passport/auth/login", post(auth::login))
        .route("/api/v2/passport/auth/forget", post(auth::forget))
        .route(
            "/api/v2/passport/auth/getQuickLoginUrl",
            post(auth::quick_login_url),
        )
        .route(
            "/api/v2/passport/auth/loginWithMailLink",
            post(auth::mail_link),
        )
        .route("/api/v2/passport/auth/token2Login", get(auth::token2login))
        .route(
            "/api/v2/passport/comm/sendEmailVerify",
            post(auth::send_email_verify),
        )
        .route("/api/v2/passport/comm/pv", post(auth::pv))
        // ---- guest ----
        .route("/api/v1/guest/plan/fetch", get(guest::plans))
        .route("/api/v1/guest/comm/config", get(guest::comm_config))
        .route(
            "/api/v1/guest/payment/notify/{method}/{uuid}",
            any(guest::payment_notify),
        )
        .route(
            "/api/v1/guest/telegram/webhook",
            post(guest::telegram_webhook),
        )
        // ---- client ----
        .route(
            "/api/v1/client/subscribe",
            get(subscribe::client_subscribe_query),
        )
        .route(
            "/api/{version}/client/app/getConfig",
            get(subscribe::app_config),
        )
        .route(
            "/api/{version}/client/app/getVersion",
            get(guest::app_version),
        )
        .route(
            &format!("/{subscribe_path}/{{token}}"),
            get(subscribe::client_subscribe),
        )
        // ---- user ----
        .route("/api/v2/user/info", get(user::info))
        .route("/api/v2/user/resetSecurity", get(user::reset_security))
        .route("/api/v1/user/info", get(user::info))
        .route("/api/v1/user/update", post(user::update))
        .route("/api/v1/user/changePassword", post(user::change_password))
        .route("/api/v1/user/resetSecurity", get(user::reset_security))
        .route("/api/v1/user/getSubscribe", get(user::get_subscribe))
        .route("/api/v1/user/getStat", get(user::get_stat))
        .route("/api/v1/user/checkLogin", get(user::check_login))
        .route("/api/v1/user/transfer", post(user::transfer))
        .route("/api/v1/user/getActiveSession", get(user::active_sessions))
        .route(
            "/api/v1/user/removeActiveSession",
            post(user::remove_session),
        )
        .route("/api/v1/user/plan/fetch", get(user::plans))
        .route("/api/v1/user/server/fetch", get(user::servers))
        .route("/api/v1/user/notice/fetch", get(user::notices))
        .route("/api/v1/user/invite/fetch", get(user::invite_fetch))
        .route("/api/v1/user/invite/save", get(user::invite_save))
        .route("/api/v1/user/invite/details", get(user::invite_details))
        .route("/api/v1/user/knowledge/fetch", get(user::knowledge))
        .route("/api/v1/user/knowledge/getCategory", get(user::knowledge_categories))
        .route("/api/v1/user/comm/config", get(user::comm_config))
        .route(
            "/api/v1/user/comm/getStripePublicKey",
            post(guest::stripe_public_key),
        )
        .route("/api/v1/user/stat/getTrafficLog", get(user::traffic_log))
        .route("/api/v1/user/gift-card/types", get(user::gift_card_types))
        .route("/api/v1/user/gift-card/check", post(user::gift_card_check))
        .route("/api/v1/user/gift-card/redeem", post(user::gift_card_redeem))
        .route("/api/v1/user/gift-card/history", get(user::gift_card_history))
        .route("/api/v1/user/gift-card/detail", get(user::gift_card_detail))
        .route("/api/v1/user/telegram/getBotInfo", get(user::telegram_bot_info))
        .route("/api/v1/user/ticket/fetch", get(user::ticket_fetch))
        .route("/api/v1/user/ticket/save", post(user::ticket_save))
        .route("/api/v1/user/ticket/reply", post(user::ticket_reply))
        .route("/api/v1/user/ticket/close", post(user::ticket_close))
        .route(
            "/api/v1/user/ticket/withdraw",
            post(user::withdrawal_create),
        )
        .route("/api/v1/user/withdrawal/fetch", get(user::withdrawal_fetch))
        .route(
            "/api/v1/user/withdrawal/detail",
            post(user::withdrawal_detail),
        )
        .route(
            "/api/v1/user/withdrawal/reply",
            post(user::withdrawal_reply),
        )
        .route("/api/v1/user/withdrawal/close", post(user::ticket_close))
        .route("/api/v1/user/order/save", post(order::save))
        .route("/api/v1/user/order/checkout", post(order::checkout))
        .route("/api/v1/user/order/check", get(order::check))
        .route("/api/v1/user/order/detail", get(order::detail))
        .route("/api/v1/user/order/fetch", get(order::fetch))
        .route(
            "/api/v1/user/order/getPaymentMethod",
            get(order::payment_methods),
        )
        .route("/api/v1/user/order/cancel", post(order::cancel))
        .route("/api/v1/user/coupon/check", post(order::coupon_check))
        .route(
            "/api/v1/user/balance_deposit/create",
            post(order::balance_deposit_create),
        )
        .route(
            "/api/v1/user/balance_deposit/detail",
            get(order::balance_deposit_detail),
        )
        .route(
            "/api/v1/balance_deposit/create",
            post(order::balance_deposit_create),
        )
        .route(
            "/api/v1/balance_deposit/detail",
            get(order::balance_deposit_detail),
        )
        .route(
            "/api/v1/balance_recharge/create",
            post(order::balance_deposit_create),
        )
        .route(
            "/api/v1/balance_recharge/detail",
            get(order::balance_deposit_detail),
        )
        // ---- server (v1 legacy + v2 machine) ----
        .route("/api/v1/server/UniProxy/config", get(server::config))
        .route("/api/v1/server/UniProxy/user", get(server::users))
        .route("/api/v1/server/UniProxy/push", post(server::push))
        .route("/api/v1/server/UniProxy/alive", post(server::alive))
        .route("/api/v1/server/UniProxy/alivelist", get(server::alive_list))
        .route("/api/v1/server/UniProxy/status", post(server::status))
        .route("/api/v1/server/ShadowsocksTidalab/user", get(server::users))
        .route(
            "/api/v1/server/ShadowsocksTidalab/submit",
            post(server::push),
        )
        .route("/api/v1/server/TrojanTidalab/config", get(server::config))
        .route("/api/v1/server/TrojanTidalab/user", get(server::users))
        .route("/api/v1/server/TrojanTidalab/submit", post(server::push))
        .route("/ws", get(crate::ws::ws_handler))
        .route("/api/v2/server/handshake", any(server::handshake))
        .route("/api/v2/server/report", post(server::report))
        .route("/api/v2/server/config", get(server::config))
        .route("/api/v2/server/user", get(server::users))
        .route("/api/v2/server/push", post(server::push))
        .route("/api/v2/server/alive", post(server::alive))
        .route("/api/v2/server/alivelist", get(server::alive_list))
        .route("/api/v2/server/status", post(server::status))
        .route("/api/v2/server/machine/nodes", post(server::machine_nodes))
        .route(
            "/api/v2/server/machine/status",
            post(server::machine_status),
        )
        .merge(admin)
        .route("/", get(spa_index))
        .route(&admin_root, get(spa_index))
        .route("/settings.js", get(settings_js))
        .route("/settings.local.js", get(settings_local_js))
        .route("/images/{*path}", get(public_image))
        .route("/locales/{*path}", get(admin_locale))
        .route("/plugins/{code}/{*path}", get(plugin_static))
        .fallback(spa_index)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    // Serve built admin assets under /assets/admin as well.
    if state.cfg.admin_dist.exists() {
        app = app.nest_service("/assets/admin", ServeDir::new(&state.cfg.admin_dist));
    }
    app
}

fn admin_router() -> Router<SharedState> {
    Router::new()
        // config & mail
        .route("/config/fetch", get(admin::config_fetch))
        .route("/config/save", post(admin::config_save))
        .route(
            "/config/getEmailTemplate",
            get(admin::config_email_template),
        )
        .route(
            "/config/setTelegramWebhook",
            post(admin::config_set_telegram_webhook),
        )
        .route("/config/testSendMail", post(admin::config_test_mail))
        .route("/mail/template/list", get(admin::mail_templates))
        .route("/mail/template/get", get(admin::mail_templates))
        .route(
            "/mail/template/save",
            post(admin::mail_template_save),
        )
        .route(
            "/mail/template/reset",
            post(admin::mail_template_reset),
        )
        .route(
            "/mail/template/test",
            post(admin::mail_template_test),
        )
        // plans
        .route("/plan/fetch", get(admin::plan_fetch))
        .route("/plan/save", post(admin::plan_save))
        .route("/plan/update", post(admin::plan_update))
        .route("/plan/drop", post(admin::plan_drop))
        .route("/plan/sort", post(admin::plan_sort))
        // server metadata
        .route("/server/group/fetch", get(admin::groups_fetch))
        .route("/server/group/save", post(admin::group_save))
        .route("/server/group/drop", post(admin::group_drop))
        .route("/server/route/fetch", get(admin::routes_fetch))
        .route("/server/route/save", post(admin::route_save))
        .route("/server/route/drop", post(admin::route_drop))
        .route(
            "/server/network-settings-template/fetch",
            get(admin::network_templates_fetch),
        )
        .route(
            "/server/network-settings-template/save",
            post(admin::network_template_save),
        )
        .route(
            "/server/network-settings-template/drop",
            post(admin::network_template_drop),
        )
        .route(
            "/server/cert-template/fetch",
            get(admin::cert_templates_fetch),
        )
        .route(
            "/server/cert-template/save",
            post(admin::cert_template_save),
        )
        .route(
            "/server/cert-template/drop",
            post(admin::cert_template_drop),
        )
        // nodes
        .route("/server/manage/getNodes", get(admin::servers_get_nodes))
        .route(
            "/server/manage/get-sort-nodes",
            get(admin::server_sort_nodes),
        )
        .route("/server/manage/save", post(admin::server_save))
        .route("/server/manage/update", post(admin::server_update))
        .route("/server/manage/drop", post(admin::server_drop))
        .route("/server/manage/copy", post(admin::server_copy))
        .route("/server/manage/sort", post(admin::server_sort))
        .route(
            "/server/manage/batchDelete",
            post(admin::server_batch_delete),
        )
        .route("/server/manage/batchUpdate", post(admin::server_batch_update))
        .route("/server/manage/batchReplace", post(admin::server_batch_replace))
        .route(
            "/server/manage/resetTraffic",
            post(admin::server_reset_traffic),
        )
        .route(
            "/server/manage/batchResetTraffic",
            post(admin::server_batch),
        )
        .route("/server/manage/restart", post(admin::server_restart))
        .route("/server/manage/upgrade", post(admin::server_upgrade))
        .route("/server/manage/batchUpgrade", post(admin::server_upgrade))
        .route(
            "/server/manage/get-virtual-nodes/{id}",
            get(admin::server_get_virtual_nodes),
        )
        .route("/server/manage/save-virtual-nodes/{id}", post(admin::server_save_virtual_nodes))
        .route(
            "/server/manage/get-children/{id}",
            get(admin::server_children),
        )
        .route(
            "/server/manage/create-child-node",
            post(admin::server_create_child),
        )
        .route(
            "/server/manage/update-child-node",
            post(admin::server_update_child),
        )
        // protocol definitions
        .route(
            "/server/protocols/definitions",
            get(admin::protocol_definitions),
        )
        .route("/server/protocols/types", get(admin::protocol_types))
        .route(
            "/server/protocols/definition/{type}",
            get(admin::protocol_definition),
        )
        .route(
            "/server/protocols/definition/{type}/fields",
            get(admin::protocol_fields),
        )
        .route(
            "/server/protocols/definition/{type}/validation",
            get(admin::protocol_validation),
        )
        // machines
        .route("/server/machine/fetch", get(admin::machines_fetch))
        .route("/server/machine/save", post(admin::machine_save))
        .route("/server/machine/drop", post(admin::machine_drop))
        .route(
            "/server/machine/resetToken",
            post(admin::machine_reset_token),
        )
        .route("/server/machine/getToken", get(admin::machine_get_token))
        .route(
            "/server/machine/installCommand",
            get(admin::machine_install_command),
        )
        .route("/server/machine/nodes", get(admin::machine_nodes))
        .route(
            "/server/machine/available-nodes",
            get(admin::machine_available_nodes),
        )
        .route(
            "/server/machine/bind-nodes",
            post(admin::machine_bind_nodes),
        )
        .route(
            "/server/machine/unbind-node",
            post(admin::machine_unbind_node),
        )
        .route("/server/machine/history", get(admin::machine_history))
        .route("/server/machine/upgrade", post(admin::machine_upgrade))
        .route("/server/machine/restart", post(admin::machine_restart))
        .route("/server/machine/stop", post(admin::machine_stop))
        .route("/server/machine/start", post(admin::machine_start))
        .route("/server/machine/reload", post(admin::machine_reload))
        .route("/server/machine/batchUpgrade", post(admin::machine_batch_upgrade))
        .route("/server/machine/ops-ack", get(admin::machine_ops_ack))
        .route("/server/machine/logs", get(admin::machine_logs))
        // users
        .route("/user/fetch", post(admin_user::users_fetch))
        .route("/user/update", post(admin_user::user_update))
        .route("/user/getUserInfoById", get(admin_user::user_info_by_id))
        .route("/user/generate", post(admin_user::user_generate))
        .route("/user/dumpCSV", post(admin_user::user_dump_csv))
        .route("/user/sendMail", post(admin_user::user_send_mail))
        .route("/user/ban", post(admin_user::user_ban))
        .route("/user/resetSecret", post(admin_user::user_reset_secret))
        .route("/user/setInviteUser", post(admin_user::user_set_invite))
        .route("/user/inviteList", get(admin_user::user_invite_list))
        .route("/user/loginLogs", get(admin_user::user_login_logs))
        .route("/user/destroy", post(admin_user::user_destroy))
        // orders
        .route("/order/fetch", any(admin_user::orders_fetch))
        .route("/order/update", post(admin_user::order_update))
        .route("/order/assign", post(admin_user::order_assign))
        .route("/order/paid", post(admin_user::order_paid))
        .route("/order/cancel", post(admin_user::order_cancel))
        .route("/order/detail", post(admin_user::order_detail))
        .route(
            "/order/bulk-confirm-commission",
            post(admin_user::order_bulk_commission),
        )
        // stats
        .route("/stat/getStats", get(admin_content::stat_stats))
        .route("/stat/getOverride", get(admin_content::stat_override))
        .route(
            "/stat/getServerLastRank",
            get(admin_content::stat_last_rank),
        )
        .route(
            "/stat/getServerYesterdayRank",
            get(admin_content::stat_last_rank),
        )
        .route("/stat/getOrder", get(admin_content::stat_order))
        .route("/stat/getStatUser", any(admin_content::stat_user))
        .route("/stat/getRanking", get(admin_content::stat_ranking))
        .route("/stat/getStatRecord", get(admin_content::stat_record))
        .route("/stat/getTrafficRank", get(admin_content::stat_ranking))
        // notices
        .route("/notice/fetch", get(admin_content::notices_fetch))
        .route("/notice/detail", get(admin_content::notice_detail))
        .route("/notice/save", post(admin_content::notice_save))
        .route("/notice/update", post(admin_content::notice_save))
        .route("/notice/drop", post(admin_content::notice_drop))
        .route("/notice/show", post(admin_content::notice_show))
        .route("/notice/sort", post(admin_content::notice_sort))
        // tickets
        .route("/ticket/fetch", any(admin_content::tickets_fetch))
        .route("/ticket/reply", post(admin_content::ticket_reply))
        .route(
            "/ticket/message/update",
            post(admin_content::ticket_message_update),
        )
        .route(
            "/ticket/message/recall",
            post(admin_content::ticket_message_recall),
        )
        .route("/ticket/close", post(admin_content::ticket_close))
        .route("/ticket/bulk-close", post(admin_content::ticket_close))
        // withdrawals
        .route("/withdrawal/fetch", any(admin_content::withdrawals_fetch))
        .route(
            "/withdrawal/confirm",
            post(admin_content::withdrawal_confirm),
        )
        .route("/withdrawal/close", post(admin_content::withdrawal_close))
        .route(
            "/withdrawal/{id}/messages",
            get(admin_content::withdrawal_messages),
        )
        .route(
            "/withdrawal/{id}/reply",
            post(admin_content::withdrawal_reply),
        )
        // coupons
        .route("/coupon/fetch", any(admin_content::coupons_fetch))
        .route("/coupon/generate", post(admin_content::coupon_generate))
        .route("/coupon/drop", post(admin_content::coupon_drop))
        .route("/coupon/show", post(admin_content::coupon_show))
        .route("/coupon/update", post(admin_content::coupon_update))
        .route("/coupon/batchDrop", post(admin_content::coupon_batch_drop))
        .route(
            "/coupon/dropExpired",
            post(admin_content::coupon_drop_expired),
        )
        .route(
            "/coupon/dropDepleted",
            post(admin_content::coupon_drop_depleted),
        )
        // gift cards
        .route("/gift-card/templates", any(admin_content::gift_templates))
        .route(
            "/gift-card/create-template",
            post(admin_content::gift_template_save),
        )
        .route(
            "/gift-card/update-template",
            post(admin_content::gift_template_save),
        )
        .route(
            "/gift-card/delete-template",
            post(admin_content::gift_template_delete),
        )
        .route(
            "/gift-card/sort-templates",
            post(admin_content::gift_template_sort),
        )
        .route(
            "/gift-card/generate-codes",
            post(admin_content::gift_generate_codes),
        )
        .route("/gift-card/codes", any(admin_content::gift_codes))
        .route(
            "/gift-card/toggle-code",
            post(admin_content::gift_code_toggle),
        )
        .route("/gift-card/export-codes", get(admin_content::gift_codes_export))
        .route(
            "/gift-card/update-code",
            post(admin_content::gift_code_toggle),
        )
        .route(
            "/gift-card/delete-code",
            post(admin_content::gift_code_toggle),
        )
        .route("/gift-card/usages", any(admin_content::gift_usages))
        .route("/gift-card/statistics", any(admin_content::gift_statistics))
        .route("/gift-card/types", get(admin_content::gift_types))
        // knowledge
        .route("/knowledge/fetch", get(admin_content::knowledge_fetch))
        .route(
            "/knowledge/getCategory",
            get(admin_content::knowledge_categories),
        )
        .route("/knowledge/save", post(admin_content::knowledge_save))
        .route("/knowledge/show", post(admin_content::knowledge_show))
        .route("/knowledge/drop", post(admin_content::knowledge_drop))
        .route("/knowledge/sort", post(admin_content::knowledge_sort))
        // payments
        .route("/payment/fetch", get(admin_user::payments_fetch))
        .route(
            "/payment/getPaymentMethods",
            get(admin_user::payment_methods),
        )
        .route("/payment/getPaymentForm", post(admin_user::payment_form))
        .route("/payment/save", post(admin_user::payment_save))
        .route("/payment/drop", post(admin_user::payment_drop))
        .route("/payment/copy", post(admin_user::payment_copy))
        .route("/payment/show", post(admin_user::payment_show))
        .route("/payment/sort", post(admin_user::payment_sort))
        // system
        .route("/system/getSystemStatus", get(admin_content::system_status))
        .route(
            "/system/getQueueStats",
            get(admin_content::system_queue_stats),
        )
        .route(
            "/system/getQueueWorkload",
            get(admin_content::system_queue_workload),
        )
        .route(
            "/system/getHorizonFailedJobs",
            get(admin_content::system_failed_jobs),
        )
        .route("/system/getAuditLog", any(admin_content::system_audit_log))
        // plugins
        .route("/plugin/types", get(admin_content::plugin_types))
        .route("/plugin/getPlugins", get(admin_content::plugins_list))
        .route("/plugin/upload", post(admin_content::plugin_upload))
        .route("/plugin/delete", post(admin_content::plugin_delete))
        .route("/plugin/install", post(admin_content::plugin_install))
        .route("/plugin/uninstall", post(admin_content::plugin_uninstall))
        .route("/plugin/enable", post(admin_content::plugin_enable))
        .route("/plugin/disable", post(admin_content::plugin_disable))
        .route("/plugin/config", get(admin_content::plugin_config_get))
        .route("/plugin/config", post(admin_content::plugin_config_update))
        .route("/plugin/upgrade", post(admin_content::plugin_upgrade))
        .route("/plugin/readme", get(admin_content::plugin_readme))
        .route(
            "/plugin/staticFiles",
            get(admin_content::plugin_static_files),
        )
        .route("/plugin/ui", get(admin_content::plugin_ui))
        .route("/plugin/ui/nav", get(admin_content::plugin_ui_nav))
        .route("/plugin/action", post(admin_content::plugin_action))
        // traffic reset
        .route(
            "/traffic-reset/logs",
            get(admin_content::traffic_reset_logs),
        )
        .route(
            "/traffic-reset/stats",
            get(admin_content::traffic_reset_stats),
        )
        .route(
            "/traffic-reset/user/{userId}/history",
            get(admin_content::traffic_reset_user_history),
        )
        .route(
            "/traffic-reset/reset-user",
            post(admin_content::traffic_reset_user),
        )
}


async fn settings_js(State(state): State<SharedState>) -> Response {
    let settings = state.settings().await;
    let secure_path = settings.secure_path(&state.cfg.app_key);
    let payload = serde_json::json!({
        "base_url": "/",
        "title": settings.str("app_name", "Fboard"),
        "version": env!("CARGO_PKG_VERSION"),
        "logo": settings.str("logo", ""),
        "secure_path": secure_path,
    });
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            "application/javascript; charset=utf-8",
        )],
        format!(
            "window.settings = Object.assign({{}}, window.settings || {{}}, {});",
            payload
        ),
    )
        .into_response()
}

async fn settings_local_js() -> Response {
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            "application/javascript; charset=utf-8",
        )],
        "/* Fboard: local overrides (none) */",
    )
        .into_response()
}

async fn public_image(
    State(state): State<SharedState>,
    Path(path): Path<String>,
) -> Response {
    if path.contains("..") {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }
    let mut roots = vec![std::path::PathBuf::from("public/images")];
    roots.push(state.cfg.admin_dist.join("images"));
    for root in roots {
        let candidate = root.join(&path);
        match tokio::fs::read(&candidate).await {
            Ok(bytes) => {
                let mime = match candidate
                    .extension()
                    .and_then(|v| v.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase()
                    .as_str()
                {
                    "svg" => "image/svg+xml",
                    "png" => "image/png",
                    "jpg" | "jpeg" => "image/jpeg",
                    "gif" => "image/gif",
                    "webp" => "image/webp",
                    "ico" => "image/x-icon",
                    _ => "application/octet-stream",
                };
                return (StatusCode::OK, [("content-type", mime)], bytes).into_response();
            }
            Err(_) => continue,
        }
    }
    if path == "favicon.svg" {
        const FAVICON: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 64 64\"><rect width=\"64\" height=\"64\" rx=\"14\" fill=\"#18181b\"/><path d=\"M18 44V20h12a8 8 0 0 1 0 16h-6v8zm6-14h6a2 2 0 0 0 0-4h-6z\" fill=\"#fff\"/></svg>";
        return (
            StatusCode::OK,
            [("content-type", "image/svg+xml")],
            FAVICON,
        )
            .into_response();
    }
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

async fn admin_locale(
    State(state): State<SharedState>,
    Path(path): Path<String>,
) -> Response {
    if path.contains("..") || !path.ends_with(".js") {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }
    let candidate = state.cfg.admin_dist.join("locales").join(&path);
    match tokio::fs::read(&candidate).await {
        Ok(bytes) => (
            StatusCode::OK,
            [("content-type", "application/javascript; charset=utf-8")],
            bytes,
        )
            .into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}

async fn spa_index(State(state): State<SharedState>, uri: Uri) -> Response {
    let path = uri.path();
    if path.starts_with("/api/") || path.starts_with("/assets/") || path.starts_with("/plugins/") {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }
    let settings = state.settings().await;
    let secure_path = settings.secure_path(&state.cfg.app_key);
    let title = settings.str("app_name", "Fboard");
    let logo = settings.str("logo", "");
    let version = env!("CARGO_PKG_VERSION");
    let injection = format!(
        "<script>window.settings = {{ base_url: \"/\", title: {}, version: {}, logo: {}, secure_path: {} }};</script>",
        serde_json::to_string(&title).unwrap_or_else(|_| "\"Fboard\"".into()),
        serde_json::to_string(version).unwrap_or_else(|_| "\"0.1.0\"".into()),
        serde_json::to_string(&logo).unwrap_or_else(|_| "\"\"".into()),
        serde_json::to_string(&secure_path).unwrap_or_else(|_| "\"admin\"".into()),
    );

    let mut locale_tags = String::new();
    if let Ok(mut entries) = tokio::fs::read_dir(state.cfg.admin_dist.join("locales")).await {
        let mut files = Vec::new();
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".js") {
                    files.push(name.to_string());
                }
            }
        }
        files.sort();
        for file in files {
            locale_tags.push_str(&format!(
                "<script src=\"/assets/admin/locales/{file}\"></script>"
            ));
        }
    }

    let index = state.cfg.admin_dist.join("index.html");
    if let Ok(html) = tokio::fs::read_to_string(&index).await {
        if let Some(pos) = html.find("</head>") {
            let mut out = String::with_capacity(html.len() + injection.len() + locale_tags.len());
            out.push_str(&html[..pos]);
            out.push_str(&injection);
            out.push_str(&locale_tags);
            out.push_str(&html[pos..]);
            return Html(out).into_response();
        }
        return Html(html).into_response();
    }

    let placeholder = format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>{title}</title>{injection}</head><body style=\"font-family:sans-serif;padding:40px\"><h1>{title} · Rust core</h1><p>管理后台静态资源尚未构建。</p><p>请在此工作树中执行 <code>bun install && bun run build</code>（或设置 <code>FBOARD_ADMIN_DIST</code> 指向已构建的 admin 产物）。</p><p>API 基址：<code>/api/v2/{secure_path}</code></p></body></html>"
    );
    Html(placeholder).into_response()
}

async fn plugin_static(
    State(state): State<SharedState>,
    Path((code, path)): Path<(String, String)>,
) -> Response {
    if !code
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
    {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }
    if path.contains("..") {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }
    let allowed = [
        ".html", ".htm", ".css", ".js", ".mjs", ".jsx", ".ts", ".json", ".xml", ".svg",
        ".png", ".jpg", ".jpeg", ".gif", ".webp", ".bmp", ".ico", ".avif", ".woff",
        ".woff2", ".ttf", ".eot", ".otf", ".wasm", ".pdf", ".txt", ".md", ".map",
    ];
    let lower = path.to_lowercase();
    if !allowed.iter().any(|ext| lower.ends_with(ext)) {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }
    for dir in &state.cfg.plugin_dirs {
        let candidate = dir.join(&code).join("public").join(&path);
        if candidate.is_file() {
            return match tokio::fs::read(&candidate).await {
                Ok(bytes) => {
                    let mime = mime_for(&candidate);
                    ([(axum::http::header::CONTENT_TYPE, mime)], bytes).into_response()
                }
                Err(_) => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            };
        }
    }
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

fn mime_for(path: &std::path::Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "js" | "mjs" => "application/javascript",
        "css" => "text/css",
        "json" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        "woff" => "font/woff",
        "ttf" => "font/ttf",
        "wasm" => "application/wasm",
        "html" | "htm" => "text/html; charset=utf-8",
        _ => "application/octet-stream",
    }
}


async fn healthz() -> Response {
    (StatusCode::OK, "ok\n").into_response()
}

async fn readyz(State(state): State<SharedState>) -> Response {
    let db_ok = sqlx::query("SELECT 1").execute(&state.db).await.is_ok();
    if db_ok {
        (StatusCode::OK, "ready\n").into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "database unavailable\n",
        )
            .into_response()
    }
}

async fn metrics(State(state): State<SharedState>) -> Response {
    let users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v2_user")
        .fetch_one(&state.db)
        .await
        .unwrap_or(0);
    let body = format!(
        "fboard_up 1\n\
fboard_uptime_seconds {}\n\
fboard_database_up {}\n\
fboard_users_total {}\n\
fboard_plugins_total {}\n\
fboard_ws_connections {}\n\
fboard_redis_enabled {}\n",
        crate::util::now() - state.started_at,
        if sqlx::query("SELECT 1").execute(&state.db).await.is_ok() { 1 } else { 0 },
        users,
        state.plugin.all().len(),
        state.ws.connection_count(),
        if state.redis.is_some() { 1 } else { 0 },
    );
    (
        [(axum::http::header::CONTENT_TYPE, "text/plain; version=0.0.4")],
        body,
    )
        .into_response()
}

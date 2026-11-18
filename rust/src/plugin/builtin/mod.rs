pub mod config_output;
pub mod core_protocols;
pub mod output;
pub mod payment_alipay;
pub mod payment_btcpay;
pub mod payment_coinbase;
pub mod payment_coinpayments;
pub mod payment_epay;
pub mod payment_mgate;
pub mod payment_util;
pub mod telegram;

use std::sync::Arc;

use crate::plugin::NativePlugin;

pub fn plugins() -> Vec<Arc<dyn NativePlugin>> {
    vec![
        Arc::new(payment_epay::EpayPlugin),
        Arc::new(payment_alipay::AlipayF2fPlugin),
        Arc::new(payment_btcpay::BtcpayPlugin),
        Arc::new(payment_coinpayments::CoinPaymentsPlugin),
        Arc::new(payment_coinbase::CoinbasePlugin),
        Arc::new(payment_mgate::MgatePlugin),
        Arc::new(core_protocols::CoreProtocolsPlugin),
        Arc::new(output::GeneralPlugin),
        Arc::new(output::ShadowsocksPlugin),
        Arc::new(output::ClashPlugin { meta: false }),
        Arc::new(output::ClashPlugin { meta: true }),
        Arc::new(output::SingBoxPlugin),
        Arc::new(config_output::SurgePlugin),
        Arc::new(config_output::SurfboardPlugin),
        Arc::new(config_output::LoonPlugin),
        Arc::new(config_output::QuantumultXPlugin),
        Arc::new(config_output::StashPlugin),
        Arc::new(config_output::ShadowrocketPlugin),
        Arc::new(telegram::TelegramPlugin),
    ]
}

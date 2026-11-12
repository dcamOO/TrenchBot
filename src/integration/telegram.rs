use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Serialize;

pub struct Telegram {
    client: Client,
    bot_token: String,
    chat_id: String,
}

#[derive(Debug)]
pub enum TradeType {
    Buy,
    Sell,
}

impl TradeType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        }
    }
}

#[derive(Serialize)]
struct SendMessage<'a> {
    chat_id: &'a str,
    text: &'a str,
}

#[derive(Debug, serde::Deserialize)]
struct TelegramResponse {
    ok: bool,
    description: Option<String>,
}

impl Telegram {
    pub fn new(bot_token: String, chat_id: String) -> Self {
        Self {
            client: Client::new(),
            bot_token,
            chat_id,
        }
    }

    pub fn from_env() -> Result<Self> {
        let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
            .context("TELEGRAM_BOT_TOKEN not set")?;

        let chat_id = std::env::var("TELEGRAM_CHAT_ID")
            .context("TELEGRAM_CHAT_ID not set")?;

        Ok(Self::new(bot_token, chat_id))
    }

    pub fn send_message(&self, message: &str) -> Result<()> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.bot_token
        );

        let body = SendMessage {
            chat_id: &self.chat_id,
            text: message,
        };

        let response = self
            .client
            .post(url)
            .json(&body)
            .send()?
            .error_for_status()?
            .json::<TelegramResponse>()?;

        if !response.ok {
            anyhow::bail!(
                "Telegram API error: {}",
                response
                    .description
                    .unwrap_or_else(|| "unknown error".to_string())
            );
        }

        Ok(())
    }
    pub fn send_trade_alert(
        &self,
        side: TradeType,
        ticker: &str,
        quantity: f64,
        price_usd: f64,
        tx_signature: &str,
    ) -> Result<()> {
        let message = format_trade_alert(
            &side,
            ticker,
            quantity,
            price_usd,
            tx_signature,
        );

        self.send_message(&message)
    }

}

fn format_trade_alert(
    side: &TradeType,
    ticker: &str,
    quantity: f64,
    price_usd: f64,
    tx_signature: &str,
) -> String {
    let total_usd = quantity * price_usd;

    format!(
        "Trade executed\n\n\
         Side: {}\n\
         Token: {}\n\
         Quantity: {:.6}\n\
         Price: ${:.8}\n\
         Total: ${:.2}\n\
         Tx: {}",
        side.as_str(),
        ticker,
        quantity,
        price_usd,
        total_usd,
        tx_signature,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn format_buy_alert() -> String {
        format_trade_alert(
            &TradeType::Buy,
            "SOL",
            2.5,
            150.0,
            "signature123",
        )
    }

    #[test]
    fn trade_type_buy_as_str() {
        assert_eq!(TradeType::Buy.as_str(), "BUY");
    }

    #[test]
    fn trade_type_sell_as_str() {
        assert_eq!(TradeType::Sell.as_str(), "SELL");
    }

    #[test]
    fn format_buy_trade_alert() {
        let message = format_buy_alert();

        let expected = "\
Trade executed

Side: BUY
Token: SOL
Quantity: 2.500000
Price: $150.00000000
Total: $375.00
Tx: signature123";

        assert_eq!(message, expected);
    }

    #[test]
    fn format_sell_trade_alert() {
        let message = format_trade_alert(
            &TradeType::Sell,
            "BONK",
            1000.0,
            0.000025,
            "signature456",
        );

        let expected = "\
Trade executed

Side: SELL
Token: BONK
Quantity: 1000.000000
Price: $0.00002500
Total: $0.03
Tx: signature456";

        assert_eq!(message, expected);
    }

    #[test]
    fn format_trade_alert_with_zero_values() {
        let message = format_trade_alert(
            &TradeType::Buy,
            "SOL",
            0.0,
            0.0,
            "signature789",
        );

        assert!(message.contains("Quantity: 0.000000"));
        assert!(message.contains("Price: $0.00000000"));
        assert!(message.contains("Total: $0.00"));
    }
}

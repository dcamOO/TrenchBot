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
        let total_usd = quantity * price_usd;

        let message = format!(
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
        );

        self.send_message(&message)
    }
}

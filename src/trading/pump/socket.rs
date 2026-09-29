use anyhow::{Result, anyhow};
use std::{collections::HashSet, net::TcpStream};
use tungstenite::{Message, WebSocket, connect, stream::MaybeTlsStream};

pub struct Feed {
    pub socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl Feed {
    pub fn connect(api_key: &str, positions: &HashSet<String>) -> Result<Self> {
        let mut url = reqwest::Url::parse("wss://pumpportal.fun/api/data")?;
        url.query_pairs_mut().append_pair("api-key", api_key);
        // Do not expose the credential-bearing URL in errors or logs.
        let (socket, _) =
            connect(url.as_str()).map_err(|_| anyhow!("falha ao conectar ao PumpPortal"))?;
        let mut feed = Self { socket };
        feed.subscribe("subscribeNewToken", &[])?;
        for chunk in positions.iter().cloned().collect::<Vec<_>>().chunks(5000) {
            feed.subscribe("subscribeTokenTrade", chunk)?;
        }
        Ok(feed)
    }

    pub fn subscribe(&mut self, method: &str, keys: &[String]) -> Result<()> {
        let mut payload = serde_json::json!({"method": method});
        if !keys.is_empty() {
            payload["keys"] = serde_json::json!(keys);
        }
        self.socket
            .send(Message::Text(payload.to_string().into()))?;
        Ok(())
    }
}

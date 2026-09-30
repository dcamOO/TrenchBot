pub use TrenchBot::trading::archive::{
    decode::{PUMP, deployments},
    model::{Archive, Deployment, Progress},
    scan::scan,
    store::Store,
};
pub use serde_json::{Value, json};
use std::collections::HashMap;

pub fn key(byte: u8) -> String {
    bs58::encode([byte; 32]).into_string()
}

pub fn instruction(mint: u8, v2: bool) -> Value {
    let mut accounts = vec![key(9); 14];
    accounts[0] = key(mint);
    accounts[if v2 { 5 } else { 7 }] = key(1);
    let bytes = if v2 {
        [214, 144, 76, 236, 95, 139, 49, 180]
    } else {
        [24, 30, 200, 40, 5, 28, 7, 119]
    };
    json!({"programId":PUMP, "accounts":accounts, "data":bs58::encode(bytes).into_string()})
}

pub fn transaction(signature: &str, mint: Option<u8>) -> Value {
    let instructions: Vec<Value> = mint.map(|m| instruction(m, false)).into_iter().collect();
    json!({"blockTime":100, "meta":{"err":null,"innerInstructions":[]},
        "transaction":{"signatures":[signature],"message":{"instructions":instructions}}})
}

pub struct Mock {
    pub order: Vec<String>,
    pub transactions: HashMap<String, Value>,
    pub calls: Vec<String>,
    pub page_size: usize,
}

impl Mock {
    pub fn new(items: &[(&str, Option<u8>)]) -> Self {
        Self {
            order: items.iter().map(|(s, _)| s.to_string()).collect(),
            transactions: items
                .iter()
                .map(|(s, m)| (s.to_string(), transaction(s, *m)))
                .collect(),
            calls: vec![],
            page_size: 2,
        }
    }
}

impl Archive for Mock {
    fn signatures(&mut self, _: &str, before: Option<&str>) -> anyhow::Result<Vec<String>> {
        self.calls.push(format!("page:{before:?}"));
        let start = match before {
            Some(before) => self.order.iter().position(|s| s == before).unwrap() + 1,
            None => 0,
        };
        Ok(self
            .order
            .iter()
            .skip(start)
            .take(self.page_size)
            .cloned()
            .collect())
    }

    fn transaction(&mut self, signature: &str) -> anyhow::Result<Value> {
        self.calls.push(signature.into());
        Ok(self
            .transactions
            .get(signature)
            .cloned()
            .unwrap_or(Value::Null))
    }
}

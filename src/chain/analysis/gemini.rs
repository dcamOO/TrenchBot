use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::processing::ProcessedInfo;

const GEMINI_API_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models";

const DEFAULT_MODEL: &str = "gemini-3.6-flash";

pub(crate) struct Gemini {
    client: Client,
    api_key: String,
    model: String,
}

impl Gemini {
    pub(crate) fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: DEFAULT_MODEL.to_string(),
        }
    }

    pub(crate) fn analyze(
        &self,
        info: &ProcessedInfo,
    ) -> Result<String> {
        let data = serde_json::to_string_pretty(info)?;

        let prompt = format!(
            r#"
You are analyzing a Solana token for security and scam-risk indicators.

The data below was collected independently from the Solana blockchain
and external token/market APIs.

Your task is to interpret the evidence and identify indicators that may
be consistent with:
- rug-pull risk
- excessive token concentration
- dangerous authorities
- suspicious Token-2022 extensions
- problematic metadata control
- unusual trading or liquidity conditions
- other relevant security concerns

Do not invent information that is not present in the data.

Distinguish clearly between:
1. directly observed facts
2. reasonable interpretations
3. things that cannot be determined from the available data

Do not treat a single indicator as proof of malicious intent.
Do not give financial advice or tell the user to buy or sell the token.

Return a concise technical assessment.

Token data:

{data}
"#,
        );

        let body = json!({
            "contents": [
                {
                    "parts": [
                        {
                            "text": prompt
                        }
                    ]
                }
            ]
        });

        let url = format!(
            "{}/{model}:generateContent",
            GEMINI_API_URL,
            model = self.model,
        );

        let response = self
            .client
            .post(url)
            .header("x-goog-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()?
            .error_for_status()
            .context("Gemini API request failed")?;

        let response = response.json::<GeminiResponse>()?;

        response
            .candidates
            .into_iter()
            .next()
            .and_then(|candidate| {
                candidate
                    .content
                    .parts
                    .into_iter()
                    .next()
                    .map(|part| part.text)
            })
            .context("Gemini returned no text response")
    }
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Debug, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
struct Part {
    text: String,
}

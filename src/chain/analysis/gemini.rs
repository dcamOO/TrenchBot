
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{json, Value};

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
        let prompt = build_prompt(info)?;
        let body = build_request_body(&prompt);

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

        extract_response_text(response)
    }
}

fn build_prompt(info: &ProcessedInfo) -> Result<String> {
    let data = serde_json::to_string_pretty(info)?;

    Ok(format!(
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
"#
    ))
}

fn build_request_body(prompt: &str) -> Value {
    json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": prompt
                    }
                ]
            }
        ]
    })
}

fn extract_response_text(response: GeminiResponse) -> Result<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_response(json: Value) -> GeminiResponse {
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn build_request_body_contains_prompt() {
        let prompt = "Analyze this token";

        let body = build_request_body(prompt);

        assert_eq!(
            body["contents"][0]["parts"][0]["text"],
            prompt
        );
    }

    #[test]
    fn build_request_body_has_expected_structure() {
        let body = build_request_body("Test prompt");

        assert!(body["contents"].is_array());
        assert_eq!(body["contents"].as_array().unwrap().len(), 1);

        assert!(body["contents"][0]["parts"].is_array());
        assert_eq!(
            body["contents"][0]["parts"].as_array().unwrap().len(),
            1
        );
    }

    #[test]
    fn extract_response_text_returns_first_candidate() {
        let response = make_response(json!({
            "candidates": [
                {
                    "content": {
                        "parts": [
                            { "text": "Token assessment" }
                        ]
                    }
                },
                {
                    "content": {
                        "parts": [
                            { "text": "Second assessment" }
                        ]
                    }
                }
            ]
        }));

        let text = extract_response_text(response).unwrap();

        assert_eq!(text, "Token assessment");
    }

    #[test]
    fn extract_response_text_returns_first_part() {
        let response = make_response(json!({
            "candidates": [
                {
                    "content": {
                        "parts": [
                            { "text": "First part" },
                            { "text": "Second part" }
                        ]
                    }
                }
            ]
        }));

        let text = extract_response_text(response).unwrap();

        assert_eq!(text, "First part");
    }

    #[test]
    fn extract_response_text_fails_without_candidates() {
        let response = make_response(json!({
            "candidates": []
        }));

        let result = extract_response_text(response);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Gemini returned no text response"
        );
    }

    #[test]
    fn extract_response_text_fails_without_parts() {
        let response = make_response(json!({
            "candidates": [
                {
                    "content": {
                        "parts": []
                    }
                }
            ]
        }));

        let result = extract_response_text(response);

        assert!(result.is_err());
    }

    #[test]
    fn deserialize_gemini_response() {
        let json = json!({
            "candidates": [
                {
                    "content": {
                        "parts": [
                            { "text": "Security assessment" }
                        ]
                    }
                }
            ]
        });

        let response: GeminiResponse =
            serde_json::from_value(json).unwrap();

        assert_eq!(response.candidates.len(), 1);
        assert_eq!(
            response.candidates[0].content.parts[0].text,
            "Security assessment"
        );
    }

    #[test]
    fn deserialize_invalid_response_fails() {
        let json = json!({
            "candidates": [
                {
                    "content": {
                        "parts": [{}]
                    }
                }
            ]
        });

        let result = serde_json::from_value::<GeminiResponse>(json);

        assert!(result.is_err());
    }
}

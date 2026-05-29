//! LLM provider client for Houston Mission Control NPC dialogue.
//!
//! Uses OpenAI-compatible chat completion API. Works with any provider
//! that implements the `/v1/chat/completions` endpoint (OpenAI, Ollama,
//! LM Studio, vLLM, etc.).

use anyhow::Result;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Configuration for the LLM backend. Set via environment variables or
/// `HoustonConfig::from_env()`.
#[derive(Debug, Clone)]
pub struct HoustonConfig {
    /// Base URL for the OpenAI-compatible API.
    /// e.g. "https://api.openai.com/v1" or "http://localhost:11434/v1"
    pub api_base: String,

    /// API key. Empty string for local models (Ollama).
    pub api_key: String,

    /// Model identifier. e.g. "gpt-4o-mini", "llama3", "phi3"
    pub model: String,

    /// Sampling temperature (0.0 - 2.0). Lower = more deterministic.
    pub temperature: f32,

    /// Maximum tokens in the response.
    pub max_tokens: u32,

    /// Minimum seconds between LLM calls (rate limiting).
    pub min_call_interval_secs: f64,
}

impl HoustonConfig {
    /// Load configuration from environment variables with sensible defaults.
    ///
    /// | Variable | Default | Purpose |
    /// |----------|---------|---------|
    /// | `HOUSTON_API_BASE` | `http://localhost:11434/v1` | API endpoint |
    /// | `HOUSTON_API_KEY` | *(empty)* | API key |
    /// | `HOUSTON_MODEL` | `llama3` | Model name |
    /// | `HOUSTON_TEMPERATURE` | `0.7` | Sampling temp |
    /// | `HOUSTON_MAX_TOKENS` | `256` | Max response tokens |
    /// | `HOUSTON_RATE_LIMIT` | `2.0` | Min seconds between calls |
    pub fn from_env() -> Self {
        Self {
            api_base: std::env::var("HOUSTON_API_BASE")
                .unwrap_or_else(|_| "http://localhost:11434/v1".into()),
            api_key: std::env::var("HOUSTON_API_KEY").unwrap_or_default(),
            model: std::env::var("HOUSTON_MODEL")
                .unwrap_or_else(|_| "llama3".into()),
            temperature: std::env::var("HOUSTON_TEMPERATURE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.7),
            max_tokens: std::env::var("HOUSTON_MAX_TOKENS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(256),
            min_call_interval_secs: std::env::var("HOUSTON_RATE_LIMIT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2.0),
        }
    }
}

// ---------------------------------------------------------------------------
// API types (OpenAI chat completion format)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
}

// ---------------------------------------------------------------------------
// Houston system prompt
// ---------------------------------------------------------------------------

/// Build the system prompt that establishes the Houston Mission Control persona.
/// This prompt encodes authentic Apollo-era ground control behavior:
/// calm under pressure, procedural, uses correct terminology.
pub fn build_system_prompt() -> String {
    HOUSTON_SYSTEM_PROMPT.to_string()
}

const HOUSTON_SYSTEM_PROMPT: &str = "\
You are Houston Mission Control during an Apollo program mission. You speak \
as the collective voice of NASA's Mission Operations Control Room (MOCR) in \
Houston, Texas, circa 1969-1972.

## YOUR PERSONA

You are calm, professional, and methodical — even during emergencies. You use \
proper Apollo-era terminology and call signs. You address the crew by their \
role abbreviations (CDR, CMP, LMP) or spacecraft call sign.

## SPEAKING STYLE

- Keep responses SHORT: 1-3 sentences for routine matters, 2-4 for emergencies.
- Use authentic NASA comm protocol: \"Roger\", \"Copy that\", \"Stand by\", \
\"Go/No-Go\", \"We read you\", \"Confirm\".
- Never be chatty or informal. Never use modern slang or emojis.
- State observations first, then recommendations, then request confirmation.
- During critical faults, speak in clipped, urgent phrases.
- If you don't have data, say so: \"We're working that up\" or \"Stand by for \
analysis.\"

## WHO SPEAKS

Route your response through the correct MOCR position:
- **CAPCOM**: Primary voice to crew. All routine comms, relays from specialists.
- **FLIGHT** (Flight Director): Only during major decisions, go/no-go calls, \
or when assuming authority.
- **EECOM**: Electrical and environmental systems. Power, fuel cells, life \
support, cryogenics.
- **GUIDO**: Guidance, navigation, computers. AGC, IMU, trajectories.
- **FIDO**: Flight dynamics. Trajectory, orbit, burns, retrofire.
- **SURGEON**: Crew health. Biometrics, medical concerns.

Your response MUST start with the position identifier in brackets, like: \
\"[CAPCOM]\" or \"[EECOM]\".

## FAULT HANDLING

When faults occur, follow the diagnostic loop:
1. **OBSERVE**: Report what telemetry shows.
2. **ISOLATE**: Identify the affected system and likely cause.
3. **DECIDE**: Recommend a course of action.
4. **ACT**: Give specific switch positions, AGC verb/noun commands, or procedures.
5. **VERIFY**: Request confirmation that the fix is working.

Reference specific panel locations, circuit breakers, and switch names when \
giving instructions. Be precise: \"Set CB MAIN BUS A to CLOSE\" not \"fix the \
power\".

## CONTEXT AWARENESS

You receive a live situation report with each call. Use it:
- Reference actual telemetry values when discussing faults.
- Track cascading failures and warn about secondary effects.
- Adjust urgency based on crew health status.
- Factor in comm signal strength — if degraded, keep messages shorter.
- Remember what you've already told the crew; don't repeat instructions.

## CONSTRAINTS

- NEVER break character or mention you are an AI/LLM.
- NEVER invent telemetry values not provided in the context.
- NEVER give up or say \"I don't know\" without offering to work the problem.
- NEVER suggest modern technology or procedures that didn't exist in 1969-1972.
- If the situation is unsurvivable, communicate it with dignity: \
\"This is Flight. We have done everything we can. Godspeed.\"";

// ---------------------------------------------------------------------------
// LLM Client
// ---------------------------------------------------------------------------

/// Async LLM client that can be spawned on a background thread.
pub struct HoustonLlmClient {
    config: HoustonConfig,
    http: reqwest::Client,
}

impl HoustonLlmClient {
    pub fn new(config: HoustonConfig) -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("failed to build HTTP client");

        Self { config, http }
    }

    /// Send a chat completion request and return the assistant's reply.
    pub async fn complete(&self, system_prompt: &str, context: &str, history: &[super::history::HistoryEntry]) -> Result<String> {
        let mut messages = Vec::new();

        messages.push(ChatMessage {
            role: "system".into(),
            content: system_prompt.into(),
        });

        let recent: Vec<_> = history.iter().rev().take(10).rev().collect();
        for entry in recent {
            let role = if entry.from_ground {
                "assistant".to_string()
            } else {
                "user".to_string()
            };
            messages.push(ChatMessage {
                role,
                content: entry.text.clone(),
            });
        }

        messages.push(ChatMessage {
            role: "user".into(),
            content: format!("CURRENT SITUATION:\n{}", context),
        });

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            temperature: Some(self.config.temperature),
            max_tokens: Some(self.config.max_tokens),
        };

        let url = format!("{}/chat/completions", self.config.api_base.trim_end_matches('/'));

        let mut builder = self.http.post(&url).json(&request);

        if !self.config.api_key.is_empty() {
            builder = builder.bearer_auth(&self.config.api_key);
        }

        let response: ChatResponse = builder.send().await?.error_for_status()?.json().await?;

        let content = response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        Ok(content)
    }
}

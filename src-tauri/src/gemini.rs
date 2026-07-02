use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use base64::Engine;

use crate::profile::Profile;

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
pub const DEFAULT_MODEL: &str = "gemini-2.5-flash";

pub const FALLBACK_MODELS: &[&str] = &[
    "gemini-2.5-flash",
    "gemini-2.0-flash",
    "gemini-2.5-flash-lite",
    "gemini-2.0-flash-lite",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub free: bool,
    pub recommended: bool,
}

pub fn list_available_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "gemini-2.5-flash".to_string(),
            name: "Gemini 2.5 Flash".to_string(),
            description: "Fast and smart. Best balance of speed and quality. Free tier.".to_string(),
            free: true,
            recommended: true,
        },
        ModelInfo {
            id: "gemini-2.5-flash-lite".to_string(),
            name: "Gemini 2.5 Flash Lite".to_string(),
            description: "Lighter and faster than Flash. Good for simple tasks. Free tier.".to_string(),
            free: true,
            recommended: false,
        },
        ModelInfo {
            id: "gemini-2.0-flash".to_string(),
            name: "Gemini 2.0 Flash".to_string(),
            description: "Older but very stable. Great fallback when 2.5 is overloaded. Free tier.".to_string(),
            free: true,
            recommended: false,
        },
        ModelInfo {
            id: "gemini-2.0-flash-lite".to_string(),
            name: "Gemini 2.0 Flash Lite".to_string(),
            description: "Lightest model. Last resort fallback. Free tier.".to_string(),
            free: true,
            recommended: false,
        },
        ModelInfo {
            id: "gemini-2.5-pro".to_string(),
            name: "Gemini 2.5 Pro".to_string(),
            description: "Most capable model. Best quality but slower. Requires paid billing enabled.".to_string(),
            free: false,
            recommended: false,
        },
        ModelInfo {
            id: "gemini-2.0-pro".to_string(),
            name: "Gemini 2.0 Pro".to_string(),
            description: "Older pro model. High quality. Requires paid billing enabled.".to_string(),
            free: false,
            recommended: false,
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    #[serde(default)]
    pub content: Option<Content>,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(default)]
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    #[serde(default)]
    pub text: Option<String>,
}

pub struct GeminiClient {
    api_key: String,
    model: String,
    client: Client,
}

impl GeminiClient {
    pub fn new(api_key: &str, model: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");
        GeminiClient {
            api_key: api_key.to_string(),
            model: if model.is_empty() { DEFAULT_MODEL.to_string() } else { model.to_string() },
            client,
        }
    }

    pub async fn extract_profile_from_pdf(&self, pdf_path: &str, prompt: &str) -> Result<Profile, String> {
        let bytes = std::fs::read(pdf_path).map_err(|e| format!("Read PDF error: {}", e))?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);

        if bytes.len() > 20 * 1024 * 1024 {
            return Err("PDF is larger than 20MB. Please use a smaller file.".to_string());
        }

        let body = json!({
            "contents": [{
                "role": "user",
                "parts": [
                    { "text": prompt },
                    { "inlineData": { "mimeType": "application/pdf", "data": encoded } }
                ]
            }],
            "generationConfig": {
                "responseMimeType": "application/json",
                "temperature": 0.1
            }
        });

        let profile = self.generate_json(body).await?;
        Ok(profile)
    }

    pub async fn generate_latex_from_source(
        &self,
        original_latex: &str,
        jd: &str,
        considerations: &[String],
    ) -> Result<String, String> {
        let considerations_text = if considerations.is_empty() {
            "No specific considerations.".to_string()
        } else {
            considerations
                .iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let system_instruction = format!(
            "You are a LaTeX resume writer. Output ONLY raw LaTeX code — no markdown, no explanations, no code fences.\n\n\
            RULES:\n\
            - Output the COMPLETE .tex file and nothing else\n\
            - Every section must be complete and syntactically valid LaTeX\n\
            - Keep the EXACT same \\documentclass, ALL \\usepackage lines, and ALL custom command definitions (\\newcommand, \\def, etc.)\n\
            - Keep the EXACT same section structure and formatting commands (\\section, \\resumeSubheading, \\resumeItem, etc.)\n\
            - ONLY change the CONTENT — text inside commands, bullet points, which projects to include, which skills to list\n\
            - The document MUST compile with pdfLaTeX\n\
            - Do NOT invent new experience, projects, or skills that aren't in the original resume\n\
            - Keep the resume to 1 page — remove less relevant content if needed\n\
            - Max 3 bullets per project/role (2 if tight on space)\n\n\
            PROJECT SELECTION RULES (CRITICAL):\n\
            - ANALYZE every project in the original LaTeX resume\n\
            - RANK them by relevance to the job description\n\
            - Pick ONLY the 2-3 MOST relevant projects for this specific JD\n\
            - REMOVE all other projects entirely\n\
            - Example: If JD is for AI/ML role, pick AI/ML projects and REMOVE web dev projects\n\
            - Example: If JD is for backend role, pick backend/API projects and REMOVE frontend projects\n\
            - Rewrite bullet points to match JD focus (backend JD = emphasize APIs/DB/Docker, AI JD = emphasize models/NLP/LLM)\n\n\
            WORK EXPERIENCE RULES:\n\
            - Keep the same job titles, companies, and dates\n\
            - Rewrite bullet points to emphasize skills relevant to the JD\n\
            - Use action verbs. Quantify where possible.\n\n\
            SKILLS RULES:\n\
            - Keep ONLY skills relevant to the JD\n\
            - Remove irrelevant skills entirely\n\
            - Keep the same category format\n\
            - Reorder to put most JD-relevant skills first\n\n\
            SECTIONS THAT CHANGE: Summary (rewrite for JD), Work Experience bullets (rewrite), Projects (pick 2-3 most relevant + rewrite), Skills (filter for JD)\n\
            SECTIONS THAT STAY THE SAME: Contact info, Education, LaTeX preamble, custom commands, \\documentclass\n\n\
            CANDIDATE'S ORIGINAL LATEX RESUME:\n{}",
            original_latex
        );

        let user_prompt = format!(
            "Customize this resume for the job description below.\n\
            Output ONLY the complete .tex file. No markdown. No explanation. No code fences.\n\n\
            JOB DESCRIPTION:\n{}\n\n\
            CONSIDERATIONS (must address these):\n{}",
            jd, considerations_text
        );

        let body = json!({
            "contents": [{
                "role": "user",
                "parts": [{ "text": user_prompt }]
            }],
            "system_instruction": {
                "parts": [{ "text": system_instruction }]
            },
            "generationConfig": {
                "temperature": 0.2,
                "maxOutputTokens": 8192
            }
        });

        let text = self.generate_text(body).await?;
        let cleaned = clean_latex_response(&text);

        if !cleaned.contains("\\documentclass") {
            return Err("AI output invalid: missing \\documentclass. Try again.".to_string());
        }
        if !cleaned.contains("\\begin{document}") {
            return Err("AI output invalid: missing \\begin{{document}}. Try again.".to_string());
        }
        if !cleaned.contains("\\end{document}") {
            return Err("AI output invalid: missing \\end{{document}}. Try again.".to_string());
        }

        Ok(cleaned)
    }

    pub async fn generate_latex_from_profile(
        &self,
        profile: &Profile,
        jd: &str,
        considerations: &[String],
        template_source: &str,
    ) -> Result<String, String> {
        let profile_json = serde_json::to_string_pretty(profile)
            .map_err(|e| format!("Profile serialize error: {}", e))?;

        let considerations_text = if considerations.is_empty() {
            "No specific considerations.".to_string()
        } else {
            considerations
                .iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let system_instruction = format!(
            "You are a LaTeX resume writer. Output ONLY raw LaTeX code — no markdown, no explanations, no code fences.\n\n\
            RULES:\n\
            - Output the COMPLETE .tex file and nothing else\n\
            - Use the template structure EXACTLY as provided — same \\documentclass, \\usepackage, and custom commands\n\
            - ONLY fill in the content with tailored information from the candidate's profile\n\
            - The document MUST compile with pdfLaTeX\n\
            - Do NOT invent experience, projects, or skills not in the candidate's profile\n\
            - Keep the resume to 1 page — remove less relevant content if needed\n\
            - Max 3 bullets per project/role (2 if tight on space)\n\n\
            PROJECT SELECTION RULES (CRITICAL):\n\
            - ANALYZE every project in the candidate's profile\n\
            - RANK them by relevance to the job description\n\
            - Pick ONLY the 2-3 MOST relevant projects for this specific JD\n\
            - REMOVE all other projects entirely\n\
            - Example: If JD is for AI/ML role, pick AI/ML projects and REMOVE web dev projects\n\
            - Example: If JD is for backend role, pick backend/API projects and REMOVE frontend projects\n\
            - Rewrite bullet points to match JD focus (backend JD = emphasize APIs/DB/Docker, AI JD = emphasize models/NLP/LLM)\n\n\
            WORK EXPERIENCE RULES:\n\
            - Include most relevant roles. Rewrite bullets to emphasize JD-relevant skills.\n\
            - Use action verbs. Quantify where possible.\n\n\
            SKILLS RULES:\n\
            - Keep ONLY skills relevant to the JD. Remove irrelevant skills entirely.\n\
            - Reorder to put most JD-relevant skills first.\n\n\
            CANDIDATE PROFILE (JSON):\n{}\n\n\
            TEMPLATE (use this exact structure):\n{}",
            profile_json, template_source
        );

        let user_prompt = format!(
            "Customize this resume for the job description below.\n\
            Output ONLY the complete .tex file. No markdown. No explanation. No code fences.\n\n\
            JOB DESCRIPTION:\n{}\n\n\
            CONSIDERATIONS (must address these):\n{}",
            jd, considerations_text
        );

        let body = json!({
            "contents": [{
                "role": "user",
                "parts": [{ "text": user_prompt }]
            }],
            "system_instruction": {
                "parts": [{ "text": system_instruction }]
            },
            "generationConfig": {
                "temperature": 0.2,
                "maxOutputTokens": 8192
            }
        });

        let text = self.generate_text(body).await?;
        let cleaned = clean_latex_response(&text);

        if !cleaned.contains("\\documentclass") {
            return Err("AI output invalid: missing \\documentclass. Try again.".to_string());
        }
        if !cleaned.contains("\\end{document}") {
            return Err("AI output invalid: missing \\end{{document}}. Try again.".to_string());
        }

        Ok(cleaned)
    }

    pub async fn test_connection(&self) -> Result<bool, String> {
        let body = json!({
            "contents": [{
                "role": "user",
                "parts": [{ "text": "Say OK" }]
            }],
            "generationConfig": {
                "maxOutputTokens": 10
            }
        });

        let url = format!("{}/models/{}:generateContent?key={}", BASE_URL, self.model, self.api_key);
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Connection error: {}", e))?;

        Ok(resp.status().is_success())
    }

    async fn generate_json(&self, body: serde_json::Value) -> Result<Profile, String> {
        let models_to_try: Vec<String> = {
            let mut list = vec![self.model.clone()];
            for m in FALLBACK_MODELS {
                if *m != self.model {
                    list.push(m.to_string());
                }
            }
            list
        };

        let user_prompt = body
            .get("contents")
            .and_then(|c| c.as_array())
            .and_then(|a| a.first())
            .and_then(|entry| entry.get("parts"))
            .and_then(|p| p.as_array())
            .and_then(|parts| parts.first())
            .and_then(|part| part.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let gen_config = body.get("generationConfig").cloned().unwrap_or(json!({}));

        let mut last_error = String::new();

        for (i, model) in models_to_iter(&models_to_try).enumerate() {
            if i > 0 {
                tokio::time::sleep(Duration::from_secs(2)).await;
            }

            let url = format!("{}/models/{}:generateContent?key={}", BASE_URL, model, self.api_key);

            let request_body = json!({
                "contents": [{
                    "role": "user",
                    "parts": [{ "text": user_prompt }]
                }],
                "generationConfig": gen_config
            });

            let resp = match self.client.post(&url).json(&request_body).send().await {
                Ok(r) => r,
                Err(e) => {
                    last_error = format!("Connection error: {}", e);
                    continue;
                }
            };

            let status = resp.status();

            if status.as_u16() == 503 || status.as_u16() == 429 {
                last_error = format!("{} overloaded (HTTP {}). Trying next model...", model, status);
                let _ = resp.text().await;
                continue;
            }

            if !status.is_success() {
                let body_text = resp.text().await.unwrap_or_default();
                if body_text.contains("not found") || body_text.contains("not supported") {
                    last_error = format!("Model {} not available", model);
                    continue;
                }
                return Err(format!("Gemini API error: {}", body_text));
            }

            let gemini_resp: GeminiResponse = match resp.json().await {
                Ok(r) => r,
                Err(e) => {
                    last_error = format!("Parse error: {}", e);
                    continue;
                }
            };

            let text = gemini_resp
                .candidates
                .first()
                .and_then(|c| c.content.as_ref())
                .and_then(|c| c.parts.first())
                .and_then(|p| p.text.as_ref())
                .ok_or("No text in Gemini response")?;

            let profile: Profile = serde_json::from_str(text)
                .map_err(|e| format!("Parse profile JSON error: {}. Response: {}", e, &text[..text.len().min(500)]))?;

            return Ok(profile);
        }

        Err(format!("All Gemini models failed. Last error: {}", last_error))
    }

    async fn generate_text(&self, body: serde_json::Value) -> Result<String, String> {
        let models_to_try: Vec<String> = {
            let mut list = vec![self.model.clone()];
            for m in FALLBACK_MODELS {
                if *m != self.model {
                    list.push(m.to_string());
                }
            }
            list
        };

        let user_prompt = body
            .get("contents")
            .and_then(|c| c.as_array())
            .and_then(|a| a.first())
            .and_then(|entry| entry.get("parts"))
            .and_then(|p| p.as_array())
            .and_then(|parts| parts.first())
            .and_then(|part| part.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let system_instruction = body.get("system_instruction").cloned();

        let mut last_error = String::new();

        for (i, model) in models_to_iter(&models_to_try).enumerate() {
            if i > 0 {
                tokio::time::sleep(Duration::from_secs(2)).await;
            }

            let url = format!("{}/models/{}:generateContent?key={}", BASE_URL, model, self.api_key);

            let mut request_body = json!({
                "contents": [{
                    "role": "user",
                    "parts": [{ "text": user_prompt }]
                }],
                "generationConfig": body.get("generationConfig").cloned().unwrap_or(json!({}))
            });

            if let Some(ref sys) = system_instruction {
                request_body["system_instruction"] = sys.clone();
            }

            let resp = match self.client.post(&url).json(&request_body).send().await {
                Ok(r) => r,
                Err(e) => {
                    last_error = format!("Connection error: {}", e);
                    continue;
                }
            };

            let status = resp.status();

            if status.as_u16() == 503 || status.as_u16() == 429 {
                let body_text = resp.text().await.unwrap_or_default();
                last_error = format!("{} overloaded (HTTP {}). Trying next model...", model, status);
                let _ = body_text;
                continue;
            }

            if !status.is_success() {
                let body_text = resp.text().await.unwrap_or_default();
                if body_text.contains("not found") || body_text.contains("not supported") {
                    last_error = format!("Model {} not available: {}", model, body_text);
                    continue;
                }
                return Err(format!("Gemini API error: {}", body_text));
            }

            let gemini_resp: GeminiResponse = match resp.json().await {
                Ok(r) => r,
                Err(e) => {
                    last_error = format!("Parse error: {}", e);
                    continue;
                }
            };

            let text = gemini_resp
                .candidates
                .first()
                .and_then(|c| c.content.as_ref())
                .and_then(|c| c.parts.first())
                .and_then(|p| p.text.as_ref())
                .ok_or("No text in Gemini response")?;

            return Ok(text.clone());
        }

        Err(format!("All Gemini models failed. Last error: {}", last_error))
    }
}

fn models_to_iter(models: &[String]) -> impl Iterator<Item = &String> {
    models.iter()
}

fn clean_latex_response(text: &str) -> String {
    let mut result = text.trim().to_string();
    if result.starts_with("```latex") {
        result = result.strip_prefix("```latex").unwrap_or(&result).trim().to_string();
    }
    if result.starts_with("```") {
        result = result.strip_prefix("```").unwrap_or(&result).trim().to_string();
    }
    if result.ends_with("```") {
        result = result.strip_suffix("```").unwrap_or(&result).trim().to_string();
    }
    result
}

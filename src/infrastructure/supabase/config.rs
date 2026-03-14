use crate::domain::error::Result;

#[derive(Clone)]
pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
}

impl SupabaseConfig {
    /// Load from env: SUPABASE_URL, SUPABASE_ANON_KEY.
    /// Values come from .env at build time (build.rs) so they work in WASM and native.
    pub fn from_env() -> Result<Self> {
        let url: String = option_env!("SUPABASE_URL")
            .map(str::to_string)
            .or_else(|| std::env::var("SUPABASE_URL").ok())
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| "Missing or empty SUPABASE_URL (set in .env and rebuild)".to_string())?;
        let anon_key: String = option_env!("SUPABASE_ANON_KEY")
            .map(str::to_string)
            .or_else(|| std::env::var("SUPABASE_ANON_KEY").ok())
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| {
                "Missing or empty SUPABASE_ANON_KEY (set in .env and rebuild)".to_string()
            })?;
        Ok(Self { url, anon_key })
    }

    pub fn auth_url(&self) -> String {
        format!("{}/auth/v1", self.url.trim_end_matches('/'))
    }

    pub fn rest_url(&self) -> String {
        format!("{}/rest/v1", self.url.trim_end_matches('/'))
    }
}

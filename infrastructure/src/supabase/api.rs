use crate::supabase::client::SupabaseClient;
use crate::supabase::config::SupabaseConfig;
use crate::supabase::repositories::SupabaseRestRepository;

pub type Api = SupabaseRestRepository;

pub struct ApiBuilder {
    config: Option<SupabaseConfig>,
}

impl ApiBuilder {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn with_config(mut self, config: SupabaseConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Api {
        let config = self
            .config
            .unwrap_or_else(|| SupabaseConfig::from_env().expect("Failed to load Supabase config"));

        SupabaseRestRepository::new(SupabaseClient::new(config))
    }
}

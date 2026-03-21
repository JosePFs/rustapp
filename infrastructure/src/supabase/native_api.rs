use crate::supabase::client::SupabaseClient;
use crate::supabase::config::SupabaseConfig;
use crate::supabase::repositories::SupabaseRestRepository;

pub type NativeApi = SupabaseRestRepository;

pub struct NativeApiBuilder {
    config: Option<SupabaseConfig>,
}

impl NativeApiBuilder {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn with_config(mut self, config: SupabaseConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> NativeApi {
        let config = self
            .config
            .unwrap_or_else(|| SupabaseConfig::from_env().expect("Failed to load Supabase config"));

        SupabaseRestRepository::new(SupabaseClient::new(config))
    }
}

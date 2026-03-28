use std::sync::Arc;

use application::facades::BackofficeFacade;
use application::ports::backoffice_api::BackofficeApi;
use domain::error::Result;
use infrastructure::supabase::default_auth;
use infrastructure::supabase::repositories::SupabaseRestRepositoryBuilder;

#[derive(Clone)]
pub struct AppContext {
    backoffice: Arc<dyn BackofficeApi>,
}

impl AppContext {
    pub fn new(backoffice: Arc<dyn BackofficeApi>) -> Self {
        Self { backoffice }
    }

    pub fn backoffice_facade(&self) -> Arc<dyn BackofficeApi> {
        self.backoffice.clone()
    }
}

pub fn build_app_context() -> Result<AppContext> {
    let auth = default_auth();
    let repository = SupabaseRestRepositoryBuilder::new().build();

    let backoffice_facade =
        Arc::new(BackofficeFacade::builder(repository.clone(), auth.clone()).build());

    Ok(AppContext::new(backoffice_facade))
}

use std::sync::Arc;

use axum::extract::FromRef;

use application::facades::{BackofficeFacade, MobileFacade};
use infrastructure::supabase::default_auth;
use infrastructure::supabase::{
    auth::SupabaseAuth,
    repositories::{SupabaseRestRepository, SupabaseRestRepositoryBuilder},
};

use crate::config::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Config,
    pub repository: Arc<SupabaseRestRepository>,
    pub facade: Arc<MobileFacade<SupabaseRestRepository, SupabaseAuth>>,
    pub backoffice_facade: Arc<BackofficeFacade<SupabaseRestRepository, SupabaseAuth>>,
}

impl AppState {
    fn new(
        config: Config,
        repository: Arc<SupabaseRestRepository>,
        facade: Arc<MobileFacade<SupabaseRestRepository, SupabaseAuth>>,
        backoffice_facade: Arc<BackofficeFacade<SupabaseRestRepository, SupabaseAuth>>,
    ) -> Self {
        Self {
            config,
            repository,
            facade,
            backoffice_facade,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn facade(&self) -> &Arc<MobileFacade<SupabaseRestRepository, SupabaseAuth>> {
        &self.facade
    }

    pub fn backoffice_facade(
        &self,
    ) -> &Arc<BackofficeFacade<SupabaseRestRepository, SupabaseAuth>> {
        &self.backoffice_facade
    }

    pub fn builder() -> StateBuilder {
        StateBuilder::new()
    }
}

pub struct StateBuilder {
    config: Option<Config>,
    repository: Option<Arc<SupabaseRestRepository>>,
    facade: Option<Arc<MobileFacade<SupabaseRestRepository, SupabaseAuth>>>,
    backoffice_facade: Option<Arc<BackofficeFacade<SupabaseRestRepository, SupabaseAuth>>>,
}

impl StateBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            repository: None,
            facade: None,
            backoffice_facade: None,
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_repository(mut self, repository: Arc<SupabaseRestRepository>) -> Self {
        self.repository = Some(repository);
        self
    }

    pub fn with_facade(
        mut self,
        facade: Arc<MobileFacade<SupabaseRestRepository, SupabaseAuth>>,
    ) -> Self {
        self.facade = Some(facade);
        self
    }

    pub fn with_backoffice_facade(
        mut self,
        backoffice_facade: Arc<BackofficeFacade<SupabaseRestRepository, SupabaseAuth>>,
    ) -> Self {
        self.backoffice_facade = Some(backoffice_facade);
        self
    }

    pub fn build(self) -> AppState {
        let config = self.config.unwrap_or_else(|| Config::from_env());
        let repository = self
            .repository
            .unwrap_or_else(|| SupabaseRestRepositoryBuilder::new().build());
        let facade = self
            .facade
            .unwrap_or_else(|| MobileFacade::builder(repository.clone(), default_auth()).build());

        let backoffice_facade = self.backoffice_facade.unwrap_or_else(|| {
            Arc::new(BackofficeFacade::builder(repository.clone(), default_auth()).build())
        });

        AppState::new(config, repository, facade, backoffice_facade)
    }
}

use std::sync::Arc;

use crate::application::Backend;
use crate::domain::entities::Program;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct CreateProgramArgs {
    pub token: String,
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
}

pub struct CreateProgramUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> CreateProgramUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: CreateProgramArgs) -> Result<Program> {
        self.backend
            .create_program(
                &args.token,
                &args.specialist_id,
                &args.name,
                args.description.as_deref(),
            )
            .await
    }
}

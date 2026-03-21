use std::sync::Arc;

use domain::entities::Program;
use domain::error::Result;
use domain::repositories::CreateProgramWrite;
use domain::vos::id::Id;
use domain::vos::{AccessToken, Description, ProgramName};

#[derive(Clone)]
pub struct CreateProgramArgs {
    pub token: String,
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
}

pub struct CreateProgramUseCase<W: CreateProgramWrite> {
    catalog_write: Arc<W>,
}

impl<W: CreateProgramWrite> CreateProgramUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: CreateProgramArgs) -> Result<Program> {
        let access = AccessToken::try_from(args.token)?;
        let specialist_id = Id::try_from(args.specialist_id)?;
        let name = ProgramName::try_from(args.name)?;
        let description = args
            .description
            .as_ref()
            .map(|s| Description::try_from(s.as_str()))
            .transpose()?;
        let description_ref = description.as_ref();
        self.catalog_write
            .create_program(&access, &specialist_id, &name, description_ref)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::test_mocks::FakeCreateProgram;
    use domain::error::DomainError;

    const TOKEN: &str = "tok";
    const SPEC: &str = "550e8400-e29b-41d4-a716-446655440010";

    #[tokio::test]
    async fn create_program_invalid_program_name() {
        let program = Program {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440011").unwrap(),
            specialist_id: Id::try_from(SPEC).unwrap(),
            name: "X".to_string(),
            description: None,
        };
        let fake = FakeCreateProgram::new_ok(program);
        let uc = CreateProgramUseCase::new(Arc::new(fake));

        let err = uc
            .execute(CreateProgramArgs {
                token: TOKEN.to_string(),
                specialist_id: SPEC.to_string(),
                name: "".to_string(),
                description: None,
            })
            .await
            .unwrap_err();

        assert!(matches!(err, DomainError::InvalidParameter(_, _)));
    }

    #[tokio::test]
    async fn create_program_happy_path_forwards_name() {
        let program = Program {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440011").unwrap(),
            specialist_id: Id::try_from(SPEC).unwrap(),
            name: "My prog".to_string(),
            description: None,
        };
        let fake = FakeCreateProgram::new_ok(program.clone());
        let uc = CreateProgramUseCase::new(Arc::new(fake.clone()));

        let got = uc
            .execute(CreateProgramArgs {
                token: TOKEN.to_string(),
                specialist_id: SPEC.to_string(),
                name: "My prog".to_string(),
                description: None,
            })
            .await
            .unwrap();

        assert_eq!(got.name, program.name);
        assert_eq!(fake.last_name.lock().unwrap().as_deref(), Some("My prog"));
    }
}

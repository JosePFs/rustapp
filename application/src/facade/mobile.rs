use std::sync::Arc;

use crate::ports::error::Result;
use domain::repositories::{PatientSessionWriteRepository, SpecialistCatalogReadRepository};

use crate::ports::api::MobileApi;
use crate::ports::auth::AuthService;
use crate::use_cases::get_patient_programs::{
    GetPatientProgramsUseCase, GetPatientProgramsUseCaseResult,
};
use crate::use_cases::login::{LoginUseCaseArgs, LoginUseCaseResult};
use crate::use_cases::mobile_login::MobileLoginUseCase;
use crate::use_cases::refresh_session::{RefreshSessionArgs, RefreshSessionUseCase};
use crate::use_cases::submit_patient_workout_feedback::{
    SubmitPatientWorkoutFeedbackArgs, SubmitPatientWorkoutFeedbackUseCase,
};
use crate::use_cases::uncomplete_patient_workout_session::{
    UncompletePatientWorkoutSessionArgs, UncompletePatientWorkoutSessionUseCase,
};

pub struct MobileFacade<D, A>
where
    D: SpecialistCatalogReadRepository + PatientSessionWriteRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    pub(crate) login_uc: Arc<MobileLoginUseCase<D, A>>,
    pub(crate) refresh_session_uc: Arc<RefreshSessionUseCase<D, A>>,
    pub(crate) get_patient_programs_uc: Arc<GetPatientProgramsUseCase<D>>,
    pub(crate) submit_patient_workout_feedback_uc: Arc<SubmitPatientWorkoutFeedbackUseCase<D>>,
    pub(crate) uncomplete_patient_workout_session_uc:
        Arc<UncompletePatientWorkoutSessionUseCase<D>>,
}

impl<D, A> MobileFacade<D, A>
where
    D: SpecialistCatalogReadRepository + PatientSessionWriteRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    pub fn builder(repository: Arc<D>, auth: Arc<A>) -> MobileFacadeBuilder<D, A> {
        MobileFacadeBuilder::new(repository, auth)
    }
}

#[common::async_trait_platform]
impl<D, A> MobileApi for MobileFacade<D, A>
where
    D: SpecialistCatalogReadRepository + PatientSessionWriteRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    async fn login(&self, args: LoginUseCaseArgs) -> Result<LoginUseCaseResult> {
        self.login_uc.execute(args).await
    }

    async fn refresh_session(&self, args: RefreshSessionArgs) -> Result<LoginUseCaseResult> {
        self.refresh_session_uc.execute(args).await
    }

    async fn get_patient_programs(&self) -> Result<GetPatientProgramsUseCaseResult> {
        self.get_patient_programs_uc.execute().await
    }

    async fn submit_patient_workout_feedback(
        &self,
        args: SubmitPatientWorkoutFeedbackArgs,
    ) -> Result<()> {
        self.submit_patient_workout_feedback_uc.execute(args).await
    }

    async fn uncomplete_patient_workout_session(
        &self,
        args: UncompletePatientWorkoutSessionArgs,
    ) -> Result<()> {
        self.uncomplete_patient_workout_session_uc
            .execute(args)
            .await
    }
}

pub struct MobileFacadeBuilder<D, A>
where
    D: SpecialistCatalogReadRepository + PatientSessionWriteRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    repository: Arc<D>,
    auth: Arc<A>,
}

impl<D, A> MobileFacadeBuilder<D, A>
where
    D: SpecialistCatalogReadRepository + PatientSessionWriteRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    pub fn new(repository: Arc<D>, auth: Arc<A>) -> Self {
        Self { repository, auth }
    }
}

impl<D, A> MobileFacadeBuilder<D, A>
where
    D: SpecialistCatalogReadRepository + PatientSessionWriteRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    pub fn build(self) -> Arc<MobileFacade<D, A>> {
        Arc::new(MobileFacade {
            login_uc: Arc::new(MobileLoginUseCase::new(
                self.repository.clone(),
                self.auth.clone(),
            )),
            refresh_session_uc: Arc::new(RefreshSessionUseCase::new(
                self.repository.clone(),
                self.auth.clone(),
            )),
            get_patient_programs_uc: Arc::new(GetPatientProgramsUseCase::new(
                self.repository.clone(),
            )),
            submit_patient_workout_feedback_uc: Arc::new(SubmitPatientWorkoutFeedbackUseCase::new(
                self.repository.clone(),
            )),
            uncomplete_patient_workout_session_uc: Arc::new(
                UncompletePatientWorkoutSessionUseCase::new(self.repository.clone()),
            ),
        })
    }
}

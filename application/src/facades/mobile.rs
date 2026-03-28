use std::sync::Arc;

use crate::error::Result;
use domain::repositories::{PatientSessionRepository, SpecialistCatalogReadRepository};

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
    D: SpecialistCatalogReadRepository + PatientSessionRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    login_uc: MobileLoginUseCase<D, A>,
    refresh_session_uc: RefreshSessionUseCase<D, A>,
    get_patient_programs_uc: GetPatientProgramsUseCase<D>,
    submit_patient_workout_feedback_uc: SubmitPatientWorkoutFeedbackUseCase<D>,
    uncomplete_patient_workout_session_uc: UncompletePatientWorkoutSessionUseCase<D>,
}

impl<D, A> MobileFacade<D, A>
where
    D: SpecialistCatalogReadRepository + PatientSessionRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    pub fn builder(repository: Arc<D>, auth: Arc<A>) -> MobileFacadeBuilder<D, A> {
        MobileFacadeBuilder::new(repository, auth)
    }

    pub async fn login(&self, args: LoginUseCaseArgs) -> Result<LoginUseCaseResult> {
        self.login_uc.execute(args).await
    }

    pub async fn refresh_session(&self, args: RefreshSessionArgs) -> Result<LoginUseCaseResult> {
        self.refresh_session_uc.execute(args).await
    }

    pub async fn get_patient_programs(&self) -> Result<GetPatientProgramsUseCaseResult> {
        self.get_patient_programs_uc.execute().await
    }

    pub async fn submit_patient_workout_feedback(
        &self,
        args: SubmitPatientWorkoutFeedbackArgs,
    ) -> Result<()> {
        self.submit_patient_workout_feedback_uc.execute(args).await
    }

    pub async fn uncomplete_patient_workout_session(
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
    D: SpecialistCatalogReadRepository + PatientSessionRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    repository: Arc<D>,
    auth: Arc<A>,
}

impl<D, A> MobileFacadeBuilder<D, A>
where
    D: SpecialistCatalogReadRepository + PatientSessionRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    pub fn new(repository: Arc<D>, auth: Arc<A>) -> Self {
        Self { repository, auth }
    }
}

impl<D, A> MobileFacadeBuilder<D, A>
where
    D: SpecialistCatalogReadRepository + PatientSessionRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    pub fn build(self) -> Arc<MobileFacade<D, A>> {
        Arc::new(MobileFacade {
            login_uc: MobileLoginUseCase::new(self.repository.clone(), self.auth.clone()),
            refresh_session_uc: RefreshSessionUseCase::new(
                self.repository.clone(),
                self.auth.clone(),
            ),
            get_patient_programs_uc: GetPatientProgramsUseCase::new(self.repository.clone()),
            submit_patient_workout_feedback_uc: SubmitPatientWorkoutFeedbackUseCase::new(
                self.repository.clone(),
            ),
            uncomplete_patient_workout_session_uc: UncompletePatientWorkoutSessionUseCase::new(
                self.repository.clone(),
            ),
        })
    }
}

use std::sync::Arc;

use async_trait::async_trait;
use domain::error::Result;
use domain::repositories::{PatientSessionWriteRepository, SpecialistCatalogReadRepository};

use crate::ports::api::MobileApi;
use crate::ports::auth::auth::AuthService;
use crate::use_cases::get_patient_programs::{
    GetPatientProgramsUseCase, GetPatientProgramsUseCaseArgs, GetPatientProgramsUseCaseResult,
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
    pub login_uc: Arc<MobileLoginUseCase<D, A>>,
    pub refresh_session_uc: Arc<RefreshSessionUseCase<D, A>>,
    pub get_patient_programs_uc: Arc<GetPatientProgramsUseCase<D>>,
    pub submit_patient_workout_feedback_uc: Arc<SubmitPatientWorkoutFeedbackUseCase<D>>,
    pub uncomplete_patient_workout_session_uc: Arc<UncompletePatientWorkoutSessionUseCase<D>>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
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

    async fn get_patient_programs(
        &self,
        args: GetPatientProgramsUseCaseArgs,
    ) -> Result<GetPatientProgramsUseCaseResult> {
        self.get_patient_programs_uc.execute(args).await
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

use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use domain::entities::WorkoutSession;
use domain::error::Result;
use domain::repositories::PatientSessionWriteRepository;
use domain::vos::id::Id;
use domain::vos::{
    AccessToken, DayIndex, EffortScore, FeedbackComment, PainScore, SessionDate,
};

#[derive(Clone)]
pub struct FakePatientSessionWrite {
    pub session_to_return: Arc<Mutex<Result<WorkoutSession>>>,
    pub get_or_create_calls: Arc<Mutex<usize>>,
    pub complete_calls: Arc<Mutex<usize>>,
    pub upsert_calls: Arc<Mutex<usize>>,
    pub uncomplete_calls: Arc<Mutex<usize>>,
    pub complete_outcome: Arc<Mutex<Result<()>>>,
    pub upsert_outcome: Arc<Mutex<Result<()>>>,
}

impl FakePatientSessionWrite {
    pub fn new(session: WorkoutSession) -> Self {
        Self {
            session_to_return: Arc::new(Mutex::new(Ok(session))),
            get_or_create_calls: Arc::new(Mutex::new(0)),
            complete_calls: Arc::new(Mutex::new(0)),
            upsert_calls: Arc::new(Mutex::new(0)),
            uncomplete_calls: Arc::new(Mutex::new(0)),
            complete_outcome: Arc::new(Mutex::new(Ok(()))),
            upsert_outcome: Arc::new(Mutex::new(Ok(()))),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl PatientSessionWriteRepository for FakePatientSessionWrite {
    async fn get_or_create_session(
        &self,
        _access_token: &AccessToken,
        _patient_program_id: &Id,
        _day_index: DayIndex,
        _session_date: &SessionDate,
    ) -> Result<WorkoutSession> {
        *self.get_or_create_calls.lock().unwrap() += 1;
        self.session_to_return.lock().unwrap().clone()
    }

    async fn complete_session(
        &self,
        _access_token: &AccessToken,
        _session_id: &Id,
        _session_date: &SessionDate,
    ) -> Result<()> {
        *self.complete_calls.lock().unwrap() += 1;
        self.complete_outcome.lock().unwrap().clone()
    }

    async fn uncomplete_session(
        &self,
        _access_token: &AccessToken,
        _session_id: &Id,
    ) -> Result<()> {
        *self.uncomplete_calls.lock().unwrap() += 1;
        Ok(())
    }

    async fn upsert_session_exercise_feedback(
        &self,
        _access_token: &AccessToken,
        _workout_session_id: &Id,
        _exercise_id: &Id,
        _effort: Option<EffortScore>,
        _pain: Option<PainScore>,
        _comment: Option<&FeedbackComment>,
    ) -> Result<()> {
        *self.upsert_calls.lock().unwrap() += 1;
        self.upsert_outcome.lock().unwrap().clone()
    }
}

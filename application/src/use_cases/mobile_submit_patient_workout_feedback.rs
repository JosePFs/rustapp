use std::{collections::HashMap, sync::Arc};

use futures::stream::{self, StreamExt};

use crate::application::MobileBackend;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct MobileSubmitPatientWorkoutFeedbackArgs {
    pub token: String,
    pub patient_program_id: String,
    pub day_index: i32,
    pub session_date: String,
    pub feedback_map: HashMap<String, (i32, i32, String)>,
    pub completion_status: Option<bool>,
}

pub struct MobileSubmitPatientWorkoutFeedbackUseCase<B: MobileBackend> {
    backend: Arc<B>,
}

impl<B: MobileBackend> MobileSubmitPatientWorkoutFeedbackUseCase<B> {
    const MAX_CONCURRENT_REQUESTS: usize = 6;

    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: MobileSubmitPatientWorkoutFeedbackArgs) -> Result<()> {
        let token = &args.token;
        let session = self
            .backend
            .get_or_create_session(token, &args.patient_program_id, args.day_index, &args.session_date)
            .await?;
        let session_id = session.id;

        self.backend
            .update_session(token, &session_id, Some(&args.session_date))
            .await?;

        match args.completion_status {
            Some(true) => self.backend.complete_session(token, &session_id).await?,
            Some(false) => self.backend.uncomplete_session(token, &session_id).await?,
            None => {}
        }

        let token = args.token.clone();
        let session_id = session_id.clone();
        stream::iter(args.feedback_map.into_iter())
            .map(|(exercise_id, (effort, pain, comment))| {
                let backend = self.backend.clone();
                let token = token.clone();
                let session_id = session_id.clone();

                async move {
                    backend
                        .upsert_session_exercise_feedback(
                            &token,
                            &session_id,
                            &exercise_id,
                            Some(effort),
                            Some(pain),
                            if comment.is_empty() {
                                None
                            } else {
                                Some(comment.as_str())
                            },
                        )
                        .await
                }
            })
            .buffer_unordered(Self::MAX_CONCURRENT_REQUESTS)
            .collect::<Vec<Result<()>>>()
            .await
            .into_iter()
            .collect::<Result<Vec<()>>>()?;

        Ok(())
    }
}

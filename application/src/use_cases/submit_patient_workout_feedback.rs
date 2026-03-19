use std::sync::Arc;

use futures::{
    stream::{self, StreamExt},
    try_join,
};

use crate::ports::Backend;
use domain::error::Result;

#[derive(Clone)]
pub struct SubmitPatientWorkoutFeedbackArgs {
    pub token: String,
    pub patient_program_id: String,
    pub day_index: i32,
    pub session_date: String,
    pub feedback_completed: bool,
    pub feedback_map: std::collections::HashMap<String, (i32, i32, String)>,
}

pub struct SubmitPatientWorkoutFeedbackUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> SubmitPatientWorkoutFeedbackUseCase<B> {
    const MAX_CONCURRENT_REQUESTS: usize = 6;

    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: SubmitPatientWorkoutFeedbackArgs) -> Result<()> {
        let token = &args.token;
        let pid = &args.patient_program_id;
        let di = args.day_index;

        let session = self
            .backend
            .get_or_create_session(token, pid, di, &args.session_date)
            .await?;
        let session_id = session.id;

        let update_fut = self
            .backend
            .update_session(token, &session_id, Some(&args.session_date));

        let complete_fut = async {
            if args.feedback_completed {
                Ok(())
            } else {
                self.backend.complete_session(token, &session_id).await
            }
        };

        try_join!(update_fut, complete_fut)?;

        let token = args.token.clone();
        let session_id = session_id.clone();

        stream::iter(args.feedback_map.into_iter())
            .map(|(exercise_id, (eff, pain, comment))| {
                let backend = self.backend.clone();
                let token = token.clone();
                let session_id = session_id.clone();

                async move {
                    backend
                        .upsert_session_exercise_feedback(
                            &token,
                            &session_id,
                            &exercise_id,
                            Some(eff as i32),
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

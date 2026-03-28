use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::backoffice_api::CreateProgramArgs;

#[derive(Clone)]
pub struct UseCreateProgram {
    pub action: Action<(String, String), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_create_program() -> UseCreateProgram {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();

    let action: Action<(String, String), ()> =
        use_action(move |name: String, description: String| {
            let facade = facade_for_action.clone();
            let mut state = state.clone();

            state.set(AsyncState::Loading);

            async move {
                let args = CreateProgramArgs {
                    name,
                    description: if description.is_empty() {
                        None
                    } else {
                        Some(description)
                    },
                };

                facade
                    .create_program(args)
                    .await
                    .map(|_| {
                        state.set(AsyncState::Ready(()));
                    })
                    .map_err(|e| {
                        state.set(AsyncState::Error(e.clone()));
                        e
                    })
            }
        });

    UseCreateProgram { action, state }
}

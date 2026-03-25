use super::catalog_write::*;

pub trait SpecialistCatalogWriteRepository:
    AddSpecialistPatient
    + CreateProgram
    + CreateWorkout
    + UpdateWorkoutWrite
    + DeleteWorkoutWrite
    + CreateProgramScheduleItemWrite
    + DeleteProgramScheduleItemWrite
    + CreateExerciseWrite
    + AddExerciseToWorkoutWrite
    + RemoveExerciseFromWorkoutWrite
    + UpdateWorkoutExerciseWrite
    + UpdateExerciseWrite
    + SoftDeleteExerciseWrite
    + RestoreExerciseWrite
    + AssignProgramToPatientWrite
    + UnassignProgramFromPatientWrite
    + GetOrCreateSessionCatalogWrite
    + CompleteSessionCatalogWrite
    + UpdateSessionWrite
    + UpsertSessionExerciseFeedbackCatalogWrite
    + UncompleteSessionCatalogWrite
{
}

impl<T> SpecialistCatalogWriteRepository for T where
    T: AddSpecialistPatient
        + CreateProgram
        + CreateWorkout
        + UpdateWorkoutWrite
        + DeleteWorkoutWrite
        + CreateProgramScheduleItemWrite
        + DeleteProgramScheduleItemWrite
        + CreateExerciseWrite
        + AddExerciseToWorkoutWrite
        + RemoveExerciseFromWorkoutWrite
        + UpdateWorkoutExerciseWrite
        + UpdateExerciseWrite
        + SoftDeleteExerciseWrite
        + RestoreExerciseWrite
        + AssignProgramToPatientWrite
        + UnassignProgramFromPatientWrite
        + GetOrCreateSessionCatalogWrite
        + CompleteSessionCatalogWrite
        + UpdateSessionWrite
        + UpsertSessionExerciseFeedbackCatalogWrite
        + UncompleteSessionCatalogWrite
{
}

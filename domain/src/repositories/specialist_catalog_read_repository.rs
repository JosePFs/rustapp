use super::catalog_read::*;

pub trait SpecialistCatalogReadRepository:
    GetProfilesByIdsRead
    + GetPatientIdByEmailRead
    + ListSpecialistPatientsRead
    + ListProgramsRead
    + GetProgramRead
    + ListWorkoutLibraryRead
    + GetWorkoutsByIdsRead
    + ListWorkoutsForProgramRead
    + ListProgramScheduleRead
    + ListExercisesForWorkoutRead
    + ListExerciseLibraryRead
    + ListPatientProgramsForSpecialistRead
    + GetPatientProgramByIdRead
    + ListWorkoutSessionsRead
    + ListSessionExerciseFeedbackRead
    + ListSessionExerciseFeedbackForProgramRead
    + ListActivePatientProgramsRead
    + GetWorkoutWithExercisesRead
    + GetProgramWithAgendaRead
    + GetPatientProgramFullRead
    + GetSpecialistDashboardRead
{
}

impl<T> SpecialistCatalogReadRepository for T where
    T: GetProfilesByIdsRead
        + GetPatientIdByEmailRead
        + ListSpecialistPatientsRead
        + ListProgramsRead
        + GetProgramRead
        + ListWorkoutLibraryRead
        + GetWorkoutsByIdsRead
        + ListWorkoutsForProgramRead
        + ListProgramScheduleRead
        + ListExercisesForWorkoutRead
        + ListExerciseLibraryRead
        + ListPatientProgramsForSpecialistRead
        + GetPatientProgramByIdRead
        + ListWorkoutSessionsRead
        + ListSessionExerciseFeedbackRead
        + ListSessionExerciseFeedbackForProgramRead
        + ListActivePatientProgramsRead
        + GetWorkoutWithExercisesRead
        + GetProgramWithAgendaRead
        + GetPatientProgramFullRead
        + GetSpecialistDashboardRead
{
}

use super::catalog_read::*;

pub trait SpecialistCatalogReadRepository:
    GetProfilesByIdsRead
    + GetPatientIdByEmailRead
    + ListSpecialistPatientsRead
    + ListProgramsRead
    + GetProgramRead
    + ListWorkoutLibrary
    + GetWorkoutsByIdsRead
    + ListWorkoutsForProgramRead
    + ListProgramScheduleRead
    + ListExercisesForWorkoutRead
    + ListExerciseLibrary
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
        + ListWorkoutLibrary
        + GetWorkoutsByIdsRead
        + ListWorkoutsForProgramRead
        + ListProgramScheduleRead
        + ListExercisesForWorkoutRead
        + ListExerciseLibrary
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

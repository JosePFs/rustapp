use std::sync::Arc;

use crate::error::{ApplicationError, Result};
use domain::repositories::GetSpecialistDashboardRead;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpecialistPatientLink {
    pub link_id: String,
    pub patient_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatientProfileSummary {
    pub patient_id: String,
    pub full_name: String,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatientProgramAssignment {
    pub id: String,
    pub patient_id: String,
    pub program_id: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpecialistProgramsDataResult {
    pub links: Vec<SpecialistPatientLink>,
    pub profiles: Vec<PatientProfileSummary>,
    pub programs: Vec<ProgramSummary>,
    pub assignments: Vec<PatientProgramAssignment>,
}

pub struct SpecialistProgramsDataUseCase<R: GetSpecialistDashboardRead> {
    catalog_read: Arc<R>,
}

impl<R: GetSpecialistDashboardRead> SpecialistProgramsDataUseCase<R> {
    pub fn new(catalog_read: Arc<R>) -> Self {
        Self { catalog_read }
    }

    pub async fn execute(&self) -> Result<SpecialistProgramsDataResult> {
        let dashboard = self
            .catalog_read
            .get_specialist_dashboard()
            .await
            .map_err(ApplicationError::from)?;

        let links: Vec<SpecialistPatientLink> = dashboard
            .links
            .into_iter()
            .map(|l| SpecialistPatientLink {
                link_id: l.id.to_string(),
                patient_id: l.patient_id.to_string(),
            })
            .collect();

        let profiles: Vec<PatientProfileSummary> = dashboard
            .profiles
            .into_iter()
            .map(|p| PatientProfileSummary {
                patient_id: p.id().to_string(),
                full_name: p.full_name().value().to_string(),
                email: p.email().to_string(),
            })
            .collect();

        let programs: Vec<ProgramSummary> = dashboard
            .programs
            .into_iter()
            .map(|p| ProgramSummary {
                id: p.id.to_string(),
                name: p.name,
                description: p.description,
            })
            .collect();

        let assignments: Vec<PatientProgramAssignment> = dashboard
            .assignments
            .into_iter()
            .map(|a| PatientProgramAssignment {
                id: a.id.to_string(),
                patient_id: a.patient_id.to_string(),
                program_id: a.program_id.to_string(),
                status: a.status,
            })
            .collect();

        Ok(SpecialistProgramsDataResult {
            links,
            profiles,
            programs,
            assignments,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use domain::aggregates::SpecialistDashboard;
    use domain::error::Result;
    use domain::repositories::GetSpecialistDashboardRead;

    #[tokio::test]
    async fn maps_empty_dashboard() {
        let dash = SpecialistDashboard {
            links: vec![],
            profiles: vec![],
            programs: vec![],
            assignments: vec![],
        };
        let fake = MockGetSpecialistDashboardRead::new_ok(dash);
        let uc = SpecialistProgramsDataUseCase::new(Arc::new(fake));

        let res = uc.execute().await.unwrap();

        assert!(res.links.is_empty());
        assert!(res.programs.is_empty());
    }

    #[derive(Clone)]
    struct MockGetSpecialistDashboardRead {
        dashboard: Arc<Mutex<Result<SpecialistDashboard>>>,
    }

    impl MockGetSpecialistDashboardRead {
        fn new_ok(dashboard: SpecialistDashboard) -> Self {
            Self {
                dashboard: Arc::new(Mutex::new(Ok(dashboard))),
            }
        }
    }

    #[common::async_trait_platform]
    impl GetSpecialistDashboardRead for MockGetSpecialistDashboardRead {
        async fn get_specialist_dashboard(&self) -> Result<SpecialistDashboard> {
            self.dashboard.lock().unwrap().clone()
        }
    }
}

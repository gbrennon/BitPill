use bitpill::{
    application::dtos::requests::{CreateMedicationRequest, GetMedicationRequest},
    infrastructure::container::Container,
};
use tempfile::tempdir;

#[test]
fn container_new_with_paths_persists_medication_files() {
    let dir = tempdir().unwrap();
    let meds = dir.path().join("meds.json");
    let doses = dir.path().join("doses.json");
    let settings = dir.path().join("settings.json");

    let container = Container::new(meds.clone(), doses.clone(), settings.clone());
    let req = CreateMedicationRequest::new("IntegrationMed", 42, vec![(8, 0)], "OnceDaily");
    let res = container
        .create_medication_service
        .execute(req)
        .expect("create should succeed");
    let id = res.id;

    let container2 = Container::new(meds, doses, settings);
    let get_req = GetMedicationRequest { id: id.clone() };
    let got = container2
        .get_medication_service
        .execute(get_req)
        .expect("get should succeed");
    assert_eq!(got.medication.name, "IntegrationMed");
}

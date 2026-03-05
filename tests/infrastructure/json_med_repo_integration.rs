use std::sync::Arc;
use tempfile::tempdir;

use bitpill::infrastructure::container::Container;
use bitpill::application::dtos::requests::CreateMedicationRequest;

#[test]
fn container_new_with_paths_persists_medication_files() {
    let dir = tempdir().unwrap();
    let meds = dir.path().join("meds.json");
    let doses = dir.path().join("doses.json");
    let settings = dir.path().join("settings.json");

    let mut container = Container::new_with_paths(meds.clone(), doses.clone(), settings.clone());
    // create a medication via the create service
    let req = CreateMedicationRequest::new("IntegrationMed", 42, vec![(8,0)], "OnceDaily");
    let res = container.create_medication_service.execute(req).expect("create should succeed");
    let id = res.id;

    // Create a fresh container pointing to same files and ensure medication exists
    let container2 = Container::new_with_paths(meds, doses, settings);
    let get_req = bitpill::application::dtos::requests::GetMedicationRequest { id: id.clone() };
    let got = container2.get_medication_service.execute(get_req).expect("get should succeed");
    assert_eq!(got.medication.name, "IntegrationMed");
}

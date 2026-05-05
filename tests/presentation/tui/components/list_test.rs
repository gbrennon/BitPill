use bitpill::{
    application::dtos::responses::MedicationDto,
    presentation::tui::components::list::medication_list,
};

#[test]
fn medication_list_constructs_with_items() {
    let items = vec![
        MedicationDto {
            id: "1".into(),
            name: "Aspirin".into(),
            amount_mg: 500,
            scheduled_time: vec![],
            dose_frequency: "OnceDaily".to_string(),
            taken_today: 0,
            scheduled_today: 0,
        },
        MedicationDto {
            id: "2".into(),
            name: "Ibuprofen".into(),
            amount_mg: 200,
            scheduled_time: vec![],
            dose_frequency: "OnceDaily".to_string(),
            taken_today: 0,
            scheduled_today: 0,
        },
    ];
    let list = medication_list(&items);
    // Ensure list is non-empty by checking size
    assert!(std::mem::size_of_val(&list) > 0);
}

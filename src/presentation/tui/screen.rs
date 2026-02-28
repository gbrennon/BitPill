pub enum Screen {
    MedicationList,
    CreateMedication {
        name: String,
        amount_mg: String,
        scheduled_times: String,
        focused_field: u8,
    },
    ScheduleResult {
        created_count: usize,
    },
}

use crate::infrastructure::container::Container;

pub fn run() {
    let container = Container::new();

    match container
        .create_medication_service
        .execute("Aspirin", 500, vec![(8, 0), (20, 0)])
    {
        Ok(med) => println!(
            "Created medication: {} — {} — scheduled at: {}",
            med.name(),
            med.dosage(),
            med.scheduled_times()
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Err(e) => eprintln!("Error: {e}"),
    }
}

use tempfile::tempdir;

#[test]
fn settings_service_arc_is_same_instance() {
    let dir = tempdir().unwrap();
    let c = bitpill::infrastructure::container::Container::new(
        dir.path().join("meds.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );

    let s1 = c.settings_service.clone();
    let s2 = c.settings_service.clone();

    assert!(std::sync::Arc::ptr_eq(&s1, &s2));
}

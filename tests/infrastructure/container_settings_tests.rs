#[test]
fn settings_service_arc_is_same_instance() {
    let c = bitpill::infrastructure::container::Container::new();

    let s1 = c.settings_service.clone();
    let s2 = c.settings_service.clone();

    assert!(std::sync::Arc::ptr_eq(&s1, &s2));
}

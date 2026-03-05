#[test]
fn get_settings_service_returns_same_reference() {
    let c = bitpill::infrastructure::container::Container::new();
    let r1 = c.get_settings_service() as *const _;
    let r2 = c.get_settings_service() as *const _;
    assert_eq!(r1, r2);
}

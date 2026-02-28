use bitpill::infrastructure::container::Container;

#[test]
fn container_new_builds_successfully() {
    let _container = Container::new();
}

#[test]
fn container_default_builds_successfully() {
    let _container = Container::default();
}

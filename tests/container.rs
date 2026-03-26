use bitpill::infrastructure::container::Container;
use tempfile::tempdir;

#[test]
fn container_new_with_paths_builds_successfully() {
    let dir = tempdir().unwrap();
    let _container = Container::new(
        dir.path().join("medications.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );
}

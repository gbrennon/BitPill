use bitpill::domain::value_objects::navigation_mode::NavigationModeVariant;

#[test]
fn navigation_mode_variant_count_is_two() {
    assert_eq!(NavigationModeVariant::count(), 2);
}

#[test]
fn navigation_mode_variant_from_index_returns_vi() {
    assert_eq!(
        NavigationModeVariant::from_index(0),
        Some(NavigationModeVariant::Vi)
    );
}

#[test]
fn navigation_mode_variant_from_index_returns_emacs() {
    assert_eq!(
        NavigationModeVariant::from_index(1),
        Some(NavigationModeVariant::Emacs)
    );
}

#[test]
fn navigation_mode_variant_from_index_invalid_returns_none() {
    assert_eq!(NavigationModeVariant::from_index(5), None);
}

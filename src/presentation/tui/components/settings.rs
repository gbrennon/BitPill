use crate::{
    domain::value_objects::navigation_mode::NavigationModeVariant,
    presentation::tui::templates::form_template::{FormField, FormTemplate},
};

pub fn settings_view(f: &mut ratatui::Frame, selected_index: usize) {
    let help = "[?] Help  [Space/j/l] Toggle  [s] Save  [Esc] Cancel";

    let options: Vec<&str> = NavigationModeVariant::variants()
        .iter()
        .map(|v| v.as_str())
        .collect();

    let template = FormTemplate {
        subtitle: "Settings",
        fields: &[FormField {
            label: "Navigation Mode",
            value: "",
            focused: true,
            choices: Some(&options),
            selected_choice: Some(selected_index),
            lines: 3,
            highlighted_line: None,
            values: None,
        }],
        help,
        mode: "NORMAL",
    };
    template.render(f);
}

#[cfg(test)]
mod tests {
    use crate::domain::value_objects::navigation_mode::NavigationModeVariant;

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
}

use crate::domain::value_objects::scheduled_time::ScheduledTime;

/// Specifies how often a medication dose should be taken.
///
/// `DoseFrequency` is a value object — two instances with the same frequency
/// are considered equal. It defines both the dosage frequency (once, twice, thrice daily)
/// and the default scheduled times for each frequency variant.
///
/// # Variants
///
/// - `OnceDaily` — One dose per day at 08:00.
/// - `TwiceDaily` — Two doses per day at 08:00 and 20:00.
/// - `ThriceDaily` — Three doses per day at 08:00, 14:00, and 20:00.
/// - `Custom` — User-defined times (minimum 4 required).
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::value_objects::medication_frequency::DoseFrequency;
///
/// let freq = DoseFrequency::TwiceDaily;
/// assert_eq!(freq.required_times_count(), Some(2));
/// assert_eq!(freq.as_str(), "TwiceDaily");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DoseFrequency {
    OnceDaily,
    TwiceDaily,
    ThriceDaily,
    Custom(Vec<ScheduledTime>),
}

impl DoseFrequency {
    /// Returns the required number of scheduled times, or `None` for `Custom`.
    pub fn required_times_count(&self) -> Option<usize> {
        match self {
            DoseFrequency::OnceDaily => Some(1),
            DoseFrequency::TwiceDaily => Some(2),
            DoseFrequency::ThriceDaily => Some(3),
            DoseFrequency::Custom(_) => None,
        }
    }

    /// Returns the default scheduled times for this frequency.
    pub fn scheduled_time(&self) -> Vec<ScheduledTime> {
        match self {
            DoseFrequency::OnceDaily => vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::TwiceDaily => vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::ThriceDaily => vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(14, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::Custom(times) => times.clone(),
        }
    }

    /// Returns a string identifier for this frequency.
    pub fn as_str(&self) -> &'static str {
        match self {
            DoseFrequency::OnceDaily => "OnceDaily",
            DoseFrequency::TwiceDaily => "TwiceDaily",
            DoseFrequency::ThriceDaily => "ThriceDaily",
            DoseFrequency::Custom(_) => "Custom",
        }
    }
}

impl std::fmt::Display for DoseFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DoseFrequency::OnceDaily => write!(f, "Once Daily"),
            DoseFrequency::TwiceDaily => write!(f, "Twice Daily"),
            DoseFrequency::ThriceDaily => write!(f, "Thrice Daily"),
            DoseFrequency::Custom(times) => {
                let times_str = times
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "Custom ({})", times_str)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduled_time_once_daily_returns_one_time() {
        let freq = DoseFrequency::OnceDaily;
        let times = freq.scheduled_time();

        let expected_time = ScheduledTime::new(8, 0).unwrap();
        assert_eq!(times.len(), 1);
        assert_eq!(times[0], expected_time);
    }

    #[test]
    fn scheduled_time_twice_daily_returns_two_times() {
        let freq = DoseFrequency::TwiceDaily;
        let times = freq.scheduled_time();

        let expected_times = vec![
            ScheduledTime::new(8, 0).unwrap(),
            ScheduledTime::new(20, 0).unwrap(),
        ];
        assert_eq!(times, expected_times);
        assert_eq!(times.len(), 2);
    }

    #[test]
    fn display_handles_custom() {
        let custom_times = vec![
            ScheduledTime::new(9, 0).unwrap(),
            ScheduledTime::new(21, 0).unwrap(),
        ];
        let freq_custom = DoseFrequency::Custom(custom_times.clone());
        assert!(freq_custom.to_string().contains("Custom"));
        assert!(freq_custom.to_string().contains("09:00"));
        assert!(freq_custom.to_string().contains("21:00"));
    }

    #[test]
    fn required_times_count_returns_expected_values() {
        assert_eq!(DoseFrequency::OnceDaily.required_times_count(), Some(1));
        assert_eq!(DoseFrequency::TwiceDaily.required_times_count(), Some(2));
        assert_eq!(DoseFrequency::ThriceDaily.required_times_count(), Some(3));
        assert_eq!(DoseFrequency::Custom(vec![]).required_times_count(), None);
    }

    #[test]
    fn custom_with_times_returns_times() {
        let times = vec![
            ScheduledTime::new(8, 0).unwrap(),
            ScheduledTime::new(12, 0).unwrap(),
            ScheduledTime::new(16, 0).unwrap(),
            ScheduledTime::new(20, 0).unwrap(),
        ];
        let freq = DoseFrequency::Custom(times.clone());
        assert_eq!(freq.scheduled_time(), times);
    }

    #[test]
    fn as_str_returns_correct_string() {
        assert_eq!(DoseFrequency::OnceDaily.as_str(), "OnceDaily");
        assert_eq!(DoseFrequency::TwiceDaily.as_str(), "TwiceDaily");
        assert_eq!(DoseFrequency::ThriceDaily.as_str(), "ThriceDaily");
        assert_eq!(DoseFrequency::Custom(vec![]).as_str(), "Custom");
    }

    #[test]
    fn display_formats_once_daily() {
        let freq = DoseFrequency::OnceDaily;
        assert_eq!(freq.to_string(), "Once Daily");
    }

    #[test]
    fn display_formats_twice_daily() {
        let freq = DoseFrequency::TwiceDaily;
        assert_eq!(freq.to_string(), "Twice Daily");
    }

    #[test]
    fn display_formats_thrice_daily() {
        let freq = DoseFrequency::ThriceDaily;
        assert_eq!(freq.to_string(), "Thrice Daily");
    }
}

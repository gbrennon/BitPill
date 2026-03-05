use crate::domain::value_objects::scheduled_time::ScheduledTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DoseFrequency {
    OnceDaily,
    TwiceDaily,
    ThriceDaily,
    EveryXHours(u32),           // e.g., Every 6 hours
    Custom(Vec<ScheduledTime>), // Specific times of date
}

impl DoseFrequency {
    /// Returns the scheduled times for this medication frequency.
    /// For fixed frequencies, this returns a predefined set of times.
    /// For `EveryXHours`, it returns an empty vector (as times are relative).
    /// For `Custom`, it returns the user-defined times.
    pub fn scheduled_time(&self) -> Vec<ScheduledTime> {
        match self {
            DoseFrequency::OnceDaily => vec![ScheduledTime::new(8, 0).unwrap()], // Default to 8:00 AM
            DoseFrequency::TwiceDaily => vec![
                ScheduledTime::new(8, 0).unwrap(),  // 8:00 AM
                ScheduledTime::new(20, 0).unwrap(), // 8:00 PM
            ],
            DoseFrequency::ThriceDaily => vec![
                ScheduledTime::new(8, 0).unwrap(),  // 8:00 AM
                ScheduledTime::new(14, 0).unwrap(), // 2:00 PM
                ScheduledTime::new(20, 0).unwrap(), // 8:00 PM
            ],
            DoseFrequency::EveryXHours(_) => vec![], // Times are relative, not fixed
            DoseFrequency::Custom(times) => times.clone(),
        }
    }
}

impl std::fmt::Display for DoseFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DoseFrequency::OnceDaily => write!(f, "Once Daily"),
            DoseFrequency::TwiceDaily => write!(f, "Twice Daily"),
            DoseFrequency::ThriceDaily => write!(f, "Thrice Daily"),
            DoseFrequency::EveryXHours(hours) => write!(f, "Every {} Hours", hours),
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
    fn display_handles_every_x_hours_and_custom() {
        let freq = DoseFrequency::EveryXHours(6);
        assert_eq!(freq.to_string(), "Every 6 Hours");

        let custom_times = vec![ScheduledTime::new(9, 0).unwrap(), ScheduledTime::new(21, 0).unwrap()];
        let freq_custom = DoseFrequency::Custom(custom_times.clone());
        assert!(freq_custom.to_string().contains("Custom"));
        assert!(freq_custom.to_string().contains("09:00"));
        assert!(freq_custom.to_string().contains("21:00"));

        // scheduled_time for EveryXHours is empty
        assert!(DoseFrequency::EveryXHours(4).scheduled_time().is_empty());
    }
}

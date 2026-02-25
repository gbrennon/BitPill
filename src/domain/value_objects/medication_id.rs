use uuid::Uuid;

/// Unique identifier for a [`Medication`] aggregate root.
///
/// Each call to [`MedicationId::new`] (or `Default::default`) generates a
/// fresh UUID v4, guaranteeing uniqueness across the system.
///
/// [`Medication`]: crate::domain::entities::medication::Medication
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::value_objects::medication_id::MedicationId;
///
/// let id = MedicationId::new();
/// assert!(!id.to_string().is_empty());
///
/// // Every call produces a different ID.
/// assert_ne!(MedicationId::new(), MedicationId::new());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MedicationId(Uuid);

impl MedicationId {
    /// Generates a new unique `MedicationId` using UUID v4.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Returns the underlying [`Uuid`] value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for MedicationId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MedicationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_unique_ids() {
        let id_a = MedicationId::new();
        let id_b = MedicationId::new();

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn value_returns_the_inner_uuid() {
        let id = MedicationId::new();

        assert_eq!(id.value(), id.0);
    }

    #[test]
    fn display_formats_as_uuid_string() {
        let id = MedicationId::new();

        assert_eq!(id.to_string(), id.value().to_string());
    }

    #[test]
    fn default_generates_a_valid_id() {
        let id = MedicationId::default();

        assert!(!id.to_string().is_empty());
    }
}

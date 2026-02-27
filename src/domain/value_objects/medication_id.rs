use uuid::Uuid;

/// Unique identifier for a [`Medication`] aggregate root.
///
/// Use [`MedicationId::generate`] to create a fresh time-sortable UUID v7, or
/// `MedicationId::from(uuid)` to reconstitute an identifier from a UUID that
/// was previously persisted or received from an external source.
///
/// [`Medication`]: crate::domain::entities::medication::Medication
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::value_objects::medication_id::MedicationId;
///
/// let id = MedicationId::generate();
/// assert!(!id.to_string().is_empty());
///
/// // Every call produces a different ID.
/// assert_ne!(MedicationId::generate(), MedicationId::generate());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MedicationId(Uuid);

impl From<Uuid> for MedicationId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl MedicationId {
    /// Generates a new unique `MedicationId` using UUID v7 (time-sortable).
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the underlying [`Uuid`] value.
    pub fn value(&self) -> Uuid {
        self.0
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
    fn generate_produces_unique_ids() {
        let id_a = MedicationId::generate();
        let id_b = MedicationId::generate();

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn from_uuid_wraps_the_given_uuid() {
        let uuid = Uuid::now_v7();

        let id = MedicationId::from(uuid);

        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn value_returns_the_inner_uuid() {
        let uuid = Uuid::now_v7();
        let id = MedicationId::from(uuid);

        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn display_formats_as_uuid_string() {
        let uuid = Uuid::now_v7();
        let id = MedicationId::from(uuid);

        assert_eq!(id.to_string(), uuid.to_string());
    }
}

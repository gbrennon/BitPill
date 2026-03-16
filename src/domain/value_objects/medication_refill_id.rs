use uuid::Uuid;

/// Unique identifier for a [`MedicationRefill`] entity.
///
/// Use [`MedicationRefillId::generate`] to create a fresh time-sortable UUID v7, or
/// `MedicationRefillId::from(uuid)` to reconstitute an identifier from a UUID that
/// was previously persisted or received from an external source.
///
/// [`MedicationRefill`]: crate::domain::entities::medication_refill::MedicationRefill
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::value_objects::medication_refill_id::MedicationRefillId;
///
/// let id = MedicationRefillId::generate();
/// assert!(!id.to_string().is_empty());
///
/// // Every call produces a different ID.
/// assert_ne!(MedicationRefillId::generate(), MedicationRefillId::generate());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MedicationRefillId(Uuid);

impl From<Uuid> for MedicationRefillId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl MedicationRefillId {
    /// Generates a new unique `MedicationRefillId` using UUID v7 (time-sortable).
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the underlying [`Uuid`] value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for MedicationRefillId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_produces_unique_ids() {
        let id_a = MedicationRefillId::generate();
        let id_b = MedicationRefillId::generate();

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn from_uuid_wraps_the_given_uuid() {
        let uuid = Uuid::now_v7();

        let id = MedicationRefillId::from(uuid);

        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn value_returns_the_inner_uuid() {
        let uuid = Uuid::now_v7();
        let id = MedicationRefillId::from(uuid);

        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn display_formats_as_uuid_string() {
        let uuid = Uuid::now_v7();
        let id = MedicationRefillId::from(uuid);

        assert_eq!(id.to_string(), uuid.to_string());
    }
}

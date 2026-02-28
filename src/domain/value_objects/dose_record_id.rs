use uuid::Uuid;

/// Unique identifier for a [`DoseRecord`] aggregate.
///
/// Use [`DoseRecordId::generate`] to create a fresh time-sortable UUID v7, or
/// `DoseRecordId::from(uuid)` to reconstitute an identifier from a UUID that
/// was previously persisted or received from an external source.
///
/// [`DoseRecord`]: crate::domain::entities::dose_record::DoseRecord
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::value_objects::dose_record_id::DoseRecordId;
///
/// let id = DoseRecordId::generate();
/// assert!(!id.to_string().is_empty());
///
/// // Every call produces a different ID.
/// assert_ne!(DoseRecordId::generate(), DoseRecordId::generate());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DoseRecordId(Uuid);

impl From<Uuid> for DoseRecordId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl DoseRecordId {
    /// Generates a new unique `DoseRecordId` using UUID v7 (time-sortable).
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the underlying [`Uuid`] value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for DoseRecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_produces_unique_ids() {
        let id_a = DoseRecordId::generate();
        let id_b = DoseRecordId::generate();

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn from_uuid_wraps_the_given_uuid() {
        let uuid = Uuid::now_v7();

        let id = DoseRecordId::from(uuid);

        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn value_returns_the_inner_uuid() {
        let uuid = Uuid::now_v7();
        let id = DoseRecordId::from(uuid);

        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn display_formats_as_uuid_string() {
        let uuid = Uuid::now_v7();
        let id = DoseRecordId::from(uuid);

        assert_eq!(id.to_string(), uuid.to_string());
    }
}

use uuid::Uuid;

/// Unique identifier for a [`DoseRecord`] aggregate.
///
/// Use [`DoseRecordId::create`] to generate a fresh time-sortable UUID v7, or
/// [`DoseRecordId::from_uuid`] to reconstitute an identifier from a UUID that
/// was previously persisted or received from an external source.
///
/// [`DoseRecord`]: crate::domain::entities::dose_record::DoseRecord
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::value_objects::dose_record_id::DoseRecordId;
///
/// let id = DoseRecordId::create();
/// assert!(!id.to_string().is_empty());
///
/// // Every call produces a different ID.
/// assert_ne!(DoseRecordId::create(), DoseRecordId::create());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoseRecordId(Uuid);

impl DoseRecordId {
    /// Generates a new unique `DoseRecordId` using UUID v7 (time-sortable).
    pub fn create() -> Self {
        Self(Uuid::now_v7())
    }

    /// Wraps an existing [`Uuid`] as a `DoseRecordId`.
    ///
    /// Use this when reconstituting an identifier that was previously stored or
    /// received from an external source rather than generating a new one.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the underlying [`Uuid`] value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for DoseRecordId {
    fn default() -> Self {
        Self::create()
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
    fn create_generates_unique_ids() {
        let id_a = DoseRecordId::create();
        let id_b = DoseRecordId::create();

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn from_uuid_wraps_the_given_uuid() {
        let uuid = Uuid::now_v7();

        let id = DoseRecordId::from_uuid(uuid);

        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn display_formats_as_uuid_string() {
        let uuid = Uuid::now_v7();
        let id = DoseRecordId::from_uuid(uuid);

        assert_eq!(id.to_string(), uuid.to_string());
    }

    #[test]
    fn default_generates_a_valid_id() {
        let id = DoseRecordId::default();

        assert!(!id.to_string().is_empty());
    }
}

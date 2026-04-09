use uuid::Uuid;

/// Unique identifier for stock entries.
///
/// Use [`MedicationStockId::generate`] to create a fresh time-sortable UUID v7, or
/// `MedicationStockId::from(uuid)` to reconstitute an identifier from a UUID that
/// was previously persisted or received from an external source.
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::value_objects::medication_stock_id::MedicationStockId;
///
/// let id = MedicationStockId::generate();
/// assert!(!id.to_string().is_empty());
///
/// // Every call produces a different ID.
/// assert_ne!(MedicationStockId::generate(), MedicationStockId::generate());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MedicationStockId(Uuid);

impl From<Uuid> for MedicationStockId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl MedicationStockId {
    /// Generates a new unique `MedicationStockId` using UUID v7 (time-sortable).
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the underlying [`Uuid`] value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for MedicationStockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_produces_unique_ids() {
        let id_a = MedicationStockId::generate();
        let id_b = MedicationStockId::generate();

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn from_uuid_wraps_the_given_uuid() {
        let uuid = Uuid::now_v7();

        let id = MedicationStockId::from(uuid);

        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn value_returns_the_inner_uuid() {
        let uuid = Uuid::now_v7();
        let id = MedicationStockId::from(uuid);

        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn display_formats_as_uuid_string() {
        let uuid = Uuid::now_v7();
        let id = MedicationStockId::from(uuid);

        assert_eq!(id.to_string(), uuid.to_string());
    }
}

use uuid::Uuid;

/// Unique identifier for a [`Stock`] entity.
///
/// Use [`StockId::generate`] to create a fresh time-sortable UUID v7, or
/// `Stock:from(uuid)` to reconstitute an identifier from an UUID that
/// was previously persisted or received from an external source.
///
/// [`Stock`]: crate::domain::entities:stock::Stock
///
/// Examples
/// ```rust
/// use bitpill::domain::value_objects::stock_id::StockId
///
/// let id = StockId::generate();
/// assert!(!id.to_string().is_empty());
///
/// // Every call produces a different StockId;
/// assert_ne!(StockId::generate(), StockId::generate());
/// ```
#[derive(Debug, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct StockId(Uuid);

impl From<Uuid> for StockId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl StockId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for StockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_produces_unique_ids() {
        let id_a = StockId::generate();
        let id_b = StockId::generate();

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn value_returns_the_inner_uuid() {
        let uuid = Uuid::now_v7();
        let id = StockId::from(uuid);

        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn display_formats_as_uuid_string() {
        let uuid = Uuid::now_v7();
        let id = StockId::from(uuid);

        assert_eq!(id.to_string(), uuid.to_string());
    }
}

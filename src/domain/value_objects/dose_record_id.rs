use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoseRecordId(Uuid);

impl DoseRecordId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for DoseRecordId {
    fn default() -> Self {
        Self::new()
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
    fn new_generates_unique_ids() {
        let id_a = DoseRecordId::new();
        let id_b = DoseRecordId::new();

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn display_formats_as_uuid_string() {
        let id = DoseRecordId::new();

        assert_eq!(id.to_string(), id.value().to_string());
    }

    #[test]
    fn default_generates_a_valid_id() {
        let id = DoseRecordId::default();

        assert!(!id.to_string().is_empty());
    }
}

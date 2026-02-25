use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PillId(Uuid);

impl PillId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for PillId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PillId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_unique_ids() {
        let id_a = PillId::new();
        let id_b = PillId::new();

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn value_returns_the_inner_uuid() {
        let id = PillId::new();

        assert_eq!(id.value(), id.0);
    }

    #[test]
    fn display_formats_as_uuid_string() {
        let id = PillId::new();

        assert_eq!(id.to_string(), id.value().to_string());
    }

    #[test]
    fn default_generates_a_valid_id() {
        let id = PillId::default();

        assert!(!id.to_string().is_empty());
    }
}

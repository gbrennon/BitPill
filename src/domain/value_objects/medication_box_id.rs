use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MedicationBoxId(Uuid);

impl MedicationBoxId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl std::fmt::Display for MedicationBoxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for MedicationBoxId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<&Uuid> for MedicationBoxId {
    fn from(uuid: &Uuid) -> Self {
        Self(*uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_returns_a_new_id() {
        let id = MedicationBoxId::generate();
        assert!(!id.0.is_nil());
    }

    #[test]
    fn generate_creates_unique_ids() {
        let a = MedicationBoxId::generate();
        let b = MedicationBoxId::generate();
        assert_ne!(a, b);
    }

    #[test]
    fn from_uuid_creates_correct_id() {
        let uuid = Uuid::parse_str("018f8a2e-1111-1111-1111-111111111111").unwrap();
        let id: MedicationBoxId = uuid.into();
        assert_eq!(id.0, uuid);
    }

    #[test]
    fn from_uuid_reference_creates_correct_id() {
        let uuid = Uuid::parse_str("018f8a2e-2222-2222-2222-222222222222").unwrap();
        let id: MedicationBoxId = (&uuid).into();
        assert_eq!(id.0, uuid);
    }

    #[test]
    fn from_owned_uuid_creates_correct_id() {
        let uuid = Uuid::parse_str("018f8a2e-5555-5555-5555-555555555555").unwrap();
        let id: MedicationBoxId = uuid.into();
        assert_eq!(id.0, uuid);
    }

    #[test]
    fn display_returns_uuid_string() {
        let uuid = Uuid::parse_str("018f8a2e-0000-0000-0000-000000000001").unwrap();
        let id = MedicationBoxId::from(uuid);
        assert_eq!(id.to_string(), "018f8a2e-0000-0000-0000-000000000001");
    }

    #[test]
    fn equality_holds_for_same_uuid() {
        let uuid = Uuid::parse_str("018f8a2e-3333-3333-3333-333333333333").unwrap();
        let a = MedicationBoxId::from(uuid);
        let b = MedicationBoxId::from(uuid);
        assert_eq!(a, b);
    }

    #[test]
    fn inequality_holds_for_different_uuids() {
        let a = MedicationBoxId::generate();
        let b = MedicationBoxId::generate();
        assert_ne!(a, b);
    }

    #[test]
    fn serialization_works() {
        let id = MedicationBoxId::generate();
        let serialized = serde_json::to_string(&id).unwrap();
        let deserialized: MedicationBoxId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserialized);
    }

    #[test]
    fn clone_produces_equal_id() {
        let id = MedicationBoxId::generate();
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn hash_is_consistent_for_same_id() {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };

        let uuid = Uuid::parse_str("018f8a2e-4444-4444-4444-444444444444").unwrap();
        let id = MedicationBoxId::from(uuid);

        let mut hasher1 = DefaultHasher::new();
        id.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        id.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }
}

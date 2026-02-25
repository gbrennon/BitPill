use crate::domain::value_objects::{dosage::Dosage, pill_id::PillId, pill_name::PillName};

#[derive(Debug, Clone)]
pub struct Pill {
    id: PillId,
    name: PillName,
    dosage: Dosage,
}

impl Pill {
    pub fn new(name: PillName, dosage: Dosage) -> Self {
        Self {
            id: PillId::new(),
            name,
            dosage,
        }
    }

    pub fn id(&self) -> &PillId {
        &self.id
    }

    pub fn name(&self) -> &PillName {
        &self.name
    }

    pub fn dosage(&self) -> &Dosage {
        &self.dosage
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{dosage::Dosage, pill_name::PillName};

    fn make_pill() -> Pill {
        let name = PillName::new("Aspirin").unwrap();
        let dosage = Dosage::new(500).unwrap();
        Pill::new(name, dosage)
    }

    #[test]
    fn new_assigns_a_unique_id() {
        let pill_a = make_pill();
        let pill_b = make_pill();

        assert_ne!(pill_a.id(), pill_b.id());
    }

    #[test]
    fn name_returns_the_name_passed_to_constructor() {
        let pill = make_pill();

        assert_eq!(pill.name().value(), "Aspirin");
    }

    #[test]
    fn dosage_returns_the_dosage_passed_to_constructor() {
        let pill = make_pill();

        assert_eq!(pill.dosage().amount_mg(), 500);
    }
}

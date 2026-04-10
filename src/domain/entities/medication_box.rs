use crate::domain::value_objects::{
    dosage::Dosage, medication_box_id::MedicationBoxId, medication_id::MedicationId,
    medication_name::MedicationName,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MedicationBox {
    id: MedicationBoxId,
    medication_id: MedicationId,
    name: MedicationName,
    pills_per_box: u16,
    dosage: Dosage,
}

impl MedicationBox {
    pub fn new(
        medication_id: MedicationId,
        name: MedicationName,
        pills_per_box: u16,
        dosage: Dosage,
    ) -> Self {
        Self {
            id: MedicationBoxId::generate(),
            medication_id,
            name,
            pills_per_box,
            dosage,
        }
    }

    pub fn with_id(
        id: MedicationBoxId,
        medication_id: MedicationId,
        name: MedicationName,
        pills_per_box: u16,
        dosage: Dosage,
    ) -> Self {
        Self {
            id,
            medication_id,
            name,
            pills_per_box,
            dosage,
        }
    }

    pub fn id(&self) -> &MedicationBoxId {
        &self.id
    }

    pub fn medication_id(&self) -> &MedicationId {
        &self.medication_id
    }

    pub fn name(&self) -> &MedicationName {
        &self.name
    }

    pub fn pills_per_box(&self) -> u16 {
        self.pills_per_box
    }

    pub fn dosage(&self) -> &Dosage {
        &self.dosage
    }

    pub fn dosage_mg(&self) -> u32 {
        self.dosage.amount_mg()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::medication_id::MedicationId;

    fn make_dosage() -> Dosage {
        Dosage::new(500).unwrap()
    }

    fn make_name() -> MedicationName {
        MedicationName::new("Standard 30-pack").unwrap()
    }

    fn make_box() -> MedicationBox {
        MedicationBox::new(MedicationId::generate(), make_name(), 30, make_dosage())
    }

    #[test]
    fn new_assigns_unique_id() {
        let a = make_box();
        let b = make_box();
        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn new_stores_medication_id() {
        let med_id = MedicationId::generate();
        let medication_box = MedicationBox::new(med_id.clone(), make_name(), 30, make_dosage());
        assert_eq!(medication_box.medication_id(), &med_id);
    }

    #[test]
    fn new_stores_name() {
        let name = MedicationName::new("My Box").unwrap();
        let medication_box = MedicationBox::new(MedicationId::generate(), name, 30, make_dosage());
        assert_eq!(
            medication_box.name(),
            &MedicationName::new("My Box").unwrap()
        );
    }

    #[test]
    fn new_stores_pills_per_box() {
        let medication_box =
            MedicationBox::new(MedicationId::generate(), make_name(), 60, make_dosage());
        assert_eq!(medication_box.pills_per_box(), 60);
    }

    #[test]
    fn new_stores_dosage() {
        let dosage = Dosage::new(250).unwrap();
        let medication_box =
            MedicationBox::new(MedicationId::generate(), make_name(), 30, dosage.clone());
        assert_eq!(medication_box.dosage(), &dosage);
    }

    #[test]
    fn dosage_mg_returns_correct_value() {
        let medication_box = MedicationBox::new(
            MedicationId::generate(),
            make_name(),
            30,
            Dosage::new(500).unwrap(),
        );
        assert_eq!(medication_box.dosage_mg(), 500);
    }

    #[test]
    fn with_id_uses_supplied_id() {
        let id = MedicationBoxId::generate();
        let medication_box = MedicationBox::with_id(
            id.clone(),
            MedicationId::generate(),
            make_name(),
            30,
            make_dosage(),
        );
        assert_eq!(medication_box.id(), &id);
    }

    #[test]
    fn clone_produces_equal_box() {
        let original = make_box();
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn equality_holds_for_same_id() {
        let med_id = MedicationId::generate();
        let box_id = MedicationBoxId::generate();
        let dosage = Dosage::new(500).unwrap();
        let name = MedicationName::new("Test").unwrap();
        let a = MedicationBox::with_id(
            box_id.clone(),
            med_id.clone(),
            name.clone(),
            30,
            dosage.clone(),
        );
        let b = MedicationBox::with_id(box_id, med_id, name, 30, dosage);
        assert_eq!(a, b);
    }

    #[test]
    fn inequality_holds_for_different_ids() {
        let med_id = MedicationId::generate();
        let a = MedicationBox::new(med_id.clone(), make_name(), 30, Dosage::new(500).unwrap());
        let b = MedicationBox::new(med_id, make_name(), 30, Dosage::new(500).unwrap());
        assert_ne!(a, b);
    }

    #[test]
    fn inequality_holds_for_different_names() {
        let med_id = MedicationId::generate();
        let name_a = MedicationName::new("Box A").unwrap();
        let name_b = MedicationName::new("Box B").unwrap();
        let a = MedicationBox::new(med_id.clone(), name_a, 30, Dosage::new(500).unwrap());
        let b = MedicationBox::new(med_id, name_b, 30, Dosage::new(500).unwrap());
        assert_ne!(a, b);
    }

    #[test]
    fn serialization_works() {
        let medication_box = make_box();
        let serialized = serde_json::to_string(&medication_box).unwrap();
        let deserialized: MedicationBox = serde_json::from_str(&serialized).unwrap();
        assert_eq!(medication_box, deserialized);
    }
}

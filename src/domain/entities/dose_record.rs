use chrono::NaiveDateTime;

use crate::domain::{
    errors::DomainError,
    value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
};

#[derive(Debug, Clone)]
pub struct DoseRecord {
    id: DoseRecordId,
    medication_id: MedicationId,
    scheduled_at: NaiveDateTime,
    taken_at: Option<NaiveDateTime>,
}

impl DoseRecord {
    pub fn new(medication_id: MedicationId, scheduled_at: NaiveDateTime) -> Self {
        Self {
            id: DoseRecordId::new(),
            medication_id,
            scheduled_at,
            taken_at: None,
        }
    }

    pub fn mark_taken(&mut self, at: NaiveDateTime) -> Result<(), DomainError> {
        if self.taken_at.is_some() {
            return Err(DomainError::DoseAlreadyTaken);
        }
        self.taken_at = Some(at);
        Ok(())
    }

    pub fn is_taken(&self) -> bool {
        self.taken_at.is_some()
    }

    pub fn id(&self) -> &DoseRecordId {
        &self.id
    }

    pub fn medication_id(&self) -> &MedicationId {
        &self.medication_id
    }

    pub fn scheduled_at(&self) -> NaiveDateTime {
        self.scheduled_at
    }

    pub fn taken_at(&self) -> Option<NaiveDateTime> {
        self.taken_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn make_datetime(h: u32, m: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }

    fn make_dose_record() -> DoseRecord {
        DoseRecord::new(MedicationId::new(), make_datetime(8, 0))
    }

    #[test]
    fn new_creates_record_as_not_taken() {
        let record = make_dose_record();

        assert!(!record.is_taken());
        assert!(record.taken_at().is_none());
    }

    #[test]
    fn new_assigns_a_unique_id() {
        let record_a = make_dose_record();
        let record_b = make_dose_record();

        assert_ne!(record_a.id(), record_b.id());
    }

    #[test]
    fn mark_taken_sets_taken_at_and_marks_as_taken() {
        let mut record = make_dose_record();
        let taken_at = make_datetime(8, 5);

        record.mark_taken(taken_at).unwrap();

        assert!(record.is_taken());
        assert_eq!(record.taken_at(), Some(taken_at));
    }

    #[test]
    fn mark_taken_twice_returns_error() {
        let mut record = make_dose_record();
        record.mark_taken(make_datetime(8, 5)).unwrap();

        let result = record.mark_taken(make_datetime(8, 10));

        assert!(matches!(result, Err(DomainError::DoseAlreadyTaken)));
    }

    #[test]
    fn scheduled_at_returns_the_datetime_passed_to_constructor() {
        let scheduled_at = make_datetime(20, 0);
        let record = DoseRecord::new(MedicationId::new(), scheduled_at);

        assert_eq!(record.scheduled_at(), scheduled_at);
    }
}

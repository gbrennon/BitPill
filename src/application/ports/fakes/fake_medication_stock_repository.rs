use std::sync::Mutex;

use crate::{
    application::{
        errors::StorageError,
        ports::medication_stock_repository_port::MedicationStockRepositoryPort,
    },
    domain::{
        entities::medication_stock::MedicationStock, value_objects::medication_id::MedicationId,
    },
};

pub struct FakeMedicationStockRepository {
    stocks: Mutex<Vec<MedicationStock>>,
    fail_on_save: bool,
    fail_on_find: bool,
}

impl Default for FakeMedicationStockRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeMedicationStockRepository {
    pub fn new() -> Self {
        Self {
            stocks: Mutex::new(Vec::new()),
            fail_on_save: false,
            fail_on_find: false,
        }
    }

    pub fn with(stock: MedicationStock) -> Self {
        Self {
            stocks: Mutex::new(vec![stock]),
            fail_on_save: false,
            fail_on_find: false,
        }
    }

    pub fn failing() -> Self {
        Self {
            stocks: Mutex::new(Vec::new()),
            fail_on_save: true,
            fail_on_find: false,
        }
    }

    pub fn failing_on_find() -> Self {
        Self {
            stocks: Mutex::new(Vec::new()),
            fail_on_save: false,
            fail_on_find: true,
        }
    }

    pub fn saved_count(&self) -> usize {
        self.stocks.lock().unwrap().len()
    }
}

impl MedicationStockRepositoryPort for FakeMedicationStockRepository {
    fn save(&self, medication_stock: &MedicationStock) -> Result<(), StorageError> {
        if self.fail_on_save {
            return Err(StorageError("forced failure".into()));
        }
        let mut stocks = self.stocks.lock().unwrap();
        if let Some(existing) = stocks.iter_mut().find(|s| s.id() == medication_stock.id()) {
            *existing = medication_stock.clone();
        } else {
            stocks.push(medication_stock.clone());
        }
        Ok(())
    }

    fn find_by_medication_id(
        &self,
        medication_id: &MedicationId,
    ) -> Result<Option<MedicationStock>, StorageError> {
        if self.fail_on_find {
            return Err(StorageError("forced failure".into()));
        }
        Ok(self
            .stocks
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.medication_id() == medication_id)
            .cloned())
    }
}

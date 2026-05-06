use std::sync::Arc;

use crate::application::ports::inbound::{
    create_medication_port::CreateMedicationPort, update_medication_port::UpdateMedicationPort,
};

pub struct FormHandlerDeps {
    pub create_medication: Arc<dyn CreateMedicationPort>,
    pub update_medication: Arc<dyn UpdateMedicationPort>,
}

impl FormHandlerDeps {
    pub fn new(
        create_medication: Arc<dyn CreateMedicationPort>,
        update_medication: Arc<dyn UpdateMedicationPort>,
    ) -> Self {
        Self {
            create_medication,
            update_medication,
        }
    }
}

use crate::{
    application::errors::DeliveryError,
    domain::entities::{dose_record::DoseRecord, medication::Medication},
};

/// Abstracts the delivery of dose-reminder notifications to the user.
///
/// Implement this trait in the infrastructure layer (console, push, email, …).
/// Inject via `Arc<dyn NotificationPort>` into `ScheduleDoseService`.
pub trait NotificationPort: Send + Sync {
    /// Notify the user that a dose of `medication` is due now.
    ///
    /// `record` is the freshly created [`DoseRecord`] for this dose slot.
    fn notify_dose_due(
        &self,
        medication: &Medication,
        record: &DoseRecord,
    ) -> Result<(), DeliveryError>;
}

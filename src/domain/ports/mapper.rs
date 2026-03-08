use crate::domain::errors::DomainError;

/// Generic, domain-level mapper trait.
///
/// `T` is the **output** type this mapper produces, declared at the `impl` site so that
/// `impl Mapper<Medication> for CreateMedicationMapper` is self-documenting.
/// `Source` is the associated type that defines what the mapper consumes.
///
/// Lives in the domain layer — no I/O, no external dependencies.
/// Concrete implementations belong in the application or infrastructure layers.
///
/// The trait is `Send + Sync` so it can be stored behind `Arc<dyn Mapper<T>>`.
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::ports::mapper::Mapper;
/// use bitpill::domain::errors::DomainError;
///
/// struct UpperMapper;
///
/// impl Mapper<String> for UpperMapper {
///     type Source = String;
///     fn map(&self, src: String) -> Result<String, DomainError> {
///         Ok(src.to_uppercase())
///     }
/// }
/// ```
pub trait Mapper<T>: Send + Sync {
    /// The input type consumed by this mapper.
    type Source;

    /// Map a value of type [`Self::Source`] into `T`.
    ///
    /// Takes `Source` by value. Callers that need to retain ownership should clone
    /// before calling.
    fn map(&self, src: Self::Source) -> Result<T, DomainError>;
}

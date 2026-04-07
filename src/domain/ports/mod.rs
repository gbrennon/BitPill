/// Domain ports — abstractions that define how the domain interacts with the outside world.
///
/// Ports define interfaces for external dependencies (repositories, mappers,
/// external services) without prescribing how they are implemented.
/// Implementations belong in the application or infrastructure layers.
///
/// # Ports
///
/// - [`mapper::Mapper`] — Generic trait for transforming domain types.
pub mod mapper;

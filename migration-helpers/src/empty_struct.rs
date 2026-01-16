pub use see_migration_test_helpers_derive::EmptyStruct;

/// Trait for creating instances of unit structs.
///
/// This trait is required for migrations used with `MigratorBeforeTested` and `MigratorWithTested`.
/// Use the `#[derive(EmptyStruct)]` macro to automatically implement this trait.
///
/// # Example
///
/// ```rust
/// use see_migration_test_helpers_derive::EmptyStruct;
///
/// #[derive(EmptyStruct)]
/// pub struct m20240101_000001_create_users;
/// ```
pub trait EmptyStruct {
    fn new() -> Self;
}

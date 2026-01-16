# see-migration-test-helpers

Helper types for testing [sea-orm-migration](https://crates.io/crates/sea-orm-migration) migrations in isolation.

## Elements

- **`MigratorBeforeTested`** - Run migrations up to (but not including) a specific migration
- **`MigratorWithTested`** - Run migrations including a specific migration
- **`EmptyStruct` derive macro** - Automatically implement the `EmptyStruct` trait for unit structs (needed for using `MigratorBeforeTested`/`MigratorWithTested`)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
see-migration-test-helpers = "1"
```

## Example

```rust
use see_migration_test_helpers::{MigratorBeforeTested, MigratorWithTested};

// Define type aliases for your test
type MigratorBefore = MigratorBeforeTested<crate::Migrator, super::Migration>;
type MigratorAfter = MigratorWithTested<crate::Migrator, super::Migration>;

#[async_std::test]
async fn test_my_migration() {
    // Set up database with all migrations before the one being tested
    let db = {
        () // Setup connection
    };
    MigratorBefore::up(&db, None)
        .await
        .expect("Initial migration failed");
    
    // Insert test data that should exist before the migration
    db.execute_unprepared("INSERT INTO users (id, email) VALUES (1, 'test@example.com')")
        .await
        .unwrap();
    
    // Run the migration being tested
    MigratorAfter::up(&db, None)
        .await
        .expect("Migration failed");
    
    // Verify the migration worked correctly
    // ... your assertions ...
    
    // Test rollback
    MigratorAfter::down(&db, Some(1))
        .await
        .expect("Rollback failed");
}
```

## Migration Requirements

Your migration struct must implement `EmptyStruct`. Use the derive macro:

```rust
use see_migration_test_helpers::EmptyStruct;

#[derive(EmptyStruct)]
pub struct m20240101_000001_create_users;

// ... migration implementation ...
```

## License

MIT

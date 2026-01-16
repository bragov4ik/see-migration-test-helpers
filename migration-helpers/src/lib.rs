//! Helper types for testing [sea-orm-migration](https://crates.io/crates/sea-orm-migration) migrations in isolation.
//!
//! ## Example
//!
//! ```rust,no_run
//! use see_migration_test_helpers::{MigratorBeforeTested, MigratorWithTested};
//!
//! // Define type aliases for your test
//! type MigratorBefore = MigratorBeforeTested<crate::Migrator, super::Migration>;
//! type MigratorAfter = MigratorWithTested<crate::Migrator, super::Migration>;
//!
//! # async fn example() {
//! // Set up database with all migrations before the one being tested
//! let db = {
//!     // Setup connection
//!     todo!()
//! };
//! MigratorBefore::up(&db, None)
//!     .await
//!     .expect("Initial migration failed");
//!
//! // Insert test data that should exist before the migration
//! // db.execute_unprepared("INSERT INTO users (id, email) VALUES (1, 'test@example.com')")
//! //     .await
//! //     .unwrap();
//!
//! // Run the migration being tested
//! MigratorAfter::up(&db, None)
//!     .await
//!     .expect("Migration failed");
//!
//! // Verify the migration worked correctly
//! // ... your assertions ...
//!
//! // Test rollback
//! MigratorAfter::down(&db, Some(1))
//!     .await
//!     .expect("Rollback failed");
//! # }
//! ```

mod empty_struct;
mod migrators;

pub use empty_struct::*;
pub use migrators::*;

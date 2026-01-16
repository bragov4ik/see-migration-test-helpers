use std::marker::PhantomData;

use sea_orm_migration::prelude::*;

use super::EmptyStruct;

/// A migrator that runs all migrations up to (but not including) a specific migration.
///
/// Use this to set up your database state before testing a specific migration.
///
/// See an example in [crate documentation](crate).
pub struct MigratorBeforeTested<FullMigrator, TestedMigration>(
    PhantomData<FullMigrator>,
    PhantomData<TestedMigration>,
);

#[async_trait::async_trait]
impl<FullMigrator, TestedMigration> MigratorTrait
    for MigratorBeforeTested<FullMigrator, TestedMigration>
where
    FullMigrator: MigratorTrait,
    TestedMigration: MigrationName + EmptyStruct + Send,
{
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        FullMigrator::migrations()
            .into_iter()
            .take_while(|m| m.name() != TestedMigration::new().name())
            .collect()
    }
}

/// A migrator that runs all migrations including a specific migration.
///
/// Use this to apply the tested migration after setting up the database with `MigratorBeforeTested`.
///
/// See an example in [crate documentation](crate).
pub struct MigratorWithTested<FullMigrator, TestedMigration>(
    PhantomData<FullMigrator>,
    PhantomData<TestedMigration>,
);

#[async_trait::async_trait]
impl<FullMigrator, TestedMigration> MigratorTrait
    for MigratorWithTested<FullMigrator, TestedMigration>
where
    FullMigrator: MigratorTrait,
    TestedMigration: MigrationTrait + EmptyStruct + Send + 'static,
{
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut m = MigratorBeforeTested::<FullMigrator, TestedMigration>::migrations();
        m.push(Box::new(TestedMigration::new()));
        m
    }
}

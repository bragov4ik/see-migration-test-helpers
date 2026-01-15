use std::marker::PhantomData;

use sea_orm_migration::prelude::*;

use super::EmptyStruct;

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

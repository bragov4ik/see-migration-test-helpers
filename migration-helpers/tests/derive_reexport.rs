use see_migration_test_helpers::EmptyStruct;

#[derive(EmptyStruct)]
struct TestStruct;

#[test]
fn derive_reexport_works() {
    let _ = TestStruct::new();
}

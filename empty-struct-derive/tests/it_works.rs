use see_migration_test_helpers::EmptyStruct;
// use see_migration_test_helpers_derive::EmptyStruct;

#[derive(EmptyStruct, PartialEq, Debug)]
struct TestStruct;

#[test]
fn derive_works_internally() {
    let instance = TestStruct::new();
    assert_eq!(instance, TestStruct);
}

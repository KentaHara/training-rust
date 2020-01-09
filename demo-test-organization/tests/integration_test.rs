use demo_test_organization;

mod common;

#[test]
fn it_adds_two() {
	assert_eq!(4, demo_test_organization::add_two(2));
}

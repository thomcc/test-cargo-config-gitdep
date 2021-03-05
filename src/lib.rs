pub fn foo() -> bool {
    cfg!(foo)
}
#[test]
fn check() {
    assert!(foo());
}

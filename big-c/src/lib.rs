extern "C" {
    fn foo_10000();
}

pub fn foo() {
    unsafe {
        foo_10000();
    }
}

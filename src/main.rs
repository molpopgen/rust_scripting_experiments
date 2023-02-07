struct CoreAPIType {
    value: i32,
}

impl Default for CoreAPIType {
    fn default() -> Self {
        Self { value: 6 }
    }
}

impl CoreAPIType {
    fn get_value(&self) -> i32 {
        self.value
    }
}

trait SquareValue {
    // Function is unsafe because a pointer will have to
    // be dereferenced.
    unsafe fn square(&self, api: *const CoreAPIType) -> i32;
}

struct RustCallBack {}

impl SquareValue for RustCallBack {
    unsafe fn square(&self, api: *const CoreAPIType) -> i32 {
        assert!(!api.is_null());
        // SAFETY: api is not null
        (*api).get_value() * (*api).get_value()
    }
}

struct RuneCallback {}

fn api_function_with_callback<C: SquareValue>(api: &CoreAPIType, callback: &C) {
    let x = api.get_value();
    // SAFETY: we are creating our const pointer from a shared
    // reference, which is always safe.
    let squared = unsafe { callback.square(api as *const CoreAPIType) };
    assert_eq!(squared, x * x);
}

fn work() {
    let api = CoreAPIType::default();
    api_function_with_callback(&api, &RustCallBack {});
}

fn main() {
    work();
}

#[test]
fn test_things() {
    work();
}

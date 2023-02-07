use std::sync::Arc;

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

    // Exploratory hacking re: rune
    // The hints here are important:
    // https://rune-rs.github.io/book/external_types.html
    let rune_code = r###"
        pub fn callback_body(value) {
            value * value
        }
    "###;

    // This doesn't seem to be the mechanism
    // to get a string literal into something
    // we can call via rust
    let mut sources = rune::sources! {
        entry => {
        pub fn callback_body(value) {
            value * value
        }
        }
    };

    let context = rune_modules::default_context().unwrap();
    let result = rune::prepare(&mut sources).with_context(&context).build();

    let unit = result.unwrap();

    let mut vm = rune::Vm::new(Arc::new(context.runtime()), Arc::new(unit));
    let output = vm
        .execute(["callback_body"], (api.get_value(),))
        .unwrap()
        .complete()
        .unwrap();
    let result = match output {
        rune::Value::Integer(x) => x,
        _ => panic!("we don't understand this"),
    };
    assert_eq!(result as i32, api.get_value() * api.get_value());
}

fn main() {
    work();
}

#[test]
fn test_things() {
    work();
}

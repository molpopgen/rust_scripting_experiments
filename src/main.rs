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

// Rune interface
#[derive(rune::Any)]
#[repr(transparent)]
struct RuneCoreAPIType(*const CoreAPIType);

impl RuneCoreAPIType {
    fn get_value(&self) -> i32 {
        assert!(!self.0.is_null());
        // SAFETY: it is not NULL
        unsafe { (*self.0).get_value() }
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

    // Code in string literal
    let rune_code = r###"
        pub fn callback_body(item) {
            item.get_value * item.get_value
        }
    "###;

    let mut sources = rune::Sources::new();
    let source = rune::Source::new("foo", rune_code);
    sources.insert(source);

    let mut context = rune::Context::new();
    let mut m = rune::Module::new();
    m.field_fn(
        rune::runtime::Protocol::GET,
        "get_value",
        RuneCoreAPIType::get_value,
    )
    .unwrap();
    m.ty::<RuneCoreAPIType>().unwrap();
    context.install(&m).unwrap();
    let result = rune::prepare(&mut sources).with_context(&context).build();

    let unit = result.unwrap();

    let mut vm = rune::Vm::new(Arc::new(context.runtime()), Arc::new(unit));
    let wrapper = RuneCoreAPIType(&api as *const CoreAPIType);
    let output = vm
        .execute(["callback_body"], (wrapper,))
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

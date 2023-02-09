use corelib::{CoreAPIType, RuneCoreAPIType, SquareValue};
use std::sync::Arc;

struct RustCallBack {}

// Stuff to move into a lua_api module of the back-end lib

struct LuaWrapper {
    data: mlua::LightUserData,
}

impl LuaWrapper {
    // Again, a lot not to like here,
    // but at least we can pass in a const
    // ref to a rust API type.
    // The down side is we have to promise
    // that we never break this contract,
    // so we are basically just writing C.
    pub fn new(api: &CoreAPIType) -> Self {
        let ptr: *const std::ffi::c_void = api as *const _ as *const std::ffi::c_void;
        let ptr: *mut std::ffi::c_void = ptr as _;
        Self {
            data: mlua::LightUserData(ptr),
        }
    }
}

impl mlua::UserData for LuaWrapper {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_value", |_, wrapper, ()| {
            assert!(!wrapper.data.0.is_null());
            // Wow, there's a lot not to like there
            // SAFETY: it ain't null
            let value = unsafe { (*(wrapper.data.0 as *const CoreAPIType)).get_value() };
            Ok(value)
        });
    }
}

impl SquareValue for RustCallBack {
    unsafe fn square(&self, api: *const CoreAPIType) -> i32 {
        assert!(!api.is_null());
        // SAFETY: api is not null
        (*api).get_value() * (*api).get_value()
    }
}

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
            println!("rune got a value of {}", item.get_value);
            item.get_value * item.get_value
        }
    "###;

    let mut sources = rune::Sources::new();
    let source = rune::Source::new("foo", rune_code);
    sources.insert(source);

    // We need rune_modules default context so that
    // stuff like io is allowed.
    // A vanilla rune::Context has NO functionality
    // from the rust std lib.
    let mut context = rune_modules::default_context().unwrap(); //rune::Context::new();
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
    let wrapper = RuneCoreAPIType::new(&api);
    let output = vm
        .execute(["callback_body"], (wrapper,))
        .unwrap()
        .complete()
        .unwrap();
    let result: i32 = rune::FromValue::from_value(output).unwrap();
    assert_eq!(result, api.get_value() * api.get_value());

    // Now, lua!
    let lua = mlua::Lua::new();

    let lua_callback = r###"
        function callback_body(item)
            print("lua got ", item:get_value());
            return item:get_value() * item:get_value()
        end
        "###;

    let globals = lua.globals();

    // Unclear if this is correct
    lua.load(lua_callback).exec().unwrap();

    globals
        .set(
            "data_from_rust",
            lua.create_userdata(LuaWrapper::new(&api)).unwrap(),
        )
        .unwrap();

    lua.create_table().unwrap();

    let result = lua
        .load("callback_body(data_from_rust)")
        .eval::<i32>()
        .unwrap();
    assert_eq!(result, api.get_value() * api.get_value());
}

fn main() {
    work();
}

#[test]
fn test_things() {
    work();
}

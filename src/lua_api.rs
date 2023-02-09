use crate::CoreAPIType;

#[repr(transparent)]
pub struct LuaCoreAPIType(mlua::LightUserData);

impl LuaCoreAPIType {
    // Again, a lot not to like here,
    // but at least we can pass in a const
    // ref to a rust API type.
    // The down side is we have to promise
    // that we never break this contract,
    // so we are basically just writing C.
    pub fn new(api: &CoreAPIType) -> Self {
        let ptr: *const std::ffi::c_void = api as *const _ as *const std::ffi::c_void;
        let ptr: *mut std::ffi::c_void = ptr as _;
        Self(mlua::LightUserData(ptr))
    }
}

impl mlua::UserData for LuaCoreAPIType {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_value", |_, wrapper, ()| {
            assert!(!wrapper.0 .0.is_null());
            // Wow, there's a lot not to like there
            // SAFETY: it ain't null
            let value = unsafe { (*(wrapper.0 .0 as *const CoreAPIType)).get_value() };
            Ok(value)
        });
    }
}

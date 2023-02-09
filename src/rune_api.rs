use crate::CoreAPIType;

#[derive(rune::Any)]
pub struct RuneCoreAPIType(*const CoreAPIType);

impl RuneCoreAPIType {
    pub fn new(api: &CoreAPIType) -> Self {
        Self(api as *const CoreAPIType)
    }

    pub fn get_value(&self) -> i32 {
        assert!(!self.0.is_null());
        // SAFETY: pointer is not null
        unsafe{(*self.0).get_value()}
    }
}

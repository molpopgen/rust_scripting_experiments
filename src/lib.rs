pub struct CoreAPIType {
    value: i32,
}

impl Default for CoreAPIType {
    fn default() -> Self {
        Self { value: 6 }
    }
}

impl CoreAPIType {
    pub fn get_value(&self) -> i32 {
        self.value
    }
}

pub trait SquareValue {
    // Function is unsafe because a pointer will have to
    // be dereferenced.
    /// # Safety
    ///
    /// Client code must ensure that api is not null
    unsafe fn square(&self, api: *const CoreAPIType) -> i32;
}

mod rune_api;

pub use rune_api::RuneCoreAPIType;

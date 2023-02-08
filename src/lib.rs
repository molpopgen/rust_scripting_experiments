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

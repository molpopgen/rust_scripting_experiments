struct CoreAPIType {
    value: i32,
}

impl CoreAPIType {
    fn get_value(&self) -> i32 {
        self.value
    }
}

trait SquareValue {
    fn square(&self, api: &CoreAPIType) -> i32;
}

struct RustCallBack {}

impl SquareValue for RustCallBack {
    fn square(&self, api: &CoreAPIType) -> i32 {
        api.get_value() * api.get_value()
    }
}

fn main() {
    println!("Hello, world!");
}

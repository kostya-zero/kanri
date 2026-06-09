pub mod fs;
pub mod os;
pub mod project;

#[macro_export]
macro_rules! runtime_error {
    ($e:expr) => {
        return Err(mlua::Error::runtime($e))
    };
}

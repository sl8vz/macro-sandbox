pub use sandbox_macro::Describe;

pub trait Describe {
    fn describe(&self) -> String;
}

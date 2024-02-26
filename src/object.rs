pub mod blob;
pub mod tree;

pub trait Object {
    fn digest(&self) -> &str;
    fn t<'a>(&self) -> &'a str;
}

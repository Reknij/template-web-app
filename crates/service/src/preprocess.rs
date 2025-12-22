pub mod user;

pub trait Preprocess {
    async fn process(&mut self) -> crate::Result<()>;
}

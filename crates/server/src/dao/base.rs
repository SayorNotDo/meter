pub trait BaseDap<T> {
    async fn insert(&self, object: &T) -> Result<(), CustomError>;
}

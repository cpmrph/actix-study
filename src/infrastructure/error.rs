use crate::domain::error::RepositoryError;

#[derive(Debug)]
pub struct InMemoryRepositoryError(RepositoryError);

impl InMemoryRepositoryError {
    pub fn into_inner(self) -> RepositoryError {
        self.0
    }
}

impl From<String> for InMemoryRepositoryError {
    fn from(message: String) -> Self {
        InMemoryRepositoryError(RepositoryError { message })
    }
}

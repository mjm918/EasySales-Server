use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("invalid schema provided `{0}`")]
    InvalidSchema(String),
    #[error("malformed schema")]
    MalformedSchema,
    #[error("invalid naming (expected {expected:?}, got {found:?})")]
    InvalidPropNaming {
        expected: String,
        found: String,
    },
    #[error("invalid type declaration for field `{0}`")]
    TypeDeclarationWrong(String),
    #[error("invalid declaration of properties")]
    MalformedPropertyDeclaration,
    #[error("unknown object `{0}`")]
    UnknownObject(String)
}
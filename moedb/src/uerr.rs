use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("invalid schema provided `{0}`")]
    InvalidSchema(String),
    #[error("malformed schema")]
    MalformedSchema,
    #[error("invalid naming (expected {expected:?}, outcome {found:?})")]
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

#[derive(Error, Debug)]
pub enum StatementError {
    #[error("invalid statement provided `{0}`")]
    InvalidStatement(String),
    #[error("unknown command `{0}`")]
    UnknownCommand(String),
}
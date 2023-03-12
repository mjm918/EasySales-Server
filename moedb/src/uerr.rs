use serde_derive::{Deserialize, Serialize};
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

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum StatementError {
    #[error("invalid statement provided `{0}`")]
    InvalidStatement(String),
    #[error("unknown command `{0}`")]
    UnknownCommand(String),
    #[error("no database selected")]
    NoDatabaseSelected,
    #[error("no store selected")]
    NoStoreSelected,
    #[error("no store schema provided for `{0}`")]
    NoStoreSchemaProvided(String),
    #[error("database already exists `{0}`")]
    DatabaseExists(String),
    #[error("store already exists `{0}`")]
    StoreExists(String),
    #[error("schema error {0}")]
    SchemaError(String),
    #[error("no argument is provided. must be a type of keyValue pair where key is either command or store prop")]
    NoArgumentWhileGet
}
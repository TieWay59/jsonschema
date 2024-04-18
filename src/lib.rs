//!  One-off validation:
//!
//! ```rust
//! #[cfg(feature = "serde_json")]
//! async fn test() -> Result<(), jsonschema::Error> {
//!     use serde_json::json;
//!
//!     let schema = json!({"type": "integer"});
//!     let instance = json!("a");
//!     jsonschema::is_valid(&schema, &instance).await?;
//!     Ok(())
//! }
//! ```
//!
//! ```rust
//! use jsonschema::format;
//!
//! #[cfg(feature = "serde_json")]
//! async fn test() -> Result<(), jsonschema::Error> {
//!     use serde_json::json;
//!
//!     let schema = json!({"type": "integer"});
//!     let instance = json!("a");
//!     let validator = jsonschema::Validator::from_schema(&schema).await?;
//!     assert!(!validator.is_valid(&instance));
//!     assert!(validator.validate(&instance).is_err());
//!     for error in validator.iter_errors(&instance) {
//!
//!     }
//!     let verbose = validator.collect_output(&instance, format::Verbose);
//!     let v = serde_json::to_string(&verbose).unwrap();
//!     for unit in validator.iter_output_units(&instance, format::Verbose) {
//!
//!     }
//!     for error in jsonschema::iter_errors(&instance, &schema).await? {
//!
//!     }
//!     Ok(())
//! }
//! ```
mod compiler;
mod drafts;
mod error;
pub mod format;
mod validation;

pub use crate::{
    drafts::{draft04, Draft},
    error::{Error, SchemaError, ValidationError},
    validation::{is_valid, validate, JsonSchemaValidator, ValidationState},
};
use drafts::{draft04::Draft04, Autodetect};

pub type Draft4Validator = validation::Validator<Draft04>;
pub type Validator = validation::Validator<Autodetect>;

pub mod blocking {
    use jsonlike::Json;

    use crate::{drafts::Draft04, validation, Error};

    pub type Draft4Validator = validation::blocking::Validator<Draft04>;
    pub type Validator = validation::blocking::Validator;

    pub fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), Error> {
        let validator = Validator::from_schema(schema)?;
        todo!()
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_from_schema() {
        let schema = json!({"type": "integer"});
        let validator = Draft4Validator::from_schema(&schema)
            .await
            .expect("Invalid schema");
    }

    #[test]
    fn test_from_schema_blocking() {
        let schema = json!({"type": "integer"});
        let validator = blocking::Draft4Validator::from_schema(&schema).expect("Invalid schema");
    }

    #[tokio::test]
    async fn test_options() {
        let schema = json!({"type": "integer"});
        let validator = Draft4Validator::options()
            .build(&schema)
            .await
            .expect("Invalid schema");
    }

    #[test]
    fn test_options_blocking() {
        let schema = json!({"type": "integer"});
        let validator = blocking::Draft4Validator::options()
            .build(&schema)
            .expect("Invalid schema");
    }
}

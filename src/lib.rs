//!
//! ```rust
//! use jsonschema::format;
//!
//! #[cfg(feature = "serde_json")]
//! async fn test() -> Result<(), jsonschema::Error> {
//!     let schema = serde_json::json!({"type": "integer"});
//!     let instance = serde_json::json!("a");
//!
//!     // One-off validation with a boolean result
//!     jsonschema::is_valid(&instance, &schema).await?;
//!     jsonschema::blocking::is_valid(&instance, &schema)?;
//!     // One-off with the first error as `Result<(), jsonschema::Error>`
//!     jsonschema::validate(&instance, &schema).await?;
//!     jsonschema::blocking::validate(&instance, &schema)?;
//!     // One-off iteration over errors
//!     for error in jsonschema::iter_errors(&instance, &schema).await? {
//!         println!("{}", error);
//!     }
//!     for error in jsonschema::blocking::iter_errors(&instance, &schema)? {
//!         println!("{}", error);
//!     }
//!     // One-off collecting validation results into a struct conforming to the JSON Schema "Verbose" output format
//!     let verbose = jsonschema::collect_output(&instance, &schema, format::Verbose).await?;
//!     // Serialize validation output to JSON (requires the `serde` feature)
//!     #[cfg(feature = "serde")]
//!     {
//!         let serialized = serde_json::to_string(&verbose).unwrap();
//!     }
//!     let verbose = jsonschema::blocking::collect_output(&instance, &schema, format::Verbose)?;
//!     #[cfg(feature = "serde")]
//!     {
//!         let serialized = serde_json::to_string(&verbose).unwrap();
//!     }
//!     // One-off iteration over validation results
//!     for unit in jsonschema::iter_output_units(&instance, &schema, format::Verbose).await? {
//!         println!("{:?}", unit);
//!     }
//!
//!     // Async by default, autodetect draft based on the `$schema` property
//!     let validator = jsonschema::validator_for(&schema).await?;
//!     let validator = jsonschema::blocking::validator_for(&schema)?;
//!     // Specific draft
//!     let validator = jsonschema::Draft4Validator::from_schema(&schema).await?;
//!     let validator = jsonschema::blocking::Draft4Validator::from_schema(&schema)?;
//!
//!     // Boolean result
//!     assert!(!validator.is_valid(&instance));
//!     // First error as `Result<(), jsonschema::Error>`
//!     assert!(validator.validate(&instance).is_err());
//!
//!     // Iterate over errors
//!     for error in validator.iter_errors(&instance) {
//!         println!("{}", error);
//!     }
//!
//!     // Collecting validation results into a struct conforming to the JSON Schema "Verbose" output format
//!     let verbose = validator.collect_output(&instance, format::Verbose)?;
//!     // Serialize validation output to JSON according to the verbose output format
//!     #[cfg(feature = "serde")]
//!     {
//!         let serialized = serde_json::to_string(&verbose).unwrap();
//!     }
//!     // Iteration over validation results
//!     for unit in validator.iter_output_units(&instance, format::Verbose) {
//!         println!("{:?}", unit);
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
    validation::{
        blocking, collect_output, is_valid, iter_errors, iter_output_units, validate,
        validator_for, JsonSchemaValidator,
    },
};
use drafts::{draft04::Draft04, Autodetect};

pub type Draft4Validator = validation::Validator<Draft04>;
pub type Validator = validation::Validator<Autodetect>;

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

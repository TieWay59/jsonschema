use std::borrow::Cow;

use jsonlike::Json;
pub(crate) mod builder;
pub(crate) mod iter;
use crate::{error::Error, graph, output::Output, vocabulary::Keyword, SchemaError};
use builder::validator_for;
use iter::ValidationErrorIter;

pub async fn is_valid<J: Json>(schema: &J, instance: &J) -> bool {
    try_is_valid(schema, instance)
        .await
        .expect("Invalid schema")
}

pub async fn try_is_valid<J: Json>(schema: &J, instance: &J) -> Result<bool, SchemaError> {
    Ok(validator_for(schema).await?.is_valid(instance))
}

pub async fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), Error> {
    validator_for(schema).await?.validate(instance)
}

pub async fn iter_errors<'schema, 'instance, J: Json>(
    schema: &'schema J,
    instance: &'instance J,
) -> ValidationErrorIter<'static, 'instance, J> {
    try_iter_errors(schema, instance)
        .await
        .expect("Invalid schema")
}

pub async fn try_iter_errors<'schema, 'instance, J: Json>(
    schema: &'schema J,
    instance: &'instance J,
) -> Result<ValidationErrorIter<'static, 'instance, J>, SchemaError> {
    let validator = validator_for(schema).await?;
    Ok(validator.iter_errors_once(instance))
}

pub async fn evaluate<'i, J: Json>(instance: &'i J, schema: &J) -> Output<'static, 'i, J> {
    try_evaluate(instance, schema)
        .await
        .expect("Invalid schema")
}

pub async fn try_evaluate<'i, J: Json>(
    instance: &'i J,
    schema: &J,
) -> Result<Output<'static, 'i, J>, SchemaError> {
    Ok(validator_for(schema).await?.evaluate_once(instance))
}

#[derive(Debug, Clone)]
pub struct Validator {
    graph: graph::Graph<Keyword>,
}

impl Validator {
    pub(crate) fn new(graph: graph::Graph<Keyword>) -> Self {
        Self { graph }
    }

    pub fn is_valid<J: Json>(&self, instance: &J) -> bool {
        true
    }
    pub fn validate<J: Json>(&self, instance: &J) -> Result<(), Error> {
        match self.iter_errors(instance).next() {
            None => Ok(()),
            Some(error) => Err(error.into()),
        }
    }
    pub fn iter_errors<'v, 'i, J: Json>(
        &'v self,
        instance: &'i J,
    ) -> ValidationErrorIter<'v, 'i, J> {
        ValidationErrorIter::new(Cow::Borrowed(self), instance)
    }
    pub(crate) fn iter_errors_once<J: Json>(
        self,
        instance: &J,
    ) -> ValidationErrorIter<'static, '_, J> {
        ValidationErrorIter::new(Cow::Owned(self), instance)
    }
    pub fn evaluate<'v, 'i, J: Json>(&'v self, instance: &'i J) -> Output<'v, 'i, J> {
        Output::new(Cow::Borrowed(self), instance)
    }
    pub(crate) fn evaluate_once<J: Json>(self, instance: &J) -> Output<'static, '_, J> {
        Output::new(Cow::Owned(self), instance)
    }
}

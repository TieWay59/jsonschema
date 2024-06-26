mod draft04;
mod draft06;
mod draft07;
mod draft201909;
mod draft202012;

use crate::vocabulary::KeywordValue;
use jsonlike::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Draft {
    Draft04,
    Draft06,
    Draft07,
    Draft201909,
    Draft202012,
}

impl Draft {
    pub fn latest() -> Self {
        Self::Draft202012
    }
    pub(crate) fn get_keyword<J: Json>(&self, key: &str, value: &J) -> Option<KeywordValue<J>> {
        match self {
            Draft::Draft04 => draft04::get_keyword(key, value),
            Draft::Draft06 => draft06::get_keyword(key, value),
            Draft::Draft07 => draft07::get_keyword(key, value),
            Draft::Draft201909 => draft201909::get_keyword(key, value),
            Draft::Draft202012 => draft202012::get_keyword(key, value),
        }
    }
}

pub(crate) fn from_url(mut url: &str) -> Option<Draft> {
    if let Some((cleaned, fragment)) = url.split_once('#') {
        if !fragment.is_empty() {
            return None;
        }
        url = cleaned;
    }
    if let Some(cleaned) = url.strip_prefix("http://") {
        url = cleaned;
    } else if let Some(cleaned) = url.strip_prefix("https://") {
        url = cleaned;
    }
    match url {
        "json-schema.org/schema" => Some(Draft::latest()),
        "json-schema.org/draft/2020-12/schema" => Some(Draft::Draft202012),
        "json-schema.org/draft/2019-09/schema" => Some(Draft::Draft201909),
        "json-schema.org/draft-07/schema" => Some(Draft::Draft07),
        "json-schema.org/draft-06/schema" => Some(Draft::Draft06),
        "json-schema.org/draft-04/schema" => Some(Draft::Draft04),
        _ => None,
    }
}
pub(crate) fn draft_from_schema(schema: &impl Json) -> Draft {
    if let Some(object) = schema.as_object() {
        if let Some(url) = object.get("$schema").and_then(Json::as_string) {
            from_url(url.as_ref()).unwrap_or_else(Draft::latest)
        } else {
            Draft::latest()
        }
    } else {
        Draft::latest()
    }
}

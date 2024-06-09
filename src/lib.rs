pub mod exporter;
pub mod bdndr_ccts_cct_schemamodule_1_1;
pub mod bdndr_unqualifieddatatypes_1_1;
pub mod ubl_commonbasiccomponents_2_1;
pub mod ubl_commonaggregatecomponents_2_1;
pub mod ubl_commonextensioncomponents_2_1;
pub mod ubl_qualifieddatatypes_2_1;
pub mod ubl_extensioncontentdatatype_2_1;


use thiserror::Error;


use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};


pub trait Componentable<T> {
    fn validate(&self) -> Result<&T, UblError>;
    fn get(self) -> Result<T, UblError>;
    /// To add additional props to struct, read it as JSON first and manipulate json object props.
    fn additional_props_allowed() -> bool;
}

pub struct Component<T> (T) where T: Componentable<T> + AsMut<T> + Clone;

impl<T> Component<T> where T: Componentable<T> + AsMut<T> + Clone {
    pub fn as_mut(&mut self) -> &mut T {
        self.0.as_mut()
    }
    pub fn as_validated(&self) -> Result<&T, UblError> {
        self.0.validate()
    }
    pub fn get_validated(self) -> Result<T, UblError> {
        self.0.get()
    }
}

#[derive(Error, Debug)]
pub enum UblError {
    #[error("value of `{0}` cannot be empty string")]
    IsEmpty(String),
    #[error("value `{0}` is optional but when provided cannot be empty string")]
    OptionalEmpty(String),
    #[error("unexpected format for `{input:?}`, should not be: {fmt:?}")]
    BadFormat {
        input: String,
        fmt: String,
    },
    #[error("unable to format input `{input:?}` as {fmt:?}: {err:?}")]
    InvalidDateTime {
        input: String,
        fmt: String,
        err: String,
    },
    #[error("value `{input:?}` is optional but when provided should not be: {fmt:?}")]
    OptionalBadFormat {
        input: String,
        fmt: String,
    },
    #[error("component `{item:?}` failed validation: {err:?}")]
    ComponentValidation {
        item: String,
        err: String,
    },
    #[error("optional component `{item:?}` failed validation: {err:?}")]
    OptionalComponentValidation {
        item: String,
        err: String,
    },
    #[error("inner item in component `{item:?}` failed validation: {err:?}")]
    InnerComponentValidation {
        item: String,
        err: String,
    },
}

impl UblError {
    pub fn date_time<I, F>(input: I, fmt: F, err: chrono::ParseError) -> Self where I: Into<String>, F: Into<String> {
        Self::InvalidDateTime {
            input: input.into(),
            fmt: fmt.into(),
            err: err.to_string(),
        }
    }
    pub fn empty<T>(input: T) -> Self where T: Into<String> {
        Self::IsEmpty(input.into())
    }
    pub fn optional_empty<T>(input: T) -> Self where T: Into<String> {
        Self::OptionalEmpty(input.into())
    }
    pub fn format<T>(input: T, fmt: &FormattedValue) -> Self where T: Into<String> {
        Self::BadFormat {
            input: input.into(),
            fmt: fmt.to_string(),
        }
    }
    pub fn optional_format<T>(input: T, fmt: &FormattedValue) -> Self where T: Into<String> {
        Self::OptionalBadFormat {
            input: input.into(),
            fmt: fmt.to_string(),
        }
    }
    pub fn component<T>(item: T, err: Self) -> Self where T: Into<String> {
        Self::ComponentValidation {
            item: item.into(),
            err: err.to_string(),
        }
    }
    pub fn optional_component<T>(item: T, err: Self) -> Self where T: Into<String> {
        Self::OptionalComponentValidation {
            item: item.into(),
            err: err.to_string(),
        }
    }
    pub fn inner_component<T, E>(item: T, err: E) -> Self where T: Into<String>, E: Into<String> {
        Self::InnerComponentValidation {
            item: item.into(),
            err: err.into(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormattedValue {
    DateTime(String),
    Date(String),
    Time(String),
}


impl Display for FormattedValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::DateTime(v) => write!(f, "{}", v),
            Self::Date(v) => write!(f, "{}", v),
            Self::Time(v) => write!(f, "{}", v),
        }
    }
}


impl FormattedValue {
    pub fn new_datetime(v: chrono::NaiveDateTime) -> Self {
        Self::DateTime(v.format("%Y-%m-%dT%H:%M:%S").to_string())
    }
    /// Create new date time from a string formatted as `YYYY-MM-DD HH:MM:SS`
    pub fn new_date_time_from_str<T>(v: T) -> Result<Self, UblError> where T: AsRef<str> {
        let s = v.as_ref();
        let fmt = "%Y-%m-%d %H:%M:%S";
        chrono::NaiveDateTime::parse_from_str(s, fmt)
            .map(|n|Self::DateTime(n.format("%Y-%m-%dT%H:%M:%S").to_string()))
            .map_err(|e|UblError::date_time(s, fmt, e))
    }
    /// Create new date time from a string with custom formatting (timezone, fractional seconds, etc.)
    /// Be advised that the formatted value is stored as string in format `YYYY-MM-DDTHH:MM:SS`
    pub fn new_date_time_from_str_in_fmt<T, F>(v: T, format: F) -> Result<Self, UblError> where T: AsRef<str>, F: AsRef<str> {
        let s = v.as_ref();
        let fmt_in = format.as_ref();
        let fmt_out = "%Y-%m-%dT%H:%M:%S";
        chrono::NaiveDateTime::parse_from_str(s, fmt_in)
            .map(|n|Self::DateTime(n.format(fmt_out).to_string()))
            .map_err(|e|UblError::date_time(s, fmt_out, e))
    }
    pub fn new_date(date: chrono::NaiveDate) -> Self {
        Self::Date(date.format("%Y-%m-%d").to_string())
    }
    pub fn new_date_from_str<T>(v: T) -> Result<Self, UblError> where T: AsRef<str> {
        let s = v.as_ref();
        let fmt = "%Y-%m-%d";
        chrono::NaiveDate::parse_from_str(s, fmt)
            .map(|n|Self::Date(n.format(fmt).to_string()))
            .map_err(|e|UblError::date_time(s, fmt, e))
    }
    pub fn new_date_from_str_in_fmt<T, F>(v: T, format: F) -> Result<Self, UblError> where T: AsRef<str>, F: AsRef<str> {
        let s = v.as_ref();
        let fmt_in = format.as_ref();
        let fmt_out = "%Y-%m-%d";
        chrono::NaiveDate::parse_from_str(s, fmt_in)
            .map(|n|Self::Date(n.format(fmt_out).to_string()))
            .map_err(|e|UblError::date_time(s, fmt_out, e))
    }
    pub fn new_time(time: chrono::NaiveTime) -> Self {
        Self::Time(time.format("%H:%M:%S").to_string())
    }
    pub fn new_time_from_str<T>(v: T) -> Result<Self, UblError> where T: AsRef<str> {
        let s = v.as_ref();
        let fmt = "%H:%M:%S";
        chrono::NaiveTime::parse_from_str(s, fmt)
            .map(|n|Self::Time(n.format(fmt).to_string()))
            .map_err(|e|UblError::date_time(s, fmt, e))
    }
    pub fn new_time_from_str_in_fmt<T, F>(v: T, format: F) -> Result<Self, UblError> where T: AsRef<str>, F: AsRef<str> {
        let s = v.as_ref();
        let fmt_in = format.as_ref();
        let fmt_out = "%H:%M:%S";
        chrono::NaiveTime::parse_from_str(s, fmt_in)
            .map(|n|Self::Time(n.format(fmt_out).to_string()))
            .map_err(|e|UblError::date_time(s, fmt_out, e))
    }
}

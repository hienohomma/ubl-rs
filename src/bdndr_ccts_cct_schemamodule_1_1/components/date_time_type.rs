use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DateTimeType {
    #[serde(rename = "_")]
    pub _uc: String,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl AsMut<DateTimeType> for DateTimeType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DateTimeType> for DateTimeType {
    fn validate(&self) -> Result<&Self, UblError> {
        if self._uc.is_empty() {
            return Err(UblError::empty("DateTimeType._uc"))
        }
        if let Some(v) = &self.format {
            if v.is_empty() {
                return Err(UblError::optional_empty("DateTimeType.format"))
            }
        }

        Ok(self)
    }

    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DateTimeType {
    pub fn title() -> &'static str {
        "Date Time. Type"
    }
    pub fn description() -> &'static str {
        "A particular point in the progression of time together with the relevant supplementary information."
    }
    pub fn new<A>(_uc: A) -> Component<Self> where A: Into<String> {
        Component(Self {
            _uc: _uc.into(), // Generic type: A
            format: None,
        })
    }
}


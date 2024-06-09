use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndicatorType {
    #[serde(rename = "_")]
    pub _uc: String,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl AsMut<IndicatorType> for IndicatorType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<IndicatorType> for IndicatorType {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.format {
            if v.is_empty() {
                return Err(UblError::optional_empty("IndicatorType.format"))
            }
        }
        if self._uc.is_empty() {
            return Err(UblError::empty("IndicatorType._uc"))
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

impl IndicatorType {
    pub fn title() -> &'static str {
        "Indicator. Type"
    }
    pub fn description() -> &'static str {
        "A list of two mutually exclusive Boolean values that express the only possible states of a Property."
    }
    pub fn new<A>(_uc: A) -> Component<Self> where A: Into<String> {
        Component(Self {
            format: None,
            _uc: _uc.into(), // Generic type: A
        })
    }
}


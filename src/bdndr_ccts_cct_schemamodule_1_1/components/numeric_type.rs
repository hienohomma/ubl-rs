use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NumericType {
    #[serde(rename = "_")]
    pub _uc: serde_json::Number,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl AsMut<NumericType> for NumericType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<NumericType> for NumericType {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.format {
            if v.is_empty() {
                return Err(UblError::optional_empty("NumericType.format"))
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

impl NumericType {
    pub fn title() -> &'static str {
        "Numeric. Type"
    }
    pub fn description() -> &'static str {
        "Numeric information that is assigned or is determined by calculation, counting, or sequencing. It does not require a unit of quantity or unit of measure."
    }
    pub fn new(_uc: serde_json::Number) -> Component<Self> {
        Component(Self {
            format: None,
            _uc,
        })
    }
}


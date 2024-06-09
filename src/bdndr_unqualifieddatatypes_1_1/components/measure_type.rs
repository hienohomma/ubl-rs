use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeasureType {
    #[serde(rename = "_")]
    pub _uc: serde_json::Number,
    #[serde(rename = "unitCode")]
    pub unit_code: String,
    #[serde(rename = "unitCodeListVersionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_code_list_version_id: Option<String>,
}

impl AsMut<MeasureType> for MeasureType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeasureType> for MeasureType {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.unit_code_list_version_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("MeasureType.unit_code_list_version_id"))
            }
        }
        if self.unit_code.is_empty() {
            return Err(UblError::empty("MeasureType.unit_code"))
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

impl MeasureType {
    pub fn title() -> &'static str {
        "Measure. Type"
    }
    pub fn description() -> &'static str {
        "A numeric value determined by measuring an object using a specified unit of measure."
    }
    pub fn new<A>(_uc: serde_json::Number, unit_code: A) -> Component<Self> where A: Into<String> {
        Component(Self {
            unit_code: unit_code.into(), // Generic type: A
            _uc,
            unit_code_list_version_id: None,
        })
    }
}


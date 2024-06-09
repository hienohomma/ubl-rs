use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeasureType {
    #[serde(rename = "_")]
    pub _uc: serde_json::Number,
    #[serde(rename = "unitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_code: Option<String>,
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
        if let Some(v) = &self.unit_code {
            if v.is_empty() {
                return Err(UblError::optional_empty("MeasureType.unit_code"))
            }
        }
        if let Some(v) = &self.unit_code_list_version_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("MeasureType.unit_code_list_version_id"))
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

impl MeasureType {
    pub fn title() -> &'static str {
        "Measure. Type"
    }
    pub fn description() -> &'static str {
        "A numeric value determined by measuring an object along with the specified unit of measure."
    }
    pub fn new(_uc: serde_json::Number) -> Component<Self> {
        Component(Self {
            _uc,
            unit_code: None,
            unit_code_list_version_id: None,
        })
    }
}


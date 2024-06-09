use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuantityType {
    #[serde(rename = "_")]
    pub _uc: serde_json::Number,
    #[serde(rename = "unitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_code: Option<String>,
    #[serde(rename = "unitCodeListAgencyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_code_list_agency_id: Option<String>,
    #[serde(rename = "unitCodeListAgencyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_code_list_agency_name: Option<String>,
    #[serde(rename = "unitCodeListID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_code_list_id: Option<String>,
}

impl AsMut<QuantityType> for QuantityType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuantityType> for QuantityType {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.unit_code_list_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("QuantityType.unit_code_list_id"))
            }
        }
        if let Some(v) = &self.unit_code_list_agency_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("QuantityType.unit_code_list_agency_id"))
            }
        }
        if let Some(v) = &self.unit_code {
            if v.is_empty() {
                return Err(UblError::optional_empty("QuantityType.unit_code"))
            }
        }
        if let Some(v) = &self.unit_code_list_agency_name {
            if v.is_empty() {
                return Err(UblError::optional_empty("QuantityType.unit_code_list_agency_name"))
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

impl QuantityType {
    pub fn title() -> &'static str {
        "Quantity. Type"
    }
    pub fn description() -> &'static str {
        "A counted number of non-monetary units possibly including fractions."
    }
    pub fn new(_uc: serde_json::Number) -> Component<Self> {
        Component(Self {
            unit_code_list_agency_id: None,
            unit_code: None,
            unit_code_list_agency_name: None,
            unit_code_list_id: None,
            _uc,
        })
    }
}


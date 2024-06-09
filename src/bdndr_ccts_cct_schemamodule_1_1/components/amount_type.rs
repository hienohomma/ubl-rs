use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AmountType {
    #[serde(rename = "_")]
    pub _uc: serde_json::Number,
    #[serde(rename = "currencyCodeListVersionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code_list_version_id: Option<String>,
    #[serde(rename = "currencyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
}

impl AsMut<AmountType> for AmountType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AmountType> for AmountType {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.currency_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("AmountType.currency_id"))
            }
        }
        if let Some(v) = &self.currency_code_list_version_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("AmountType.currency_code_list_version_id"))
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

impl AmountType {
    pub fn title() -> &'static str {
        "Amount. Type"
    }
    pub fn description() -> &'static str {
        "A number of monetary units specified in a currency where the unit of the currency is explicit or implied."
    }
    pub fn new(_uc: serde_json::Number) -> Component<Self> {
        Component(Self {
            currency_code_list_version_id: None,
            _uc,
            currency_id: None,
        })
    }
}


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
    pub currency_id: String,
}

impl AsMut<AmountType> for AmountType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AmountType> for AmountType {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.currency_code_list_version_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("AmountType.currency_code_list_version_id"))
            }
        }
        if self.currency_id.is_empty() {
            return Err(UblError::empty("AmountType.currency_id"))
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
        "A number of monetary units specified using a given unit of currency."
    }
    pub fn new<A>(_uc: serde_json::Number, currency_id: A) -> Component<Self> where A: Into<String> {
        Component(Self {
            currency_code_list_version_id: None,
            currency_id: currency_id.into(), // Generic type: A
            _uc,
        })
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndicatorType {
    #[serde(rename = "_")]
    pub _uc: bool,
}

impl AsMut<IndicatorType> for IndicatorType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<IndicatorType> for IndicatorType {
    fn validate(&self) -> Result<&Self, UblError> {

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
        "A list of two mutually exclusive Boolean values that express the only possible states of a property."
    }
    pub fn new(_uc: bool) -> Component<Self> {
        Component(Self {
            _uc,
        })
    }
}


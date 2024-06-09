use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DateType {
    #[serde(rename = "_")]
    pub _uc: crate::FormattedValue,
}

impl AsMut<DateType> for DateType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DateType> for DateType {
    fn validate(&self) -> Result<&Self, UblError> {
        match &self._uc {
            crate::FormattedValue::Date(s) => if s.is_empty() {
                return Err(UblError::empty("DateType._uc.crate::FormattedValue::Date(s)"))
            },
            e => return Err(UblError::format("DateType._uc", e))
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

impl DateType {
    pub fn title() -> &'static str {
        "Date. Type"
    }
    pub fn description() -> &'static str {
        "One calendar day according the Gregorian calendar."
    }
    pub fn new(_uc: crate::FormattedValue) -> Component<Self> {
        Component(Self {
            _uc,
        })
    }
}


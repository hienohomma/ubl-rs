use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DateTimeType {
    #[serde(rename = "_")]
    pub _uc: crate::FormattedValue,
}

impl AsMut<DateTimeType> for DateTimeType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DateTimeType> for DateTimeType {
    fn validate(&self) -> Result<&Self, UblError> {
        match &self._uc {
            crate::FormattedValue::DateTime(s) => if s.is_empty() {
                return Err(UblError::empty("DateTimeType._uc.crate::FormattedValue::DateTime(s)"))
            },
            e => return Err(UblError::format("DateTimeType._uc", e))
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
        "A particular point in the progression of time, together with relevant supplementary information."
    }
    pub fn new(_uc: crate::FormattedValue) -> Component<Self> {
        Component(Self {
            _uc,
        })
    }
}


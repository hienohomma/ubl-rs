use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeType {
    #[serde(rename = "_")]
    pub _uc: crate::FormattedValue,
}

impl AsMut<TimeType> for TimeType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TimeType> for TimeType {
    fn validate(&self) -> Result<&Self, UblError> {
        match &self._uc {
            crate::FormattedValue::Time(s) => if s.is_empty() {
                return Err(UblError::empty("TimeType._uc.crate::FormattedValue::Time(s)"))
            },
            e => return Err(UblError::format("TimeType._uc", e))
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

impl TimeType {
    pub fn title() -> &'static str {
        "Time. Type"
    }
    pub fn description() -> &'static str {
        "An instance of time that occurs every day."
    }
    pub fn new(_uc: crate::FormattedValue) -> Component<Self> {
        Component(Self {
            _uc,
        })
    }
}


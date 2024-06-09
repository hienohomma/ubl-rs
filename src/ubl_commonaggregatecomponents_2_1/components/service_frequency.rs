use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceFrequency {
    #[serde(rename = "WeekDayCode")]
    pub week_day_code: ServiceFrequencyArrayOfWeekDayCodeComponent,
}

impl AsMut<ServiceFrequency> for ServiceFrequency {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ServiceFrequency> for ServiceFrequency {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.week_day_code.validate() {
            return Err(UblError::component("ServiceFrequency.week_day_code", e));
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

impl ServiceFrequency {
    pub fn title() -> &'static str {
        "Service Frequency. Details"
    }
    pub fn description() -> &'static str {
        "A class to specify which day of the week a transport service is operational."
    }
    pub fn new(week_day_code: ServiceFrequencyArrayOfWeekDayCodeComponent) -> Component<Self> {
        Component(Self {
            week_day_code,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ServiceFrequencyArrayOfWeekDayCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::WeekDayCode>,
}

impl AsMut<ServiceFrequencyArrayOfWeekDayCodeComponent> for ServiceFrequencyArrayOfWeekDayCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ServiceFrequencyArrayOfWeekDayCodeComponent> for ServiceFrequencyArrayOfWeekDayCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ServiceFrequencyArrayOfWeekDayCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ServiceFrequencyArrayOfWeekDayCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ServiceFrequencyArrayOfWeekDayCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::WeekDayCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::WeekDayCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::WeekDayCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::WeekDayCode> {
        self.items.iter()
    }
}


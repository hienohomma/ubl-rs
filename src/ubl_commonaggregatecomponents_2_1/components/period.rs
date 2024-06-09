use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Period {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<PeriodArrayOfDescriptionComponent>,
    #[serde(rename = "DescriptionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_code: Option<PeriodArrayOfDescriptionCodeComponent>,
    #[serde(rename = "DurationMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_measure: Option<PeriodArrayOfDurationMeasureComponent>,
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<PeriodArrayOfEndDateComponent>,
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<PeriodArrayOfEndTimeComponent>,
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<PeriodArrayOfStartDateComponent>,
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<PeriodArrayOfStartTimeComponent>,
}

impl AsMut<Period> for Period {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Period> for Period {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Period.description", e));
            }
        }
        if let Some(v) = &self.start_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Period.start_time", e));
            }
        }
        if let Some(v) = &self.end_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Period.end_date", e));
            }
        }
        if let Some(v) = &self.end_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Period.end_time", e));
            }
        }
        if let Some(v) = &self.start_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Period.start_date", e));
            }
        }
        if let Some(v) = &self.duration_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Period.duration_measure", e));
            }
        }
        if let Some(v) = &self.description_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Period.description_code", e));
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

impl Period {
    pub fn title() -> &'static str {
        "Period. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a period of time."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            description_code: None,
            duration_measure: None,
            end_time: None,
            start_date: None,
            description: None,
            end_date: None,
            start_time: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PeriodArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<PeriodArrayOfDescriptionComponent> for PeriodArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PeriodArrayOfDescriptionComponent> for PeriodArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PeriodArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PeriodArrayOfDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Description) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Description) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PeriodArrayOfDescriptionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DescriptionCode>,
}

impl AsMut<PeriodArrayOfDescriptionCodeComponent> for PeriodArrayOfDescriptionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PeriodArrayOfDescriptionCodeComponent> for PeriodArrayOfDescriptionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PeriodArrayOfDescriptionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PeriodArrayOfDescriptionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DescriptionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DescriptionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DescriptionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DescriptionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PeriodArrayOfDurationMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DurationMeasure>,
}

impl AsMut<PeriodArrayOfDurationMeasureComponent> for PeriodArrayOfDurationMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PeriodArrayOfDurationMeasureComponent> for PeriodArrayOfDurationMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PeriodArrayOfDurationMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PeriodArrayOfDurationMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PeriodArrayOfDurationMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DurationMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DurationMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DurationMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DurationMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PeriodArrayOfEndDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EndDate>,
}

impl AsMut<PeriodArrayOfEndDateComponent> for PeriodArrayOfEndDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PeriodArrayOfEndDateComponent> for PeriodArrayOfEndDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PeriodArrayOfEndDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PeriodArrayOfEndDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PeriodArrayOfEndDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EndDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EndDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EndDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EndDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PeriodArrayOfEndTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EndTime>,
}

impl AsMut<PeriodArrayOfEndTimeComponent> for PeriodArrayOfEndTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PeriodArrayOfEndTimeComponent> for PeriodArrayOfEndTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PeriodArrayOfEndTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PeriodArrayOfEndTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PeriodArrayOfEndTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EndTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EndTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EndTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EndTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PeriodArrayOfStartDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::StartDate>,
}

impl AsMut<PeriodArrayOfStartDateComponent> for PeriodArrayOfStartDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PeriodArrayOfStartDateComponent> for PeriodArrayOfStartDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PeriodArrayOfStartDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PeriodArrayOfStartDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PeriodArrayOfStartDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::StartDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::StartDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::StartDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::StartDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PeriodArrayOfStartTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::StartTime>,
}

impl AsMut<PeriodArrayOfStartTimeComponent> for PeriodArrayOfStartTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PeriodArrayOfStartTimeComponent> for PeriodArrayOfStartTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PeriodArrayOfStartTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PeriodArrayOfStartTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PeriodArrayOfStartTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::StartTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::StartTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::StartTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::StartTime> {
        self.items.iter()
    }
}


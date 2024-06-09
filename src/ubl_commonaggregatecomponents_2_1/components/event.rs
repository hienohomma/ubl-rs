use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "CompletionIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_indicator: Option<EventArrayOfCompletionIndicatorComponent>,
    #[serde(rename = "Contact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<EventArrayOfContactComponent>,
    #[serde(rename = "CurrentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_status: Option<EventArrayOfCurrentStatusComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<EventArrayOfDescriptionComponent>,
    #[serde(rename = "IdentificationID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_id: Option<EventArrayOfIdentificationIDComponent>,
    #[serde(rename = "OccurenceLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurence_location: Option<EventArrayOfOccurenceLocationComponent>,
    #[serde(rename = "OccurrenceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrence_date: Option<EventArrayOfOccurrenceDateComponent>,
    #[serde(rename = "OccurrenceTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrence_time: Option<EventArrayOfOccurrenceTimeComponent>,
    #[serde(rename = "TypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_code: Option<EventArrayOfTypeCodeComponent>,
}

impl AsMut<Event> for Event {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Event> for Event {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.current_status {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Event.current_status", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Event.description", e));
            }
        }
        if let Some(v) = &self.occurence_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Event.occurence_location", e));
            }
        }
        if let Some(v) = &self.type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Event.type_code", e));
            }
        }
        if let Some(v) = &self.contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Event.contact", e));
            }
        }
        if let Some(v) = &self.completion_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Event.completion_indicator", e));
            }
        }
        if let Some(v) = &self.occurrence_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Event.occurrence_date", e));
            }
        }
        if let Some(v) = &self.identification_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Event.identification_id", e));
            }
        }
        if let Some(v) = &self.occurrence_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Event.occurrence_time", e));
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

impl Event {
    pub fn title() -> &'static str {
        "Event. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a significant occurrence relating to an object, process, or person."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            identification_id: None,
            current_status: None,
            completion_indicator: None,
            occurence_location: None,
            type_code: None,
            contact: None,
            description: None,
            occurrence_date: None,
            occurrence_time: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventArrayOfCompletionIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CompletionIndicator>,
}

impl AsMut<EventArrayOfCompletionIndicatorComponent> for EventArrayOfCompletionIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventArrayOfCompletionIndicatorComponent> for EventArrayOfCompletionIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventArrayOfCompletionIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventArrayOfCompletionIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventArrayOfCompletionIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CompletionIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CompletionIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CompletionIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CompletionIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventArrayOfContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Contact>,
}

impl AsMut<EventArrayOfContactComponent> for EventArrayOfContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventArrayOfContactComponent> for EventArrayOfContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EventArrayOfContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventArrayOfContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Contact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Contact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Contact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Contact> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventArrayOfCurrentStatusComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CurrentStatus>,
}

impl AsMut<EventArrayOfCurrentStatusComponent> for EventArrayOfCurrentStatusComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventArrayOfCurrentStatusComponent> for EventArrayOfCurrentStatusComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EventArrayOfCurrentStatusComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventArrayOfCurrentStatusComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CurrentStatus) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CurrentStatus) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CurrentStatus> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CurrentStatus> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<EventArrayOfDescriptionComponent> for EventArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventArrayOfDescriptionComponent> for EventArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EventArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventArrayOfDescriptionComponent {
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
pub struct EventArrayOfIdentificationIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IdentificationID>,
}

impl AsMut<EventArrayOfIdentificationIDComponent> for EventArrayOfIdentificationIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventArrayOfIdentificationIDComponent> for EventArrayOfIdentificationIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventArrayOfIdentificationIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventArrayOfIdentificationIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventArrayOfIdentificationIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IdentificationID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IdentificationID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IdentificationID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IdentificationID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventArrayOfOccurenceLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OccurenceLocation>,
}

impl AsMut<EventArrayOfOccurenceLocationComponent> for EventArrayOfOccurenceLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventArrayOfOccurenceLocationComponent> for EventArrayOfOccurenceLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventArrayOfOccurenceLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventArrayOfOccurenceLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventArrayOfOccurenceLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OccurenceLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OccurenceLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OccurenceLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OccurenceLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventArrayOfOccurrenceDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OccurrenceDate>,
}

impl AsMut<EventArrayOfOccurrenceDateComponent> for EventArrayOfOccurrenceDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventArrayOfOccurrenceDateComponent> for EventArrayOfOccurrenceDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventArrayOfOccurrenceDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventArrayOfOccurrenceDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventArrayOfOccurrenceDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OccurrenceDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OccurrenceDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OccurrenceDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OccurrenceDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventArrayOfOccurrenceTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OccurrenceTime>,
}

impl AsMut<EventArrayOfOccurrenceTimeComponent> for EventArrayOfOccurrenceTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventArrayOfOccurrenceTimeComponent> for EventArrayOfOccurrenceTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventArrayOfOccurrenceTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventArrayOfOccurrenceTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventArrayOfOccurrenceTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OccurrenceTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OccurrenceTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OccurrenceTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OccurrenceTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventArrayOfTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TypeCode>,
}

impl AsMut<EventArrayOfTypeCodeComponent> for EventArrayOfTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventArrayOfTypeCodeComponent> for EventArrayOfTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventArrayOfTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventArrayOfTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventArrayOfTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TypeCode> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MiscellaneousEvent {
    #[serde(rename = "EventLineItem")]
    pub event_line_item: MiscellaneousEventArrayOfEventLineItemComponent,
    #[serde(rename = "MiscellaneousEventTypeCode")]
    pub miscellaneous_event_type_code: MiscellaneousEventArrayOfMiscellaneousEventTypeCodeComponent,
}

impl AsMut<MiscellaneousEvent> for MiscellaneousEvent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MiscellaneousEvent> for MiscellaneousEvent {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.event_line_item.validate() {
            return Err(UblError::component("MiscellaneousEvent.event_line_item", e));
        }
        if let Err(e) = self.miscellaneous_event_type_code.validate() {
            return Err(UblError::component("MiscellaneousEvent.miscellaneous_event_type_code", e));
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

impl MiscellaneousEvent {
    pub fn title() -> &'static str {
        "Miscellaneous Event. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a miscellaneous event associated with a retail event."
    }
    pub fn new(event_line_item: MiscellaneousEventArrayOfEventLineItemComponent, miscellaneous_event_type_code: MiscellaneousEventArrayOfMiscellaneousEventTypeCodeComponent) -> Component<Self> {
        Component(Self {
            event_line_item,
            miscellaneous_event_type_code,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MiscellaneousEventArrayOfEventLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EventLineItem>,
}

impl AsMut<MiscellaneousEventArrayOfEventLineItemComponent> for MiscellaneousEventArrayOfEventLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MiscellaneousEventArrayOfEventLineItemComponent> for MiscellaneousEventArrayOfEventLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("MiscellaneousEventArrayOfEventLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MiscellaneousEventArrayOfEventLineItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EventLineItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EventLineItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EventLineItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EventLineItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MiscellaneousEventArrayOfMiscellaneousEventTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MiscellaneousEventTypeCode>,
}

impl AsMut<MiscellaneousEventArrayOfMiscellaneousEventTypeCodeComponent> for MiscellaneousEventArrayOfMiscellaneousEventTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MiscellaneousEventArrayOfMiscellaneousEventTypeCodeComponent> for MiscellaneousEventArrayOfMiscellaneousEventTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MiscellaneousEventArrayOfMiscellaneousEventTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MiscellaneousEventArrayOfMiscellaneousEventTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MiscellaneousEventArrayOfMiscellaneousEventTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MiscellaneousEventTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MiscellaneousEventTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MiscellaneousEventTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MiscellaneousEventTypeCode> {
        self.items.iter()
    }
}


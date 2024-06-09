use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventLineItem {
    #[serde(rename = "LineNumberNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number_numeric: Option<EventLineItemArrayOfLineNumberNumericComponent>,
    #[serde(rename = "ParticipatingLocationsLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participating_locations_location: Option<EventLineItemArrayOfParticipatingLocationsLocationComponent>,
    #[serde(rename = "RetailPlannedImpact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retail_planned_impact: Option<EventLineItemArrayOfRetailPlannedImpactComponent>,
    #[serde(rename = "SupplyItem")]
    pub supply_item: EventLineItemArrayOfSupplyItemComponent,
}

impl AsMut<EventLineItem> for EventLineItem {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventLineItem> for EventLineItem {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.line_number_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventLineItem.line_number_numeric", e));
            }
        }
        if let Some(v) = &self.participating_locations_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventLineItem.participating_locations_location", e));
            }
        }
        if let Err(e) = self.supply_item.validate() {
            return Err(UblError::component("EventLineItem.supply_item", e));
        }
        if let Some(v) = &self.retail_planned_impact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventLineItem.retail_planned_impact", e));
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

impl EventLineItem {
    pub fn title() -> &'static str {
        "Event Line Item. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line item describing the expected impacts associated with a retail event involving a specific product at a specific location."
    }
    pub fn new(supply_item: EventLineItemArrayOfSupplyItemComponent) -> Component<Self> {
        Component(Self {
            supply_item,
            retail_planned_impact: None,
            line_number_numeric: None,
            participating_locations_location: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventLineItemArrayOfLineNumberNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineNumberNumeric>,
}

impl AsMut<EventLineItemArrayOfLineNumberNumericComponent> for EventLineItemArrayOfLineNumberNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventLineItemArrayOfLineNumberNumericComponent> for EventLineItemArrayOfLineNumberNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventLineItemArrayOfLineNumberNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventLineItemArrayOfLineNumberNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventLineItemArrayOfLineNumberNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LineNumberNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LineNumberNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LineNumberNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LineNumberNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventLineItemArrayOfParticipatingLocationsLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ParticipatingLocationsLocation>,
}

impl AsMut<EventLineItemArrayOfParticipatingLocationsLocationComponent> for EventLineItemArrayOfParticipatingLocationsLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventLineItemArrayOfParticipatingLocationsLocationComponent> for EventLineItemArrayOfParticipatingLocationsLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventLineItemArrayOfParticipatingLocationsLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventLineItemArrayOfParticipatingLocationsLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventLineItemArrayOfParticipatingLocationsLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ParticipatingLocationsLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ParticipatingLocationsLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ParticipatingLocationsLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ParticipatingLocationsLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventLineItemArrayOfRetailPlannedImpactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RetailPlannedImpact>,
}

impl AsMut<EventLineItemArrayOfRetailPlannedImpactComponent> for EventLineItemArrayOfRetailPlannedImpactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventLineItemArrayOfRetailPlannedImpactComponent> for EventLineItemArrayOfRetailPlannedImpactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EventLineItemArrayOfRetailPlannedImpactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventLineItemArrayOfRetailPlannedImpactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RetailPlannedImpact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RetailPlannedImpact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RetailPlannedImpact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RetailPlannedImpact> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventLineItemArrayOfSupplyItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SupplyItem>,
}

impl AsMut<EventLineItemArrayOfSupplyItemComponent> for EventLineItemArrayOfSupplyItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventLineItemArrayOfSupplyItemComponent> for EventLineItemArrayOfSupplyItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventLineItemArrayOfSupplyItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventLineItemArrayOfSupplyItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventLineItemArrayOfSupplyItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SupplyItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SupplyItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SupplyItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SupplyItem> {
        self.items.iter()
    }
}


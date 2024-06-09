use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PromotionalSpecification {
    #[serde(rename = "EventTactic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tactic: Option<PromotionalSpecificationArrayOfEventTacticComponent>,
    #[serde(rename = "PromotionalEventLineItem")]
    pub promotional_event_line_item: PromotionalSpecificationArrayOfPromotionalEventLineItemComponent,
    #[serde(rename = "SpecificationID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification_id: Option<PromotionalSpecificationArrayOfSpecificationIDComponent>,
}

impl AsMut<PromotionalSpecification> for PromotionalSpecification {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalSpecification> for PromotionalSpecification {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.event_tactic {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PromotionalSpecification.event_tactic", e));
            }
        }
        if let Some(v) = &self.specification_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PromotionalSpecification.specification_id", e));
            }
        }
        if let Err(e) = self.promotional_event_line_item.validate() {
            return Err(UblError::component("PromotionalSpecification.promotional_event_line_item", e));
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

impl PromotionalSpecification {
    pub fn title() -> &'static str {
        "Promotional Specification. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a promotional event as a set of item locations that share a set of promotional tactics."
    }
    pub fn new(promotional_event_line_item: PromotionalSpecificationArrayOfPromotionalEventLineItemComponent) -> Component<Self> {
        Component(Self {
            promotional_event_line_item,
            event_tactic: None,
            specification_id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PromotionalSpecificationArrayOfEventTacticComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EventTactic>,
}

impl AsMut<PromotionalSpecificationArrayOfEventTacticComponent> for PromotionalSpecificationArrayOfEventTacticComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalSpecificationArrayOfEventTacticComponent> for PromotionalSpecificationArrayOfEventTacticComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PromotionalSpecificationArrayOfEventTacticComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PromotionalSpecificationArrayOfEventTacticComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EventTactic) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EventTactic) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EventTactic> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EventTactic> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PromotionalSpecificationArrayOfPromotionalEventLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PromotionalEventLineItem>,
}

impl AsMut<PromotionalSpecificationArrayOfPromotionalEventLineItemComponent> for PromotionalSpecificationArrayOfPromotionalEventLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalSpecificationArrayOfPromotionalEventLineItemComponent> for PromotionalSpecificationArrayOfPromotionalEventLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PromotionalSpecificationArrayOfPromotionalEventLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PromotionalSpecificationArrayOfPromotionalEventLineItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PromotionalEventLineItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PromotionalEventLineItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PromotionalEventLineItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PromotionalEventLineItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PromotionalSpecificationArrayOfSpecificationIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SpecificationID>,
}

impl AsMut<PromotionalSpecificationArrayOfSpecificationIDComponent> for PromotionalSpecificationArrayOfSpecificationIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalSpecificationArrayOfSpecificationIDComponent> for PromotionalSpecificationArrayOfSpecificationIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PromotionalSpecificationArrayOfSpecificationIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PromotionalSpecificationArrayOfSpecificationIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PromotionalSpecificationArrayOfSpecificationIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SpecificationID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SpecificationID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SpecificationID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SpecificationID> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PromotionalEventLineItem {
    #[serde(rename = "Amount")]
    pub amount: PromotionalEventLineItemArrayOfAmountComponent,
    #[serde(rename = "EventLineItem")]
    pub event_line_item: PromotionalEventLineItemArrayOfEventLineItemComponent,
}

impl AsMut<PromotionalEventLineItem> for PromotionalEventLineItem {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalEventLineItem> for PromotionalEventLineItem {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.amount.validate() {
            return Err(UblError::component("PromotionalEventLineItem.amount", e));
        }
        if let Err(e) = self.event_line_item.validate() {
            return Err(UblError::component("PromotionalEventLineItem.event_line_item", e));
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

impl PromotionalEventLineItem {
    pub fn title() -> &'static str {
        "Promotional Event Line Item. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a line item associated with a promotional event."
    }
    pub fn new(amount: PromotionalEventLineItemArrayOfAmountComponent, event_line_item: PromotionalEventLineItemArrayOfEventLineItemComponent) -> Component<Self> {
        Component(Self {
            amount,
            event_line_item,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PromotionalEventLineItemArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<PromotionalEventLineItemArrayOfAmountComponent> for PromotionalEventLineItemArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalEventLineItemArrayOfAmountComponent> for PromotionalEventLineItemArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PromotionalEventLineItemArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PromotionalEventLineItemArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PromotionalEventLineItemArrayOfAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Amount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Amount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Amount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Amount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PromotionalEventLineItemArrayOfEventLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EventLineItem>,
}

impl AsMut<PromotionalEventLineItemArrayOfEventLineItemComponent> for PromotionalEventLineItemArrayOfEventLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalEventLineItemArrayOfEventLineItemComponent> for PromotionalEventLineItemArrayOfEventLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PromotionalEventLineItemArrayOfEventLineItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PromotionalEventLineItemArrayOfEventLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PromotionalEventLineItemArrayOfEventLineItemComponent {
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


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PricingReference {
    #[serde(rename = "AlternativeConditionPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_condition_price: Option<PricingReferenceArrayOfAlternativeConditionPriceComponent>,
    #[serde(rename = "OriginalItemLocationQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_item_location_quantity: Option<PricingReferenceArrayOfOriginalItemLocationQuantityComponent>,
}

impl AsMut<PricingReference> for PricingReference {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PricingReference> for PricingReference {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.original_item_location_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PricingReference.original_item_location_quantity", e));
            }
        }
        if let Some(v) = &self.alternative_condition_price {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PricingReference.alternative_condition_price", e));
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

impl PricingReference {
    pub fn title() -> &'static str {
        "Pricing Reference. Details"
    }
    pub fn description() -> &'static str {
        "A reference to the basis for pricing. This may be based on a catalogue or a quoted amount from a price list and include some alternative pricing conditions."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            original_item_location_quantity: None,
            alternative_condition_price: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PricingReferenceArrayOfAlternativeConditionPriceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AlternativeConditionPrice>,
}

impl AsMut<PricingReferenceArrayOfAlternativeConditionPriceComponent> for PricingReferenceArrayOfAlternativeConditionPriceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PricingReferenceArrayOfAlternativeConditionPriceComponent> for PricingReferenceArrayOfAlternativeConditionPriceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PricingReferenceArrayOfAlternativeConditionPriceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PricingReferenceArrayOfAlternativeConditionPriceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AlternativeConditionPrice) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AlternativeConditionPrice) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AlternativeConditionPrice> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AlternativeConditionPrice> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PricingReferenceArrayOfOriginalItemLocationQuantityComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginalItemLocationQuantity>,
}

impl AsMut<PricingReferenceArrayOfOriginalItemLocationQuantityComponent> for PricingReferenceArrayOfOriginalItemLocationQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PricingReferenceArrayOfOriginalItemLocationQuantityComponent> for PricingReferenceArrayOfOriginalItemLocationQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PricingReferenceArrayOfOriginalItemLocationQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PricingReferenceArrayOfOriginalItemLocationQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PricingReferenceArrayOfOriginalItemLocationQuantityComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OriginalItemLocationQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OriginalItemLocationQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OriginalItemLocationQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OriginalItemLocationQuantity> {
        self.items.iter()
    }
}


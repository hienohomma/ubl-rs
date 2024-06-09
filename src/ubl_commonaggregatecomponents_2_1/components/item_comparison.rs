use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemComparison {
    #[serde(rename = "PriceAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_amount: Option<ItemComparisonArrayOfPriceAmountComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ItemComparisonArrayOfQuantityComponent>,
}

impl AsMut<ItemComparison> for ItemComparison {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemComparison> for ItemComparison {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.price_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemComparison.price_amount", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemComparison.quantity", e));
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

impl ItemComparison {
    pub fn title() -> &'static str {
        "Item Comparison. Details"
    }
    pub fn description() -> &'static str {
        "A class to provide information about price and quantity of an item for use in price comparisons based on price, quantity, or measurements."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            quantity: None,
            price_amount: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemComparisonArrayOfPriceAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PriceAmount>,
}

impl AsMut<ItemComparisonArrayOfPriceAmountComponent> for ItemComparisonArrayOfPriceAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemComparisonArrayOfPriceAmountComponent> for ItemComparisonArrayOfPriceAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemComparisonArrayOfPriceAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemComparisonArrayOfPriceAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemComparisonArrayOfPriceAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PriceAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PriceAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PriceAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PriceAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemComparisonArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<ItemComparisonArrayOfQuantityComponent> for ItemComparisonArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemComparisonArrayOfQuantityComponent> for ItemComparisonArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemComparisonArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemComparisonArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemComparisonArrayOfQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Quantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Quantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter()
    }
}


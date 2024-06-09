use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SalesItem {
    #[serde(rename = "ActivityProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_property: Option<SalesItemArrayOfActivityPropertyComponent>,
    #[serde(rename = "Item")]
    pub item: SalesItemArrayOfItemComponent,
    #[serde(rename = "Quantity")]
    pub quantity: SalesItemArrayOfQuantityComponent,
    #[serde(rename = "TaxExclusivePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exclusive_price: Option<SalesItemArrayOfTaxExclusivePriceComponent>,
    #[serde(rename = "TaxInclusivePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_inclusive_price: Option<SalesItemArrayOfTaxInclusivePriceComponent>,
}

impl AsMut<SalesItem> for SalesItem {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SalesItem> for SalesItem {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("SalesItem.item", e));
        }
        if let Err(e) = self.quantity.validate() {
            return Err(UblError::component("SalesItem.quantity", e));
        }
        if let Some(v) = &self.tax_inclusive_price {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SalesItem.tax_inclusive_price", e));
            }
        }
        if let Some(v) = &self.activity_property {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SalesItem.activity_property", e));
            }
        }
        if let Some(v) = &self.tax_exclusive_price {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SalesItem.tax_exclusive_price", e));
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

impl SalesItem {
    pub fn title() -> &'static str {
        "Sales Item. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe information related to an item in a sales context"
    }
    pub fn new(item: SalesItemArrayOfItemComponent, quantity: SalesItemArrayOfQuantityComponent) -> Component<Self> {
        Component(Self {
            activity_property: None,
            tax_inclusive_price: None,
            item,
            tax_exclusive_price: None,
            quantity,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SalesItemArrayOfActivityPropertyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ActivityProperty>,
}

impl AsMut<SalesItemArrayOfActivityPropertyComponent> for SalesItemArrayOfActivityPropertyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SalesItemArrayOfActivityPropertyComponent> for SalesItemArrayOfActivityPropertyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SalesItemArrayOfActivityPropertyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SalesItemArrayOfActivityPropertyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ActivityProperty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ActivityProperty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ActivityProperty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ActivityProperty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SalesItemArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<SalesItemArrayOfItemComponent> for SalesItemArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SalesItemArrayOfItemComponent> for SalesItemArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SalesItemArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SalesItemArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SalesItemArrayOfItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Item) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Item) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SalesItemArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<SalesItemArrayOfQuantityComponent> for SalesItemArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SalesItemArrayOfQuantityComponent> for SalesItemArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SalesItemArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SalesItemArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SalesItemArrayOfQuantityComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SalesItemArrayOfTaxExclusivePriceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxExclusivePrice>,
}

impl AsMut<SalesItemArrayOfTaxExclusivePriceComponent> for SalesItemArrayOfTaxExclusivePriceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SalesItemArrayOfTaxExclusivePriceComponent> for SalesItemArrayOfTaxExclusivePriceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SalesItemArrayOfTaxExclusivePriceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SalesItemArrayOfTaxExclusivePriceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxExclusivePrice) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxExclusivePrice) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxExclusivePrice> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxExclusivePrice> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SalesItemArrayOfTaxInclusivePriceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxInclusivePrice>,
}

impl AsMut<SalesItemArrayOfTaxInclusivePriceComponent> for SalesItemArrayOfTaxInclusivePriceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SalesItemArrayOfTaxInclusivePriceComponent> for SalesItemArrayOfTaxInclusivePriceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SalesItemArrayOfTaxInclusivePriceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SalesItemArrayOfTaxInclusivePriceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxInclusivePrice) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxInclusivePrice) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxInclusivePrice> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxInclusivePrice> {
        self.items.iter()
    }
}


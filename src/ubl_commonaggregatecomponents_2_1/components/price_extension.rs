use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceExtension {
    #[serde(rename = "Amount")]
    pub amount: PriceExtensionArrayOfAmountComponent,
    #[serde(rename = "TaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<PriceExtensionArrayOfTaxTotalComponent>,
}

impl AsMut<PriceExtension> for PriceExtension {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceExtension> for PriceExtension {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.amount.validate() {
            return Err(UblError::component("PriceExtension.amount", e));
        }
        if let Some(v) = &self.tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PriceExtension.tax_total", e));
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

impl PriceExtension {
    pub fn title() -> &'static str {
        "Price Extension. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a price extension, calculated by multiplying the price per unit by the quantity of items."
    }
    pub fn new(amount: PriceExtensionArrayOfAmountComponent) -> Component<Self> {
        Component(Self {
            amount,
            tax_total: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceExtensionArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<PriceExtensionArrayOfAmountComponent> for PriceExtensionArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceExtensionArrayOfAmountComponent> for PriceExtensionArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PriceExtensionArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PriceExtensionArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceExtensionArrayOfAmountComponent {
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
pub struct PriceExtensionArrayOfTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxTotal>,
}

impl AsMut<PriceExtensionArrayOfTaxTotalComponent> for PriceExtensionArrayOfTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceExtensionArrayOfTaxTotalComponent> for PriceExtensionArrayOfTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PriceExtensionArrayOfTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceExtensionArrayOfTaxTotalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter()
    }
}


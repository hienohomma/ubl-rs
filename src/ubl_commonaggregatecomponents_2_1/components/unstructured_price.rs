use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnstructuredPrice {
    #[serde(rename = "PriceAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_amount: Option<UnstructuredPriceArrayOfPriceAmountComponent>,
    #[serde(rename = "TimeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_amount: Option<UnstructuredPriceArrayOfTimeAmountComponent>,
}

impl AsMut<UnstructuredPrice> for UnstructuredPrice {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UnstructuredPrice> for UnstructuredPrice {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.price_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UnstructuredPrice.price_amount", e));
            }
        }
        if let Some(v) = &self.time_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UnstructuredPrice.time_amount", e));
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

impl UnstructuredPrice {
    pub fn title() -> &'static str {
        "Unstructured Price. Details"
    }
    pub fn description() -> &'static str {
        "A simplified version of the Price class intended for applications such as telephone billing."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            price_amount: None,
            time_amount: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UnstructuredPriceArrayOfPriceAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PriceAmount>,
}

impl AsMut<UnstructuredPriceArrayOfPriceAmountComponent> for UnstructuredPriceArrayOfPriceAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UnstructuredPriceArrayOfPriceAmountComponent> for UnstructuredPriceArrayOfPriceAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UnstructuredPriceArrayOfPriceAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UnstructuredPriceArrayOfPriceAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UnstructuredPriceArrayOfPriceAmountComponent {
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
pub struct UnstructuredPriceArrayOfTimeAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TimeAmount>,
}

impl AsMut<UnstructuredPriceArrayOfTimeAmountComponent> for UnstructuredPriceArrayOfTimeAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UnstructuredPriceArrayOfTimeAmountComponent> for UnstructuredPriceArrayOfTimeAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UnstructuredPriceArrayOfTimeAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UnstructuredPriceArrayOfTimeAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UnstructuredPriceArrayOfTimeAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TimeAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TimeAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TimeAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TimeAmount> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceList {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PriceListArrayOfIDComponent>,
    #[serde(rename = "PreviousPriceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_price_list: Option<PriceListArrayOfPreviousPriceListComponent>,
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<PriceListArrayOfStatusCodeComponent>,
    #[serde(rename = "ValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<PriceListArrayOfValidityPeriodComponent>,
}

impl AsMut<PriceList> for PriceList {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceList> for PriceList {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.previous_price_list {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PriceList.previous_price_list", e));
            }
        }
        if let Some(v) = &self.status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PriceList.status_code", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PriceList.id", e));
            }
        }
        if let Some(v) = &self.validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PriceList.validity_period", e));
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

impl PriceList {
    pub fn title() -> &'static str {
        "Price List. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a price list."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            status_code: None,
            id: None,
            validity_period: None,
            previous_price_list: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceListArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<PriceListArrayOfIDComponent> for PriceListArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceListArrayOfIDComponent> for PriceListArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PriceListArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PriceListArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceListArrayOfIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceListArrayOfPreviousPriceListComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PreviousPriceList>,
}

impl AsMut<PriceListArrayOfPreviousPriceListComponent> for PriceListArrayOfPreviousPriceListComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceListArrayOfPreviousPriceListComponent> for PriceListArrayOfPreviousPriceListComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PriceListArrayOfPreviousPriceListComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PriceListArrayOfPreviousPriceListComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceListArrayOfPreviousPriceListComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PreviousPriceList) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PreviousPriceList) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PreviousPriceList> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PreviousPriceList> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceListArrayOfStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::StatusCode>,
}

impl AsMut<PriceListArrayOfStatusCodeComponent> for PriceListArrayOfStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceListArrayOfStatusCodeComponent> for PriceListArrayOfStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PriceListArrayOfStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PriceListArrayOfStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceListArrayOfStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::StatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::StatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::StatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::StatusCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceListArrayOfValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod>,
}

impl AsMut<PriceListArrayOfValidityPeriodComponent> for PriceListArrayOfValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceListArrayOfValidityPeriodComponent> for PriceListArrayOfValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PriceListArrayOfValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceListArrayOfValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod> {
        self.items.iter()
    }
}


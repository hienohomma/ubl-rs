use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaxScheme {
    #[serde(rename = "CurrencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<TaxSchemeArrayOfCurrencyCodeComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TaxSchemeArrayOfIDComponent>,
    #[serde(rename = "JurisdictionRegionAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction_region_address: Option<TaxSchemeArrayOfJurisdictionRegionAddressComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TaxSchemeArrayOfNameComponent>,
    #[serde(rename = "TaxTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type_code: Option<TaxSchemeArrayOfTaxTypeCodeComponent>,
}

impl AsMut<TaxScheme> for TaxScheme {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxScheme> for TaxScheme {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxScheme.id", e));
            }
        }
        if let Some(v) = &self.currency_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxScheme.currency_code", e));
            }
        }
        if let Some(v) = &self.jurisdiction_region_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxScheme.jurisdiction_region_address", e));
            }
        }
        if let Some(v) = &self.tax_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxScheme.tax_type_code", e));
            }
        }
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxScheme.name", e));
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

impl TaxScheme {
    pub fn title() -> &'static str {
        "Tax Scheme. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a taxation scheme (e.g., VAT, State tax, County tax)."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            currency_code: None,
            jurisdiction_region_address: None,
            tax_type_code: None,
            id: None,
            name: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSchemeArrayOfCurrencyCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CurrencyCode>,
}

impl AsMut<TaxSchemeArrayOfCurrencyCodeComponent> for TaxSchemeArrayOfCurrencyCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSchemeArrayOfCurrencyCodeComponent> for TaxSchemeArrayOfCurrencyCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSchemeArrayOfCurrencyCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSchemeArrayOfCurrencyCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSchemeArrayOfCurrencyCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CurrencyCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CurrencyCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CurrencyCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CurrencyCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSchemeArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<TaxSchemeArrayOfIDComponent> for TaxSchemeArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSchemeArrayOfIDComponent> for TaxSchemeArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSchemeArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSchemeArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSchemeArrayOfIDComponent {
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
pub struct TaxSchemeArrayOfJurisdictionRegionAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::JurisdictionRegionAddress>,
}

impl AsMut<TaxSchemeArrayOfJurisdictionRegionAddressComponent> for TaxSchemeArrayOfJurisdictionRegionAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSchemeArrayOfJurisdictionRegionAddressComponent> for TaxSchemeArrayOfJurisdictionRegionAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TaxSchemeArrayOfJurisdictionRegionAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSchemeArrayOfJurisdictionRegionAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::JurisdictionRegionAddress) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::JurisdictionRegionAddress) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::JurisdictionRegionAddress> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::JurisdictionRegionAddress> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSchemeArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<TaxSchemeArrayOfNameComponent> for TaxSchemeArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSchemeArrayOfNameComponent> for TaxSchemeArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSchemeArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSchemeArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSchemeArrayOfNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Name) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Name) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSchemeArrayOfTaxTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxTypeCode>,
}

impl AsMut<TaxSchemeArrayOfTaxTypeCodeComponent> for TaxSchemeArrayOfTaxTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSchemeArrayOfTaxTypeCodeComponent> for TaxSchemeArrayOfTaxTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSchemeArrayOfTaxTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSchemeArrayOfTaxTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSchemeArrayOfTaxTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxTypeCode> {
        self.items.iter()
    }
}


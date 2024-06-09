use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorporateRegistrationScheme {
    #[serde(rename = "CorporateRegistrationTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corporate_registration_type_code: Option<CorporateRegistrationSchemeArrayOfCorporateRegistrationTypeCodeComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CorporateRegistrationSchemeArrayOfIDComponent>,
    #[serde(rename = "JurisdictionRegionAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction_region_address: Option<CorporateRegistrationSchemeArrayOfJurisdictionRegionAddressComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CorporateRegistrationSchemeArrayOfNameComponent>,
}

impl AsMut<CorporateRegistrationScheme> for CorporateRegistrationScheme {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CorporateRegistrationScheme> for CorporateRegistrationScheme {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.corporate_registration_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CorporateRegistrationScheme.corporate_registration_type_code", e));
            }
        }
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CorporateRegistrationScheme.name", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CorporateRegistrationScheme.id", e));
            }
        }
        if let Some(v) = &self.jurisdiction_region_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CorporateRegistrationScheme.jurisdiction_region_address", e));
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

impl CorporateRegistrationScheme {
    pub fn title() -> &'static str {
        "Corporate Registration Scheme. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a scheme for corporate registration."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            name: None,
            jurisdiction_region_address: None,
            corporate_registration_type_code: None,
            id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CorporateRegistrationSchemeArrayOfCorporateRegistrationTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CorporateRegistrationTypeCode>,
}

impl AsMut<CorporateRegistrationSchemeArrayOfCorporateRegistrationTypeCodeComponent> for CorporateRegistrationSchemeArrayOfCorporateRegistrationTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CorporateRegistrationSchemeArrayOfCorporateRegistrationTypeCodeComponent> for CorporateRegistrationSchemeArrayOfCorporateRegistrationTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CorporateRegistrationSchemeArrayOfCorporateRegistrationTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CorporateRegistrationSchemeArrayOfCorporateRegistrationTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CorporateRegistrationSchemeArrayOfCorporateRegistrationTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CorporateRegistrationTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CorporateRegistrationTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CorporateRegistrationTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CorporateRegistrationTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CorporateRegistrationSchemeArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<CorporateRegistrationSchemeArrayOfIDComponent> for CorporateRegistrationSchemeArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CorporateRegistrationSchemeArrayOfIDComponent> for CorporateRegistrationSchemeArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CorporateRegistrationSchemeArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CorporateRegistrationSchemeArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CorporateRegistrationSchemeArrayOfIDComponent {
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
pub struct CorporateRegistrationSchemeArrayOfJurisdictionRegionAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::JurisdictionRegionAddress>,
}

impl AsMut<CorporateRegistrationSchemeArrayOfJurisdictionRegionAddressComponent> for CorporateRegistrationSchemeArrayOfJurisdictionRegionAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CorporateRegistrationSchemeArrayOfJurisdictionRegionAddressComponent> for CorporateRegistrationSchemeArrayOfJurisdictionRegionAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CorporateRegistrationSchemeArrayOfJurisdictionRegionAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CorporateRegistrationSchemeArrayOfJurisdictionRegionAddressComponent {
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
pub struct CorporateRegistrationSchemeArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<CorporateRegistrationSchemeArrayOfNameComponent> for CorporateRegistrationSchemeArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CorporateRegistrationSchemeArrayOfNameComponent> for CorporateRegistrationSchemeArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CorporateRegistrationSchemeArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CorporateRegistrationSchemeArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CorporateRegistrationSchemeArrayOfNameComponent {
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


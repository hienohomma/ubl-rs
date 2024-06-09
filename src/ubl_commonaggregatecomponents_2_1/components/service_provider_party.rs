use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceProviderParty {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ServiceProviderPartyArrayOfIDComponent>,
    #[serde(rename = "Party")]
    pub party: ServiceProviderPartyArrayOfPartyComponent,
    #[serde(rename = "SellerContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_contact: Option<ServiceProviderPartyArrayOfSellerContactComponent>,
    #[serde(rename = "ServiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceProviderPartyArrayOfServiceTypeComponent>,
    #[serde(rename = "ServiceTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type_code: Option<ServiceProviderPartyArrayOfServiceTypeCodeComponent>,
}

impl AsMut<ServiceProviderParty> for ServiceProviderParty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ServiceProviderParty> for ServiceProviderParty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.party.validate() {
            return Err(UblError::component("ServiceProviderParty.party", e));
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ServiceProviderParty.id", e));
            }
        }
        if let Some(v) = &self.seller_contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ServiceProviderParty.seller_contact", e));
            }
        }
        if let Some(v) = &self.service_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ServiceProviderParty.service_type", e));
            }
        }
        if let Some(v) = &self.service_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ServiceProviderParty.service_type_code", e));
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

impl ServiceProviderParty {
    pub fn title() -> &'static str {
        "Service Provider Party. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a party contracting to provide services, such as transportation, finance, etc."
    }
    pub fn new(party: ServiceProviderPartyArrayOfPartyComponent) -> Component<Self> {
        Component(Self {
            service_type: None,
            id: None,
            seller_contact: None,
            service_type_code: None,
            party,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ServiceProviderPartyArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ServiceProviderPartyArrayOfIDComponent> for ServiceProviderPartyArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ServiceProviderPartyArrayOfIDComponent> for ServiceProviderPartyArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ServiceProviderPartyArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ServiceProviderPartyArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ServiceProviderPartyArrayOfIDComponent {
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
pub struct ServiceProviderPartyArrayOfPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Party>,
}

impl AsMut<ServiceProviderPartyArrayOfPartyComponent> for ServiceProviderPartyArrayOfPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ServiceProviderPartyArrayOfPartyComponent> for ServiceProviderPartyArrayOfPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ServiceProviderPartyArrayOfPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ServiceProviderPartyArrayOfPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ServiceProviderPartyArrayOfPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Party) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Party) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Party> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Party> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ServiceProviderPartyArrayOfSellerContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellerContact>,
}

impl AsMut<ServiceProviderPartyArrayOfSellerContactComponent> for ServiceProviderPartyArrayOfSellerContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ServiceProviderPartyArrayOfSellerContactComponent> for ServiceProviderPartyArrayOfSellerContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ServiceProviderPartyArrayOfSellerContactComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ServiceProviderPartyArrayOfSellerContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ServiceProviderPartyArrayOfSellerContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SellerContact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SellerContact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SellerContact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SellerContact> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ServiceProviderPartyArrayOfServiceTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ServiceType>,
}

impl AsMut<ServiceProviderPartyArrayOfServiceTypeComponent> for ServiceProviderPartyArrayOfServiceTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ServiceProviderPartyArrayOfServiceTypeComponent> for ServiceProviderPartyArrayOfServiceTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ServiceProviderPartyArrayOfServiceTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ServiceProviderPartyArrayOfServiceTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ServiceType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ServiceType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ServiceType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ServiceType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ServiceProviderPartyArrayOfServiceTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ServiceTypeCode>,
}

impl AsMut<ServiceProviderPartyArrayOfServiceTypeCodeComponent> for ServiceProviderPartyArrayOfServiceTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ServiceProviderPartyArrayOfServiceTypeCodeComponent> for ServiceProviderPartyArrayOfServiceTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ServiceProviderPartyArrayOfServiceTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ServiceProviderPartyArrayOfServiceTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ServiceProviderPartyArrayOfServiceTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ServiceTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ServiceTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ServiceTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ServiceTypeCode> {
        self.items.iter()
    }
}


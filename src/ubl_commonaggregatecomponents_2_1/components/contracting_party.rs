use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractingParty {
    #[serde(rename = "BuyerProfileURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_profile_uri: Option<ContractingPartyArrayOfBuyerProfileURIComponent>,
    #[serde(rename = "ContractingActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracting_activity: Option<ContractingPartyArrayOfContractingActivityComponent>,
    #[serde(rename = "ContractingPartyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracting_party_type: Option<ContractingPartyArrayOfContractingPartyTypeComponent>,
    #[serde(rename = "Party")]
    pub party: ContractingPartyArrayOfPartyComponent,
}

impl AsMut<ContractingParty> for ContractingParty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractingParty> for ContractingParty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.buyer_profile_uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractingParty.buyer_profile_uri", e));
            }
        }
        if let Some(v) = &self.contracting_party_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractingParty.contracting_party_type", e));
            }
        }
        if let Some(v) = &self.contracting_activity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractingParty.contracting_activity", e));
            }
        }
        if let Err(e) = self.party.validate() {
            return Err(UblError::component("ContractingParty.party", e));
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

impl ContractingParty {
    pub fn title() -> &'static str {
        "Contracting Party. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an individual, a group, or a body having a procurement role in a tendering process."
    }
    pub fn new(party: ContractingPartyArrayOfPartyComponent) -> Component<Self> {
        Component(Self {
            buyer_profile_uri: None,
            contracting_party_type: None,
            party,
            contracting_activity: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractingPartyArrayOfBuyerProfileURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BuyerProfileURI>,
}

impl AsMut<ContractingPartyArrayOfBuyerProfileURIComponent> for ContractingPartyArrayOfBuyerProfileURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractingPartyArrayOfBuyerProfileURIComponent> for ContractingPartyArrayOfBuyerProfileURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContractingPartyArrayOfBuyerProfileURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContractingPartyArrayOfBuyerProfileURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractingPartyArrayOfBuyerProfileURIComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BuyerProfileURI) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BuyerProfileURI) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BuyerProfileURI> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BuyerProfileURI> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractingPartyArrayOfContractingActivityComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractingActivity>,
}

impl AsMut<ContractingPartyArrayOfContractingActivityComponent> for ContractingPartyArrayOfContractingActivityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractingPartyArrayOfContractingActivityComponent> for ContractingPartyArrayOfContractingActivityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ContractingPartyArrayOfContractingActivityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractingPartyArrayOfContractingActivityComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContractingActivity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContractingActivity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContractingActivity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContractingActivity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractingPartyArrayOfContractingPartyTypeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractingPartyType>,
}

impl AsMut<ContractingPartyArrayOfContractingPartyTypeComponent> for ContractingPartyArrayOfContractingPartyTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractingPartyArrayOfContractingPartyTypeComponent> for ContractingPartyArrayOfContractingPartyTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ContractingPartyArrayOfContractingPartyTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractingPartyArrayOfContractingPartyTypeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContractingPartyType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContractingPartyType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContractingPartyType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContractingPartyType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractingPartyArrayOfPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Party>,
}

impl AsMut<ContractingPartyArrayOfPartyComponent> for ContractingPartyArrayOfPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractingPartyArrayOfPartyComponent> for ContractingPartyArrayOfPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContractingPartyArrayOfPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContractingPartyArrayOfPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractingPartyArrayOfPartyComponent {
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


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractingPartyType {
    #[serde(rename = "PartyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_type: Option<ContractingPartyTypeArrayOfPartyTypeComponent>,
    #[serde(rename = "PartyTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_type_code: Option<ContractingPartyTypeArrayOfPartyTypeCodeComponent>,
}

impl AsMut<ContractingPartyType> for ContractingPartyType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractingPartyType> for ContractingPartyType {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.party_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractingPartyType.party_type", e));
            }
        }
        if let Some(v) = &self.party_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractingPartyType.party_type_code", e));
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

impl ContractingPartyType {
    pub fn title() -> &'static str {
        "Contracting Party Type. Details"
    }
    pub fn description() -> &'static str {
        "The type of contracting party that is independent of its role."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            party_type: None,
            party_type_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractingPartyTypeArrayOfPartyTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PartyType>,
}

impl AsMut<ContractingPartyTypeArrayOfPartyTypeComponent> for ContractingPartyTypeArrayOfPartyTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractingPartyTypeArrayOfPartyTypeComponent> for ContractingPartyTypeArrayOfPartyTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContractingPartyTypeArrayOfPartyTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContractingPartyTypeArrayOfPartyTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractingPartyTypeArrayOfPartyTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PartyType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PartyType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PartyType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PartyType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractingPartyTypeArrayOfPartyTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PartyTypeCode>,
}

impl AsMut<ContractingPartyTypeArrayOfPartyTypeCodeComponent> for ContractingPartyTypeArrayOfPartyTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractingPartyTypeArrayOfPartyTypeCodeComponent> for ContractingPartyTypeArrayOfPartyTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContractingPartyTypeArrayOfPartyTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContractingPartyTypeArrayOfPartyTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractingPartyTypeArrayOfPartyTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PartyTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PartyTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PartyTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PartyTypeCode> {
        self.items.iter()
    }
}


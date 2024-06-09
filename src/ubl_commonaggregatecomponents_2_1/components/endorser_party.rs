use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EndorserParty {
    #[serde(rename = "Party")]
    pub party: EndorserPartyArrayOfPartyComponent,
    #[serde(rename = "RoleCode")]
    pub role_code: EndorserPartyArrayOfRoleCodeComponent,
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: EndorserPartyArrayOfSequenceNumericComponent,
    #[serde(rename = "SignatoryContact")]
    pub signatory_contact: EndorserPartyArrayOfSignatoryContactComponent,
}

impl AsMut<EndorserParty> for EndorserParty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EndorserParty> for EndorserParty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.signatory_contact.validate() {
            return Err(UblError::component("EndorserParty.signatory_contact", e));
        }
        if let Err(e) = self.sequence_numeric.validate() {
            return Err(UblError::component("EndorserParty.sequence_numeric", e));
        }
        if let Err(e) = self.role_code.validate() {
            return Err(UblError::component("EndorserParty.role_code", e));
        }
        if let Err(e) = self.party.validate() {
            return Err(UblError::component("EndorserParty.party", e));
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

impl EndorserParty {
    pub fn title() -> &'static str {
        "Endorser Party. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the party endorsing a document."
    }
    pub fn new(party: EndorserPartyArrayOfPartyComponent, role_code: EndorserPartyArrayOfRoleCodeComponent, sequence_numeric: EndorserPartyArrayOfSequenceNumericComponent, signatory_contact: EndorserPartyArrayOfSignatoryContactComponent) -> Component<Self> {
        Component(Self {
            party,
            signatory_contact,
            role_code,
            sequence_numeric,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EndorserPartyArrayOfPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Party>,
}

impl AsMut<EndorserPartyArrayOfPartyComponent> for EndorserPartyArrayOfPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EndorserPartyArrayOfPartyComponent> for EndorserPartyArrayOfPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EndorserPartyArrayOfPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EndorserPartyArrayOfPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EndorserPartyArrayOfPartyComponent {
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
pub struct EndorserPartyArrayOfRoleCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RoleCode>,
}

impl AsMut<EndorserPartyArrayOfRoleCodeComponent> for EndorserPartyArrayOfRoleCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EndorserPartyArrayOfRoleCodeComponent> for EndorserPartyArrayOfRoleCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EndorserPartyArrayOfRoleCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EndorserPartyArrayOfRoleCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EndorserPartyArrayOfRoleCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RoleCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RoleCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RoleCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RoleCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EndorserPartyArrayOfSequenceNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric>,
}

impl AsMut<EndorserPartyArrayOfSequenceNumericComponent> for EndorserPartyArrayOfSequenceNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EndorserPartyArrayOfSequenceNumericComponent> for EndorserPartyArrayOfSequenceNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EndorserPartyArrayOfSequenceNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EndorserPartyArrayOfSequenceNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EndorserPartyArrayOfSequenceNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SequenceNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SequenceNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EndorserPartyArrayOfSignatoryContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SignatoryContact>,
}

impl AsMut<EndorserPartyArrayOfSignatoryContactComponent> for EndorserPartyArrayOfSignatoryContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EndorserPartyArrayOfSignatoryContactComponent> for EndorserPartyArrayOfSignatoryContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EndorserPartyArrayOfSignatoryContactComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EndorserPartyArrayOfSignatoryContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EndorserPartyArrayOfSignatoryContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SignatoryContact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SignatoryContact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SignatoryContact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SignatoryContact> {
        self.items.iter()
    }
}


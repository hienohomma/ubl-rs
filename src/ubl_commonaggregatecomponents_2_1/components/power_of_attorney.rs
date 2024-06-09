use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PowerOfAttorney {
    #[serde(rename = "AgentParty")]
    pub agent_party: PowerOfAttorneyArrayOfAgentPartyComponent,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<PowerOfAttorneyArrayOfDescriptionComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PowerOfAttorneyArrayOfIDComponent>,
    #[serde(rename = "IssueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<PowerOfAttorneyArrayOfIssueDateComponent>,
    #[serde(rename = "IssueTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<PowerOfAttorneyArrayOfIssueTimeComponent>,
    #[serde(rename = "MandateDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_document_reference: Option<PowerOfAttorneyArrayOfMandateDocumentReferenceComponent>,
    #[serde(rename = "NotaryParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notary_party: Option<PowerOfAttorneyArrayOfNotaryPartyComponent>,
    #[serde(rename = "WitnessParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_party: Option<PowerOfAttorneyArrayOfWitnessPartyComponent>,
}

impl AsMut<PowerOfAttorney> for PowerOfAttorney {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PowerOfAttorney> for PowerOfAttorney {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.witness_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PowerOfAttorney.witness_party", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PowerOfAttorney.description", e));
            }
        }
        if let Some(v) = &self.issue_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PowerOfAttorney.issue_time", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PowerOfAttorney.id", e));
            }
        }
        if let Some(v) = &self.mandate_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PowerOfAttorney.mandate_document_reference", e));
            }
        }
        if let Some(v) = &self.issue_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PowerOfAttorney.issue_date", e));
            }
        }
        if let Err(e) = self.agent_party.validate() {
            return Err(UblError::component("PowerOfAttorney.agent_party", e));
        }
        if let Some(v) = &self.notary_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PowerOfAttorney.notary_party", e));
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

impl PowerOfAttorney {
    pub fn title() -> &'static str {
        "Power Of Attorney. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a power of attorney."
    }
    pub fn new(agent_party: PowerOfAttorneyArrayOfAgentPartyComponent) -> Component<Self> {
        Component(Self {
            description: None,
            agent_party,
            id: None,
            mandate_document_reference: None,
            notary_party: None,
            issue_date: None,
            issue_time: None,
            witness_party: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PowerOfAttorneyArrayOfAgentPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AgentParty>,
}

impl AsMut<PowerOfAttorneyArrayOfAgentPartyComponent> for PowerOfAttorneyArrayOfAgentPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PowerOfAttorneyArrayOfAgentPartyComponent> for PowerOfAttorneyArrayOfAgentPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfAgentPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfAgentPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PowerOfAttorneyArrayOfAgentPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AgentParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AgentParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AgentParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AgentParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PowerOfAttorneyArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<PowerOfAttorneyArrayOfDescriptionComponent> for PowerOfAttorneyArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PowerOfAttorneyArrayOfDescriptionComponent> for PowerOfAttorneyArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PowerOfAttorneyArrayOfDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Description) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Description) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PowerOfAttorneyArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<PowerOfAttorneyArrayOfIDComponent> for PowerOfAttorneyArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PowerOfAttorneyArrayOfIDComponent> for PowerOfAttorneyArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PowerOfAttorneyArrayOfIDComponent {
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
pub struct PowerOfAttorneyArrayOfIssueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueDate>,
}

impl AsMut<PowerOfAttorneyArrayOfIssueDateComponent> for PowerOfAttorneyArrayOfIssueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PowerOfAttorneyArrayOfIssueDateComponent> for PowerOfAttorneyArrayOfIssueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfIssueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfIssueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PowerOfAttorneyArrayOfIssueDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IssueDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IssueDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IssueDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IssueDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PowerOfAttorneyArrayOfIssueTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueTime>,
}

impl AsMut<PowerOfAttorneyArrayOfIssueTimeComponent> for PowerOfAttorneyArrayOfIssueTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PowerOfAttorneyArrayOfIssueTimeComponent> for PowerOfAttorneyArrayOfIssueTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfIssueTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfIssueTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PowerOfAttorneyArrayOfIssueTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IssueTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IssueTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IssueTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IssueTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PowerOfAttorneyArrayOfMandateDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MandateDocumentReference>,
}

impl AsMut<PowerOfAttorneyArrayOfMandateDocumentReferenceComponent> for PowerOfAttorneyArrayOfMandateDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PowerOfAttorneyArrayOfMandateDocumentReferenceComponent> for PowerOfAttorneyArrayOfMandateDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfMandateDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PowerOfAttorneyArrayOfMandateDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MandateDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MandateDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MandateDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MandateDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PowerOfAttorneyArrayOfNotaryPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::NotaryParty>,
}

impl AsMut<PowerOfAttorneyArrayOfNotaryPartyComponent> for PowerOfAttorneyArrayOfNotaryPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PowerOfAttorneyArrayOfNotaryPartyComponent> for PowerOfAttorneyArrayOfNotaryPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfNotaryPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfNotaryPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PowerOfAttorneyArrayOfNotaryPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::NotaryParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::NotaryParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::NotaryParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::NotaryParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PowerOfAttorneyArrayOfWitnessPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WitnessParty>,
}

impl AsMut<PowerOfAttorneyArrayOfWitnessPartyComponent> for PowerOfAttorneyArrayOfWitnessPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PowerOfAttorneyArrayOfWitnessPartyComponent> for PowerOfAttorneyArrayOfWitnessPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PowerOfAttorneyArrayOfWitnessPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PowerOfAttorneyArrayOfWitnessPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WitnessParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WitnessParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WitnessParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WitnessParty> {
        self.items.iter()
    }
}


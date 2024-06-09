use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Endorsement {
    #[serde(rename = "ApprovalStatus")]
    pub approval_status: EndorsementArrayOfApprovalStatusComponent,
    #[serde(rename = "DocumentID")]
    pub document_id: EndorsementArrayOfDocumentIDComponent,
    #[serde(rename = "EndorserParty")]
    pub endorser_party: EndorsementArrayOfEndorserPartyComponent,
    #[serde(rename = "Remarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<EndorsementArrayOfRemarksComponent>,
    #[serde(rename = "Signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<EndorsementArrayOfSignatureComponent>,
}

impl AsMut<Endorsement> for Endorsement {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Endorsement> for Endorsement {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.document_id.validate() {
            return Err(UblError::component("Endorsement.document_id", e));
        }
        if let Some(v) = &self.remarks {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Endorsement.remarks", e));
            }
        }
        if let Err(e) = self.approval_status.validate() {
            return Err(UblError::component("Endorsement.approval_status", e));
        }
        if let Err(e) = self.endorser_party.validate() {
            return Err(UblError::component("Endorsement.endorser_party", e));
        }
        if let Some(v) = &self.signature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Endorsement.signature", e));
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

impl Endorsement {
    pub fn title() -> &'static str {
        "Endorsement. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an endorsement of a document."
    }
    pub fn new(approval_status: EndorsementArrayOfApprovalStatusComponent, document_id: EndorsementArrayOfDocumentIDComponent, endorser_party: EndorsementArrayOfEndorserPartyComponent) -> Component<Self> {
        Component(Self {
            endorser_party,
            approval_status,
            signature: None,
            remarks: None,
            document_id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EndorsementArrayOfApprovalStatusComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ApprovalStatus>,
}

impl AsMut<EndorsementArrayOfApprovalStatusComponent> for EndorsementArrayOfApprovalStatusComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EndorsementArrayOfApprovalStatusComponent> for EndorsementArrayOfApprovalStatusComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EndorsementArrayOfApprovalStatusComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EndorsementArrayOfApprovalStatusComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EndorsementArrayOfApprovalStatusComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ApprovalStatus) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ApprovalStatus) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ApprovalStatus> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ApprovalStatus> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EndorsementArrayOfDocumentIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DocumentID>,
}

impl AsMut<EndorsementArrayOfDocumentIDComponent> for EndorsementArrayOfDocumentIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EndorsementArrayOfDocumentIDComponent> for EndorsementArrayOfDocumentIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EndorsementArrayOfDocumentIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EndorsementArrayOfDocumentIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EndorsementArrayOfDocumentIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DocumentID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DocumentID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DocumentID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DocumentID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EndorsementArrayOfEndorserPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EndorserParty>,
}

impl AsMut<EndorsementArrayOfEndorserPartyComponent> for EndorsementArrayOfEndorserPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EndorsementArrayOfEndorserPartyComponent> for EndorsementArrayOfEndorserPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EndorsementArrayOfEndorserPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EndorsementArrayOfEndorserPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EndorsementArrayOfEndorserPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EndorserParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EndorserParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EndorserParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EndorserParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EndorsementArrayOfRemarksComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Remarks>,
}

impl AsMut<EndorsementArrayOfRemarksComponent> for EndorsementArrayOfRemarksComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EndorsementArrayOfRemarksComponent> for EndorsementArrayOfRemarksComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EndorsementArrayOfRemarksComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EndorsementArrayOfRemarksComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Remarks) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Remarks) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Remarks> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Remarks> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EndorsementArrayOfSignatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Signature>,
}

impl AsMut<EndorsementArrayOfSignatureComponent> for EndorsementArrayOfSignatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EndorsementArrayOfSignatureComponent> for EndorsementArrayOfSignatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EndorsementArrayOfSignatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EndorsementArrayOfSignatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Signature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Signature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Signature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Signature> {
        self.items.iter()
    }
}


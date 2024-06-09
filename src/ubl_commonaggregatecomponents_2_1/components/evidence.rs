use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Evidence {
    #[serde(rename = "CandidateStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_statement: Option<EvidenceArrayOfCandidateStatementComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<EvidenceArrayOfDescriptionComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<EvidenceArrayOfDocumentReferenceComponent>,
    #[serde(rename = "EvidenceIssuingParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_issuing_party: Option<EvidenceArrayOfEvidenceIssuingPartyComponent>,
    #[serde(rename = "EvidenceTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_type_code: Option<EvidenceArrayOfEvidenceTypeCodeComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<EvidenceArrayOfIDComponent>,
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<EvidenceArrayOfLanguageComponent>,
}

impl AsMut<Evidence> for Evidence {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Evidence> for Evidence {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.evidence_issuing_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Evidence.evidence_issuing_party", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Evidence.id", e));
            }
        }
        if let Some(v) = &self.evidence_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Evidence.evidence_type_code", e));
            }
        }
        if let Some(v) = &self.candidate_statement {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Evidence.candidate_statement", e));
            }
        }
        if let Some(v) = &self.language {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Evidence.language", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Evidence.document_reference", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Evidence.description", e));
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

impl Evidence {
    pub fn title() -> &'static str {
        "Evidence. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an item of evidentiary support for representations of capabilities or the ability to meet tendering requirements, which an economic operator must provide for acceptance into a tendering process."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            candidate_statement: None,
            description: None,
            document_reference: None,
            evidence_issuing_party: None,
            evidence_type_code: None,
            id: None,
            language: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvidenceArrayOfCandidateStatementComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CandidateStatement>,
}

impl AsMut<EvidenceArrayOfCandidateStatementComponent> for EvidenceArrayOfCandidateStatementComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvidenceArrayOfCandidateStatementComponent> for EvidenceArrayOfCandidateStatementComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EvidenceArrayOfCandidateStatementComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvidenceArrayOfCandidateStatementComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CandidateStatement) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CandidateStatement) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CandidateStatement> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CandidateStatement> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvidenceArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<EvidenceArrayOfDescriptionComponent> for EvidenceArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvidenceArrayOfDescriptionComponent> for EvidenceArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EvidenceArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvidenceArrayOfDescriptionComponent {
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
pub struct EvidenceArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<EvidenceArrayOfDocumentReferenceComponent> for EvidenceArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvidenceArrayOfDocumentReferenceComponent> for EvidenceArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EvidenceArrayOfDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EvidenceArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvidenceArrayOfDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvidenceArrayOfEvidenceIssuingPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EvidenceIssuingParty>,
}

impl AsMut<EvidenceArrayOfEvidenceIssuingPartyComponent> for EvidenceArrayOfEvidenceIssuingPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvidenceArrayOfEvidenceIssuingPartyComponent> for EvidenceArrayOfEvidenceIssuingPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EvidenceArrayOfEvidenceIssuingPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EvidenceArrayOfEvidenceIssuingPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvidenceArrayOfEvidenceIssuingPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EvidenceIssuingParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EvidenceIssuingParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EvidenceIssuingParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EvidenceIssuingParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvidenceArrayOfEvidenceTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EvidenceTypeCode>,
}

impl AsMut<EvidenceArrayOfEvidenceTypeCodeComponent> for EvidenceArrayOfEvidenceTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvidenceArrayOfEvidenceTypeCodeComponent> for EvidenceArrayOfEvidenceTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EvidenceArrayOfEvidenceTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EvidenceArrayOfEvidenceTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvidenceArrayOfEvidenceTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EvidenceTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EvidenceTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EvidenceTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EvidenceTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvidenceArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<EvidenceArrayOfIDComponent> for EvidenceArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvidenceArrayOfIDComponent> for EvidenceArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EvidenceArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EvidenceArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvidenceArrayOfIDComponent {
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
pub struct EvidenceArrayOfLanguageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Language>,
}

impl AsMut<EvidenceArrayOfLanguageComponent> for EvidenceArrayOfLanguageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvidenceArrayOfLanguageComponent> for EvidenceArrayOfLanguageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EvidenceArrayOfLanguageComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EvidenceArrayOfLanguageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvidenceArrayOfLanguageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Language) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Language) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Language> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Language> {
        self.items.iter()
    }
}


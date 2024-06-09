use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocumentResponse {
    #[serde(rename = "DocumentReference")]
    pub document_reference: DocumentResponseArrayOfDocumentReferenceComponent,
    #[serde(rename = "IssuerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_party: Option<DocumentResponseArrayOfIssuerPartyComponent>,
    #[serde(rename = "LineResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_response: Option<DocumentResponseArrayOfLineResponseComponent>,
    #[serde(rename = "RecipientParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_party: Option<DocumentResponseArrayOfRecipientPartyComponent>,
    #[serde(rename = "Response")]
    pub response: DocumentResponseArrayOfResponseComponent,
}

impl AsMut<DocumentResponse> for DocumentResponse {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentResponse> for DocumentResponse {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.document_reference.validate() {
            return Err(UblError::component("DocumentResponse.document_reference", e));
        }
        if let Some(v) = &self.issuer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentResponse.issuer_party", e));
            }
        }
        if let Some(v) = &self.line_response {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentResponse.line_response", e));
            }
        }
        if let Some(v) = &self.recipient_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentResponse.recipient_party", e));
            }
        }
        if let Err(e) = self.response.validate() {
            return Err(UblError::component("DocumentResponse.response", e));
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

impl DocumentResponse {
    pub fn title() -> &'static str {
        "Document Response. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an application-level response to a document."
    }
    pub fn new(document_reference: DocumentResponseArrayOfDocumentReferenceComponent, response: DocumentResponseArrayOfResponseComponent) -> Component<Self> {
        Component(Self {
            line_response: None,
            document_reference,
            response,
            recipient_party: None,
            issuer_party: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentResponseArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<DocumentResponseArrayOfDocumentReferenceComponent> for DocumentResponseArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentResponseArrayOfDocumentReferenceComponent> for DocumentResponseArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DocumentResponseArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentResponseArrayOfDocumentReferenceComponent {
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
pub struct DocumentResponseArrayOfIssuerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::IssuerParty>,
}

impl AsMut<DocumentResponseArrayOfIssuerPartyComponent> for DocumentResponseArrayOfIssuerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentResponseArrayOfIssuerPartyComponent> for DocumentResponseArrayOfIssuerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentResponseArrayOfIssuerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentResponseArrayOfIssuerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentResponseArrayOfIssuerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::IssuerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::IssuerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::IssuerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::IssuerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentResponseArrayOfLineResponseComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LineResponse>,
}

impl AsMut<DocumentResponseArrayOfLineResponseComponent> for DocumentResponseArrayOfLineResponseComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentResponseArrayOfLineResponseComponent> for DocumentResponseArrayOfLineResponseComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DocumentResponseArrayOfLineResponseComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentResponseArrayOfLineResponseComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LineResponse) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LineResponse) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LineResponse> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LineResponse> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentResponseArrayOfRecipientPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RecipientParty>,
}

impl AsMut<DocumentResponseArrayOfRecipientPartyComponent> for DocumentResponseArrayOfRecipientPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentResponseArrayOfRecipientPartyComponent> for DocumentResponseArrayOfRecipientPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentResponseArrayOfRecipientPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentResponseArrayOfRecipientPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentResponseArrayOfRecipientPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RecipientParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RecipientParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RecipientParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RecipientParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentResponseArrayOfResponseComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Response>,
}

impl AsMut<DocumentResponseArrayOfResponseComponent> for DocumentResponseArrayOfResponseComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentResponseArrayOfResponseComponent> for DocumentResponseArrayOfResponseComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentResponseArrayOfResponseComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentResponseArrayOfResponseComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentResponseArrayOfResponseComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Response) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Response) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Response> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Response> {
        self.items.iter()
    }
}


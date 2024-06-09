use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attachment {
    #[serde(rename = "EmbeddedDocumentBinaryObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_document_binary_object: Option<AttachmentArrayOfEmbeddedDocumentBinaryObjectComponent>,
    #[serde(rename = "ExternalReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<AttachmentArrayOfExternalReferenceComponent>,
}

impl AsMut<Attachment> for Attachment {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Attachment> for Attachment {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.embedded_document_binary_object {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Attachment.embedded_document_binary_object", e));
            }
        }
        if let Some(v) = &self.external_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Attachment.external_reference", e));
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

impl Attachment {
    pub fn title() -> &'static str {
        "Attachment. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an attached document. An attachment can refer to an external document or be included with the document being exchanged."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            embedded_document_binary_object: None,
            external_reference: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AttachmentArrayOfEmbeddedDocumentBinaryObjectComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EmbeddedDocumentBinaryObject>,
}

impl AsMut<AttachmentArrayOfEmbeddedDocumentBinaryObjectComponent> for AttachmentArrayOfEmbeddedDocumentBinaryObjectComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AttachmentArrayOfEmbeddedDocumentBinaryObjectComponent> for AttachmentArrayOfEmbeddedDocumentBinaryObjectComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AttachmentArrayOfEmbeddedDocumentBinaryObjectComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AttachmentArrayOfEmbeddedDocumentBinaryObjectComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AttachmentArrayOfEmbeddedDocumentBinaryObjectComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EmbeddedDocumentBinaryObject) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EmbeddedDocumentBinaryObject) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EmbeddedDocumentBinaryObject> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EmbeddedDocumentBinaryObject> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AttachmentArrayOfExternalReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExternalReference>,
}

impl AsMut<AttachmentArrayOfExternalReferenceComponent> for AttachmentArrayOfExternalReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AttachmentArrayOfExternalReferenceComponent> for AttachmentArrayOfExternalReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AttachmentArrayOfExternalReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AttachmentArrayOfExternalReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AttachmentArrayOfExternalReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ExternalReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ExternalReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ExternalReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ExternalReference> {
        self.items.iter()
    }
}


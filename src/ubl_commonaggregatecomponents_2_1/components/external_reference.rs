use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExternalReference {
    #[serde(rename = "CharacterSetCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_code: Option<ExternalReferenceArrayOfCharacterSetCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ExternalReferenceArrayOfDescriptionComponent>,
    #[serde(rename = "DocumentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash: Option<ExternalReferenceArrayOfDocumentHashComponent>,
    #[serde(rename = "EncodingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_code: Option<ExternalReferenceArrayOfEncodingCodeComponent>,
    #[serde(rename = "ExpiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<ExternalReferenceArrayOfExpiryDateComponent>,
    #[serde(rename = "ExpiryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<ExternalReferenceArrayOfExpiryTimeComponent>,
    #[serde(rename = "FileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<ExternalReferenceArrayOfFileNameComponent>,
    #[serde(rename = "FormatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_code: Option<ExternalReferenceArrayOfFormatCodeComponent>,
    #[serde(rename = "HashAlgorithmMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_algorithm_method: Option<ExternalReferenceArrayOfHashAlgorithmMethodComponent>,
    #[serde(rename = "MimeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_code: Option<ExternalReferenceArrayOfMimeCodeComponent>,
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<ExternalReferenceArrayOfURIComponent>,
}

impl AsMut<ExternalReference> for ExternalReference {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReference> for ExternalReference {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.document_hash {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExternalReference.document_hash", e));
            }
        }
        if let Some(v) = &self.expiry_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExternalReference.expiry_date", e));
            }
        }
        if let Some(v) = &self.character_set_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExternalReference.character_set_code", e));
            }
        }
        if let Some(v) = &self.expiry_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExternalReference.expiry_time", e));
            }
        }
        if let Some(v) = &self.format_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExternalReference.format_code", e));
            }
        }
        if let Some(v) = &self.file_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExternalReference.file_name", e));
            }
        }
        if let Some(v) = &self.encoding_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExternalReference.encoding_code", e));
            }
        }
        if let Some(v) = &self.mime_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExternalReference.mime_code", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExternalReference.description", e));
            }
        }
        if let Some(v) = &self.uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExternalReference.uri", e));
            }
        }
        if let Some(v) = &self.hash_algorithm_method {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExternalReference.hash_algorithm_method", e));
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

impl ExternalReference {
    pub fn title() -> &'static str {
        "External Reference. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an external object, such as a document stored at a remote location."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            format_code: None,
            encoding_code: None,
            expiry_date: None,
            file_name: None,
            document_hash: None,
            character_set_code: None,
            mime_code: None,
            description: None,
            hash_algorithm_method: None,
            uri: None,
            expiry_time: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExternalReferenceArrayOfCharacterSetCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CharacterSetCode>,
}

impl AsMut<ExternalReferenceArrayOfCharacterSetCodeComponent> for ExternalReferenceArrayOfCharacterSetCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReferenceArrayOfCharacterSetCodeComponent> for ExternalReferenceArrayOfCharacterSetCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfCharacterSetCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfCharacterSetCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExternalReferenceArrayOfCharacterSetCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CharacterSetCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CharacterSetCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CharacterSetCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CharacterSetCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExternalReferenceArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ExternalReferenceArrayOfDescriptionComponent> for ExternalReferenceArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReferenceArrayOfDescriptionComponent> for ExternalReferenceArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExternalReferenceArrayOfDescriptionComponent {
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
pub struct ExternalReferenceArrayOfDocumentHashComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DocumentHash>,
}

impl AsMut<ExternalReferenceArrayOfDocumentHashComponent> for ExternalReferenceArrayOfDocumentHashComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReferenceArrayOfDocumentHashComponent> for ExternalReferenceArrayOfDocumentHashComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfDocumentHashComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfDocumentHashComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExternalReferenceArrayOfDocumentHashComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DocumentHash) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DocumentHash) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DocumentHash> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DocumentHash> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExternalReferenceArrayOfEncodingCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EncodingCode>,
}

impl AsMut<ExternalReferenceArrayOfEncodingCodeComponent> for ExternalReferenceArrayOfEncodingCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReferenceArrayOfEncodingCodeComponent> for ExternalReferenceArrayOfEncodingCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfEncodingCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfEncodingCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExternalReferenceArrayOfEncodingCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EncodingCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EncodingCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EncodingCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EncodingCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExternalReferenceArrayOfExpiryDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExpiryDate>,
}

impl AsMut<ExternalReferenceArrayOfExpiryDateComponent> for ExternalReferenceArrayOfExpiryDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReferenceArrayOfExpiryDateComponent> for ExternalReferenceArrayOfExpiryDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfExpiryDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfExpiryDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExternalReferenceArrayOfExpiryDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExpiryDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExpiryDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExpiryDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExpiryDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExternalReferenceArrayOfExpiryTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExpiryTime>,
}

impl AsMut<ExternalReferenceArrayOfExpiryTimeComponent> for ExternalReferenceArrayOfExpiryTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReferenceArrayOfExpiryTimeComponent> for ExternalReferenceArrayOfExpiryTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfExpiryTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfExpiryTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExternalReferenceArrayOfExpiryTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExpiryTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExpiryTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExpiryTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExpiryTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExternalReferenceArrayOfFileNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FileName>,
}

impl AsMut<ExternalReferenceArrayOfFileNameComponent> for ExternalReferenceArrayOfFileNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReferenceArrayOfFileNameComponent> for ExternalReferenceArrayOfFileNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfFileNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfFileNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExternalReferenceArrayOfFileNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FileName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FileName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FileName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FileName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExternalReferenceArrayOfFormatCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FormatCode>,
}

impl AsMut<ExternalReferenceArrayOfFormatCodeComponent> for ExternalReferenceArrayOfFormatCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReferenceArrayOfFormatCodeComponent> for ExternalReferenceArrayOfFormatCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfFormatCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfFormatCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExternalReferenceArrayOfFormatCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FormatCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FormatCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FormatCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FormatCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExternalReferenceArrayOfHashAlgorithmMethodComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HashAlgorithmMethod>,
}

impl AsMut<ExternalReferenceArrayOfHashAlgorithmMethodComponent> for ExternalReferenceArrayOfHashAlgorithmMethodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReferenceArrayOfHashAlgorithmMethodComponent> for ExternalReferenceArrayOfHashAlgorithmMethodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfHashAlgorithmMethodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfHashAlgorithmMethodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExternalReferenceArrayOfHashAlgorithmMethodComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HashAlgorithmMethod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HashAlgorithmMethod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HashAlgorithmMethod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HashAlgorithmMethod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExternalReferenceArrayOfMimeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MimeCode>,
}

impl AsMut<ExternalReferenceArrayOfMimeCodeComponent> for ExternalReferenceArrayOfMimeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReferenceArrayOfMimeCodeComponent> for ExternalReferenceArrayOfMimeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfMimeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfMimeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExternalReferenceArrayOfMimeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MimeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MimeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MimeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MimeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExternalReferenceArrayOfURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::URI>,
}

impl AsMut<ExternalReferenceArrayOfURIComponent> for ExternalReferenceArrayOfURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExternalReferenceArrayOfURIComponent> for ExternalReferenceArrayOfURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExternalReferenceArrayOfURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExternalReferenceArrayOfURIComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::URI) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::URI) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::URI> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::URI> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signature {
    #[serde(rename = "CanonicalizationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonicalization_method: Option<SignatureArrayOfCanonicalizationMethodComponent>,
    #[serde(rename = "DigitalSignatureAttachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_signature_attachment: Option<SignatureArrayOfDigitalSignatureAttachmentComponent>,
    #[serde(rename = "ID")]
    pub id: SignatureArrayOfIDComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<SignatureArrayOfNoteComponent>,
    #[serde(rename = "OriginalDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_document_reference: Option<SignatureArrayOfOriginalDocumentReferenceComponent>,
    #[serde(rename = "SignatoryParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatory_party: Option<SignatureArrayOfSignatoryPartyComponent>,
    #[serde(rename = "SignatureMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_method: Option<SignatureArrayOfSignatureMethodComponent>,
    #[serde(rename = "ValidationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_date: Option<SignatureArrayOfValidationDateComponent>,
    #[serde(rename = "ValidationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_time: Option<SignatureArrayOfValidationTimeComponent>,
    #[serde(rename = "ValidatorID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator_id: Option<SignatureArrayOfValidatorIDComponent>,
}

impl AsMut<Signature> for Signature {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Signature> for Signature {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.canonicalization_method {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Signature.canonicalization_method", e));
            }
        }
        if let Some(v) = &self.validation_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Signature.validation_date", e));
            }
        }
        if let Some(v) = &self.original_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Signature.original_document_reference", e));
            }
        }
        if let Some(v) = &self.digital_signature_attachment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Signature.digital_signature_attachment", e));
            }
        }
        if let Some(v) = &self.validator_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Signature.validator_id", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Signature.note", e));
            }
        }
        if let Some(v) = &self.signature_method {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Signature.signature_method", e));
            }
        }
        if let Some(v) = &self.validation_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Signature.validation_time", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("Signature.id", e));
        }
        if let Some(v) = &self.signatory_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Signature.signatory_party", e));
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

impl Signature {
    pub fn title() -> &'static str {
        "Signature. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a signature."
    }
    pub fn new(id: SignatureArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            note: None,
            validator_id: None,
            digital_signature_attachment: None,
            validation_time: None,
            canonicalization_method: None,
            signature_method: None,
            id,
            original_document_reference: None,
            signatory_party: None,
            validation_date: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureArrayOfCanonicalizationMethodComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CanonicalizationMethod>,
}

impl AsMut<SignatureArrayOfCanonicalizationMethodComponent> for SignatureArrayOfCanonicalizationMethodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SignatureArrayOfCanonicalizationMethodComponent> for SignatureArrayOfCanonicalizationMethodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SignatureArrayOfCanonicalizationMethodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SignatureArrayOfCanonicalizationMethodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SignatureArrayOfCanonicalizationMethodComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CanonicalizationMethod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CanonicalizationMethod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CanonicalizationMethod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CanonicalizationMethod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureArrayOfDigitalSignatureAttachmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DigitalSignatureAttachment>,
}

impl AsMut<SignatureArrayOfDigitalSignatureAttachmentComponent> for SignatureArrayOfDigitalSignatureAttachmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SignatureArrayOfDigitalSignatureAttachmentComponent> for SignatureArrayOfDigitalSignatureAttachmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SignatureArrayOfDigitalSignatureAttachmentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SignatureArrayOfDigitalSignatureAttachmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SignatureArrayOfDigitalSignatureAttachmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DigitalSignatureAttachment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DigitalSignatureAttachment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DigitalSignatureAttachment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DigitalSignatureAttachment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<SignatureArrayOfIDComponent> for SignatureArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SignatureArrayOfIDComponent> for SignatureArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SignatureArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SignatureArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SignatureArrayOfIDComponent {
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
pub struct SignatureArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<SignatureArrayOfNoteComponent> for SignatureArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SignatureArrayOfNoteComponent> for SignatureArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SignatureArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SignatureArrayOfNoteComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Note) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Note) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureArrayOfOriginalDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginalDocumentReference>,
}

impl AsMut<SignatureArrayOfOriginalDocumentReferenceComponent> for SignatureArrayOfOriginalDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SignatureArrayOfOriginalDocumentReferenceComponent> for SignatureArrayOfOriginalDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SignatureArrayOfOriginalDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SignatureArrayOfOriginalDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SignatureArrayOfOriginalDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OriginalDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OriginalDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OriginalDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OriginalDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureArrayOfSignatoryPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SignatoryParty>,
}

impl AsMut<SignatureArrayOfSignatoryPartyComponent> for SignatureArrayOfSignatoryPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SignatureArrayOfSignatoryPartyComponent> for SignatureArrayOfSignatoryPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SignatureArrayOfSignatoryPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SignatureArrayOfSignatoryPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SignatureArrayOfSignatoryPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SignatoryParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SignatoryParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SignatoryParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SignatoryParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureArrayOfSignatureMethodComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SignatureMethod>,
}

impl AsMut<SignatureArrayOfSignatureMethodComponent> for SignatureArrayOfSignatureMethodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SignatureArrayOfSignatureMethodComponent> for SignatureArrayOfSignatureMethodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SignatureArrayOfSignatureMethodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SignatureArrayOfSignatureMethodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SignatureArrayOfSignatureMethodComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SignatureMethod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SignatureMethod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SignatureMethod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SignatureMethod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureArrayOfValidationDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValidationDate>,
}

impl AsMut<SignatureArrayOfValidationDateComponent> for SignatureArrayOfValidationDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SignatureArrayOfValidationDateComponent> for SignatureArrayOfValidationDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SignatureArrayOfValidationDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SignatureArrayOfValidationDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SignatureArrayOfValidationDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValidationDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValidationDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValidationDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValidationDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureArrayOfValidationTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValidationTime>,
}

impl AsMut<SignatureArrayOfValidationTimeComponent> for SignatureArrayOfValidationTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SignatureArrayOfValidationTimeComponent> for SignatureArrayOfValidationTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SignatureArrayOfValidationTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SignatureArrayOfValidationTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SignatureArrayOfValidationTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValidationTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValidationTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValidationTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValidationTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureArrayOfValidatorIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValidatorID>,
}

impl AsMut<SignatureArrayOfValidatorIDComponent> for SignatureArrayOfValidatorIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SignatureArrayOfValidatorIDComponent> for SignatureArrayOfValidatorIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SignatureArrayOfValidatorIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SignatureArrayOfValidatorIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SignatureArrayOfValidatorIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValidatorID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValidatorID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValidatorID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValidatorID> {
        self.items.iter()
    }
}


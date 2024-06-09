use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocumentReference {
    #[serde(rename = "Attachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<DocumentReferenceArrayOfAttachmentComponent>,
    #[serde(rename = "CopyIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<DocumentReferenceArrayOfCopyIndicatorComponent>,
    #[serde(rename = "DocumentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentReferenceArrayOfDocumentDescriptionComponent>,
    #[serde(rename = "DocumentStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_status_code: Option<DocumentReferenceArrayOfDocumentStatusCodeComponent>,
    #[serde(rename = "DocumentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<DocumentReferenceArrayOfDocumentTypeComponent>,
    #[serde(rename = "DocumentTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type_code: Option<DocumentReferenceArrayOfDocumentTypeCodeComponent>,
    #[serde(rename = "ID")]
    pub id: DocumentReferenceArrayOfIDComponent,
    #[serde(rename = "IssueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<DocumentReferenceArrayOfIssueDateComponent>,
    #[serde(rename = "IssueTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<DocumentReferenceArrayOfIssueTimeComponent>,
    #[serde(rename = "IssuerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_party: Option<DocumentReferenceArrayOfIssuerPartyComponent>,
    #[serde(rename = "LanguageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<DocumentReferenceArrayOfLanguageIDComponent>,
    #[serde(rename = "LocaleCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_code: Option<DocumentReferenceArrayOfLocaleCodeComponent>,
    #[serde(rename = "ResultOfVerification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_of_verification: Option<DocumentReferenceArrayOfResultOfVerificationComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<DocumentReferenceArrayOfUUIDComponent>,
    #[serde(rename = "ValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<DocumentReferenceArrayOfValidityPeriodComponent>,
    #[serde(rename = "VersionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<DocumentReferenceArrayOfVersionIDComponent>,
    #[serde(rename = "XPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpath: Option<DocumentReferenceArrayOfXPathComponent>,
}

impl AsMut<DocumentReference> for DocumentReference {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReference> for DocumentReference {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.issue_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.issue_time", e));
            }
        }
        if let Some(v) = &self.document_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.document_description", e));
            }
        }
        if let Some(v) = &self.version_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.version_id", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.uuid", e));
            }
        }
        if let Some(v) = &self.validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.validity_period", e));
            }
        }
        if let Some(v) = &self.language_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.language_id", e));
            }
        }
        if let Some(v) = &self.result_of_verification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.result_of_verification", e));
            }
        }
        if let Some(v) = &self.document_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.document_status_code", e));
            }
        }
        if let Some(v) = &self.xpath {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.xpath", e));
            }
        }
        if let Some(v) = &self.locale_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.locale_code", e));
            }
        }
        if let Some(v) = &self.copy_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.copy_indicator", e));
            }
        }
        if let Some(v) = &self.document_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.document_type", e));
            }
        }
        if let Some(v) = &self.issue_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.issue_date", e));
            }
        }
        if let Some(v) = &self.document_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.document_type_code", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("DocumentReference.id", e));
        }
        if let Some(v) = &self.attachment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.attachment", e));
            }
        }
        if let Some(v) = &self.issuer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DocumentReference.issuer_party", e));
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

impl DocumentReference {
    pub fn title() -> &'static str {
        "Document Reference. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a reference to a document."
    }
    pub fn new(id: DocumentReferenceArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            issuer_party: None,
            validity_period: None,
            result_of_verification: None,
            locale_code: None,
            copy_indicator: None,
            version_id: None,
            xpath: None,
            attachment: None,
            uuid: None,
            issue_date: None,
            issue_time: None,
            id,
            language_id: None,
            document_type_code: None,
            document_description: None,
            document_status_code: None,
            document_type: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfAttachmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Attachment>,
}

impl AsMut<DocumentReferenceArrayOfAttachmentComponent> for DocumentReferenceArrayOfAttachmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfAttachmentComponent> for DocumentReferenceArrayOfAttachmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfAttachmentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfAttachmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfAttachmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Attachment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Attachment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Attachment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Attachment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfCopyIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CopyIndicator>,
}

impl AsMut<DocumentReferenceArrayOfCopyIndicatorComponent> for DocumentReferenceArrayOfCopyIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfCopyIndicatorComponent> for DocumentReferenceArrayOfCopyIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfCopyIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfCopyIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfCopyIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CopyIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CopyIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CopyIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CopyIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfDocumentDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DocumentDescription>,
}

impl AsMut<DocumentReferenceArrayOfDocumentDescriptionComponent> for DocumentReferenceArrayOfDocumentDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfDocumentDescriptionComponent> for DocumentReferenceArrayOfDocumentDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfDocumentDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfDocumentDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DocumentDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DocumentDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DocumentDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DocumentDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfDocumentStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DocumentStatusCode>,
}

impl AsMut<DocumentReferenceArrayOfDocumentStatusCodeComponent> for DocumentReferenceArrayOfDocumentStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfDocumentStatusCodeComponent> for DocumentReferenceArrayOfDocumentStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfDocumentStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfDocumentStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfDocumentStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DocumentStatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DocumentStatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DocumentStatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DocumentStatusCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfDocumentTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DocumentType>,
}

impl AsMut<DocumentReferenceArrayOfDocumentTypeComponent> for DocumentReferenceArrayOfDocumentTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfDocumentTypeComponent> for DocumentReferenceArrayOfDocumentTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfDocumentTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfDocumentTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfDocumentTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DocumentType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DocumentType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DocumentType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DocumentType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfDocumentTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DocumentTypeCode>,
}

impl AsMut<DocumentReferenceArrayOfDocumentTypeCodeComponent> for DocumentReferenceArrayOfDocumentTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfDocumentTypeCodeComponent> for DocumentReferenceArrayOfDocumentTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfDocumentTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfDocumentTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfDocumentTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DocumentTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DocumentTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DocumentTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DocumentTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<DocumentReferenceArrayOfIDComponent> for DocumentReferenceArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfIDComponent> for DocumentReferenceArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfIDComponent {
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
pub struct DocumentReferenceArrayOfIssueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueDate>,
}

impl AsMut<DocumentReferenceArrayOfIssueDateComponent> for DocumentReferenceArrayOfIssueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfIssueDateComponent> for DocumentReferenceArrayOfIssueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfIssueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfIssueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfIssueDateComponent {
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
pub struct DocumentReferenceArrayOfIssueTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueTime>,
}

impl AsMut<DocumentReferenceArrayOfIssueTimeComponent> for DocumentReferenceArrayOfIssueTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfIssueTimeComponent> for DocumentReferenceArrayOfIssueTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfIssueTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfIssueTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfIssueTimeComponent {
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
pub struct DocumentReferenceArrayOfIssuerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::IssuerParty>,
}

impl AsMut<DocumentReferenceArrayOfIssuerPartyComponent> for DocumentReferenceArrayOfIssuerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfIssuerPartyComponent> for DocumentReferenceArrayOfIssuerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfIssuerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfIssuerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfIssuerPartyComponent {
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
pub struct DocumentReferenceArrayOfLanguageIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LanguageID>,
}

impl AsMut<DocumentReferenceArrayOfLanguageIDComponent> for DocumentReferenceArrayOfLanguageIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfLanguageIDComponent> for DocumentReferenceArrayOfLanguageIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfLanguageIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfLanguageIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfLanguageIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LanguageID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LanguageID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LanguageID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LanguageID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfLocaleCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LocaleCode>,
}

impl AsMut<DocumentReferenceArrayOfLocaleCodeComponent> for DocumentReferenceArrayOfLocaleCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfLocaleCodeComponent> for DocumentReferenceArrayOfLocaleCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfLocaleCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfLocaleCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfLocaleCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LocaleCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LocaleCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LocaleCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LocaleCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfResultOfVerificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ResultOfVerification>,
}

impl AsMut<DocumentReferenceArrayOfResultOfVerificationComponent> for DocumentReferenceArrayOfResultOfVerificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfResultOfVerificationComponent> for DocumentReferenceArrayOfResultOfVerificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfResultOfVerificationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfResultOfVerificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfResultOfVerificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ResultOfVerification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ResultOfVerification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ResultOfVerification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ResultOfVerification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<DocumentReferenceArrayOfUUIDComponent> for DocumentReferenceArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfUUIDComponent> for DocumentReferenceArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfUUIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::UUID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::UUID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::UUID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::UUID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod>,
}

impl AsMut<DocumentReferenceArrayOfValidityPeriodComponent> for DocumentReferenceArrayOfValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfValidityPeriodComponent> for DocumentReferenceArrayOfValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfVersionIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::VersionID>,
}

impl AsMut<DocumentReferenceArrayOfVersionIDComponent> for DocumentReferenceArrayOfVersionIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfVersionIDComponent> for DocumentReferenceArrayOfVersionIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfVersionIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfVersionIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfVersionIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::VersionID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::VersionID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::VersionID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::VersionID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentReferenceArrayOfXPathComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::XPath>,
}

impl AsMut<DocumentReferenceArrayOfXPathComponent> for DocumentReferenceArrayOfXPathComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentReferenceArrayOfXPathComponent> for DocumentReferenceArrayOfXPathComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DocumentReferenceArrayOfXPathComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentReferenceArrayOfXPathComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::XPath) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::XPath) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::XPath> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::XPath> {
        self.items.iter()
    }
}


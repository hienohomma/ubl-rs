use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestForQuotationLine {
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<RequestForQuotationLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestForQuotationLineArrayOfIDComponent>,
    #[serde(rename = "LineItem")]
    pub line_item: RequestForQuotationLineArrayOfLineItemComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<RequestForQuotationLineArrayOfNoteComponent>,
    #[serde(rename = "OptionalLineItemIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_line_item_indicator: Option<RequestForQuotationLineArrayOfOptionalLineItemIndicatorComponent>,
    #[serde(rename = "PrivacyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_code: Option<RequestForQuotationLineArrayOfPrivacyCodeComponent>,
    #[serde(rename = "SecurityClassificationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_classification_code: Option<RequestForQuotationLineArrayOfSecurityClassificationCodeComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<RequestForQuotationLineArrayOfUUIDComponent>,
}

impl AsMut<RequestForQuotationLine> for RequestForQuotationLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForQuotationLine> for RequestForQuotationLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForQuotationLine.document_reference", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForQuotationLine.note", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForQuotationLine.uuid", e));
            }
        }
        if let Some(v) = &self.privacy_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForQuotationLine.privacy_code", e));
            }
        }
        if let Some(v) = &self.security_classification_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForQuotationLine.security_classification_code", e));
            }
        }
        if let Some(v) = &self.optional_line_item_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForQuotationLine.optional_line_item_indicator", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForQuotationLine.id", e));
            }
        }
        if let Err(e) = self.line_item.validate() {
            return Err(UblError::component("RequestForQuotationLine.line_item", e));
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

impl RequestForQuotationLine {
    pub fn title() -> &'static str {
        "Request For Quotation Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Request for Quotation."
    }
    pub fn new(line_item: RequestForQuotationLineArrayOfLineItemComponent) -> Component<Self> {
        Component(Self {
            document_reference: None,
            security_classification_code: None,
            uuid: None,
            optional_line_item_indicator: None,
            privacy_code: None,
            note: None,
            line_item,
            id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForQuotationLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<RequestForQuotationLineArrayOfDocumentReferenceComponent> for RequestForQuotationLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForQuotationLineArrayOfDocumentReferenceComponent> for RequestForQuotationLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForQuotationLineArrayOfDocumentReferenceComponent {
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
pub struct RequestForQuotationLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<RequestForQuotationLineArrayOfIDComponent> for RequestForQuotationLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForQuotationLineArrayOfIDComponent> for RequestForQuotationLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForQuotationLineArrayOfIDComponent {
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
pub struct RequestForQuotationLineArrayOfLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LineItem>,
}

impl AsMut<RequestForQuotationLineArrayOfLineItemComponent> for RequestForQuotationLineArrayOfLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForQuotationLineArrayOfLineItemComponent> for RequestForQuotationLineArrayOfLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfLineItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForQuotationLineArrayOfLineItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LineItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LineItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LineItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LineItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForQuotationLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<RequestForQuotationLineArrayOfNoteComponent> for RequestForQuotationLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForQuotationLineArrayOfNoteComponent> for RequestForQuotationLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForQuotationLineArrayOfNoteComponent {
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
pub struct RequestForQuotationLineArrayOfOptionalLineItemIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OptionalLineItemIndicator>,
}

impl AsMut<RequestForQuotationLineArrayOfOptionalLineItemIndicatorComponent> for RequestForQuotationLineArrayOfOptionalLineItemIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForQuotationLineArrayOfOptionalLineItemIndicatorComponent> for RequestForQuotationLineArrayOfOptionalLineItemIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfOptionalLineItemIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfOptionalLineItemIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForQuotationLineArrayOfOptionalLineItemIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OptionalLineItemIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OptionalLineItemIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OptionalLineItemIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OptionalLineItemIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForQuotationLineArrayOfPrivacyCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PrivacyCode>,
}

impl AsMut<RequestForQuotationLineArrayOfPrivacyCodeComponent> for RequestForQuotationLineArrayOfPrivacyCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForQuotationLineArrayOfPrivacyCodeComponent> for RequestForQuotationLineArrayOfPrivacyCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfPrivacyCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfPrivacyCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForQuotationLineArrayOfPrivacyCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PrivacyCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PrivacyCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PrivacyCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PrivacyCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForQuotationLineArrayOfSecurityClassificationCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SecurityClassificationCode>,
}

impl AsMut<RequestForQuotationLineArrayOfSecurityClassificationCodeComponent> for RequestForQuotationLineArrayOfSecurityClassificationCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForQuotationLineArrayOfSecurityClassificationCodeComponent> for RequestForQuotationLineArrayOfSecurityClassificationCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfSecurityClassificationCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfSecurityClassificationCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForQuotationLineArrayOfSecurityClassificationCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SecurityClassificationCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SecurityClassificationCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SecurityClassificationCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SecurityClassificationCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForQuotationLineArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<RequestForQuotationLineArrayOfUUIDComponent> for RequestForQuotationLineArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForQuotationLineArrayOfUUIDComponent> for RequestForQuotationLineArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForQuotationLineArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForQuotationLineArrayOfUUIDComponent {
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


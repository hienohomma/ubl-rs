use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuotationLine {
    #[serde(rename = "AlternativeLineItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_line_item: Option<QuotationLineArrayOfAlternativeLineItemComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<QuotationLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<QuotationLineArrayOfIDComponent>,
    #[serde(rename = "LineExtensionAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_extension_amount: Option<QuotationLineArrayOfLineExtensionAmountComponent>,
    #[serde(rename = "LineItem")]
    pub line_item: QuotationLineArrayOfLineItemComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<QuotationLineArrayOfNoteComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<QuotationLineArrayOfQuantityComponent>,
    #[serde(rename = "RequestForQuotationLineID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_for_quotation_line_id: Option<QuotationLineArrayOfRequestForQuotationLineIDComponent>,
    #[serde(rename = "RequestLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_line_reference: Option<QuotationLineArrayOfRequestLineReferenceComponent>,
    #[serde(rename = "SellerProposedSubstituteLineItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_proposed_substitute_line_item: Option<QuotationLineArrayOfSellerProposedSubstituteLineItemComponent>,
    #[serde(rename = "TotalTaxAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_amount: Option<QuotationLineArrayOfTotalTaxAmountComponent>,
}

impl AsMut<QuotationLine> for QuotationLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLine> for QuotationLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.request_for_quotation_line_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QuotationLine.request_for_quotation_line_id", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QuotationLine.quantity", e));
            }
        }
        if let Some(v) = &self.request_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QuotationLine.request_line_reference", e));
            }
        }
        if let Some(v) = &self.total_tax_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QuotationLine.total_tax_amount", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QuotationLine.id", e));
            }
        }
        if let Some(v) = &self.alternative_line_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QuotationLine.alternative_line_item", e));
            }
        }
        if let Some(v) = &self.line_extension_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QuotationLine.line_extension_amount", e));
            }
        }
        if let Some(v) = &self.seller_proposed_substitute_line_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QuotationLine.seller_proposed_substitute_line_item", e));
            }
        }
        if let Err(e) = self.line_item.validate() {
            return Err(UblError::component("QuotationLine.line_item", e));
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QuotationLine.note", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QuotationLine.document_reference", e));
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

impl QuotationLine {
    pub fn title() -> &'static str {
        "Quotation Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Quotation."
    }
    pub fn new(line_item: QuotationLineArrayOfLineItemComponent) -> Component<Self> {
        Component(Self {
            alternative_line_item: None,
            total_tax_amount: None,
            line_extension_amount: None,
            quantity: None,
            id: None,
            request_for_quotation_line_id: None,
            document_reference: None,
            request_line_reference: None,
            note: None,
            line_item,
            seller_proposed_substitute_line_item: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QuotationLineArrayOfAlternativeLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AlternativeLineItem>,
}

impl AsMut<QuotationLineArrayOfAlternativeLineItemComponent> for QuotationLineArrayOfAlternativeLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLineArrayOfAlternativeLineItemComponent> for QuotationLineArrayOfAlternativeLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfAlternativeLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QuotationLineArrayOfAlternativeLineItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AlternativeLineItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AlternativeLineItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AlternativeLineItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AlternativeLineItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QuotationLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<QuotationLineArrayOfDocumentReferenceComponent> for QuotationLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLineArrayOfDocumentReferenceComponent> for QuotationLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QuotationLineArrayOfDocumentReferenceComponent {
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
pub struct QuotationLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<QuotationLineArrayOfIDComponent> for QuotationLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLineArrayOfIDComponent> for QuotationLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QuotationLineArrayOfIDComponent {
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
pub struct QuotationLineArrayOfLineExtensionAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount>,
}

impl AsMut<QuotationLineArrayOfLineExtensionAmountComponent> for QuotationLineArrayOfLineExtensionAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLineArrayOfLineExtensionAmountComponent> for QuotationLineArrayOfLineExtensionAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfLineExtensionAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfLineExtensionAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QuotationLineArrayOfLineExtensionAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QuotationLineArrayOfLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LineItem>,
}

impl AsMut<QuotationLineArrayOfLineItemComponent> for QuotationLineArrayOfLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLineArrayOfLineItemComponent> for QuotationLineArrayOfLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfLineItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QuotationLineArrayOfLineItemComponent {
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
pub struct QuotationLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<QuotationLineArrayOfNoteComponent> for QuotationLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLineArrayOfNoteComponent> for QuotationLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QuotationLineArrayOfNoteComponent {
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
pub struct QuotationLineArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<QuotationLineArrayOfQuantityComponent> for QuotationLineArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLineArrayOfQuantityComponent> for QuotationLineArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QuotationLineArrayOfQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Quantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Quantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QuotationLineArrayOfRequestForQuotationLineIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RequestForQuotationLineID>,
}

impl AsMut<QuotationLineArrayOfRequestForQuotationLineIDComponent> for QuotationLineArrayOfRequestForQuotationLineIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLineArrayOfRequestForQuotationLineIDComponent> for QuotationLineArrayOfRequestForQuotationLineIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfRequestForQuotationLineIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfRequestForQuotationLineIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QuotationLineArrayOfRequestForQuotationLineIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RequestForQuotationLineID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RequestForQuotationLineID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RequestForQuotationLineID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RequestForQuotationLineID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QuotationLineArrayOfRequestLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequestLineReference>,
}

impl AsMut<QuotationLineArrayOfRequestLineReferenceComponent> for QuotationLineArrayOfRequestLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLineArrayOfRequestLineReferenceComponent> for QuotationLineArrayOfRequestLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfRequestLineReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfRequestLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QuotationLineArrayOfRequestLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequestLineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequestLineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequestLineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequestLineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QuotationLineArrayOfSellerProposedSubstituteLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellerProposedSubstituteLineItem>,
}

impl AsMut<QuotationLineArrayOfSellerProposedSubstituteLineItemComponent> for QuotationLineArrayOfSellerProposedSubstituteLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLineArrayOfSellerProposedSubstituteLineItemComponent> for QuotationLineArrayOfSellerProposedSubstituteLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfSellerProposedSubstituteLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QuotationLineArrayOfSellerProposedSubstituteLineItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SellerProposedSubstituteLineItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SellerProposedSubstituteLineItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SellerProposedSubstituteLineItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SellerProposedSubstituteLineItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QuotationLineArrayOfTotalTaxAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalTaxAmount>,
}

impl AsMut<QuotationLineArrayOfTotalTaxAmountComponent> for QuotationLineArrayOfTotalTaxAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QuotationLineArrayOfTotalTaxAmountComponent> for QuotationLineArrayOfTotalTaxAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfTotalTaxAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QuotationLineArrayOfTotalTaxAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QuotationLineArrayOfTotalTaxAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalTaxAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalTaxAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalTaxAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalTaxAmount> {
        self.items.iter()
    }
}


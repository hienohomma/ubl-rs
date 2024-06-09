use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BillingReference {
    #[serde(rename = "AdditionalDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document_reference: Option<BillingReferenceArrayOfAdditionalDocumentReferenceComponent>,
    #[serde(rename = "BillingReferenceLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reference_line: Option<BillingReferenceArrayOfBillingReferenceLineComponent>,
    #[serde(rename = "CreditNoteDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note_document_reference: Option<BillingReferenceArrayOfCreditNoteDocumentReferenceComponent>,
    #[serde(rename = "DebitNoteDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_note_document_reference: Option<BillingReferenceArrayOfDebitNoteDocumentReferenceComponent>,
    #[serde(rename = "InvoiceDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_document_reference: Option<BillingReferenceArrayOfInvoiceDocumentReferenceComponent>,
    #[serde(rename = "ReminderDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_document_reference: Option<BillingReferenceArrayOfReminderDocumentReferenceComponent>,
    #[serde(rename = "SelfBilledCreditNoteDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_billed_credit_note_document_reference: Option<BillingReferenceArrayOfSelfBilledCreditNoteDocumentReferenceComponent>,
    #[serde(rename = "SelfBilledInvoiceDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_billed_invoice_document_reference: Option<BillingReferenceArrayOfSelfBilledInvoiceDocumentReferenceComponent>,
}

impl AsMut<BillingReference> for BillingReference {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BillingReference> for BillingReference {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.self_billed_invoice_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BillingReference.self_billed_invoice_document_reference", e));
            }
        }
        if let Some(v) = &self.invoice_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BillingReference.invoice_document_reference", e));
            }
        }
        if let Some(v) = &self.credit_note_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BillingReference.credit_note_document_reference", e));
            }
        }
        if let Some(v) = &self.reminder_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BillingReference.reminder_document_reference", e));
            }
        }
        if let Some(v) = &self.self_billed_credit_note_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BillingReference.self_billed_credit_note_document_reference", e));
            }
        }
        if let Some(v) = &self.debit_note_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BillingReference.debit_note_document_reference", e));
            }
        }
        if let Some(v) = &self.additional_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BillingReference.additional_document_reference", e));
            }
        }
        if let Some(v) = &self.billing_reference_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BillingReference.billing_reference_line", e));
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

impl BillingReference {
    pub fn title() -> &'static str {
        "Billing Reference. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a reference to a billing document."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            additional_document_reference: None,
            credit_note_document_reference: None,
            debit_note_document_reference: None,
            self_billed_invoice_document_reference: None,
            reminder_document_reference: None,
            self_billed_credit_note_document_reference: None,
            billing_reference_line: None,
            invoice_document_reference: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BillingReferenceArrayOfAdditionalDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AdditionalDocumentReference>,
}

impl AsMut<BillingReferenceArrayOfAdditionalDocumentReferenceComponent> for BillingReferenceArrayOfAdditionalDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BillingReferenceArrayOfAdditionalDocumentReferenceComponent> for BillingReferenceArrayOfAdditionalDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfAdditionalDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfAdditionalDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BillingReferenceArrayOfAdditionalDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AdditionalDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AdditionalDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AdditionalDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AdditionalDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BillingReferenceArrayOfBillingReferenceLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BillingReferenceLine>,
}

impl AsMut<BillingReferenceArrayOfBillingReferenceLineComponent> for BillingReferenceArrayOfBillingReferenceLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BillingReferenceArrayOfBillingReferenceLineComponent> for BillingReferenceArrayOfBillingReferenceLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfBillingReferenceLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BillingReferenceArrayOfBillingReferenceLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BillingReferenceLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BillingReferenceLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BillingReferenceLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BillingReferenceLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BillingReferenceArrayOfCreditNoteDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CreditNoteDocumentReference>,
}

impl AsMut<BillingReferenceArrayOfCreditNoteDocumentReferenceComponent> for BillingReferenceArrayOfCreditNoteDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BillingReferenceArrayOfCreditNoteDocumentReferenceComponent> for BillingReferenceArrayOfCreditNoteDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfCreditNoteDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfCreditNoteDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BillingReferenceArrayOfCreditNoteDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CreditNoteDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CreditNoteDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CreditNoteDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CreditNoteDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BillingReferenceArrayOfDebitNoteDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DebitNoteDocumentReference>,
}

impl AsMut<BillingReferenceArrayOfDebitNoteDocumentReferenceComponent> for BillingReferenceArrayOfDebitNoteDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BillingReferenceArrayOfDebitNoteDocumentReferenceComponent> for BillingReferenceArrayOfDebitNoteDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfDebitNoteDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfDebitNoteDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BillingReferenceArrayOfDebitNoteDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DebitNoteDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DebitNoteDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DebitNoteDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DebitNoteDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BillingReferenceArrayOfInvoiceDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::InvoiceDocumentReference>,
}

impl AsMut<BillingReferenceArrayOfInvoiceDocumentReferenceComponent> for BillingReferenceArrayOfInvoiceDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BillingReferenceArrayOfInvoiceDocumentReferenceComponent> for BillingReferenceArrayOfInvoiceDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfInvoiceDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfInvoiceDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BillingReferenceArrayOfInvoiceDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::InvoiceDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::InvoiceDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::InvoiceDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::InvoiceDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BillingReferenceArrayOfReminderDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReminderDocumentReference>,
}

impl AsMut<BillingReferenceArrayOfReminderDocumentReferenceComponent> for BillingReferenceArrayOfReminderDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BillingReferenceArrayOfReminderDocumentReferenceComponent> for BillingReferenceArrayOfReminderDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfReminderDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfReminderDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BillingReferenceArrayOfReminderDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReminderDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReminderDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReminderDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReminderDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BillingReferenceArrayOfSelfBilledCreditNoteDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SelfBilledCreditNoteDocumentReference>,
}

impl AsMut<BillingReferenceArrayOfSelfBilledCreditNoteDocumentReferenceComponent> for BillingReferenceArrayOfSelfBilledCreditNoteDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BillingReferenceArrayOfSelfBilledCreditNoteDocumentReferenceComponent> for BillingReferenceArrayOfSelfBilledCreditNoteDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfSelfBilledCreditNoteDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfSelfBilledCreditNoteDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BillingReferenceArrayOfSelfBilledCreditNoteDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SelfBilledCreditNoteDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SelfBilledCreditNoteDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SelfBilledCreditNoteDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SelfBilledCreditNoteDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BillingReferenceArrayOfSelfBilledInvoiceDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SelfBilledInvoiceDocumentReference>,
}

impl AsMut<BillingReferenceArrayOfSelfBilledInvoiceDocumentReferenceComponent> for BillingReferenceArrayOfSelfBilledInvoiceDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BillingReferenceArrayOfSelfBilledInvoiceDocumentReferenceComponent> for BillingReferenceArrayOfSelfBilledInvoiceDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfSelfBilledInvoiceDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BillingReferenceArrayOfSelfBilledInvoiceDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BillingReferenceArrayOfSelfBilledInvoiceDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SelfBilledInvoiceDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SelfBilledInvoiceDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SelfBilledInvoiceDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SelfBilledInvoiceDocumentReference> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DebitNoteLine {
    #[serde(rename = "AccountingCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<DebitNoteLineArrayOfAccountingCostComponent>,
    #[serde(rename = "AccountingCostCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<DebitNoteLineArrayOfAccountingCostCodeComponent>,
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<DebitNoteLineArrayOfAllowanceChargeComponent>,
    #[serde(rename = "BillingReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reference: Option<DebitNoteLineArrayOfBillingReferenceComponent>,
    #[serde(rename = "DebitedQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debited_quantity: Option<DebitNoteLineArrayOfDebitedQuantityComponent>,
    #[serde(rename = "Delivery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<DebitNoteLineArrayOfDeliveryComponent>,
    #[serde(rename = "DespatchLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_line_reference: Option<DebitNoteLineArrayOfDespatchLineReferenceComponent>,
    #[serde(rename = "DiscrepancyResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discrepancy_response: Option<DebitNoteLineArrayOfDiscrepancyResponseComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<DebitNoteLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ID")]
    pub id: DebitNoteLineArrayOfIDComponent,
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<DebitNoteLineArrayOfItemComponent>,
    #[serde(rename = "LineExtensionAmount")]
    pub line_extension_amount: DebitNoteLineArrayOfLineExtensionAmountComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<DebitNoteLineArrayOfNoteComponent>,
    #[serde(rename = "PaymentPurposeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_purpose_code: Option<DebitNoteLineArrayOfPaymentPurposeCodeComponent>,
    #[serde(rename = "Price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<DebitNoteLineArrayOfPriceComponent>,
    #[serde(rename = "PricingReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_reference: Option<DebitNoteLineArrayOfPricingReferenceComponent>,
    #[serde(rename = "ReceiptLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_line_reference: Option<DebitNoteLineArrayOfReceiptLineReferenceComponent>,
    #[serde(rename = "SubDebitNoteLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_debit_note_line: Option<DebitNoteLineArrayOfSubDebitNoteLineComponent>,
    #[serde(rename = "TaxPointDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_point_date: Option<DebitNoteLineArrayOfTaxPointDateComponent>,
    #[serde(rename = "TaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<DebitNoteLineArrayOfTaxTotalComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<DebitNoteLineArrayOfUUIDComponent>,
}

impl AsMut<DebitNoteLine> for DebitNoteLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLine> for DebitNoteLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("DebitNoteLine.id", e));
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.document_reference", e));
            }
        }
        if let Err(e) = self.line_extension_amount.validate() {
            return Err(UblError::component("DebitNoteLine.line_extension_amount", e));
        }
        if let Some(v) = &self.accounting_cost_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.accounting_cost_code", e));
            }
        }
        if let Some(v) = &self.payment_purpose_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.payment_purpose_code", e));
            }
        }
        if let Some(v) = &self.receipt_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.receipt_line_reference", e));
            }
        }
        if let Some(v) = &self.tax_point_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.tax_point_date", e));
            }
        }
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.allowance_charge", e));
            }
        }
        if let Some(v) = &self.price {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.price", e));
            }
        }
        if let Some(v) = &self.item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.item", e));
            }
        }
        if let Some(v) = &self.despatch_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.despatch_line_reference", e));
            }
        }
        if let Some(v) = &self.discrepancy_response {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.discrepancy_response", e));
            }
        }
        if let Some(v) = &self.billing_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.billing_reference", e));
            }
        }
        if let Some(v) = &self.pricing_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.pricing_reference", e));
            }
        }
        if let Some(v) = &self.debited_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.debited_quantity", e));
            }
        }
        if let Some(v) = &self.sub_debit_note_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.sub_debit_note_line", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.uuid", e));
            }
        }
        if let Some(v) = &self.accounting_cost {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.accounting_cost", e));
            }
        }
        if let Some(v) = &self.tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.tax_total", e));
            }
        }
        if let Some(v) = &self.delivery {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.delivery", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DebitNoteLine.note", e));
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

impl DebitNoteLine {
    pub fn title() -> &'static str {
        "Debit Note Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Debit Note."
    }
    pub fn new(id: DebitNoteLineArrayOfIDComponent, line_extension_amount: DebitNoteLineArrayOfLineExtensionAmountComponent) -> Component<Self> {
        Component(Self {
            pricing_reference: None,
            discrepancy_response: None,
            allowance_charge: None,
            billing_reference: None,
            note: None,
            despatch_line_reference: None,
            tax_total: None,
            uuid: None,
            id,
            receipt_line_reference: None,
            accounting_cost_code: None,
            accounting_cost: None,
            tax_point_date: None,
            delivery: None,
            price: None,
            sub_debit_note_line: None,
            document_reference: None,
            payment_purpose_code: None,
            debited_quantity: None,
            line_extension_amount,
            item: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfAccountingCostComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCost>,
}

impl AsMut<DebitNoteLineArrayOfAccountingCostComponent> for DebitNoteLineArrayOfAccountingCostComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfAccountingCostComponent> for DebitNoteLineArrayOfAccountingCostComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfAccountingCostComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfAccountingCostComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfAccountingCostComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AccountingCost) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AccountingCost) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AccountingCost> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AccountingCost> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfAccountingCostCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCostCode>,
}

impl AsMut<DebitNoteLineArrayOfAccountingCostCodeComponent> for DebitNoteLineArrayOfAccountingCostCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfAccountingCostCodeComponent> for DebitNoteLineArrayOfAccountingCostCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfAccountingCostCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfAccountingCostCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfAccountingCostCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AccountingCostCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AccountingCostCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AccountingCostCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AccountingCostCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<DebitNoteLineArrayOfAllowanceChargeComponent> for DebitNoteLineArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfAllowanceChargeComponent> for DebitNoteLineArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfAllowanceChargeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfBillingReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BillingReference>,
}

impl AsMut<DebitNoteLineArrayOfBillingReferenceComponent> for DebitNoteLineArrayOfBillingReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfBillingReferenceComponent> for DebitNoteLineArrayOfBillingReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfBillingReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfBillingReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BillingReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BillingReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BillingReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BillingReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfDebitedQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DebitedQuantity>,
}

impl AsMut<DebitNoteLineArrayOfDebitedQuantityComponent> for DebitNoteLineArrayOfDebitedQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfDebitedQuantityComponent> for DebitNoteLineArrayOfDebitedQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfDebitedQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfDebitedQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfDebitedQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DebitedQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DebitedQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DebitedQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DebitedQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfDeliveryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Delivery>,
}

impl AsMut<DebitNoteLineArrayOfDeliveryComponent> for DebitNoteLineArrayOfDeliveryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfDeliveryComponent> for DebitNoteLineArrayOfDeliveryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfDeliveryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfDeliveryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Delivery) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Delivery) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Delivery> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Delivery> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfDespatchLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference>,
}

impl AsMut<DebitNoteLineArrayOfDespatchLineReferenceComponent> for DebitNoteLineArrayOfDespatchLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfDespatchLineReferenceComponent> for DebitNoteLineArrayOfDespatchLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfDespatchLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfDespatchLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfDiscrepancyResponseComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DiscrepancyResponse>,
}

impl AsMut<DebitNoteLineArrayOfDiscrepancyResponseComponent> for DebitNoteLineArrayOfDiscrepancyResponseComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfDiscrepancyResponseComponent> for DebitNoteLineArrayOfDiscrepancyResponseComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfDiscrepancyResponseComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfDiscrepancyResponseComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DiscrepancyResponse) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DiscrepancyResponse) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DiscrepancyResponse> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DiscrepancyResponse> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<DebitNoteLineArrayOfDocumentReferenceComponent> for DebitNoteLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfDocumentReferenceComponent> for DebitNoteLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfDocumentReferenceComponent {
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
pub struct DebitNoteLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<DebitNoteLineArrayOfIDComponent> for DebitNoteLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfIDComponent> for DebitNoteLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfIDComponent {
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
pub struct DebitNoteLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<DebitNoteLineArrayOfItemComponent> for DebitNoteLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfItemComponent> for DebitNoteLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Item) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Item) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfLineExtensionAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount>,
}

impl AsMut<DebitNoteLineArrayOfLineExtensionAmountComponent> for DebitNoteLineArrayOfLineExtensionAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfLineExtensionAmountComponent> for DebitNoteLineArrayOfLineExtensionAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfLineExtensionAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfLineExtensionAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfLineExtensionAmountComponent {
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
pub struct DebitNoteLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<DebitNoteLineArrayOfNoteComponent> for DebitNoteLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfNoteComponent> for DebitNoteLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfNoteComponent {
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
pub struct DebitNoteLineArrayOfPaymentPurposeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentPurposeCode>,
}

impl AsMut<DebitNoteLineArrayOfPaymentPurposeCodeComponent> for DebitNoteLineArrayOfPaymentPurposeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfPaymentPurposeCodeComponent> for DebitNoteLineArrayOfPaymentPurposeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfPaymentPurposeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfPaymentPurposeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfPaymentPurposeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaymentPurposeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaymentPurposeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaymentPurposeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaymentPurposeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfPriceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Price>,
}

impl AsMut<DebitNoteLineArrayOfPriceComponent> for DebitNoteLineArrayOfPriceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfPriceComponent> for DebitNoteLineArrayOfPriceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfPriceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfPriceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfPriceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Price) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Price) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Price> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Price> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfPricingReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PricingReference>,
}

impl AsMut<DebitNoteLineArrayOfPricingReferenceComponent> for DebitNoteLineArrayOfPricingReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfPricingReferenceComponent> for DebitNoteLineArrayOfPricingReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfPricingReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfPricingReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfPricingReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PricingReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PricingReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PricingReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PricingReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfReceiptLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReceiptLineReference>,
}

impl AsMut<DebitNoteLineArrayOfReceiptLineReferenceComponent> for DebitNoteLineArrayOfReceiptLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfReceiptLineReferenceComponent> for DebitNoteLineArrayOfReceiptLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfReceiptLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfReceiptLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReceiptLineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReceiptLineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReceiptLineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReceiptLineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfSubDebitNoteLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubDebitNoteLine>,
}

impl AsMut<DebitNoteLineArrayOfSubDebitNoteLineComponent> for DebitNoteLineArrayOfSubDebitNoteLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfSubDebitNoteLineComponent> for DebitNoteLineArrayOfSubDebitNoteLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfSubDebitNoteLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfSubDebitNoteLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubDebitNoteLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubDebitNoteLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubDebitNoteLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubDebitNoteLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfTaxPointDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxPointDate>,
}

impl AsMut<DebitNoteLineArrayOfTaxPointDateComponent> for DebitNoteLineArrayOfTaxPointDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfTaxPointDateComponent> for DebitNoteLineArrayOfTaxPointDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfTaxPointDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfTaxPointDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfTaxPointDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxPointDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxPointDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxPointDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxPointDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxTotal>,
}

impl AsMut<DebitNoteLineArrayOfTaxTotalComponent> for DebitNoteLineArrayOfTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfTaxTotalComponent> for DebitNoteLineArrayOfTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfTaxTotalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DebitNoteLineArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<DebitNoteLineArrayOfUUIDComponent> for DebitNoteLineArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DebitNoteLineArrayOfUUIDComponent> for DebitNoteLineArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DebitNoteLineArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DebitNoteLineArrayOfUUIDComponent {
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


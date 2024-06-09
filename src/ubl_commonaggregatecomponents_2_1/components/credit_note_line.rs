use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreditNoteLine {
    #[serde(rename = "AccountingCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<CreditNoteLineArrayOfAccountingCostComponent>,
    #[serde(rename = "AccountingCostCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<CreditNoteLineArrayOfAccountingCostCodeComponent>,
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<CreditNoteLineArrayOfAllowanceChargeComponent>,
    #[serde(rename = "BillingReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reference: Option<CreditNoteLineArrayOfBillingReferenceComponent>,
    #[serde(rename = "CreditedQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_quantity: Option<CreditNoteLineArrayOfCreditedQuantityComponent>,
    #[serde(rename = "Delivery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<CreditNoteLineArrayOfDeliveryComponent>,
    #[serde(rename = "DeliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<CreditNoteLineArrayOfDeliveryTermsComponent>,
    #[serde(rename = "DespatchLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_line_reference: Option<CreditNoteLineArrayOfDespatchLineReferenceComponent>,
    #[serde(rename = "DiscrepancyResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discrepancy_response: Option<CreditNoteLineArrayOfDiscrepancyResponseComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<CreditNoteLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "FreeOfChargeIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_of_charge_indicator: Option<CreditNoteLineArrayOfFreeOfChargeIndicatorComponent>,
    #[serde(rename = "ID")]
    pub id: CreditNoteLineArrayOfIDComponent,
    #[serde(rename = "InvoicePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_period: Option<CreditNoteLineArrayOfInvoicePeriodComponent>,
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<CreditNoteLineArrayOfItemComponent>,
    #[serde(rename = "ItemPriceExtension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_price_extension: Option<CreditNoteLineArrayOfItemPriceExtensionComponent>,
    #[serde(rename = "LineExtensionAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_extension_amount: Option<CreditNoteLineArrayOfLineExtensionAmountComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<CreditNoteLineArrayOfNoteComponent>,
    #[serde(rename = "OrderLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_line_reference: Option<CreditNoteLineArrayOfOrderLineReferenceComponent>,
    #[serde(rename = "OriginatorParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_party: Option<CreditNoteLineArrayOfOriginatorPartyComponent>,
    #[serde(rename = "PaymentPurposeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_purpose_code: Option<CreditNoteLineArrayOfPaymentPurposeCodeComponent>,
    #[serde(rename = "PaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<CreditNoteLineArrayOfPaymentTermsComponent>,
    #[serde(rename = "Price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<CreditNoteLineArrayOfPriceComponent>,
    #[serde(rename = "PricingReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_reference: Option<CreditNoteLineArrayOfPricingReferenceComponent>,
    #[serde(rename = "ReceiptLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_line_reference: Option<CreditNoteLineArrayOfReceiptLineReferenceComponent>,
    #[serde(rename = "SubCreditNoteLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_credit_note_line: Option<CreditNoteLineArrayOfSubCreditNoteLineComponent>,
    #[serde(rename = "TaxPointDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_point_date: Option<CreditNoteLineArrayOfTaxPointDateComponent>,
    #[serde(rename = "TaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<CreditNoteLineArrayOfTaxTotalComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<CreditNoteLineArrayOfUUIDComponent>,
}

impl AsMut<CreditNoteLine> for CreditNoteLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLine> for CreditNoteLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.tax_point_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.tax_point_date", e));
            }
        }
        if let Some(v) = &self.free_of_charge_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.free_of_charge_indicator", e));
            }
        }
        if let Some(v) = &self.delivery {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.delivery", e));
            }
        }
        if let Some(v) = &self.delivery_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.delivery_terms", e));
            }
        }
        if let Some(v) = &self.line_extension_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.line_extension_amount", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("CreditNoteLine.id", e));
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.document_reference", e));
            }
        }
        if let Some(v) = &self.price {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.price", e));
            }
        }
        if let Some(v) = &self.despatch_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.despatch_line_reference", e));
            }
        }
        if let Some(v) = &self.credited_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.credited_quantity", e));
            }
        }
        if let Some(v) = &self.accounting_cost_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.accounting_cost_code", e));
            }
        }
        if let Some(v) = &self.item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.item", e));
            }
        }
        if let Some(v) = &self.billing_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.billing_reference", e));
            }
        }
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.allowance_charge", e));
            }
        }
        if let Some(v) = &self.accounting_cost {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.accounting_cost", e));
            }
        }
        if let Some(v) = &self.order_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.order_line_reference", e));
            }
        }
        if let Some(v) = &self.payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.payment_terms", e));
            }
        }
        if let Some(v) = &self.item_price_extension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.item_price_extension", e));
            }
        }
        if let Some(v) = &self.originator_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.originator_party", e));
            }
        }
        if let Some(v) = &self.receipt_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.receipt_line_reference", e));
            }
        }
        if let Some(v) = &self.invoice_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.invoice_period", e));
            }
        }
        if let Some(v) = &self.sub_credit_note_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.sub_credit_note_line", e));
            }
        }
        if let Some(v) = &self.tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.tax_total", e));
            }
        }
        if let Some(v) = &self.discrepancy_response {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.discrepancy_response", e));
            }
        }
        if let Some(v) = &self.payment_purpose_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.payment_purpose_code", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.uuid", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.note", e));
            }
        }
        if let Some(v) = &self.pricing_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CreditNoteLine.pricing_reference", e));
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

impl CreditNoteLine {
    pub fn title() -> &'static str {
        "Credit Note Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Credit Note or Self Billed Credit Note."
    }
    pub fn new(id: CreditNoteLineArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            accounting_cost: None,
            payment_purpose_code: None,
            price: None,
            item_price_extension: None,
            pricing_reference: None,
            uuid: None,
            document_reference: None,
            note: None,
            tax_total: None,
            payment_terms: None,
            receipt_line_reference: None,
            sub_credit_note_line: None,
            originator_party: None,
            tax_point_date: None,
            free_of_charge_indicator: None,
            billing_reference: None,
            credited_quantity: None,
            delivery: None,
            accounting_cost_code: None,
            discrepancy_response: None,
            id,
            invoice_period: None,
            despatch_line_reference: None,
            item: None,
            line_extension_amount: None,
            order_line_reference: None,
            allowance_charge: None,
            delivery_terms: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreditNoteLineArrayOfAccountingCostComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCost>,
}

impl AsMut<CreditNoteLineArrayOfAccountingCostComponent> for CreditNoteLineArrayOfAccountingCostComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfAccountingCostComponent> for CreditNoteLineArrayOfAccountingCostComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfAccountingCostComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfAccountingCostComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfAccountingCostComponent {
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
pub struct CreditNoteLineArrayOfAccountingCostCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCostCode>,
}

impl AsMut<CreditNoteLineArrayOfAccountingCostCodeComponent> for CreditNoteLineArrayOfAccountingCostCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfAccountingCostCodeComponent> for CreditNoteLineArrayOfAccountingCostCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfAccountingCostCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfAccountingCostCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfAccountingCostCodeComponent {
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
pub struct CreditNoteLineArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<CreditNoteLineArrayOfAllowanceChargeComponent> for CreditNoteLineArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfAllowanceChargeComponent> for CreditNoteLineArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfAllowanceChargeComponent {
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
pub struct CreditNoteLineArrayOfBillingReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BillingReference>,
}

impl AsMut<CreditNoteLineArrayOfBillingReferenceComponent> for CreditNoteLineArrayOfBillingReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfBillingReferenceComponent> for CreditNoteLineArrayOfBillingReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfBillingReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfBillingReferenceComponent {
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
pub struct CreditNoteLineArrayOfCreditedQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CreditedQuantity>,
}

impl AsMut<CreditNoteLineArrayOfCreditedQuantityComponent> for CreditNoteLineArrayOfCreditedQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfCreditedQuantityComponent> for CreditNoteLineArrayOfCreditedQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfCreditedQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfCreditedQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfCreditedQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CreditedQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CreditedQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CreditedQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CreditedQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreditNoteLineArrayOfDeliveryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Delivery>,
}

impl AsMut<CreditNoteLineArrayOfDeliveryComponent> for CreditNoteLineArrayOfDeliveryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfDeliveryComponent> for CreditNoteLineArrayOfDeliveryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfDeliveryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfDeliveryComponent {
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
pub struct CreditNoteLineArrayOfDeliveryTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms>,
}

impl AsMut<CreditNoteLineArrayOfDeliveryTermsComponent> for CreditNoteLineArrayOfDeliveryTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfDeliveryTermsComponent> for CreditNoteLineArrayOfDeliveryTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfDeliveryTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfDeliveryTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreditNoteLineArrayOfDespatchLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference>,
}

impl AsMut<CreditNoteLineArrayOfDespatchLineReferenceComponent> for CreditNoteLineArrayOfDespatchLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfDespatchLineReferenceComponent> for CreditNoteLineArrayOfDespatchLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfDespatchLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfDespatchLineReferenceComponent {
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
pub struct CreditNoteLineArrayOfDiscrepancyResponseComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DiscrepancyResponse>,
}

impl AsMut<CreditNoteLineArrayOfDiscrepancyResponseComponent> for CreditNoteLineArrayOfDiscrepancyResponseComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfDiscrepancyResponseComponent> for CreditNoteLineArrayOfDiscrepancyResponseComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfDiscrepancyResponseComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfDiscrepancyResponseComponent {
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
pub struct CreditNoteLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<CreditNoteLineArrayOfDocumentReferenceComponent> for CreditNoteLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfDocumentReferenceComponent> for CreditNoteLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfDocumentReferenceComponent {
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
pub struct CreditNoteLineArrayOfFreeOfChargeIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FreeOfChargeIndicator>,
}

impl AsMut<CreditNoteLineArrayOfFreeOfChargeIndicatorComponent> for CreditNoteLineArrayOfFreeOfChargeIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfFreeOfChargeIndicatorComponent> for CreditNoteLineArrayOfFreeOfChargeIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfFreeOfChargeIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfFreeOfChargeIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfFreeOfChargeIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FreeOfChargeIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FreeOfChargeIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FreeOfChargeIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FreeOfChargeIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreditNoteLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<CreditNoteLineArrayOfIDComponent> for CreditNoteLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfIDComponent> for CreditNoteLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfIDComponent {
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
pub struct CreditNoteLineArrayOfInvoicePeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::InvoicePeriod>,
}

impl AsMut<CreditNoteLineArrayOfInvoicePeriodComponent> for CreditNoteLineArrayOfInvoicePeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfInvoicePeriodComponent> for CreditNoteLineArrayOfInvoicePeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfInvoicePeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfInvoicePeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::InvoicePeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::InvoicePeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::InvoicePeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::InvoicePeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreditNoteLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<CreditNoteLineArrayOfItemComponent> for CreditNoteLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfItemComponent> for CreditNoteLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfItemComponent {
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
pub struct CreditNoteLineArrayOfItemPriceExtensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ItemPriceExtension>,
}

impl AsMut<CreditNoteLineArrayOfItemPriceExtensionComponent> for CreditNoteLineArrayOfItemPriceExtensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfItemPriceExtensionComponent> for CreditNoteLineArrayOfItemPriceExtensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfItemPriceExtensionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfItemPriceExtensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfItemPriceExtensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ItemPriceExtension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ItemPriceExtension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ItemPriceExtension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ItemPriceExtension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreditNoteLineArrayOfLineExtensionAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount>,
}

impl AsMut<CreditNoteLineArrayOfLineExtensionAmountComponent> for CreditNoteLineArrayOfLineExtensionAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfLineExtensionAmountComponent> for CreditNoteLineArrayOfLineExtensionAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfLineExtensionAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfLineExtensionAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfLineExtensionAmountComponent {
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
pub struct CreditNoteLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<CreditNoteLineArrayOfNoteComponent> for CreditNoteLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfNoteComponent> for CreditNoteLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfNoteComponent {
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
pub struct CreditNoteLineArrayOfOrderLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OrderLineReference>,
}

impl AsMut<CreditNoteLineArrayOfOrderLineReferenceComponent> for CreditNoteLineArrayOfOrderLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfOrderLineReferenceComponent> for CreditNoteLineArrayOfOrderLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfOrderLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfOrderLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OrderLineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OrderLineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OrderLineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OrderLineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreditNoteLineArrayOfOriginatorPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginatorParty>,
}

impl AsMut<CreditNoteLineArrayOfOriginatorPartyComponent> for CreditNoteLineArrayOfOriginatorPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfOriginatorPartyComponent> for CreditNoteLineArrayOfOriginatorPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfOriginatorPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfOriginatorPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfOriginatorPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OriginatorParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OriginatorParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OriginatorParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OriginatorParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreditNoteLineArrayOfPaymentPurposeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentPurposeCode>,
}

impl AsMut<CreditNoteLineArrayOfPaymentPurposeCodeComponent> for CreditNoteLineArrayOfPaymentPurposeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfPaymentPurposeCodeComponent> for CreditNoteLineArrayOfPaymentPurposeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfPaymentPurposeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfPaymentPurposeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfPaymentPurposeCodeComponent {
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
pub struct CreditNoteLineArrayOfPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms>,
}

impl AsMut<CreditNoteLineArrayOfPaymentTermsComponent> for CreditNoteLineArrayOfPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfPaymentTermsComponent> for CreditNoteLineArrayOfPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfPaymentTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PaymentTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PaymentTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreditNoteLineArrayOfPriceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Price>,
}

impl AsMut<CreditNoteLineArrayOfPriceComponent> for CreditNoteLineArrayOfPriceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfPriceComponent> for CreditNoteLineArrayOfPriceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfPriceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfPriceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfPriceComponent {
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
pub struct CreditNoteLineArrayOfPricingReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PricingReference>,
}

impl AsMut<CreditNoteLineArrayOfPricingReferenceComponent> for CreditNoteLineArrayOfPricingReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfPricingReferenceComponent> for CreditNoteLineArrayOfPricingReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfPricingReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfPricingReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfPricingReferenceComponent {
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
pub struct CreditNoteLineArrayOfReceiptLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReceiptLineReference>,
}

impl AsMut<CreditNoteLineArrayOfReceiptLineReferenceComponent> for CreditNoteLineArrayOfReceiptLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfReceiptLineReferenceComponent> for CreditNoteLineArrayOfReceiptLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfReceiptLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfReceiptLineReferenceComponent {
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
pub struct CreditNoteLineArrayOfSubCreditNoteLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubCreditNoteLine>,
}

impl AsMut<CreditNoteLineArrayOfSubCreditNoteLineComponent> for CreditNoteLineArrayOfSubCreditNoteLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfSubCreditNoteLineComponent> for CreditNoteLineArrayOfSubCreditNoteLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfSubCreditNoteLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfSubCreditNoteLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubCreditNoteLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubCreditNoteLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubCreditNoteLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubCreditNoteLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreditNoteLineArrayOfTaxPointDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxPointDate>,
}

impl AsMut<CreditNoteLineArrayOfTaxPointDateComponent> for CreditNoteLineArrayOfTaxPointDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfTaxPointDateComponent> for CreditNoteLineArrayOfTaxPointDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfTaxPointDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfTaxPointDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfTaxPointDateComponent {
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
pub struct CreditNoteLineArrayOfTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxTotal>,
}

impl AsMut<CreditNoteLineArrayOfTaxTotalComponent> for CreditNoteLineArrayOfTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfTaxTotalComponent> for CreditNoteLineArrayOfTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfTaxTotalComponent {
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
pub struct CreditNoteLineArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<CreditNoteLineArrayOfUUIDComponent> for CreditNoteLineArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditNoteLineArrayOfUUIDComponent> for CreditNoteLineArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditNoteLineArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditNoteLineArrayOfUUIDComponent {
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


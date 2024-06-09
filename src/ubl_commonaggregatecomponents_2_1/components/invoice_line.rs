use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InvoiceLine {
    #[serde(rename = "AccountingCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<InvoiceLineArrayOfAccountingCostComponent>,
    #[serde(rename = "AccountingCostCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<InvoiceLineArrayOfAccountingCostCodeComponent>,
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<InvoiceLineArrayOfAllowanceChargeComponent>,
    #[serde(rename = "BillingReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reference: Option<InvoiceLineArrayOfBillingReferenceComponent>,
    #[serde(rename = "Delivery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<InvoiceLineArrayOfDeliveryComponent>,
    #[serde(rename = "DeliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<InvoiceLineArrayOfDeliveryTermsComponent>,
    #[serde(rename = "DespatchLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_line_reference: Option<InvoiceLineArrayOfDespatchLineReferenceComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<InvoiceLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "FreeOfChargeIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_of_charge_indicator: Option<InvoiceLineArrayOfFreeOfChargeIndicatorComponent>,
    #[serde(rename = "ID")]
    pub id: InvoiceLineArrayOfIDComponent,
    #[serde(rename = "InvoicePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_period: Option<InvoiceLineArrayOfInvoicePeriodComponent>,
    #[serde(rename = "InvoicedQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiced_quantity: Option<InvoiceLineArrayOfInvoicedQuantityComponent>,
    #[serde(rename = "Item")]
    pub item: InvoiceLineArrayOfItemComponent,
    #[serde(rename = "ItemPriceExtension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_price_extension: Option<InvoiceLineArrayOfItemPriceExtensionComponent>,
    #[serde(rename = "LineExtensionAmount")]
    pub line_extension_amount: InvoiceLineArrayOfLineExtensionAmountComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<InvoiceLineArrayOfNoteComponent>,
    #[serde(rename = "OrderLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_line_reference: Option<InvoiceLineArrayOfOrderLineReferenceComponent>,
    #[serde(rename = "OriginatorParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_party: Option<InvoiceLineArrayOfOriginatorPartyComponent>,
    #[serde(rename = "PaymentPurposeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_purpose_code: Option<InvoiceLineArrayOfPaymentPurposeCodeComponent>,
    #[serde(rename = "PaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<InvoiceLineArrayOfPaymentTermsComponent>,
    #[serde(rename = "Price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<InvoiceLineArrayOfPriceComponent>,
    #[serde(rename = "PricingReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_reference: Option<InvoiceLineArrayOfPricingReferenceComponent>,
    #[serde(rename = "ReceiptLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_line_reference: Option<InvoiceLineArrayOfReceiptLineReferenceComponent>,
    #[serde(rename = "SubInvoiceLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_invoice_line: Option<InvoiceLineArrayOfSubInvoiceLineComponent>,
    #[serde(rename = "TaxPointDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_point_date: Option<InvoiceLineArrayOfTaxPointDateComponent>,
    #[serde(rename = "TaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<InvoiceLineArrayOfTaxTotalComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<InvoiceLineArrayOfUUIDComponent>,
    #[serde(rename = "WithholdingTaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withholding_tax_total: Option<InvoiceLineArrayOfWithholdingTaxTotalComponent>,
}

impl AsMut<InvoiceLine> for InvoiceLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLine> for InvoiceLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.delivery_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.delivery_terms", e));
            }
        }
        if let Some(v) = &self.accounting_cost_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.accounting_cost_code", e));
            }
        }
        if let Some(v) = &self.order_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.order_line_reference", e));
            }
        }
        if let Some(v) = &self.invoiced_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.invoiced_quantity", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("InvoiceLine.id", e));
        }
        if let Some(v) = &self.invoice_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.invoice_period", e));
            }
        }
        if let Some(v) = &self.payment_purpose_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.payment_purpose_code", e));
            }
        }
        if let Some(v) = &self.payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.payment_terms", e));
            }
        }
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("InvoiceLine.item", e));
        }
        if let Some(v) = &self.price {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.price", e));
            }
        }
        if let Some(v) = &self.item_price_extension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.item_price_extension", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.note", e));
            }
        }
        if let Some(v) = &self.billing_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.billing_reference", e));
            }
        }
        if let Some(v) = &self.pricing_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.pricing_reference", e));
            }
        }
        if let Some(v) = &self.delivery {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.delivery", e));
            }
        }
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.allowance_charge", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.document_reference", e));
            }
        }
        if let Some(v) = &self.tax_point_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.tax_point_date", e));
            }
        }
        if let Some(v) = &self.sub_invoice_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.sub_invoice_line", e));
            }
        }
        if let Some(v) = &self.accounting_cost {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.accounting_cost", e));
            }
        }
        if let Some(v) = &self.despatch_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.despatch_line_reference", e));
            }
        }
        if let Err(e) = self.line_extension_amount.validate() {
            return Err(UblError::component("InvoiceLine.line_extension_amount", e));
        }
        if let Some(v) = &self.receipt_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.receipt_line_reference", e));
            }
        }
        if let Some(v) = &self.tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.tax_total", e));
            }
        }
        if let Some(v) = &self.withholding_tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.withholding_tax_total", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.uuid", e));
            }
        }
        if let Some(v) = &self.free_of_charge_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.free_of_charge_indicator", e));
            }
        }
        if let Some(v) = &self.originator_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InvoiceLine.originator_party", e));
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

impl InvoiceLine {
    pub fn title() -> &'static str {
        "Invoice Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in an Invoice."
    }
    pub fn new(id: InvoiceLineArrayOfIDComponent, item: InvoiceLineArrayOfItemComponent, line_extension_amount: InvoiceLineArrayOfLineExtensionAmountComponent) -> Component<Self> {
        Component(Self {
            billing_reference: None,
            delivery_terms: None,
            item_price_extension: None,
            line_extension_amount,
            pricing_reference: None,
            receipt_line_reference: None,
            sub_invoice_line: None,
            tax_total: None,
            uuid: None,
            tax_point_date: None,
            originator_party: None,
            invoice_period: None,
            payment_purpose_code: None,
            price: None,
            withholding_tax_total: None,
            allowance_charge: None,
            delivery: None,
            accounting_cost_code: None,
            despatch_line_reference: None,
            item,
            note: None,
            order_line_reference: None,
            payment_terms: None,
            invoiced_quantity: None,
            id,
            accounting_cost: None,
            free_of_charge_indicator: None,
            document_reference: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InvoiceLineArrayOfAccountingCostComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCost>,
}

impl AsMut<InvoiceLineArrayOfAccountingCostComponent> for InvoiceLineArrayOfAccountingCostComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfAccountingCostComponent> for InvoiceLineArrayOfAccountingCostComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfAccountingCostComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfAccountingCostComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfAccountingCostComponent {
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
pub struct InvoiceLineArrayOfAccountingCostCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCostCode>,
}

impl AsMut<InvoiceLineArrayOfAccountingCostCodeComponent> for InvoiceLineArrayOfAccountingCostCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfAccountingCostCodeComponent> for InvoiceLineArrayOfAccountingCostCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfAccountingCostCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfAccountingCostCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfAccountingCostCodeComponent {
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
pub struct InvoiceLineArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<InvoiceLineArrayOfAllowanceChargeComponent> for InvoiceLineArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfAllowanceChargeComponent> for InvoiceLineArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfAllowanceChargeComponent {
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
pub struct InvoiceLineArrayOfBillingReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BillingReference>,
}

impl AsMut<InvoiceLineArrayOfBillingReferenceComponent> for InvoiceLineArrayOfBillingReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfBillingReferenceComponent> for InvoiceLineArrayOfBillingReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfBillingReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfBillingReferenceComponent {
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
pub struct InvoiceLineArrayOfDeliveryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Delivery>,
}

impl AsMut<InvoiceLineArrayOfDeliveryComponent> for InvoiceLineArrayOfDeliveryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfDeliveryComponent> for InvoiceLineArrayOfDeliveryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfDeliveryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfDeliveryComponent {
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
pub struct InvoiceLineArrayOfDeliveryTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms>,
}

impl AsMut<InvoiceLineArrayOfDeliveryTermsComponent> for InvoiceLineArrayOfDeliveryTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfDeliveryTermsComponent> for InvoiceLineArrayOfDeliveryTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfDeliveryTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfDeliveryTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfDeliveryTermsComponent {
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
pub struct InvoiceLineArrayOfDespatchLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference>,
}

impl AsMut<InvoiceLineArrayOfDespatchLineReferenceComponent> for InvoiceLineArrayOfDespatchLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfDespatchLineReferenceComponent> for InvoiceLineArrayOfDespatchLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfDespatchLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfDespatchLineReferenceComponent {
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
pub struct InvoiceLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<InvoiceLineArrayOfDocumentReferenceComponent> for InvoiceLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfDocumentReferenceComponent> for InvoiceLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfDocumentReferenceComponent {
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
pub struct InvoiceLineArrayOfFreeOfChargeIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FreeOfChargeIndicator>,
}

impl AsMut<InvoiceLineArrayOfFreeOfChargeIndicatorComponent> for InvoiceLineArrayOfFreeOfChargeIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfFreeOfChargeIndicatorComponent> for InvoiceLineArrayOfFreeOfChargeIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfFreeOfChargeIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfFreeOfChargeIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfFreeOfChargeIndicatorComponent {
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
pub struct InvoiceLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<InvoiceLineArrayOfIDComponent> for InvoiceLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfIDComponent> for InvoiceLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfIDComponent {
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
pub struct InvoiceLineArrayOfInvoicePeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::InvoicePeriod>,
}

impl AsMut<InvoiceLineArrayOfInvoicePeriodComponent> for InvoiceLineArrayOfInvoicePeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfInvoicePeriodComponent> for InvoiceLineArrayOfInvoicePeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfInvoicePeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfInvoicePeriodComponent {
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
pub struct InvoiceLineArrayOfInvoicedQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InvoicedQuantity>,
}

impl AsMut<InvoiceLineArrayOfInvoicedQuantityComponent> for InvoiceLineArrayOfInvoicedQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfInvoicedQuantityComponent> for InvoiceLineArrayOfInvoicedQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfInvoicedQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfInvoicedQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfInvoicedQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InvoicedQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InvoicedQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InvoicedQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InvoicedQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InvoiceLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<InvoiceLineArrayOfItemComponent> for InvoiceLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfItemComponent> for InvoiceLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfItemComponent {
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
pub struct InvoiceLineArrayOfItemPriceExtensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ItemPriceExtension>,
}

impl AsMut<InvoiceLineArrayOfItemPriceExtensionComponent> for InvoiceLineArrayOfItemPriceExtensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfItemPriceExtensionComponent> for InvoiceLineArrayOfItemPriceExtensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfItemPriceExtensionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfItemPriceExtensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfItemPriceExtensionComponent {
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
pub struct InvoiceLineArrayOfLineExtensionAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount>,
}

impl AsMut<InvoiceLineArrayOfLineExtensionAmountComponent> for InvoiceLineArrayOfLineExtensionAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfLineExtensionAmountComponent> for InvoiceLineArrayOfLineExtensionAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfLineExtensionAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfLineExtensionAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfLineExtensionAmountComponent {
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
pub struct InvoiceLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<InvoiceLineArrayOfNoteComponent> for InvoiceLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfNoteComponent> for InvoiceLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfNoteComponent {
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
pub struct InvoiceLineArrayOfOrderLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OrderLineReference>,
}

impl AsMut<InvoiceLineArrayOfOrderLineReferenceComponent> for InvoiceLineArrayOfOrderLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfOrderLineReferenceComponent> for InvoiceLineArrayOfOrderLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfOrderLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfOrderLineReferenceComponent {
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
pub struct InvoiceLineArrayOfOriginatorPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginatorParty>,
}

impl AsMut<InvoiceLineArrayOfOriginatorPartyComponent> for InvoiceLineArrayOfOriginatorPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfOriginatorPartyComponent> for InvoiceLineArrayOfOriginatorPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfOriginatorPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfOriginatorPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfOriginatorPartyComponent {
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
pub struct InvoiceLineArrayOfPaymentPurposeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentPurposeCode>,
}

impl AsMut<InvoiceLineArrayOfPaymentPurposeCodeComponent> for InvoiceLineArrayOfPaymentPurposeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfPaymentPurposeCodeComponent> for InvoiceLineArrayOfPaymentPurposeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfPaymentPurposeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfPaymentPurposeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfPaymentPurposeCodeComponent {
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
pub struct InvoiceLineArrayOfPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms>,
}

impl AsMut<InvoiceLineArrayOfPaymentTermsComponent> for InvoiceLineArrayOfPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfPaymentTermsComponent> for InvoiceLineArrayOfPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfPaymentTermsComponent {
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
pub struct InvoiceLineArrayOfPriceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Price>,
}

impl AsMut<InvoiceLineArrayOfPriceComponent> for InvoiceLineArrayOfPriceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfPriceComponent> for InvoiceLineArrayOfPriceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfPriceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfPriceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfPriceComponent {
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
pub struct InvoiceLineArrayOfPricingReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PricingReference>,
}

impl AsMut<InvoiceLineArrayOfPricingReferenceComponent> for InvoiceLineArrayOfPricingReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfPricingReferenceComponent> for InvoiceLineArrayOfPricingReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfPricingReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfPricingReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfPricingReferenceComponent {
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
pub struct InvoiceLineArrayOfReceiptLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReceiptLineReference>,
}

impl AsMut<InvoiceLineArrayOfReceiptLineReferenceComponent> for InvoiceLineArrayOfReceiptLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfReceiptLineReferenceComponent> for InvoiceLineArrayOfReceiptLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfReceiptLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfReceiptLineReferenceComponent {
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
pub struct InvoiceLineArrayOfSubInvoiceLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubInvoiceLine>,
}

impl AsMut<InvoiceLineArrayOfSubInvoiceLineComponent> for InvoiceLineArrayOfSubInvoiceLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfSubInvoiceLineComponent> for InvoiceLineArrayOfSubInvoiceLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfSubInvoiceLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfSubInvoiceLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubInvoiceLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubInvoiceLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubInvoiceLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubInvoiceLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InvoiceLineArrayOfTaxPointDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxPointDate>,
}

impl AsMut<InvoiceLineArrayOfTaxPointDateComponent> for InvoiceLineArrayOfTaxPointDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfTaxPointDateComponent> for InvoiceLineArrayOfTaxPointDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfTaxPointDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfTaxPointDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfTaxPointDateComponent {
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
pub struct InvoiceLineArrayOfTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxTotal>,
}

impl AsMut<InvoiceLineArrayOfTaxTotalComponent> for InvoiceLineArrayOfTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfTaxTotalComponent> for InvoiceLineArrayOfTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfTaxTotalComponent {
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
pub struct InvoiceLineArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<InvoiceLineArrayOfUUIDComponent> for InvoiceLineArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfUUIDComponent> for InvoiceLineArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfUUIDComponent {
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
pub struct InvoiceLineArrayOfWithholdingTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WithholdingTaxTotal>,
}

impl AsMut<InvoiceLineArrayOfWithholdingTaxTotalComponent> for InvoiceLineArrayOfWithholdingTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InvoiceLineArrayOfWithholdingTaxTotalComponent> for InvoiceLineArrayOfWithholdingTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InvoiceLineArrayOfWithholdingTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InvoiceLineArrayOfWithholdingTaxTotalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WithholdingTaxTotal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WithholdingTaxTotal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WithholdingTaxTotal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WithholdingTaxTotal> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemittanceAdviceLine {
    #[serde(rename = "AccountingCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_customer_party: Option<RemittanceAdviceLineArrayOfAccountingCustomerPartyComponent>,
    #[serde(rename = "AccountingSupplierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_supplier_party: Option<RemittanceAdviceLineArrayOfAccountingSupplierPartyComponent>,
    #[serde(rename = "BalanceAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_amount: Option<RemittanceAdviceLineArrayOfBalanceAmountComponent>,
    #[serde(rename = "BillingReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reference: Option<RemittanceAdviceLineArrayOfBillingReferenceComponent>,
    #[serde(rename = "BuyerCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<RemittanceAdviceLineArrayOfBuyerCustomerPartyComponent>,
    #[serde(rename = "CreditLineAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_line_amount: Option<RemittanceAdviceLineArrayOfCreditLineAmountComponent>,
    #[serde(rename = "DebitLineAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_line_amount: Option<RemittanceAdviceLineArrayOfDebitLineAmountComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<RemittanceAdviceLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ExchangeRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<RemittanceAdviceLineArrayOfExchangeRateComponent>,
    #[serde(rename = "ID")]
    pub id: RemittanceAdviceLineArrayOfIDComponent,
    #[serde(rename = "InvoicePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_period: Option<RemittanceAdviceLineArrayOfInvoicePeriodComponent>,
    #[serde(rename = "InvoicingPartyReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoicing_party_reference: Option<RemittanceAdviceLineArrayOfInvoicingPartyReferenceComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<RemittanceAdviceLineArrayOfNoteComponent>,
    #[serde(rename = "OriginatorCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<RemittanceAdviceLineArrayOfOriginatorCustomerPartyComponent>,
    #[serde(rename = "PayeeParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_party: Option<RemittanceAdviceLineArrayOfPayeePartyComponent>,
    #[serde(rename = "PaymentPurposeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_purpose_code: Option<RemittanceAdviceLineArrayOfPaymentPurposeCodeComponent>,
    #[serde(rename = "SellerSupplierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<RemittanceAdviceLineArrayOfSellerSupplierPartyComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<RemittanceAdviceLineArrayOfUUIDComponent>,
}

impl AsMut<RemittanceAdviceLine> for RemittanceAdviceLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLine> for RemittanceAdviceLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("RemittanceAdviceLine.id", e));
        }
        if let Some(v) = &self.payment_purpose_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.payment_purpose_code", e));
            }
        }
        if let Some(v) = &self.originator_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.originator_customer_party", e));
            }
        }
        if let Some(v) = &self.buyer_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.buyer_customer_party", e));
            }
        }
        if let Some(v) = &self.accounting_supplier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.accounting_supplier_party", e));
            }
        }
        if let Some(v) = &self.payee_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.payee_party", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.uuid", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.note", e));
            }
        }
        if let Some(v) = &self.invoicing_party_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.invoicing_party_reference", e));
            }
        }
        if let Some(v) = &self.debit_line_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.debit_line_amount", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.document_reference", e));
            }
        }
        if let Some(v) = &self.invoice_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.invoice_period", e));
            }
        }
        if let Some(v) = &self.credit_line_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.credit_line_amount", e));
            }
        }
        if let Some(v) = &self.accounting_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.accounting_customer_party", e));
            }
        }
        if let Some(v) = &self.billing_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.billing_reference", e));
            }
        }
        if let Some(v) = &self.balance_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.balance_amount", e));
            }
        }
        if let Some(v) = &self.exchange_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.exchange_rate", e));
            }
        }
        if let Some(v) = &self.seller_supplier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RemittanceAdviceLine.seller_supplier_party", e));
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

impl RemittanceAdviceLine {
    pub fn title() -> &'static str {
        "Remittance Advice Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Remittance Advice."
    }
    pub fn new(id: RemittanceAdviceLineArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            buyer_customer_party: None,
            credit_line_amount: None,
            payee_party: None,
            seller_supplier_party: None,
            balance_amount: None,
            debit_line_amount: None,
            note: None,
            payment_purpose_code: None,
            exchange_rate: None,
            accounting_customer_party: None,
            accounting_supplier_party: None,
            document_reference: None,
            invoice_period: None,
            originator_customer_party: None,
            id,
            uuid: None,
            invoicing_party_reference: None,
            billing_reference: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfAccountingCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AccountingCustomerParty>,
}

impl AsMut<RemittanceAdviceLineArrayOfAccountingCustomerPartyComponent> for RemittanceAdviceLineArrayOfAccountingCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfAccountingCustomerPartyComponent> for RemittanceAdviceLineArrayOfAccountingCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfAccountingCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfAccountingCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfAccountingCustomerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AccountingCustomerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AccountingCustomerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AccountingCustomerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AccountingCustomerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfAccountingSupplierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AccountingSupplierParty>,
}

impl AsMut<RemittanceAdviceLineArrayOfAccountingSupplierPartyComponent> for RemittanceAdviceLineArrayOfAccountingSupplierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfAccountingSupplierPartyComponent> for RemittanceAdviceLineArrayOfAccountingSupplierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfAccountingSupplierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfAccountingSupplierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfAccountingSupplierPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AccountingSupplierParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AccountingSupplierParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AccountingSupplierParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AccountingSupplierParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfBalanceAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BalanceAmount>,
}

impl AsMut<RemittanceAdviceLineArrayOfBalanceAmountComponent> for RemittanceAdviceLineArrayOfBalanceAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfBalanceAmountComponent> for RemittanceAdviceLineArrayOfBalanceAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfBalanceAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfBalanceAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfBalanceAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BalanceAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BalanceAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BalanceAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BalanceAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfBillingReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BillingReference>,
}

impl AsMut<RemittanceAdviceLineArrayOfBillingReferenceComponent> for RemittanceAdviceLineArrayOfBillingReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfBillingReferenceComponent> for RemittanceAdviceLineArrayOfBillingReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfBillingReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfBillingReferenceComponent {
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
pub struct RemittanceAdviceLineArrayOfBuyerCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BuyerCustomerParty>,
}

impl AsMut<RemittanceAdviceLineArrayOfBuyerCustomerPartyComponent> for RemittanceAdviceLineArrayOfBuyerCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfBuyerCustomerPartyComponent> for RemittanceAdviceLineArrayOfBuyerCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfBuyerCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfBuyerCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfBuyerCustomerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BuyerCustomerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BuyerCustomerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BuyerCustomerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BuyerCustomerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfCreditLineAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CreditLineAmount>,
}

impl AsMut<RemittanceAdviceLineArrayOfCreditLineAmountComponent> for RemittanceAdviceLineArrayOfCreditLineAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfCreditLineAmountComponent> for RemittanceAdviceLineArrayOfCreditLineAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfCreditLineAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfCreditLineAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfCreditLineAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CreditLineAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CreditLineAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CreditLineAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CreditLineAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfDebitLineAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DebitLineAmount>,
}

impl AsMut<RemittanceAdviceLineArrayOfDebitLineAmountComponent> for RemittanceAdviceLineArrayOfDebitLineAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfDebitLineAmountComponent> for RemittanceAdviceLineArrayOfDebitLineAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfDebitLineAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfDebitLineAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfDebitLineAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DebitLineAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DebitLineAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DebitLineAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DebitLineAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<RemittanceAdviceLineArrayOfDocumentReferenceComponent> for RemittanceAdviceLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfDocumentReferenceComponent> for RemittanceAdviceLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfDocumentReferenceComponent {
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
pub struct RemittanceAdviceLineArrayOfExchangeRateComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExchangeRate>,
}

impl AsMut<RemittanceAdviceLineArrayOfExchangeRateComponent> for RemittanceAdviceLineArrayOfExchangeRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfExchangeRateComponent> for RemittanceAdviceLineArrayOfExchangeRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfExchangeRateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfExchangeRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfExchangeRateComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ExchangeRate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ExchangeRate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ExchangeRate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ExchangeRate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<RemittanceAdviceLineArrayOfIDComponent> for RemittanceAdviceLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfIDComponent> for RemittanceAdviceLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfIDComponent {
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
pub struct RemittanceAdviceLineArrayOfInvoicePeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::InvoicePeriod>,
}

impl AsMut<RemittanceAdviceLineArrayOfInvoicePeriodComponent> for RemittanceAdviceLineArrayOfInvoicePeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfInvoicePeriodComponent> for RemittanceAdviceLineArrayOfInvoicePeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfInvoicePeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfInvoicePeriodComponent {
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
pub struct RemittanceAdviceLineArrayOfInvoicingPartyReferenceComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InvoicingPartyReference>,
}

impl AsMut<RemittanceAdviceLineArrayOfInvoicingPartyReferenceComponent> for RemittanceAdviceLineArrayOfInvoicingPartyReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfInvoicingPartyReferenceComponent> for RemittanceAdviceLineArrayOfInvoicingPartyReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfInvoicingPartyReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfInvoicingPartyReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfInvoicingPartyReferenceComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InvoicingPartyReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InvoicingPartyReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InvoicingPartyReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InvoicingPartyReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<RemittanceAdviceLineArrayOfNoteComponent> for RemittanceAdviceLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfNoteComponent> for RemittanceAdviceLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfNoteComponent {
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
pub struct RemittanceAdviceLineArrayOfOriginatorCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginatorCustomerParty>,
}

impl AsMut<RemittanceAdviceLineArrayOfOriginatorCustomerPartyComponent> for RemittanceAdviceLineArrayOfOriginatorCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfOriginatorCustomerPartyComponent> for RemittanceAdviceLineArrayOfOriginatorCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfOriginatorCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfOriginatorCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfOriginatorCustomerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OriginatorCustomerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OriginatorCustomerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OriginatorCustomerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OriginatorCustomerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfPayeePartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PayeeParty>,
}

impl AsMut<RemittanceAdviceLineArrayOfPayeePartyComponent> for RemittanceAdviceLineArrayOfPayeePartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfPayeePartyComponent> for RemittanceAdviceLineArrayOfPayeePartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfPayeePartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfPayeePartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfPayeePartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PayeeParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PayeeParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PayeeParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PayeeParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfPaymentPurposeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentPurposeCode>,
}

impl AsMut<RemittanceAdviceLineArrayOfPaymentPurposeCodeComponent> for RemittanceAdviceLineArrayOfPaymentPurposeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfPaymentPurposeCodeComponent> for RemittanceAdviceLineArrayOfPaymentPurposeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfPaymentPurposeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfPaymentPurposeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfPaymentPurposeCodeComponent {
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
pub struct RemittanceAdviceLineArrayOfSellerSupplierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty>,
}

impl AsMut<RemittanceAdviceLineArrayOfSellerSupplierPartyComponent> for RemittanceAdviceLineArrayOfSellerSupplierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfSellerSupplierPartyComponent> for RemittanceAdviceLineArrayOfSellerSupplierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfSellerSupplierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfSellerSupplierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfSellerSupplierPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RemittanceAdviceLineArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<RemittanceAdviceLineArrayOfUUIDComponent> for RemittanceAdviceLineArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RemittanceAdviceLineArrayOfUUIDComponent> for RemittanceAdviceLineArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RemittanceAdviceLineArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RemittanceAdviceLineArrayOfUUIDComponent {
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


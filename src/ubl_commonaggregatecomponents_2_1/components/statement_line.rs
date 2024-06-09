use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatementLine {
    #[serde(rename = "AccountingCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_customer_party: Option<StatementLineArrayOfAccountingCustomerPartyComponent>,
    #[serde(rename = "AccountingSupplierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_supplier_party: Option<StatementLineArrayOfAccountingSupplierPartyComponent>,
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<StatementLineArrayOfAllowanceChargeComponent>,
    #[serde(rename = "BalanceAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_amount: Option<StatementLineArrayOfBalanceAmountComponent>,
    #[serde(rename = "BalanceBroughtForwardIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_brought_forward_indicator: Option<StatementLineArrayOfBalanceBroughtForwardIndicatorComponent>,
    #[serde(rename = "BillingReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reference: Option<StatementLineArrayOfBillingReferenceComponent>,
    #[serde(rename = "BuyerCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<StatementLineArrayOfBuyerCustomerPartyComponent>,
    #[serde(rename = "CollectedPayment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collected_payment: Option<StatementLineArrayOfCollectedPaymentComponent>,
    #[serde(rename = "CreditLineAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_line_amount: Option<StatementLineArrayOfCreditLineAmountComponent>,
    #[serde(rename = "DebitLineAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_line_amount: Option<StatementLineArrayOfDebitLineAmountComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<StatementLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ExchangeRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<StatementLineArrayOfExchangeRateComponent>,
    #[serde(rename = "ID")]
    pub id: StatementLineArrayOfIDComponent,
    #[serde(rename = "InvoicePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_period: Option<StatementLineArrayOfInvoicePeriodComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<StatementLineArrayOfNoteComponent>,
    #[serde(rename = "OriginatorCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<StatementLineArrayOfOriginatorCustomerPartyComponent>,
    #[serde(rename = "PayeeParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_party: Option<StatementLineArrayOfPayeePartyComponent>,
    #[serde(rename = "PaymentMeans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_means: Option<StatementLineArrayOfPaymentMeansComponent>,
    #[serde(rename = "PaymentPurposeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_purpose_code: Option<StatementLineArrayOfPaymentPurposeCodeComponent>,
    #[serde(rename = "PaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<StatementLineArrayOfPaymentTermsComponent>,
    #[serde(rename = "SellerSupplierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<StatementLineArrayOfSellerSupplierPartyComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<StatementLineArrayOfUUIDComponent>,
}

impl AsMut<StatementLine> for StatementLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLine> for StatementLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.exchange_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.exchange_rate", e));
            }
        }
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.allowance_charge", e));
            }
        }
        if let Some(v) = &self.originator_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.originator_customer_party", e));
            }
        }
        if let Some(v) = &self.debit_line_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.debit_line_amount", e));
            }
        }
        if let Some(v) = &self.buyer_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.buyer_customer_party", e));
            }
        }
        if let Some(v) = &self.payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.payment_terms", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.uuid", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("StatementLine.id", e));
        }
        if let Some(v) = &self.accounting_supplier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.accounting_supplier_party", e));
            }
        }
        if let Some(v) = &self.accounting_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.accounting_customer_party", e));
            }
        }
        if let Some(v) = &self.balance_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.balance_amount", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.note", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.document_reference", e));
            }
        }
        if let Some(v) = &self.payment_means {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.payment_means", e));
            }
        }
        if let Some(v) = &self.invoice_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.invoice_period", e));
            }
        }
        if let Some(v) = &self.payment_purpose_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.payment_purpose_code", e));
            }
        }
        if let Some(v) = &self.credit_line_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.credit_line_amount", e));
            }
        }
        if let Some(v) = &self.payee_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.payee_party", e));
            }
        }
        if let Some(v) = &self.balance_brought_forward_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.balance_brought_forward_indicator", e));
            }
        }
        if let Some(v) = &self.billing_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.billing_reference", e));
            }
        }
        if let Some(v) = &self.seller_supplier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.seller_supplier_party", e));
            }
        }
        if let Some(v) = &self.collected_payment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StatementLine.collected_payment", e));
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

impl StatementLine {
    pub fn title() -> &'static str {
        "Statement Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Statement of account."
    }
    pub fn new(id: StatementLineArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            accounting_customer_party: None,
            invoice_period: None,
            payee_party: None,
            payment_purpose_code: None,
            uuid: None,
            balance_amount: None,
            document_reference: None,
            id,
            collected_payment: None,
            debit_line_amount: None,
            payment_terms: None,
            note: None,
            payment_means: None,
            accounting_supplier_party: None,
            seller_supplier_party: None,
            exchange_rate: None,
            balance_brought_forward_indicator: None,
            allowance_charge: None,
            billing_reference: None,
            buyer_customer_party: None,
            credit_line_amount: None,
            originator_customer_party: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatementLineArrayOfAccountingCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AccountingCustomerParty>,
}

impl AsMut<StatementLineArrayOfAccountingCustomerPartyComponent> for StatementLineArrayOfAccountingCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfAccountingCustomerPartyComponent> for StatementLineArrayOfAccountingCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfAccountingCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfAccountingCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfAccountingCustomerPartyComponent {
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
pub struct StatementLineArrayOfAccountingSupplierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AccountingSupplierParty>,
}

impl AsMut<StatementLineArrayOfAccountingSupplierPartyComponent> for StatementLineArrayOfAccountingSupplierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfAccountingSupplierPartyComponent> for StatementLineArrayOfAccountingSupplierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfAccountingSupplierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfAccountingSupplierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfAccountingSupplierPartyComponent {
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
pub struct StatementLineArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<StatementLineArrayOfAllowanceChargeComponent> for StatementLineArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfAllowanceChargeComponent> for StatementLineArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfAllowanceChargeComponent {
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
pub struct StatementLineArrayOfBalanceAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BalanceAmount>,
}

impl AsMut<StatementLineArrayOfBalanceAmountComponent> for StatementLineArrayOfBalanceAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfBalanceAmountComponent> for StatementLineArrayOfBalanceAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfBalanceAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfBalanceAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfBalanceAmountComponent {
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
pub struct StatementLineArrayOfBalanceBroughtForwardIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BalanceBroughtForwardIndicator>,
}

impl AsMut<StatementLineArrayOfBalanceBroughtForwardIndicatorComponent> for StatementLineArrayOfBalanceBroughtForwardIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfBalanceBroughtForwardIndicatorComponent> for StatementLineArrayOfBalanceBroughtForwardIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfBalanceBroughtForwardIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfBalanceBroughtForwardIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfBalanceBroughtForwardIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BalanceBroughtForwardIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BalanceBroughtForwardIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BalanceBroughtForwardIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BalanceBroughtForwardIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatementLineArrayOfBillingReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BillingReference>,
}

impl AsMut<StatementLineArrayOfBillingReferenceComponent> for StatementLineArrayOfBillingReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfBillingReferenceComponent> for StatementLineArrayOfBillingReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfBillingReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfBillingReferenceComponent {
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
pub struct StatementLineArrayOfBuyerCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BuyerCustomerParty>,
}

impl AsMut<StatementLineArrayOfBuyerCustomerPartyComponent> for StatementLineArrayOfBuyerCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfBuyerCustomerPartyComponent> for StatementLineArrayOfBuyerCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfBuyerCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfBuyerCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfBuyerCustomerPartyComponent {
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
pub struct StatementLineArrayOfCollectedPaymentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CollectedPayment>,
}

impl AsMut<StatementLineArrayOfCollectedPaymentComponent> for StatementLineArrayOfCollectedPaymentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfCollectedPaymentComponent> for StatementLineArrayOfCollectedPaymentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfCollectedPaymentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfCollectedPaymentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CollectedPayment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CollectedPayment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CollectedPayment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CollectedPayment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatementLineArrayOfCreditLineAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CreditLineAmount>,
}

impl AsMut<StatementLineArrayOfCreditLineAmountComponent> for StatementLineArrayOfCreditLineAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfCreditLineAmountComponent> for StatementLineArrayOfCreditLineAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfCreditLineAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfCreditLineAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfCreditLineAmountComponent {
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
pub struct StatementLineArrayOfDebitLineAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DebitLineAmount>,
}

impl AsMut<StatementLineArrayOfDebitLineAmountComponent> for StatementLineArrayOfDebitLineAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfDebitLineAmountComponent> for StatementLineArrayOfDebitLineAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfDebitLineAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfDebitLineAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfDebitLineAmountComponent {
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
pub struct StatementLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<StatementLineArrayOfDocumentReferenceComponent> for StatementLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfDocumentReferenceComponent> for StatementLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfDocumentReferenceComponent {
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
pub struct StatementLineArrayOfExchangeRateComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExchangeRate>,
}

impl AsMut<StatementLineArrayOfExchangeRateComponent> for StatementLineArrayOfExchangeRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfExchangeRateComponent> for StatementLineArrayOfExchangeRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfExchangeRateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfExchangeRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfExchangeRateComponent {
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
pub struct StatementLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<StatementLineArrayOfIDComponent> for StatementLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfIDComponent> for StatementLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfIDComponent {
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
pub struct StatementLineArrayOfInvoicePeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::InvoicePeriod>,
}

impl AsMut<StatementLineArrayOfInvoicePeriodComponent> for StatementLineArrayOfInvoicePeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfInvoicePeriodComponent> for StatementLineArrayOfInvoicePeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfInvoicePeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfInvoicePeriodComponent {
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
pub struct StatementLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<StatementLineArrayOfNoteComponent> for StatementLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfNoteComponent> for StatementLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfNoteComponent {
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
pub struct StatementLineArrayOfOriginatorCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginatorCustomerParty>,
}

impl AsMut<StatementLineArrayOfOriginatorCustomerPartyComponent> for StatementLineArrayOfOriginatorCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfOriginatorCustomerPartyComponent> for StatementLineArrayOfOriginatorCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfOriginatorCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfOriginatorCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfOriginatorCustomerPartyComponent {
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
pub struct StatementLineArrayOfPayeePartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PayeeParty>,
}

impl AsMut<StatementLineArrayOfPayeePartyComponent> for StatementLineArrayOfPayeePartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfPayeePartyComponent> for StatementLineArrayOfPayeePartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfPayeePartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfPayeePartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfPayeePartyComponent {
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
pub struct StatementLineArrayOfPaymentMeansComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PaymentMeans>,
}

impl AsMut<StatementLineArrayOfPaymentMeansComponent> for StatementLineArrayOfPaymentMeansComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfPaymentMeansComponent> for StatementLineArrayOfPaymentMeansComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfPaymentMeansComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfPaymentMeansComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfPaymentMeansComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PaymentMeans) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PaymentMeans) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PaymentMeans> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PaymentMeans> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatementLineArrayOfPaymentPurposeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentPurposeCode>,
}

impl AsMut<StatementLineArrayOfPaymentPurposeCodeComponent> for StatementLineArrayOfPaymentPurposeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfPaymentPurposeCodeComponent> for StatementLineArrayOfPaymentPurposeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfPaymentPurposeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfPaymentPurposeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfPaymentPurposeCodeComponent {
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
pub struct StatementLineArrayOfPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms>,
}

impl AsMut<StatementLineArrayOfPaymentTermsComponent> for StatementLineArrayOfPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfPaymentTermsComponent> for StatementLineArrayOfPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfPaymentTermsComponent {
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
pub struct StatementLineArrayOfSellerSupplierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty>,
}

impl AsMut<StatementLineArrayOfSellerSupplierPartyComponent> for StatementLineArrayOfSellerSupplierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfSellerSupplierPartyComponent> for StatementLineArrayOfSellerSupplierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfSellerSupplierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfSellerSupplierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfSellerSupplierPartyComponent {
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
pub struct StatementLineArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<StatementLineArrayOfUUIDComponent> for StatementLineArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatementLineArrayOfUUIDComponent> for StatementLineArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatementLineArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatementLineArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatementLineArrayOfUUIDComponent {
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


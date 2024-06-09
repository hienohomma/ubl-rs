use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaymentMandate {
    #[serde(rename = "Clause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clause: Option<PaymentMandateArrayOfClauseComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PaymentMandateArrayOfIDComponent>,
    #[serde(rename = "MandateTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_type_code: Option<PaymentMandateArrayOfMandateTypeCodeComponent>,
    #[serde(rename = "MaximumPaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_paid_amount: Option<PaymentMandateArrayOfMaximumPaidAmountComponent>,
    #[serde(rename = "MaximumPaymentInstructionsNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_payment_instructions_numeric: Option<PaymentMandateArrayOfMaximumPaymentInstructionsNumericComponent>,
    #[serde(rename = "PayerFinancialAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_financial_account: Option<PaymentMandateArrayOfPayerFinancialAccountComponent>,
    #[serde(rename = "PayerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_party: Option<PaymentMandateArrayOfPayerPartyComponent>,
    #[serde(rename = "PaymentReversalPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_reversal_period: Option<PaymentMandateArrayOfPaymentReversalPeriodComponent>,
    #[serde(rename = "SignatureID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_id: Option<PaymentMandateArrayOfSignatureIDComponent>,
    #[serde(rename = "ValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<PaymentMandateArrayOfValidityPeriodComponent>,
}

impl AsMut<PaymentMandate> for PaymentMandate {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMandate> for PaymentMandate {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.payer_financial_account {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMandate.payer_financial_account", e));
            }
        }
        if let Some(v) = &self.payer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMandate.payer_party", e));
            }
        }
        if let Some(v) = &self.signature_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMandate.signature_id", e));
            }
        }
        if let Some(v) = &self.clause {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMandate.clause", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMandate.id", e));
            }
        }
        if let Some(v) = &self.mandate_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMandate.mandate_type_code", e));
            }
        }
        if let Some(v) = &self.validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMandate.validity_period", e));
            }
        }
        if let Some(v) = &self.maximum_paid_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMandate.maximum_paid_amount", e));
            }
        }
        if let Some(v) = &self.payment_reversal_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMandate.payment_reversal_period", e));
            }
        }
        if let Some(v) = &self.maximum_payment_instructions_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMandate.maximum_payment_instructions_numeric", e));
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

impl PaymentMandate {
    pub fn title() -> &'static str {
        "Payment Mandate. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a payment mandate."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            payer_financial_account: None,
            payment_reversal_period: None,
            signature_id: None,
            validity_period: None,
            clause: None,
            mandate_type_code: None,
            payer_party: None,
            maximum_payment_instructions_numeric: None,
            id: None,
            maximum_paid_amount: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMandateArrayOfClauseComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Clause>,
}

impl AsMut<PaymentMandateArrayOfClauseComponent> for PaymentMandateArrayOfClauseComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMandateArrayOfClauseComponent> for PaymentMandateArrayOfClauseComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfClauseComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMandateArrayOfClauseComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Clause) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Clause) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Clause> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Clause> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMandateArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<PaymentMandateArrayOfIDComponent> for PaymentMandateArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMandateArrayOfIDComponent> for PaymentMandateArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMandateArrayOfIDComponent {
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
pub struct PaymentMandateArrayOfMandateTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MandateTypeCode>,
}

impl AsMut<PaymentMandateArrayOfMandateTypeCodeComponent> for PaymentMandateArrayOfMandateTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMandateArrayOfMandateTypeCodeComponent> for PaymentMandateArrayOfMandateTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfMandateTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfMandateTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMandateArrayOfMandateTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MandateTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MandateTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MandateTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MandateTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMandateArrayOfMaximumPaidAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumPaidAmount>,
}

impl AsMut<PaymentMandateArrayOfMaximumPaidAmountComponent> for PaymentMandateArrayOfMaximumPaidAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMandateArrayOfMaximumPaidAmountComponent> for PaymentMandateArrayOfMaximumPaidAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfMaximumPaidAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfMaximumPaidAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMandateArrayOfMaximumPaidAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumPaidAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumPaidAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumPaidAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumPaidAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMandateArrayOfMaximumPaymentInstructionsNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumPaymentInstructionsNumeric>,
}

impl AsMut<PaymentMandateArrayOfMaximumPaymentInstructionsNumericComponent> for PaymentMandateArrayOfMaximumPaymentInstructionsNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMandateArrayOfMaximumPaymentInstructionsNumericComponent> for PaymentMandateArrayOfMaximumPaymentInstructionsNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfMaximumPaymentInstructionsNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfMaximumPaymentInstructionsNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMandateArrayOfMaximumPaymentInstructionsNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumPaymentInstructionsNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumPaymentInstructionsNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumPaymentInstructionsNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumPaymentInstructionsNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMandateArrayOfPayerFinancialAccountComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PayerFinancialAccount>,
}

impl AsMut<PaymentMandateArrayOfPayerFinancialAccountComponent> for PaymentMandateArrayOfPayerFinancialAccountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMandateArrayOfPayerFinancialAccountComponent> for PaymentMandateArrayOfPayerFinancialAccountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfPayerFinancialAccountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfPayerFinancialAccountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMandateArrayOfPayerFinancialAccountComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PayerFinancialAccount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PayerFinancialAccount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PayerFinancialAccount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PayerFinancialAccount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMandateArrayOfPayerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PayerParty>,
}

impl AsMut<PaymentMandateArrayOfPayerPartyComponent> for PaymentMandateArrayOfPayerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMandateArrayOfPayerPartyComponent> for PaymentMandateArrayOfPayerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfPayerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfPayerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMandateArrayOfPayerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PayerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PayerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PayerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PayerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMandateArrayOfPaymentReversalPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PaymentReversalPeriod>,
}

impl AsMut<PaymentMandateArrayOfPaymentReversalPeriodComponent> for PaymentMandateArrayOfPaymentReversalPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMandateArrayOfPaymentReversalPeriodComponent> for PaymentMandateArrayOfPaymentReversalPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfPaymentReversalPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfPaymentReversalPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMandateArrayOfPaymentReversalPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PaymentReversalPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PaymentReversalPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PaymentReversalPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PaymentReversalPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMandateArrayOfSignatureIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SignatureID>,
}

impl AsMut<PaymentMandateArrayOfSignatureIDComponent> for PaymentMandateArrayOfSignatureIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMandateArrayOfSignatureIDComponent> for PaymentMandateArrayOfSignatureIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfSignatureIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfSignatureIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMandateArrayOfSignatureIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SignatureID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SignatureID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SignatureID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SignatureID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMandateArrayOfValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod>,
}

impl AsMut<PaymentMandateArrayOfValidityPeriodComponent> for PaymentMandateArrayOfValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMandateArrayOfValidityPeriodComponent> for PaymentMandateArrayOfValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMandateArrayOfValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMandateArrayOfValidityPeriodComponent {
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


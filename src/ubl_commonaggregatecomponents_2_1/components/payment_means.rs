use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaymentMeans {
    #[serde(rename = "CardAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_account: Option<PaymentMeansArrayOfCardAccountComponent>,
    #[serde(rename = "CreditAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_account: Option<PaymentMeansArrayOfCreditAccountComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PaymentMeansArrayOfIDComponent>,
    #[serde(rename = "InstructionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_id: Option<PaymentMeansArrayOfInstructionIDComponent>,
    #[serde(rename = "InstructionNote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_note: Option<PaymentMeansArrayOfInstructionNoteComponent>,
    #[serde(rename = "PayeeFinancialAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_financial_account: Option<PaymentMeansArrayOfPayeeFinancialAccountComponent>,
    #[serde(rename = "PayerFinancialAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_financial_account: Option<PaymentMeansArrayOfPayerFinancialAccountComponent>,
    #[serde(rename = "PaymentChannelCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_channel_code: Option<PaymentMeansArrayOfPaymentChannelCodeComponent>,
    #[serde(rename = "PaymentDueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_date: Option<PaymentMeansArrayOfPaymentDueDateComponent>,
    #[serde(rename = "PaymentID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<PaymentMeansArrayOfPaymentIDComponent>,
    #[serde(rename = "PaymentMandate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_mandate: Option<PaymentMeansArrayOfPaymentMandateComponent>,
    #[serde(rename = "PaymentMeansCode")]
    pub payment_means_code: PaymentMeansArrayOfPaymentMeansCodeComponent,
    #[serde(rename = "TradeFinancing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_financing: Option<PaymentMeansArrayOfTradeFinancingComponent>,
}

impl AsMut<PaymentMeans> for PaymentMeans {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeans> for PaymentMeans {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.trade_financing {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.trade_financing", e));
            }
        }
        if let Some(v) = &self.payment_mandate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.payment_mandate", e));
            }
        }
        if let Some(v) = &self.instruction_note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.instruction_note", e));
            }
        }
        if let Some(v) = &self.payer_financial_account {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.payer_financial_account", e));
            }
        }
        if let Err(e) = self.payment_means_code.validate() {
            return Err(UblError::component("PaymentMeans.payment_means_code", e));
        }
        if let Some(v) = &self.instruction_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.instruction_id", e));
            }
        }
        if let Some(v) = &self.payment_due_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.payment_due_date", e));
            }
        }
        if let Some(v) = &self.card_account {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.card_account", e));
            }
        }
        if let Some(v) = &self.payee_financial_account {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.payee_financial_account", e));
            }
        }
        if let Some(v) = &self.payment_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.payment_id", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.id", e));
            }
        }
        if let Some(v) = &self.payment_channel_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.payment_channel_code", e));
            }
        }
        if let Some(v) = &self.credit_account {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentMeans.credit_account", e));
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

impl PaymentMeans {
    pub fn title() -> &'static str {
        "Payment Means. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a means of payment."
    }
    pub fn new(payment_means_code: PaymentMeansArrayOfPaymentMeansCodeComponent) -> Component<Self> {
        Component(Self {
            payer_financial_account: None,
            instruction_id: None,
            instruction_note: None,
            payment_means_code,
            payment_due_date: None,
            payment_id: None,
            credit_account: None,
            payment_channel_code: None,
            id: None,
            card_account: None,
            trade_financing: None,
            payment_mandate: None,
            payee_financial_account: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMeansArrayOfCardAccountComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CardAccount>,
}

impl AsMut<PaymentMeansArrayOfCardAccountComponent> for PaymentMeansArrayOfCardAccountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfCardAccountComponent> for PaymentMeansArrayOfCardAccountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfCardAccountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfCardAccountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfCardAccountComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CardAccount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CardAccount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CardAccount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CardAccount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMeansArrayOfCreditAccountComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CreditAccount>,
}

impl AsMut<PaymentMeansArrayOfCreditAccountComponent> for PaymentMeansArrayOfCreditAccountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfCreditAccountComponent> for PaymentMeansArrayOfCreditAccountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfCreditAccountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfCreditAccountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfCreditAccountComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CreditAccount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CreditAccount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CreditAccount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CreditAccount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMeansArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<PaymentMeansArrayOfIDComponent> for PaymentMeansArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfIDComponent> for PaymentMeansArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfIDComponent {
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
pub struct PaymentMeansArrayOfInstructionIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InstructionID>,
}

impl AsMut<PaymentMeansArrayOfInstructionIDComponent> for PaymentMeansArrayOfInstructionIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfInstructionIDComponent> for PaymentMeansArrayOfInstructionIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfInstructionIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfInstructionIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfInstructionIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InstructionID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InstructionID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InstructionID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InstructionID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMeansArrayOfInstructionNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InstructionNote>,
}

impl AsMut<PaymentMeansArrayOfInstructionNoteComponent> for PaymentMeansArrayOfInstructionNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfInstructionNoteComponent> for PaymentMeansArrayOfInstructionNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfInstructionNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfInstructionNoteComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InstructionNote) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InstructionNote) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InstructionNote> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InstructionNote> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMeansArrayOfPayeeFinancialAccountComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PayeeFinancialAccount>,
}

impl AsMut<PaymentMeansArrayOfPayeeFinancialAccountComponent> for PaymentMeansArrayOfPayeeFinancialAccountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfPayeeFinancialAccountComponent> for PaymentMeansArrayOfPayeeFinancialAccountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPayeeFinancialAccountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPayeeFinancialAccountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfPayeeFinancialAccountComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PayeeFinancialAccount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PayeeFinancialAccount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PayeeFinancialAccount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PayeeFinancialAccount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMeansArrayOfPayerFinancialAccountComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PayerFinancialAccount>,
}

impl AsMut<PaymentMeansArrayOfPayerFinancialAccountComponent> for PaymentMeansArrayOfPayerFinancialAccountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfPayerFinancialAccountComponent> for PaymentMeansArrayOfPayerFinancialAccountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPayerFinancialAccountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPayerFinancialAccountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfPayerFinancialAccountComponent {
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
pub struct PaymentMeansArrayOfPaymentChannelCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentChannelCode>,
}

impl AsMut<PaymentMeansArrayOfPaymentChannelCodeComponent> for PaymentMeansArrayOfPaymentChannelCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfPaymentChannelCodeComponent> for PaymentMeansArrayOfPaymentChannelCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPaymentChannelCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPaymentChannelCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfPaymentChannelCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaymentChannelCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaymentChannelCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaymentChannelCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaymentChannelCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMeansArrayOfPaymentDueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentDueDate>,
}

impl AsMut<PaymentMeansArrayOfPaymentDueDateComponent> for PaymentMeansArrayOfPaymentDueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfPaymentDueDateComponent> for PaymentMeansArrayOfPaymentDueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPaymentDueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPaymentDueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfPaymentDueDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaymentDueDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaymentDueDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaymentDueDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaymentDueDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMeansArrayOfPaymentIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentID>,
}

impl AsMut<PaymentMeansArrayOfPaymentIDComponent> for PaymentMeansArrayOfPaymentIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfPaymentIDComponent> for PaymentMeansArrayOfPaymentIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPaymentIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfPaymentIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaymentID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaymentID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaymentID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaymentID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMeansArrayOfPaymentMandateComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PaymentMandate>,
}

impl AsMut<PaymentMeansArrayOfPaymentMandateComponent> for PaymentMeansArrayOfPaymentMandateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfPaymentMandateComponent> for PaymentMeansArrayOfPaymentMandateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPaymentMandateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPaymentMandateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfPaymentMandateComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PaymentMandate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PaymentMandate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PaymentMandate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PaymentMandate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMeansArrayOfPaymentMeansCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentMeansCode>,
}

impl AsMut<PaymentMeansArrayOfPaymentMeansCodeComponent> for PaymentMeansArrayOfPaymentMeansCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfPaymentMeansCodeComponent> for PaymentMeansArrayOfPaymentMeansCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPaymentMeansCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfPaymentMeansCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfPaymentMeansCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaymentMeansCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaymentMeansCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaymentMeansCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaymentMeansCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentMeansArrayOfTradeFinancingComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TradeFinancing>,
}

impl AsMut<PaymentMeansArrayOfTradeFinancingComponent> for PaymentMeansArrayOfTradeFinancingComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentMeansArrayOfTradeFinancingComponent> for PaymentMeansArrayOfTradeFinancingComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfTradeFinancingComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentMeansArrayOfTradeFinancingComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentMeansArrayOfTradeFinancingComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TradeFinancing) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TradeFinancing) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TradeFinancing> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TradeFinancing> {
        self.items.iter()
    }
}


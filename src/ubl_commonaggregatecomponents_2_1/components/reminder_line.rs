use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReminderLine {
    #[serde(rename = "AccountingCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<ReminderLineArrayOfAccountingCostComponent>,
    #[serde(rename = "AccountingCostCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<ReminderLineArrayOfAccountingCostCodeComponent>,
    #[serde(rename = "Amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<ReminderLineArrayOfAmountComponent>,
    #[serde(rename = "BalanceBroughtForwardIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_brought_forward_indicator: Option<ReminderLineArrayOfBalanceBroughtForwardIndicatorComponent>,
    #[serde(rename = "BillingReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reference: Option<ReminderLineArrayOfBillingReferenceComponent>,
    #[serde(rename = "CreditLineAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_line_amount: Option<ReminderLineArrayOfCreditLineAmountComponent>,
    #[serde(rename = "DebitLineAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_line_amount: Option<ReminderLineArrayOfDebitLineAmountComponent>,
    #[serde(rename = "ExchangeRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<ReminderLineArrayOfExchangeRateComponent>,
    #[serde(rename = "ID")]
    pub id: ReminderLineArrayOfIDComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<ReminderLineArrayOfNoteComponent>,
    #[serde(rename = "PaymentPurposeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_purpose_code: Option<ReminderLineArrayOfPaymentPurposeCodeComponent>,
    #[serde(rename = "PenaltySurchargePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_surcharge_percent: Option<ReminderLineArrayOfPenaltySurchargePercentComponent>,
    #[serde(rename = "ReminderPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_period: Option<ReminderLineArrayOfReminderPeriodComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ReminderLineArrayOfUUIDComponent>,
}

impl AsMut<ReminderLine> for ReminderLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLine> for ReminderLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.penalty_surcharge_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.penalty_surcharge_percent", e));
            }
        }
        if let Some(v) = &self.credit_line_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.credit_line_amount", e));
            }
        }
        if let Some(v) = &self.exchange_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.exchange_rate", e));
            }
        }
        if let Some(v) = &self.payment_purpose_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.payment_purpose_code", e));
            }
        }
        if let Some(v) = &self.reminder_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.reminder_period", e));
            }
        }
        if let Some(v) = &self.debit_line_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.debit_line_amount", e));
            }
        }
        if let Some(v) = &self.billing_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.billing_reference", e));
            }
        }
        if let Some(v) = &self.accounting_cost {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.accounting_cost", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.uuid", e));
            }
        }
        if let Some(v) = &self.accounting_cost_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.accounting_cost_code", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.note", e));
            }
        }
        if let Some(v) = &self.balance_brought_forward_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.balance_brought_forward_indicator", e));
            }
        }
        if let Some(v) = &self.amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReminderLine.amount", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ReminderLine.id", e));
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

impl ReminderLine {
    pub fn title() -> &'static str {
        "Reminder Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Reminder document."
    }
    pub fn new(id: ReminderLineArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            accounting_cost: None,
            payment_purpose_code: None,
            note: None,
            uuid: None,
            balance_brought_forward_indicator: None,
            penalty_surcharge_percent: None,
            reminder_period: None,
            amount: None,
            credit_line_amount: None,
            debit_line_amount: None,
            billing_reference: None,
            id,
            exchange_rate: None,
            accounting_cost_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReminderLineArrayOfAccountingCostComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCost>,
}

impl AsMut<ReminderLineArrayOfAccountingCostComponent> for ReminderLineArrayOfAccountingCostComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfAccountingCostComponent> for ReminderLineArrayOfAccountingCostComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfAccountingCostComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfAccountingCostComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfAccountingCostComponent {
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
pub struct ReminderLineArrayOfAccountingCostCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCostCode>,
}

impl AsMut<ReminderLineArrayOfAccountingCostCodeComponent> for ReminderLineArrayOfAccountingCostCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfAccountingCostCodeComponent> for ReminderLineArrayOfAccountingCostCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfAccountingCostCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfAccountingCostCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfAccountingCostCodeComponent {
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
pub struct ReminderLineArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<ReminderLineArrayOfAmountComponent> for ReminderLineArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfAmountComponent> for ReminderLineArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Amount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Amount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Amount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Amount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReminderLineArrayOfBalanceBroughtForwardIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BalanceBroughtForwardIndicator>,
}

impl AsMut<ReminderLineArrayOfBalanceBroughtForwardIndicatorComponent> for ReminderLineArrayOfBalanceBroughtForwardIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfBalanceBroughtForwardIndicatorComponent> for ReminderLineArrayOfBalanceBroughtForwardIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfBalanceBroughtForwardIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfBalanceBroughtForwardIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfBalanceBroughtForwardIndicatorComponent {
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
pub struct ReminderLineArrayOfBillingReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BillingReference>,
}

impl AsMut<ReminderLineArrayOfBillingReferenceComponent> for ReminderLineArrayOfBillingReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfBillingReferenceComponent> for ReminderLineArrayOfBillingReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfBillingReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfBillingReferenceComponent {
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
pub struct ReminderLineArrayOfCreditLineAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CreditLineAmount>,
}

impl AsMut<ReminderLineArrayOfCreditLineAmountComponent> for ReminderLineArrayOfCreditLineAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfCreditLineAmountComponent> for ReminderLineArrayOfCreditLineAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfCreditLineAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfCreditLineAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfCreditLineAmountComponent {
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
pub struct ReminderLineArrayOfDebitLineAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DebitLineAmount>,
}

impl AsMut<ReminderLineArrayOfDebitLineAmountComponent> for ReminderLineArrayOfDebitLineAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfDebitLineAmountComponent> for ReminderLineArrayOfDebitLineAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfDebitLineAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfDebitLineAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfDebitLineAmountComponent {
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
pub struct ReminderLineArrayOfExchangeRateComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExchangeRate>,
}

impl AsMut<ReminderLineArrayOfExchangeRateComponent> for ReminderLineArrayOfExchangeRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfExchangeRateComponent> for ReminderLineArrayOfExchangeRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfExchangeRateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfExchangeRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfExchangeRateComponent {
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
pub struct ReminderLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ReminderLineArrayOfIDComponent> for ReminderLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfIDComponent> for ReminderLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfIDComponent {
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
pub struct ReminderLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<ReminderLineArrayOfNoteComponent> for ReminderLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfNoteComponent> for ReminderLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfNoteComponent {
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
pub struct ReminderLineArrayOfPaymentPurposeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentPurposeCode>,
}

impl AsMut<ReminderLineArrayOfPaymentPurposeCodeComponent> for ReminderLineArrayOfPaymentPurposeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfPaymentPurposeCodeComponent> for ReminderLineArrayOfPaymentPurposeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfPaymentPurposeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfPaymentPurposeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfPaymentPurposeCodeComponent {
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
pub struct ReminderLineArrayOfPenaltySurchargePercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PenaltySurchargePercent>,
}

impl AsMut<ReminderLineArrayOfPenaltySurchargePercentComponent> for ReminderLineArrayOfPenaltySurchargePercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfPenaltySurchargePercentComponent> for ReminderLineArrayOfPenaltySurchargePercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfPenaltySurchargePercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfPenaltySurchargePercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfPenaltySurchargePercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PenaltySurchargePercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PenaltySurchargePercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PenaltySurchargePercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PenaltySurchargePercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReminderLineArrayOfReminderPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReminderPeriod>,
}

impl AsMut<ReminderLineArrayOfReminderPeriodComponent> for ReminderLineArrayOfReminderPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfReminderPeriodComponent> for ReminderLineArrayOfReminderPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfReminderPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfReminderPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReminderPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReminderPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReminderPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReminderPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReminderLineArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<ReminderLineArrayOfUUIDComponent> for ReminderLineArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReminderLineArrayOfUUIDComponent> for ReminderLineArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReminderLineArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReminderLineArrayOfUUIDComponent {
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


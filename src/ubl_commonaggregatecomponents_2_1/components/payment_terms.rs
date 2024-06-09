use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaymentTerms {
    #[serde(rename = "Amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<PaymentTermsArrayOfAmountComponent>,
    #[serde(rename = "ExchangeRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<PaymentTermsArrayOfExchangeRateComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PaymentTermsArrayOfIDComponent>,
    #[serde(rename = "InstallmentDueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_due_date: Option<PaymentTermsArrayOfInstallmentDueDateComponent>,
    #[serde(rename = "InvoicingPartyReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoicing_party_reference: Option<PaymentTermsArrayOfInvoicingPartyReferenceComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<PaymentTermsArrayOfNoteComponent>,
    #[serde(rename = "PaymentDueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_date: Option<PaymentTermsArrayOfPaymentDueDateComponent>,
    #[serde(rename = "PaymentMeansID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_means_id: Option<PaymentTermsArrayOfPaymentMeansIDComponent>,
    #[serde(rename = "PaymentPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_percent: Option<PaymentTermsArrayOfPaymentPercentComponent>,
    #[serde(rename = "PaymentTermsDetailsURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms_details_uri: Option<PaymentTermsArrayOfPaymentTermsDetailsURIComponent>,
    #[serde(rename = "PenaltyAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_amount: Option<PaymentTermsArrayOfPenaltyAmountComponent>,
    #[serde(rename = "PenaltyPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_period: Option<PaymentTermsArrayOfPenaltyPeriodComponent>,
    #[serde(rename = "PenaltySurchargePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_surcharge_percent: Option<PaymentTermsArrayOfPenaltySurchargePercentComponent>,
    #[serde(rename = "PrepaidPaymentReferenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_payment_reference_id: Option<PaymentTermsArrayOfPrepaidPaymentReferenceIDComponent>,
    #[serde(rename = "ReferenceEventCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_event_code: Option<PaymentTermsArrayOfReferenceEventCodeComponent>,
    #[serde(rename = "SettlementDiscountAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_discount_amount: Option<PaymentTermsArrayOfSettlementDiscountAmountComponent>,
    #[serde(rename = "SettlementDiscountPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_discount_percent: Option<PaymentTermsArrayOfSettlementDiscountPercentComponent>,
    #[serde(rename = "SettlementPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_period: Option<PaymentTermsArrayOfSettlementPeriodComponent>,
    #[serde(rename = "ValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<PaymentTermsArrayOfValidityPeriodComponent>,
}

impl AsMut<PaymentTerms> for PaymentTerms {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTerms> for PaymentTerms {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.invoicing_party_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.invoicing_party_reference", e));
            }
        }
        if let Some(v) = &self.payment_due_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.payment_due_date", e));
            }
        }
        if let Some(v) = &self.payment_means_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.payment_means_id", e));
            }
        }
        if let Some(v) = &self.exchange_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.exchange_rate", e));
            }
        }
        if let Some(v) = &self.payment_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.payment_percent", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.note", e));
            }
        }
        if let Some(v) = &self.penalty_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.penalty_amount", e));
            }
        }
        if let Some(v) = &self.prepaid_payment_reference_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.prepaid_payment_reference_id", e));
            }
        }
        if let Some(v) = &self.settlement_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.settlement_period", e));
            }
        }
        if let Some(v) = &self.settlement_discount_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.settlement_discount_percent", e));
            }
        }
        if let Some(v) = &self.penalty_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.penalty_period", e));
            }
        }
        if let Some(v) = &self.penalty_surcharge_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.penalty_surcharge_percent", e));
            }
        }
        if let Some(v) = &self.amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.amount", e));
            }
        }
        if let Some(v) = &self.settlement_discount_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.settlement_discount_amount", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.id", e));
            }
        }
        if let Some(v) = &self.reference_event_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.reference_event_code", e));
            }
        }
        if let Some(v) = &self.payment_terms_details_uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.payment_terms_details_uri", e));
            }
        }
        if let Some(v) = &self.installment_due_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.installment_due_date", e));
            }
        }
        if let Some(v) = &self.validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PaymentTerms.validity_period", e));
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

impl PaymentTerms {
    pub fn title() -> &'static str {
        "Payment Terms. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a set of payment terms."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            validity_period: None,
            amount: None,
            reference_event_code: None,
            prepaid_payment_reference_id: None,
            settlement_discount_amount: None,
            invoicing_party_reference: None,
            settlement_period: None,
            penalty_period: None,
            note: None,
            payment_due_date: None,
            penalty_amount: None,
            exchange_rate: None,
            payment_terms_details_uri: None,
            penalty_surcharge_percent: None,
            id: None,
            payment_percent: None,
            settlement_discount_percent: None,
            payment_means_id: None,
            installment_due_date: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<PaymentTermsArrayOfAmountComponent> for PaymentTermsArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfAmountComponent> for PaymentTermsArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfAmountComponent {
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
pub struct PaymentTermsArrayOfExchangeRateComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExchangeRate>,
}

impl AsMut<PaymentTermsArrayOfExchangeRateComponent> for PaymentTermsArrayOfExchangeRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfExchangeRateComponent> for PaymentTermsArrayOfExchangeRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfExchangeRateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfExchangeRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfExchangeRateComponent {
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
pub struct PaymentTermsArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<PaymentTermsArrayOfIDComponent> for PaymentTermsArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfIDComponent> for PaymentTermsArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfIDComponent {
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
pub struct PaymentTermsArrayOfInstallmentDueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InstallmentDueDate>,
}

impl AsMut<PaymentTermsArrayOfInstallmentDueDateComponent> for PaymentTermsArrayOfInstallmentDueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfInstallmentDueDateComponent> for PaymentTermsArrayOfInstallmentDueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfInstallmentDueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfInstallmentDueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfInstallmentDueDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InstallmentDueDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InstallmentDueDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InstallmentDueDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InstallmentDueDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfInvoicingPartyReferenceComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InvoicingPartyReference>,
}

impl AsMut<PaymentTermsArrayOfInvoicingPartyReferenceComponent> for PaymentTermsArrayOfInvoicingPartyReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfInvoicingPartyReferenceComponent> for PaymentTermsArrayOfInvoicingPartyReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfInvoicingPartyReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfInvoicingPartyReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfInvoicingPartyReferenceComponent {
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
pub struct PaymentTermsArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<PaymentTermsArrayOfNoteComponent> for PaymentTermsArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfNoteComponent> for PaymentTermsArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfNoteComponent {
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
pub struct PaymentTermsArrayOfPaymentDueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentDueDate>,
}

impl AsMut<PaymentTermsArrayOfPaymentDueDateComponent> for PaymentTermsArrayOfPaymentDueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfPaymentDueDateComponent> for PaymentTermsArrayOfPaymentDueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPaymentDueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPaymentDueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfPaymentDueDateComponent {
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
pub struct PaymentTermsArrayOfPaymentMeansIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentMeansID>,
}

impl AsMut<PaymentTermsArrayOfPaymentMeansIDComponent> for PaymentTermsArrayOfPaymentMeansIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfPaymentMeansIDComponent> for PaymentTermsArrayOfPaymentMeansIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPaymentMeansIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfPaymentMeansIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaymentMeansID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaymentMeansID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaymentMeansID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaymentMeansID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfPaymentPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentPercent>,
}

impl AsMut<PaymentTermsArrayOfPaymentPercentComponent> for PaymentTermsArrayOfPaymentPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfPaymentPercentComponent> for PaymentTermsArrayOfPaymentPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPaymentPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPaymentPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfPaymentPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaymentPercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaymentPercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaymentPercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaymentPercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfPaymentTermsDetailsURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentTermsDetailsURI>,
}

impl AsMut<PaymentTermsArrayOfPaymentTermsDetailsURIComponent> for PaymentTermsArrayOfPaymentTermsDetailsURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfPaymentTermsDetailsURIComponent> for PaymentTermsArrayOfPaymentTermsDetailsURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPaymentTermsDetailsURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPaymentTermsDetailsURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfPaymentTermsDetailsURIComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaymentTermsDetailsURI) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaymentTermsDetailsURI) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaymentTermsDetailsURI> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaymentTermsDetailsURI> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfPenaltyAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PenaltyAmount>,
}

impl AsMut<PaymentTermsArrayOfPenaltyAmountComponent> for PaymentTermsArrayOfPenaltyAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfPenaltyAmountComponent> for PaymentTermsArrayOfPenaltyAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPenaltyAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPenaltyAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfPenaltyAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PenaltyAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PenaltyAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PenaltyAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PenaltyAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfPenaltyPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PenaltyPeriod>,
}

impl AsMut<PaymentTermsArrayOfPenaltyPeriodComponent> for PaymentTermsArrayOfPenaltyPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfPenaltyPeriodComponent> for PaymentTermsArrayOfPenaltyPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPenaltyPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPenaltyPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfPenaltyPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PenaltyPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PenaltyPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PenaltyPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PenaltyPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfPenaltySurchargePercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PenaltySurchargePercent>,
}

impl AsMut<PaymentTermsArrayOfPenaltySurchargePercentComponent> for PaymentTermsArrayOfPenaltySurchargePercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfPenaltySurchargePercentComponent> for PaymentTermsArrayOfPenaltySurchargePercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPenaltySurchargePercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPenaltySurchargePercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfPenaltySurchargePercentComponent {
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
pub struct PaymentTermsArrayOfPrepaidPaymentReferenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PrepaidPaymentReferenceID>,
}

impl AsMut<PaymentTermsArrayOfPrepaidPaymentReferenceIDComponent> for PaymentTermsArrayOfPrepaidPaymentReferenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfPrepaidPaymentReferenceIDComponent> for PaymentTermsArrayOfPrepaidPaymentReferenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPrepaidPaymentReferenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfPrepaidPaymentReferenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfPrepaidPaymentReferenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PrepaidPaymentReferenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PrepaidPaymentReferenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PrepaidPaymentReferenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PrepaidPaymentReferenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfReferenceEventCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReferenceEventCode>,
}

impl AsMut<PaymentTermsArrayOfReferenceEventCodeComponent> for PaymentTermsArrayOfReferenceEventCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfReferenceEventCodeComponent> for PaymentTermsArrayOfReferenceEventCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfReferenceEventCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfReferenceEventCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfReferenceEventCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReferenceEventCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReferenceEventCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReferenceEventCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReferenceEventCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfSettlementDiscountAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SettlementDiscountAmount>,
}

impl AsMut<PaymentTermsArrayOfSettlementDiscountAmountComponent> for PaymentTermsArrayOfSettlementDiscountAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfSettlementDiscountAmountComponent> for PaymentTermsArrayOfSettlementDiscountAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfSettlementDiscountAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfSettlementDiscountAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfSettlementDiscountAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SettlementDiscountAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SettlementDiscountAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SettlementDiscountAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SettlementDiscountAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfSettlementDiscountPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SettlementDiscountPercent>,
}

impl AsMut<PaymentTermsArrayOfSettlementDiscountPercentComponent> for PaymentTermsArrayOfSettlementDiscountPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfSettlementDiscountPercentComponent> for PaymentTermsArrayOfSettlementDiscountPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfSettlementDiscountPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfSettlementDiscountPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfSettlementDiscountPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SettlementDiscountPercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SettlementDiscountPercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SettlementDiscountPercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SettlementDiscountPercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfSettlementPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SettlementPeriod>,
}

impl AsMut<PaymentTermsArrayOfSettlementPeriodComponent> for PaymentTermsArrayOfSettlementPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfSettlementPeriodComponent> for PaymentTermsArrayOfSettlementPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfSettlementPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfSettlementPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfSettlementPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SettlementPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SettlementPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SettlementPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SettlementPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentTermsArrayOfValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod>,
}

impl AsMut<PaymentTermsArrayOfValidityPeriodComponent> for PaymentTermsArrayOfValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentTermsArrayOfValidityPeriodComponent> for PaymentTermsArrayOfValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentTermsArrayOfValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentTermsArrayOfValidityPeriodComponent {
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


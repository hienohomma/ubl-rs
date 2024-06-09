use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OnAccountPayment {
    #[serde(rename = "EstimatedConsumedQuantity")]
    pub estimated_consumed_quantity: OnAccountPaymentArrayOfEstimatedConsumedQuantityComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<OnAccountPaymentArrayOfNoteComponent>,
    #[serde(rename = "PaymentTerms")]
    pub payment_terms: OnAccountPaymentArrayOfPaymentTermsComponent,
}

impl AsMut<OnAccountPayment> for OnAccountPayment {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OnAccountPayment> for OnAccountPayment {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OnAccountPayment.note", e));
            }
        }
        if let Err(e) = self.estimated_consumed_quantity.validate() {
            return Err(UblError::component("OnAccountPayment.estimated_consumed_quantity", e));
        }
        if let Err(e) = self.payment_terms.validate() {
            return Err(UblError::component("OnAccountPayment.payment_terms", e));
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

impl OnAccountPayment {
    pub fn title() -> &'static str {
        "On Account Payment. Details"
    }
    pub fn description() -> &'static str {
        "A scheduled prepayment (on-account payment) for a estimated utility consumption"
    }
    pub fn new(estimated_consumed_quantity: OnAccountPaymentArrayOfEstimatedConsumedQuantityComponent, payment_terms: OnAccountPaymentArrayOfPaymentTermsComponent) -> Component<Self> {
        Component(Self {
            payment_terms,
            estimated_consumed_quantity,
            note: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OnAccountPaymentArrayOfEstimatedConsumedQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EstimatedConsumedQuantity>,
}

impl AsMut<OnAccountPaymentArrayOfEstimatedConsumedQuantityComponent> for OnAccountPaymentArrayOfEstimatedConsumedQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OnAccountPaymentArrayOfEstimatedConsumedQuantityComponent> for OnAccountPaymentArrayOfEstimatedConsumedQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OnAccountPaymentArrayOfEstimatedConsumedQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OnAccountPaymentArrayOfEstimatedConsumedQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OnAccountPaymentArrayOfEstimatedConsumedQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EstimatedConsumedQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EstimatedConsumedQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EstimatedConsumedQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EstimatedConsumedQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OnAccountPaymentArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<OnAccountPaymentArrayOfNoteComponent> for OnAccountPaymentArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OnAccountPaymentArrayOfNoteComponent> for OnAccountPaymentArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("OnAccountPaymentArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OnAccountPaymentArrayOfNoteComponent {
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
pub struct OnAccountPaymentArrayOfPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms>,
}

impl AsMut<OnAccountPaymentArrayOfPaymentTermsComponent> for OnAccountPaymentArrayOfPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OnAccountPaymentArrayOfPaymentTermsComponent> for OnAccountPaymentArrayOfPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("OnAccountPaymentArrayOfPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OnAccountPaymentArrayOfPaymentTermsComponent {
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


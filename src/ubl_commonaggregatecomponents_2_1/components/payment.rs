use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Payment {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PaymentArrayOfIDComponent>,
    #[serde(rename = "InstructionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_id: Option<PaymentArrayOfInstructionIDComponent>,
    #[serde(rename = "PaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_amount: Option<PaymentArrayOfPaidAmountComponent>,
    #[serde(rename = "PaidDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<PaymentArrayOfPaidDateComponent>,
    #[serde(rename = "PaidTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_time: Option<PaymentArrayOfPaidTimeComponent>,
    #[serde(rename = "ReceivedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_date: Option<PaymentArrayOfReceivedDateComponent>,
}

impl AsMut<Payment> for Payment {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Payment> for Payment {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.paid_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Payment.paid_time", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Payment.id", e));
            }
        }
        if let Some(v) = &self.received_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Payment.received_date", e));
            }
        }
        if let Some(v) = &self.paid_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Payment.paid_amount", e));
            }
        }
        if let Some(v) = &self.instruction_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Payment.instruction_id", e));
            }
        }
        if let Some(v) = &self.paid_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Payment.paid_date", e));
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

impl Payment {
    pub fn title() -> &'static str {
        "Payment. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a payment."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            paid_amount: None,
            paid_time: None,
            id: None,
            instruction_id: None,
            paid_date: None,
            received_date: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<PaymentArrayOfIDComponent> for PaymentArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentArrayOfIDComponent> for PaymentArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentArrayOfIDComponent {
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
pub struct PaymentArrayOfInstructionIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InstructionID>,
}

impl AsMut<PaymentArrayOfInstructionIDComponent> for PaymentArrayOfInstructionIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentArrayOfInstructionIDComponent> for PaymentArrayOfInstructionIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentArrayOfInstructionIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentArrayOfInstructionIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentArrayOfInstructionIDComponent {
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
pub struct PaymentArrayOfPaidAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaidAmount>,
}

impl AsMut<PaymentArrayOfPaidAmountComponent> for PaymentArrayOfPaidAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentArrayOfPaidAmountComponent> for PaymentArrayOfPaidAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentArrayOfPaidAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentArrayOfPaidAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentArrayOfPaidAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaidAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaidAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaidAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaidAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentArrayOfPaidDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaidDate>,
}

impl AsMut<PaymentArrayOfPaidDateComponent> for PaymentArrayOfPaidDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentArrayOfPaidDateComponent> for PaymentArrayOfPaidDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentArrayOfPaidDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentArrayOfPaidDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentArrayOfPaidDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaidDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaidDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaidDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaidDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentArrayOfPaidTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaidTime>,
}

impl AsMut<PaymentArrayOfPaidTimeComponent> for PaymentArrayOfPaidTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentArrayOfPaidTimeComponent> for PaymentArrayOfPaidTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentArrayOfPaidTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentArrayOfPaidTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentArrayOfPaidTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaidTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaidTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaidTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaidTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PaymentArrayOfReceivedDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReceivedDate>,
}

impl AsMut<PaymentArrayOfReceivedDateComponent> for PaymentArrayOfReceivedDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PaymentArrayOfReceivedDateComponent> for PaymentArrayOfReceivedDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PaymentArrayOfReceivedDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PaymentArrayOfReceivedDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PaymentArrayOfReceivedDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReceivedDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReceivedDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReceivedDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReceivedDate> {
        self.items.iter()
    }
}


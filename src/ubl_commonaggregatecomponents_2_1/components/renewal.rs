use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Renewal {
    #[serde(rename = "Amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<RenewalArrayOfAmountComponent>,
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<RenewalArrayOfPeriodComponent>,
}

impl AsMut<Renewal> for Renewal {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Renewal> for Renewal {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Renewal.amount", e));
            }
        }
        if let Some(v) = &self.period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Renewal.period", e));
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

impl Renewal {
    pub fn title() -> &'static str {
        "Renewal. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the renewal of a commercial arrangement, such as a contract or licence fee."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            amount: None,
            period: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RenewalArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<RenewalArrayOfAmountComponent> for RenewalArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RenewalArrayOfAmountComponent> for RenewalArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RenewalArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RenewalArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RenewalArrayOfAmountComponent {
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
pub struct RenewalArrayOfPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Period>,
}

impl AsMut<RenewalArrayOfPeriodComponent> for RenewalArrayOfPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RenewalArrayOfPeriodComponent> for RenewalArrayOfPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RenewalArrayOfPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RenewalArrayOfPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RenewalArrayOfPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Period) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Period) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Period> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Period> {
        self.items.iter()
    }
}


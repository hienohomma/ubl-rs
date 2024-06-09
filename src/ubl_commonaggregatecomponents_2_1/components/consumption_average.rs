use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsumptionAverage {
    #[serde(rename = "AverageAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_amount: Option<ConsumptionAverageArrayOfAverageAmountComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ConsumptionAverageArrayOfDescriptionComponent>,
}

impl AsMut<ConsumptionAverage> for ConsumptionAverage {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionAverage> for ConsumptionAverage {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionAverage.description", e));
            }
        }
        if let Some(v) = &self.average_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionAverage.average_amount", e));
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

impl ConsumptionAverage {
    pub fn title() -> &'static str {
        "Consumption Average. Details"
    }
    pub fn description() -> &'static str {
        "A class to define an average consumption as a monetary amount."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            average_amount: None,
            description: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionAverageArrayOfAverageAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AverageAmount>,
}

impl AsMut<ConsumptionAverageArrayOfAverageAmountComponent> for ConsumptionAverageArrayOfAverageAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionAverageArrayOfAverageAmountComponent> for ConsumptionAverageArrayOfAverageAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionAverageArrayOfAverageAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionAverageArrayOfAverageAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionAverageArrayOfAverageAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AverageAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AverageAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AverageAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AverageAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionAverageArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ConsumptionAverageArrayOfDescriptionComponent> for ConsumptionAverageArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionAverageArrayOfDescriptionComponent> for ConsumptionAverageArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionAverageArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionAverageArrayOfDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Description) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Description) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter()
    }
}


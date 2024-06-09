use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsumptionHistory {
    #[serde(rename = "Amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<ConsumptionHistoryArrayOfAmountComponent>,
    #[serde(rename = "ConsumptionLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_level: Option<ConsumptionHistoryArrayOfConsumptionLevelComponent>,
    #[serde(rename = "ConsumptionLevelCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_level_code: Option<ConsumptionHistoryArrayOfConsumptionLevelCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ConsumptionHistoryArrayOfDescriptionComponent>,
    #[serde(rename = "MeterNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_number: Option<ConsumptionHistoryArrayOfMeterNumberComponent>,
    #[serde(rename = "Period")]
    pub period: ConsumptionHistoryArrayOfPeriodComponent,
    #[serde(rename = "Quantity")]
    pub quantity: ConsumptionHistoryArrayOfQuantityComponent,
}

impl AsMut<ConsumptionHistory> for ConsumptionHistory {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionHistory> for ConsumptionHistory {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.meter_number {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionHistory.meter_number", e));
            }
        }
        if let Err(e) = self.quantity.validate() {
            return Err(UblError::component("ConsumptionHistory.quantity", e));
        }
        if let Err(e) = self.period.validate() {
            return Err(UblError::component("ConsumptionHistory.period", e));
        }
        if let Some(v) = &self.amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionHistory.amount", e));
            }
        }
        if let Some(v) = &self.consumption_level_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionHistory.consumption_level_code", e));
            }
        }
        if let Some(v) = &self.consumption_level {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionHistory.consumption_level", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionHistory.description", e));
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

impl ConsumptionHistory {
    pub fn title() -> &'static str {
        "Consumption History. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the measurement of a type of consumption during a particular period, used for the subscriber to get an overview of his consumption"
    }
    pub fn new(period: ConsumptionHistoryArrayOfPeriodComponent, quantity: ConsumptionHistoryArrayOfQuantityComponent) -> Component<Self> {
        Component(Self {
            consumption_level_code: None,
            meter_number: None,
            amount: None,
            consumption_level: None,
            period,
            quantity,
            description: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionHistoryArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<ConsumptionHistoryArrayOfAmountComponent> for ConsumptionHistoryArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionHistoryArrayOfAmountComponent> for ConsumptionHistoryArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionHistoryArrayOfAmountComponent {
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
pub struct ConsumptionHistoryArrayOfConsumptionLevelComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionLevel>,
}

impl AsMut<ConsumptionHistoryArrayOfConsumptionLevelComponent> for ConsumptionHistoryArrayOfConsumptionLevelComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionHistoryArrayOfConsumptionLevelComponent> for ConsumptionHistoryArrayOfConsumptionLevelComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfConsumptionLevelComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfConsumptionLevelComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionHistoryArrayOfConsumptionLevelComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumptionLevel) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumptionLevel) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumptionLevel> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumptionLevel> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionHistoryArrayOfConsumptionLevelCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionLevelCode>,
}

impl AsMut<ConsumptionHistoryArrayOfConsumptionLevelCodeComponent> for ConsumptionHistoryArrayOfConsumptionLevelCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionHistoryArrayOfConsumptionLevelCodeComponent> for ConsumptionHistoryArrayOfConsumptionLevelCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfConsumptionLevelCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfConsumptionLevelCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionHistoryArrayOfConsumptionLevelCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumptionLevelCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumptionLevelCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumptionLevelCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumptionLevelCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionHistoryArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ConsumptionHistoryArrayOfDescriptionComponent> for ConsumptionHistoryArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionHistoryArrayOfDescriptionComponent> for ConsumptionHistoryArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionHistoryArrayOfDescriptionComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionHistoryArrayOfMeterNumberComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MeterNumber>,
}

impl AsMut<ConsumptionHistoryArrayOfMeterNumberComponent> for ConsumptionHistoryArrayOfMeterNumberComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionHistoryArrayOfMeterNumberComponent> for ConsumptionHistoryArrayOfMeterNumberComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfMeterNumberComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfMeterNumberComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionHistoryArrayOfMeterNumberComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MeterNumber) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MeterNumber) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MeterNumber> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MeterNumber> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionHistoryArrayOfPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Period>,
}

impl AsMut<ConsumptionHistoryArrayOfPeriodComponent> for ConsumptionHistoryArrayOfPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionHistoryArrayOfPeriodComponent> for ConsumptionHistoryArrayOfPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionHistoryArrayOfPeriodComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionHistoryArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<ConsumptionHistoryArrayOfQuantityComponent> for ConsumptionHistoryArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionHistoryArrayOfQuantityComponent> for ConsumptionHistoryArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionHistoryArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionHistoryArrayOfQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Quantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Quantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetailPlannedImpact {
    #[serde(rename = "Amount")]
    pub amount: RetailPlannedImpactArrayOfAmountComponent,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: RetailPlannedImpactArrayOfForecastPurposeCodeComponent,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: RetailPlannedImpactArrayOfForecastTypeCodeComponent,
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<RetailPlannedImpactArrayOfPeriodComponent>,
}

impl AsMut<RetailPlannedImpact> for RetailPlannedImpact {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RetailPlannedImpact> for RetailPlannedImpact {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.amount.validate() {
            return Err(UblError::component("RetailPlannedImpact.amount", e));
        }
        if let Err(e) = self.forecast_purpose_code.validate() {
            return Err(UblError::component("RetailPlannedImpact.forecast_purpose_code", e));
        }
        if let Err(e) = self.forecast_type_code.validate() {
            return Err(UblError::component("RetailPlannedImpact.forecast_type_code", e));
        }
        if let Some(v) = &self.period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RetailPlannedImpact.period", e));
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

impl RetailPlannedImpact {
    pub fn title() -> &'static str {
        "Retail Planned Impact. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a planned effect of a retail event (e.g., a promotion or a change in inventory policy) upon supply or demand."
    }
    pub fn new(amount: RetailPlannedImpactArrayOfAmountComponent, forecast_purpose_code: RetailPlannedImpactArrayOfForecastPurposeCodeComponent, forecast_type_code: RetailPlannedImpactArrayOfForecastTypeCodeComponent) -> Component<Self> {
        Component(Self {
            forecast_purpose_code,
            period: None,
            forecast_type_code,
            amount,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RetailPlannedImpactArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<RetailPlannedImpactArrayOfAmountComponent> for RetailPlannedImpactArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RetailPlannedImpactArrayOfAmountComponent> for RetailPlannedImpactArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RetailPlannedImpactArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RetailPlannedImpactArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RetailPlannedImpactArrayOfAmountComponent {
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
pub struct RetailPlannedImpactArrayOfForecastPurposeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ForecastPurposeCode>,
}

impl AsMut<RetailPlannedImpactArrayOfForecastPurposeCodeComponent> for RetailPlannedImpactArrayOfForecastPurposeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RetailPlannedImpactArrayOfForecastPurposeCodeComponent> for RetailPlannedImpactArrayOfForecastPurposeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RetailPlannedImpactArrayOfForecastPurposeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RetailPlannedImpactArrayOfForecastPurposeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RetailPlannedImpactArrayOfForecastPurposeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ForecastPurposeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ForecastPurposeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ForecastPurposeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ForecastPurposeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RetailPlannedImpactArrayOfForecastTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode>,
}

impl AsMut<RetailPlannedImpactArrayOfForecastTypeCodeComponent> for RetailPlannedImpactArrayOfForecastTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RetailPlannedImpactArrayOfForecastTypeCodeComponent> for RetailPlannedImpactArrayOfForecastTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RetailPlannedImpactArrayOfForecastTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RetailPlannedImpactArrayOfForecastTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RetailPlannedImpactArrayOfForecastTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RetailPlannedImpactArrayOfPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Period>,
}

impl AsMut<RetailPlannedImpactArrayOfPeriodComponent> for RetailPlannedImpactArrayOfPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RetailPlannedImpactArrayOfPeriodComponent> for RetailPlannedImpactArrayOfPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RetailPlannedImpactArrayOfPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RetailPlannedImpactArrayOfPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RetailPlannedImpactArrayOfPeriodComponent {
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


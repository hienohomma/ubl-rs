use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnergyWaterSupply {
    #[serde(rename = "ConsumptionAverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_average: Option<EnergyWaterSupplyArrayOfConsumptionAverageComponent>,
    #[serde(rename = "ConsumptionReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_report: Option<EnergyWaterSupplyArrayOfConsumptionReportComponent>,
    #[serde(rename = "EnergyTaxReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_tax_report: Option<EnergyWaterSupplyArrayOfEnergyTaxReportComponent>,
    #[serde(rename = "EnergyWaterConsumptionCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_water_consumption_correction: Option<EnergyWaterSupplyArrayOfEnergyWaterConsumptionCorrectionComponent>,
}

impl AsMut<EnergyWaterSupply> for EnergyWaterSupply {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnergyWaterSupply> for EnergyWaterSupply {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.consumption_average {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EnergyWaterSupply.consumption_average", e));
            }
        }
        if let Some(v) = &self.energy_tax_report {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EnergyWaterSupply.energy_tax_report", e));
            }
        }
        if let Some(v) = &self.consumption_report {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EnergyWaterSupply.consumption_report", e));
            }
        }
        if let Some(v) = &self.energy_water_consumption_correction {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EnergyWaterSupply.energy_water_consumption_correction", e));
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

impl EnergyWaterSupply {
    pub fn title() -> &'static str {
        "Energy Water Supply. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the supply (and therefore consumption) of an amount of energy or water."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            consumption_report: None,
            energy_water_consumption_correction: None,
            consumption_average: None,
            energy_tax_report: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EnergyWaterSupplyArrayOfConsumptionAverageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ConsumptionAverage>,
}

impl AsMut<EnergyWaterSupplyArrayOfConsumptionAverageComponent> for EnergyWaterSupplyArrayOfConsumptionAverageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnergyWaterSupplyArrayOfConsumptionAverageComponent> for EnergyWaterSupplyArrayOfConsumptionAverageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EnergyWaterSupplyArrayOfConsumptionAverageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnergyWaterSupplyArrayOfConsumptionAverageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ConsumptionAverage) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ConsumptionAverage) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ConsumptionAverage> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ConsumptionAverage> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EnergyWaterSupplyArrayOfConsumptionReportComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ConsumptionReport>,
}

impl AsMut<EnergyWaterSupplyArrayOfConsumptionReportComponent> for EnergyWaterSupplyArrayOfConsumptionReportComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnergyWaterSupplyArrayOfConsumptionReportComponent> for EnergyWaterSupplyArrayOfConsumptionReportComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EnergyWaterSupplyArrayOfConsumptionReportComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnergyWaterSupplyArrayOfConsumptionReportComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ConsumptionReport) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ConsumptionReport) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ConsumptionReport> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ConsumptionReport> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EnergyWaterSupplyArrayOfEnergyTaxReportComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EnergyTaxReport>,
}

impl AsMut<EnergyWaterSupplyArrayOfEnergyTaxReportComponent> for EnergyWaterSupplyArrayOfEnergyTaxReportComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnergyWaterSupplyArrayOfEnergyTaxReportComponent> for EnergyWaterSupplyArrayOfEnergyTaxReportComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EnergyWaterSupplyArrayOfEnergyTaxReportComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnergyWaterSupplyArrayOfEnergyTaxReportComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EnergyTaxReport) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EnergyTaxReport) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EnergyTaxReport> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EnergyTaxReport> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EnergyWaterSupplyArrayOfEnergyWaterConsumptionCorrectionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EnergyWaterConsumptionCorrection>,
}

impl AsMut<EnergyWaterSupplyArrayOfEnergyWaterConsumptionCorrectionComponent> for EnergyWaterSupplyArrayOfEnergyWaterConsumptionCorrectionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnergyWaterSupplyArrayOfEnergyWaterConsumptionCorrectionComponent> for EnergyWaterSupplyArrayOfEnergyWaterConsumptionCorrectionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EnergyWaterSupplyArrayOfEnergyWaterConsumptionCorrectionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnergyWaterSupplyArrayOfEnergyWaterConsumptionCorrectionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EnergyWaterConsumptionCorrection) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EnergyWaterConsumptionCorrection) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EnergyWaterConsumptionCorrection> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EnergyWaterConsumptionCorrection> {
        self.items.iter()
    }
}


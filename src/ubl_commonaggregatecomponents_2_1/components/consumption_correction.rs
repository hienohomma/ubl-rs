use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsumptionCorrection {
    #[serde(rename = "ActualTemperatureReductionQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_temperature_reduction_quantity: Option<ConsumptionCorrectionArrayOfActualTemperatureReductionQuantityComponent>,
    #[serde(rename = "ConsumptionEnergyQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_energy_quantity: Option<ConsumptionCorrectionArrayOfConsumptionEnergyQuantityComponent>,
    #[serde(rename = "ConsumptionWaterQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_water_quantity: Option<ConsumptionCorrectionArrayOfConsumptionWaterQuantityComponent>,
    #[serde(rename = "CorrectionAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_amount: Option<ConsumptionCorrectionArrayOfCorrectionAmountComponent>,
    #[serde(rename = "CorrectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_type: Option<ConsumptionCorrectionArrayOfCorrectionTypeComponent>,
    #[serde(rename = "CorrectionTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_type_code: Option<ConsumptionCorrectionArrayOfCorrectionTypeCodeComponent>,
    #[serde(rename = "CorrectionUnitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_unit_amount: Option<ConsumptionCorrectionArrayOfCorrectionUnitAmountComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ConsumptionCorrectionArrayOfDescriptionComponent>,
    #[serde(rename = "DifferenceTemperatureReductionQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difference_temperature_reduction_quantity: Option<ConsumptionCorrectionArrayOfDifferenceTemperatureReductionQuantityComponent>,
    #[serde(rename = "GasPressureQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_pressure_quantity: Option<ConsumptionCorrectionArrayOfGasPressureQuantityComponent>,
    #[serde(rename = "MeterNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_number: Option<ConsumptionCorrectionArrayOfMeterNumberComponent>,
    #[serde(rename = "NormalTemperatureReductionQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_temperature_reduction_quantity: Option<ConsumptionCorrectionArrayOfNormalTemperatureReductionQuantityComponent>,
}

impl AsMut<ConsumptionCorrection> for ConsumptionCorrection {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrection> for ConsumptionCorrection {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.consumption_water_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.consumption_water_quantity", e));
            }
        }
        if let Some(v) = &self.correction_unit_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.correction_unit_amount", e));
            }
        }
        if let Some(v) = &self.consumption_energy_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.consumption_energy_quantity", e));
            }
        }
        if let Some(v) = &self.correction_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.correction_amount", e));
            }
        }
        if let Some(v) = &self.gas_pressure_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.gas_pressure_quantity", e));
            }
        }
        if let Some(v) = &self.correction_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.correction_type", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.description", e));
            }
        }
        if let Some(v) = &self.normal_temperature_reduction_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.normal_temperature_reduction_quantity", e));
            }
        }
        if let Some(v) = &self.actual_temperature_reduction_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.actual_temperature_reduction_quantity", e));
            }
        }
        if let Some(v) = &self.meter_number {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.meter_number", e));
            }
        }
        if let Some(v) = &self.correction_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.correction_type_code", e));
            }
        }
        if let Some(v) = &self.difference_temperature_reduction_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionCorrection.difference_temperature_reduction_quantity", e));
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

impl ConsumptionCorrection {
    pub fn title() -> &'static str {
        "Consumption Correction. Details"
    }
    pub fn description() -> &'static str {
        "The Statement of correction, for examples heating correction."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            correction_amount: None,
            correction_type_code: None,
            consumption_water_quantity: None,
            description: None,
            difference_temperature_reduction_quantity: None,
            correction_type: None,
            consumption_energy_quantity: None,
            gas_pressure_quantity: None,
            correction_unit_amount: None,
            normal_temperature_reduction_quantity: None,
            meter_number: None,
            actual_temperature_reduction_quantity: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionCorrectionArrayOfActualTemperatureReductionQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ActualTemperatureReductionQuantity>,
}

impl AsMut<ConsumptionCorrectionArrayOfActualTemperatureReductionQuantityComponent> for ConsumptionCorrectionArrayOfActualTemperatureReductionQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfActualTemperatureReductionQuantityComponent> for ConsumptionCorrectionArrayOfActualTemperatureReductionQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfActualTemperatureReductionQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfActualTemperatureReductionQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfActualTemperatureReductionQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ActualTemperatureReductionQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ActualTemperatureReductionQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ActualTemperatureReductionQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ActualTemperatureReductionQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionCorrectionArrayOfConsumptionEnergyQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionEnergyQuantity>,
}

impl AsMut<ConsumptionCorrectionArrayOfConsumptionEnergyQuantityComponent> for ConsumptionCorrectionArrayOfConsumptionEnergyQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfConsumptionEnergyQuantityComponent> for ConsumptionCorrectionArrayOfConsumptionEnergyQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfConsumptionEnergyQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfConsumptionEnergyQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfConsumptionEnergyQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumptionEnergyQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumptionEnergyQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumptionEnergyQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumptionEnergyQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionCorrectionArrayOfConsumptionWaterQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionWaterQuantity>,
}

impl AsMut<ConsumptionCorrectionArrayOfConsumptionWaterQuantityComponent> for ConsumptionCorrectionArrayOfConsumptionWaterQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfConsumptionWaterQuantityComponent> for ConsumptionCorrectionArrayOfConsumptionWaterQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfConsumptionWaterQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfConsumptionWaterQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfConsumptionWaterQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumptionWaterQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumptionWaterQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumptionWaterQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumptionWaterQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionCorrectionArrayOfCorrectionAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CorrectionAmount>,
}

impl AsMut<ConsumptionCorrectionArrayOfCorrectionAmountComponent> for ConsumptionCorrectionArrayOfCorrectionAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfCorrectionAmountComponent> for ConsumptionCorrectionArrayOfCorrectionAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfCorrectionAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfCorrectionAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfCorrectionAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CorrectionAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CorrectionAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CorrectionAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CorrectionAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionCorrectionArrayOfCorrectionTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CorrectionType>,
}

impl AsMut<ConsumptionCorrectionArrayOfCorrectionTypeComponent> for ConsumptionCorrectionArrayOfCorrectionTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfCorrectionTypeComponent> for ConsumptionCorrectionArrayOfCorrectionTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfCorrectionTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfCorrectionTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfCorrectionTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CorrectionType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CorrectionType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CorrectionType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CorrectionType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionCorrectionArrayOfCorrectionTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CorrectionTypeCode>,
}

impl AsMut<ConsumptionCorrectionArrayOfCorrectionTypeCodeComponent> for ConsumptionCorrectionArrayOfCorrectionTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfCorrectionTypeCodeComponent> for ConsumptionCorrectionArrayOfCorrectionTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfCorrectionTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfCorrectionTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfCorrectionTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CorrectionTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CorrectionTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CorrectionTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CorrectionTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionCorrectionArrayOfCorrectionUnitAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CorrectionUnitAmount>,
}

impl AsMut<ConsumptionCorrectionArrayOfCorrectionUnitAmountComponent> for ConsumptionCorrectionArrayOfCorrectionUnitAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfCorrectionUnitAmountComponent> for ConsumptionCorrectionArrayOfCorrectionUnitAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfCorrectionUnitAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfCorrectionUnitAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfCorrectionUnitAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CorrectionUnitAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CorrectionUnitAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CorrectionUnitAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CorrectionUnitAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionCorrectionArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ConsumptionCorrectionArrayOfDescriptionComponent> for ConsumptionCorrectionArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfDescriptionComponent> for ConsumptionCorrectionArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfDescriptionComponent {
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
pub struct ConsumptionCorrectionArrayOfDifferenceTemperatureReductionQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DifferenceTemperatureReductionQuantity>,
}

impl AsMut<ConsumptionCorrectionArrayOfDifferenceTemperatureReductionQuantityComponent> for ConsumptionCorrectionArrayOfDifferenceTemperatureReductionQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfDifferenceTemperatureReductionQuantityComponent> for ConsumptionCorrectionArrayOfDifferenceTemperatureReductionQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfDifferenceTemperatureReductionQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfDifferenceTemperatureReductionQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfDifferenceTemperatureReductionQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DifferenceTemperatureReductionQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DifferenceTemperatureReductionQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DifferenceTemperatureReductionQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DifferenceTemperatureReductionQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionCorrectionArrayOfGasPressureQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GasPressureQuantity>,
}

impl AsMut<ConsumptionCorrectionArrayOfGasPressureQuantityComponent> for ConsumptionCorrectionArrayOfGasPressureQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfGasPressureQuantityComponent> for ConsumptionCorrectionArrayOfGasPressureQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfGasPressureQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfGasPressureQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfGasPressureQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GasPressureQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GasPressureQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GasPressureQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GasPressureQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionCorrectionArrayOfMeterNumberComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MeterNumber>,
}

impl AsMut<ConsumptionCorrectionArrayOfMeterNumberComponent> for ConsumptionCorrectionArrayOfMeterNumberComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfMeterNumberComponent> for ConsumptionCorrectionArrayOfMeterNumberComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfMeterNumberComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfMeterNumberComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfMeterNumberComponent {
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
pub struct ConsumptionCorrectionArrayOfNormalTemperatureReductionQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NormalTemperatureReductionQuantity>,
}

impl AsMut<ConsumptionCorrectionArrayOfNormalTemperatureReductionQuantityComponent> for ConsumptionCorrectionArrayOfNormalTemperatureReductionQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionCorrectionArrayOfNormalTemperatureReductionQuantityComponent> for ConsumptionCorrectionArrayOfNormalTemperatureReductionQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfNormalTemperatureReductionQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionCorrectionArrayOfNormalTemperatureReductionQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionCorrectionArrayOfNormalTemperatureReductionQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NormalTemperatureReductionQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NormalTemperatureReductionQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NormalTemperatureReductionQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NormalTemperatureReductionQuantity> {
        self.items.iter()
    }
}


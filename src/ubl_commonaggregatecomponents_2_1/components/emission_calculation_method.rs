use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmissionCalculationMethod {
    #[serde(rename = "CalculationMethodCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_method_code: Option<EmissionCalculationMethodArrayOfCalculationMethodCodeComponent>,
    #[serde(rename = "FullnessIndicationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullness_indication_code: Option<EmissionCalculationMethodArrayOfFullnessIndicationCodeComponent>,
    #[serde(rename = "MeasurementFromLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_from_location: Option<EmissionCalculationMethodArrayOfMeasurementFromLocationComponent>,
    #[serde(rename = "MeasurementToLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_to_location: Option<EmissionCalculationMethodArrayOfMeasurementToLocationComponent>,
}

impl AsMut<EmissionCalculationMethod> for EmissionCalculationMethod {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EmissionCalculationMethod> for EmissionCalculationMethod {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.fullness_indication_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EmissionCalculationMethod.fullness_indication_code", e));
            }
        }
        if let Some(v) = &self.calculation_method_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EmissionCalculationMethod.calculation_method_code", e));
            }
        }
        if let Some(v) = &self.measurement_to_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EmissionCalculationMethod.measurement_to_location", e));
            }
        }
        if let Some(v) = &self.measurement_from_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EmissionCalculationMethod.measurement_from_location", e));
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

impl EmissionCalculationMethod {
    pub fn title() -> &'static str {
        "Emission Calculation Method. Details"
    }
    pub fn description() -> &'static str {
        "A class to define how an environmental emission is calculated."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            measurement_from_location: None,
            measurement_to_location: None,
            calculation_method_code: None,
            fullness_indication_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EmissionCalculationMethodArrayOfCalculationMethodCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CalculationMethodCode>,
}

impl AsMut<EmissionCalculationMethodArrayOfCalculationMethodCodeComponent> for EmissionCalculationMethodArrayOfCalculationMethodCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EmissionCalculationMethodArrayOfCalculationMethodCodeComponent> for EmissionCalculationMethodArrayOfCalculationMethodCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EmissionCalculationMethodArrayOfCalculationMethodCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EmissionCalculationMethodArrayOfCalculationMethodCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EmissionCalculationMethodArrayOfCalculationMethodCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CalculationMethodCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CalculationMethodCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CalculationMethodCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CalculationMethodCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EmissionCalculationMethodArrayOfFullnessIndicationCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FullnessIndicationCode>,
}

impl AsMut<EmissionCalculationMethodArrayOfFullnessIndicationCodeComponent> for EmissionCalculationMethodArrayOfFullnessIndicationCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EmissionCalculationMethodArrayOfFullnessIndicationCodeComponent> for EmissionCalculationMethodArrayOfFullnessIndicationCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EmissionCalculationMethodArrayOfFullnessIndicationCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EmissionCalculationMethodArrayOfFullnessIndicationCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EmissionCalculationMethodArrayOfFullnessIndicationCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FullnessIndicationCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FullnessIndicationCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FullnessIndicationCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FullnessIndicationCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EmissionCalculationMethodArrayOfMeasurementFromLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MeasurementFromLocation>,
}

impl AsMut<EmissionCalculationMethodArrayOfMeasurementFromLocationComponent> for EmissionCalculationMethodArrayOfMeasurementFromLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EmissionCalculationMethodArrayOfMeasurementFromLocationComponent> for EmissionCalculationMethodArrayOfMeasurementFromLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EmissionCalculationMethodArrayOfMeasurementFromLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EmissionCalculationMethodArrayOfMeasurementFromLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EmissionCalculationMethodArrayOfMeasurementFromLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MeasurementFromLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MeasurementFromLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MeasurementFromLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MeasurementFromLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EmissionCalculationMethodArrayOfMeasurementToLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MeasurementToLocation>,
}

impl AsMut<EmissionCalculationMethodArrayOfMeasurementToLocationComponent> for EmissionCalculationMethodArrayOfMeasurementToLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EmissionCalculationMethodArrayOfMeasurementToLocationComponent> for EmissionCalculationMethodArrayOfMeasurementToLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EmissionCalculationMethodArrayOfMeasurementToLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EmissionCalculationMethodArrayOfMeasurementToLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EmissionCalculationMethodArrayOfMeasurementToLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MeasurementToLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MeasurementToLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MeasurementToLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MeasurementToLocation> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentalEmission {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<EnvironmentalEmissionArrayOfDescriptionComponent>,
    #[serde(rename = "EmissionCalculationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_calculation_method: Option<EnvironmentalEmissionArrayOfEmissionCalculationMethodComponent>,
    #[serde(rename = "EnvironmentalEmissionTypeCode")]
    pub environmental_emission_type_code: EnvironmentalEmissionArrayOfEnvironmentalEmissionTypeCodeComponent,
    #[serde(rename = "ValueMeasure")]
    pub value_measure: EnvironmentalEmissionArrayOfValueMeasureComponent,
}

impl AsMut<EnvironmentalEmission> for EnvironmentalEmission {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnvironmentalEmission> for EnvironmentalEmission {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EnvironmentalEmission.description", e));
            }
        }
        if let Some(v) = &self.emission_calculation_method {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EnvironmentalEmission.emission_calculation_method", e));
            }
        }
        if let Err(e) = self.value_measure.validate() {
            return Err(UblError::component("EnvironmentalEmission.value_measure", e));
        }
        if let Err(e) = self.environmental_emission_type_code.validate() {
            return Err(UblError::component("EnvironmentalEmission.environmental_emission_type_code", e));
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

impl EnvironmentalEmission {
    pub fn title() -> &'static str {
        "Environmental Emission. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an environmental emission."
    }
    pub fn new(environmental_emission_type_code: EnvironmentalEmissionArrayOfEnvironmentalEmissionTypeCodeComponent, value_measure: EnvironmentalEmissionArrayOfValueMeasureComponent) -> Component<Self> {
        Component(Self {
            emission_calculation_method: None,
            value_measure,
            environmental_emission_type_code,
            description: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EnvironmentalEmissionArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<EnvironmentalEmissionArrayOfDescriptionComponent> for EnvironmentalEmissionArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnvironmentalEmissionArrayOfDescriptionComponent> for EnvironmentalEmissionArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EnvironmentalEmissionArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnvironmentalEmissionArrayOfDescriptionComponent {
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
pub struct EnvironmentalEmissionArrayOfEmissionCalculationMethodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EmissionCalculationMethod>,
}

impl AsMut<EnvironmentalEmissionArrayOfEmissionCalculationMethodComponent> for EnvironmentalEmissionArrayOfEmissionCalculationMethodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnvironmentalEmissionArrayOfEmissionCalculationMethodComponent> for EnvironmentalEmissionArrayOfEmissionCalculationMethodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EnvironmentalEmissionArrayOfEmissionCalculationMethodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnvironmentalEmissionArrayOfEmissionCalculationMethodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EmissionCalculationMethod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EmissionCalculationMethod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EmissionCalculationMethod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EmissionCalculationMethod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EnvironmentalEmissionArrayOfEnvironmentalEmissionTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EnvironmentalEmissionTypeCode>,
}

impl AsMut<EnvironmentalEmissionArrayOfEnvironmentalEmissionTypeCodeComponent> for EnvironmentalEmissionArrayOfEnvironmentalEmissionTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnvironmentalEmissionArrayOfEnvironmentalEmissionTypeCodeComponent> for EnvironmentalEmissionArrayOfEnvironmentalEmissionTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EnvironmentalEmissionArrayOfEnvironmentalEmissionTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EnvironmentalEmissionArrayOfEnvironmentalEmissionTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnvironmentalEmissionArrayOfEnvironmentalEmissionTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EnvironmentalEmissionTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EnvironmentalEmissionTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EnvironmentalEmissionTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EnvironmentalEmissionTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EnvironmentalEmissionArrayOfValueMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValueMeasure>,
}

impl AsMut<EnvironmentalEmissionArrayOfValueMeasureComponent> for EnvironmentalEmissionArrayOfValueMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnvironmentalEmissionArrayOfValueMeasureComponent> for EnvironmentalEmissionArrayOfValueMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EnvironmentalEmissionArrayOfValueMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EnvironmentalEmissionArrayOfValueMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnvironmentalEmissionArrayOfValueMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValueMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValueMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValueMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValueMeasure> {
        self.items.iter()
    }
}


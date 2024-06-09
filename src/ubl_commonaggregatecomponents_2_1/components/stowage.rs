use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stowage {
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<StowageArrayOfLocationComponent>,
    #[serde(rename = "LocationID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<StowageArrayOfLocationIDComponent>,
    #[serde(rename = "MeasurementDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_dimension: Option<StowageArrayOfMeasurementDimensionComponent>,
}

impl AsMut<Stowage> for Stowage {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Stowage> for Stowage {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.location_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Stowage.location_id", e));
            }
        }
        if let Some(v) = &self.measurement_dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Stowage.measurement_dimension", e));
            }
        }
        if let Some(v) = &self.location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Stowage.location", e));
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

impl Stowage {
    pub fn title() -> &'static str {
        "Stowage. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a location on board a means of transport where specified goods or transport equipment have been stowed or are to be stowed."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            location: None,
            location_id: None,
            measurement_dimension: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StowageArrayOfLocationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Location>,
}

impl AsMut<StowageArrayOfLocationComponent> for StowageArrayOfLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StowageArrayOfLocationComponent> for StowageArrayOfLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StowageArrayOfLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StowageArrayOfLocationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Location) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Location) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Location> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Location> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StowageArrayOfLocationIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LocationID>,
}

impl AsMut<StowageArrayOfLocationIDComponent> for StowageArrayOfLocationIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StowageArrayOfLocationIDComponent> for StowageArrayOfLocationIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StowageArrayOfLocationIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StowageArrayOfLocationIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StowageArrayOfLocationIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LocationID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LocationID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LocationID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LocationID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StowageArrayOfMeasurementDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension>,
}

impl AsMut<StowageArrayOfMeasurementDimensionComponent> for StowageArrayOfMeasurementDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StowageArrayOfMeasurementDimensionComponent> for StowageArrayOfMeasurementDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StowageArrayOfMeasurementDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StowageArrayOfMeasurementDimensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension> {
        self.items.iter()
    }
}


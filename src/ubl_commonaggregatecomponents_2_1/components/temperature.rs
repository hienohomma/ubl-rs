use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Temperature {
    #[serde(rename = "AttributeID")]
    pub attribute_id: TemperatureArrayOfAttributeIDComponent,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TemperatureArrayOfDescriptionComponent>,
    #[serde(rename = "Measure")]
    pub measure: TemperatureArrayOfMeasureComponent,
}

impl AsMut<Temperature> for Temperature {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Temperature> for Temperature {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Temperature.description", e));
            }
        }
        if let Err(e) = self.measure.validate() {
            return Err(UblError::component("Temperature.measure", e));
        }
        if let Err(e) = self.attribute_id.validate() {
            return Err(UblError::component("Temperature.attribute_id", e));
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

impl Temperature {
    pub fn title() -> &'static str {
        "Temperature. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a measurement of temperature."
    }
    pub fn new(attribute_id: TemperatureArrayOfAttributeIDComponent, measure: TemperatureArrayOfMeasureComponent) -> Component<Self> {
        Component(Self {
            description: None,
            measure,
            attribute_id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TemperatureArrayOfAttributeIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AttributeID>,
}

impl AsMut<TemperatureArrayOfAttributeIDComponent> for TemperatureArrayOfAttributeIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TemperatureArrayOfAttributeIDComponent> for TemperatureArrayOfAttributeIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TemperatureArrayOfAttributeIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TemperatureArrayOfAttributeIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TemperatureArrayOfAttributeIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AttributeID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AttributeID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AttributeID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AttributeID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TemperatureArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<TemperatureArrayOfDescriptionComponent> for TemperatureArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TemperatureArrayOfDescriptionComponent> for TemperatureArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TemperatureArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TemperatureArrayOfDescriptionComponent {
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
pub struct TemperatureArrayOfMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Measure>,
}

impl AsMut<TemperatureArrayOfMeasureComponent> for TemperatureArrayOfMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TemperatureArrayOfMeasureComponent> for TemperatureArrayOfMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TemperatureArrayOfMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TemperatureArrayOfMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TemperatureArrayOfMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Measure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Measure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Measure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Measure> {
        self.items.iter()
    }
}


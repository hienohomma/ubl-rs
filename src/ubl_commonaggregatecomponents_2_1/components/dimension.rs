use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dimension {
    #[serde(rename = "AttributeID")]
    pub attribute_id: DimensionArrayOfAttributeIDComponent,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<DimensionArrayOfDescriptionComponent>,
    #[serde(rename = "MaximumMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_measure: Option<DimensionArrayOfMaximumMeasureComponent>,
    #[serde(rename = "Measure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure: Option<DimensionArrayOfMeasureComponent>,
    #[serde(rename = "MinimumMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_measure: Option<DimensionArrayOfMinimumMeasureComponent>,
}

impl AsMut<Dimension> for Dimension {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Dimension> for Dimension {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Dimension.measure", e));
            }
        }
        if let Err(e) = self.attribute_id.validate() {
            return Err(UblError::component("Dimension.attribute_id", e));
        }
        if let Some(v) = &self.maximum_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Dimension.maximum_measure", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Dimension.description", e));
            }
        }
        if let Some(v) = &self.minimum_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Dimension.minimum_measure", e));
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

impl Dimension {
    pub fn title() -> &'static str {
        "Dimension. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a measurable dimension (length, mass, weight, volume, or area) of an item."
    }
    pub fn new(attribute_id: DimensionArrayOfAttributeIDComponent) -> Component<Self> {
        Component(Self {
            minimum_measure: None,
            attribute_id,
            description: None,
            maximum_measure: None,
            measure: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DimensionArrayOfAttributeIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AttributeID>,
}

impl AsMut<DimensionArrayOfAttributeIDComponent> for DimensionArrayOfAttributeIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DimensionArrayOfAttributeIDComponent> for DimensionArrayOfAttributeIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DimensionArrayOfAttributeIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DimensionArrayOfAttributeIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DimensionArrayOfAttributeIDComponent {
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
pub struct DimensionArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<DimensionArrayOfDescriptionComponent> for DimensionArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DimensionArrayOfDescriptionComponent> for DimensionArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DimensionArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DimensionArrayOfDescriptionComponent {
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
pub struct DimensionArrayOfMaximumMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumMeasure>,
}

impl AsMut<DimensionArrayOfMaximumMeasureComponent> for DimensionArrayOfMaximumMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DimensionArrayOfMaximumMeasureComponent> for DimensionArrayOfMaximumMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DimensionArrayOfMaximumMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DimensionArrayOfMaximumMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DimensionArrayOfMaximumMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DimensionArrayOfMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Measure>,
}

impl AsMut<DimensionArrayOfMeasureComponent> for DimensionArrayOfMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DimensionArrayOfMeasureComponent> for DimensionArrayOfMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DimensionArrayOfMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DimensionArrayOfMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DimensionArrayOfMeasureComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DimensionArrayOfMinimumMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumMeasure>,
}

impl AsMut<DimensionArrayOfMinimumMeasureComponent> for DimensionArrayOfMinimumMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DimensionArrayOfMinimumMeasureComponent> for DimensionArrayOfMinimumMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DimensionArrayOfMinimumMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DimensionArrayOfMinimumMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DimensionArrayOfMinimumMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumMeasure> {
        self.items.iter()
    }
}


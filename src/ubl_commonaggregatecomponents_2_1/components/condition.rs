use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Condition {
    #[serde(rename = "AttributeID")]
    pub attribute_id: ConditionArrayOfAttributeIDComponent,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ConditionArrayOfDescriptionComponent>,
    #[serde(rename = "MaximumMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_measure: Option<ConditionArrayOfMaximumMeasureComponent>,
    #[serde(rename = "Measure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure: Option<ConditionArrayOfMeasureComponent>,
    #[serde(rename = "MinimumMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_measure: Option<ConditionArrayOfMinimumMeasureComponent>,
}

impl AsMut<Condition> for Condition {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Condition> for Condition {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.maximum_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Condition.maximum_measure", e));
            }
        }
        if let Some(v) = &self.measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Condition.measure", e));
            }
        }
        if let Err(e) = self.attribute_id.validate() {
            return Err(UblError::component("Condition.attribute_id", e));
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Condition.description", e));
            }
        }
        if let Some(v) = &self.minimum_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Condition.minimum_measure", e));
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

impl Condition {
    pub fn title() -> &'static str {
        "Condition. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a measurable condition of an object."
    }
    pub fn new(attribute_id: ConditionArrayOfAttributeIDComponent) -> Component<Self> {
        Component(Self {
            measure: None,
            description: None,
            maximum_measure: None,
            attribute_id,
            minimum_measure: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConditionArrayOfAttributeIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AttributeID>,
}

impl AsMut<ConditionArrayOfAttributeIDComponent> for ConditionArrayOfAttributeIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConditionArrayOfAttributeIDComponent> for ConditionArrayOfAttributeIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConditionArrayOfAttributeIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConditionArrayOfAttributeIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConditionArrayOfAttributeIDComponent {
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
pub struct ConditionArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ConditionArrayOfDescriptionComponent> for ConditionArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConditionArrayOfDescriptionComponent> for ConditionArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConditionArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConditionArrayOfDescriptionComponent {
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
pub struct ConditionArrayOfMaximumMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumMeasure>,
}

impl AsMut<ConditionArrayOfMaximumMeasureComponent> for ConditionArrayOfMaximumMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConditionArrayOfMaximumMeasureComponent> for ConditionArrayOfMaximumMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConditionArrayOfMaximumMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConditionArrayOfMaximumMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConditionArrayOfMaximumMeasureComponent {
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
pub struct ConditionArrayOfMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Measure>,
}

impl AsMut<ConditionArrayOfMeasureComponent> for ConditionArrayOfMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConditionArrayOfMeasureComponent> for ConditionArrayOfMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConditionArrayOfMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConditionArrayOfMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConditionArrayOfMeasureComponent {
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
pub struct ConditionArrayOfMinimumMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumMeasure>,
}

impl AsMut<ConditionArrayOfMinimumMeasureComponent> for ConditionArrayOfMinimumMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConditionArrayOfMinimumMeasureComponent> for ConditionArrayOfMinimumMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConditionArrayOfMinimumMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConditionArrayOfMinimumMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConditionArrayOfMinimumMeasureComponent {
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


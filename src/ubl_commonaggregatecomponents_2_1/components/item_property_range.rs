use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemPropertyRange {
    #[serde(rename = "MaximumValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_value: Option<ItemPropertyRangeArrayOfMaximumValueComponent>,
    #[serde(rename = "MinimumValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_value: Option<ItemPropertyRangeArrayOfMinimumValueComponent>,
}

impl AsMut<ItemPropertyRange> for ItemPropertyRange {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyRange> for ItemPropertyRange {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.maximum_value {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemPropertyRange.maximum_value", e));
            }
        }
        if let Some(v) = &self.minimum_value {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemPropertyRange.minimum_value", e));
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

impl ItemPropertyRange {
    pub fn title() -> &'static str {
        "Item Property Range. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a range of values for an item property."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            maximum_value: None,
            minimum_value: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyRangeArrayOfMaximumValueComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumValue>,
}

impl AsMut<ItemPropertyRangeArrayOfMaximumValueComponent> for ItemPropertyRangeArrayOfMaximumValueComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyRangeArrayOfMaximumValueComponent> for ItemPropertyRangeArrayOfMaximumValueComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyRangeArrayOfMaximumValueComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyRangeArrayOfMaximumValueComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyRangeArrayOfMaximumValueComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumValue) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumValue) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumValue> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumValue> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyRangeArrayOfMinimumValueComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumValue>,
}

impl AsMut<ItemPropertyRangeArrayOfMinimumValueComponent> for ItemPropertyRangeArrayOfMinimumValueComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyRangeArrayOfMinimumValueComponent> for ItemPropertyRangeArrayOfMinimumValueComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyRangeArrayOfMinimumValueComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyRangeArrayOfMinimumValueComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyRangeArrayOfMinimumValueComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumValue) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumValue) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumValue> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumValue> {
        self.items.iter()
    }
}


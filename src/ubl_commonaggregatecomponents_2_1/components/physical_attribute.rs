use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PhysicalAttribute {
    #[serde(rename = "AttributeID")]
    pub attribute_id: PhysicalAttributeArrayOfAttributeIDComponent,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<PhysicalAttributeArrayOfDescriptionComponent>,
    #[serde(rename = "DescriptionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_code: Option<PhysicalAttributeArrayOfDescriptionCodeComponent>,
    #[serde(rename = "PositionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_code: Option<PhysicalAttributeArrayOfPositionCodeComponent>,
}

impl AsMut<PhysicalAttribute> for PhysicalAttribute {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PhysicalAttribute> for PhysicalAttribute {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.attribute_id.validate() {
            return Err(UblError::component("PhysicalAttribute.attribute_id", e));
        }
        if let Some(v) = &self.description_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PhysicalAttribute.description_code", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PhysicalAttribute.description", e));
            }
        }
        if let Some(v) = &self.position_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PhysicalAttribute.position_code", e));
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

impl PhysicalAttribute {
    pub fn title() -> &'static str {
        "Physical Attribute. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a physical attribute."
    }
    pub fn new(attribute_id: PhysicalAttributeArrayOfAttributeIDComponent) -> Component<Self> {
        Component(Self {
            description: None,
            position_code: None,
            attribute_id,
            description_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PhysicalAttributeArrayOfAttributeIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AttributeID>,
}

impl AsMut<PhysicalAttributeArrayOfAttributeIDComponent> for PhysicalAttributeArrayOfAttributeIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PhysicalAttributeArrayOfAttributeIDComponent> for PhysicalAttributeArrayOfAttributeIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PhysicalAttributeArrayOfAttributeIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PhysicalAttributeArrayOfAttributeIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PhysicalAttributeArrayOfAttributeIDComponent {
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
pub struct PhysicalAttributeArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<PhysicalAttributeArrayOfDescriptionComponent> for PhysicalAttributeArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PhysicalAttributeArrayOfDescriptionComponent> for PhysicalAttributeArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PhysicalAttributeArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PhysicalAttributeArrayOfDescriptionComponent {
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
pub struct PhysicalAttributeArrayOfDescriptionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DescriptionCode>,
}

impl AsMut<PhysicalAttributeArrayOfDescriptionCodeComponent> for PhysicalAttributeArrayOfDescriptionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PhysicalAttributeArrayOfDescriptionCodeComponent> for PhysicalAttributeArrayOfDescriptionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PhysicalAttributeArrayOfDescriptionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PhysicalAttributeArrayOfDescriptionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PhysicalAttributeArrayOfDescriptionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DescriptionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DescriptionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DescriptionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DescriptionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PhysicalAttributeArrayOfPositionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PositionCode>,
}

impl AsMut<PhysicalAttributeArrayOfPositionCodeComponent> for PhysicalAttributeArrayOfPositionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PhysicalAttributeArrayOfPositionCodeComponent> for PhysicalAttributeArrayOfPositionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PhysicalAttributeArrayOfPositionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PhysicalAttributeArrayOfPositionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PhysicalAttributeArrayOfPositionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PositionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PositionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PositionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PositionCode> {
        self.items.iter()
    }
}


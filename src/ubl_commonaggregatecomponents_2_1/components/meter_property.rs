use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeterProperty {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<MeterPropertyArrayOfNameComponent>,
    #[serde(rename = "NameCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_code: Option<MeterPropertyArrayOfNameCodeComponent>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MeterPropertyArrayOfValueComponent>,
    #[serde(rename = "ValueQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_qualifier: Option<MeterPropertyArrayOfValueQualifierComponent>,
    #[serde(rename = "ValueQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_quantity: Option<MeterPropertyArrayOfValueQuantityComponent>,
}

impl AsMut<MeterProperty> for MeterProperty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterProperty> for MeterProperty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterProperty.name", e));
            }
        }
        if let Some(v) = &self.value {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterProperty.value", e));
            }
        }
        if let Some(v) = &self.value_qualifier {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterProperty.value_qualifier", e));
            }
        }
        if let Some(v) = &self.value_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterProperty.value_quantity", e));
            }
        }
        if let Some(v) = &self.name_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterProperty.name_code", e));
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

impl MeterProperty {
    pub fn title() -> &'static str {
        "Meter Property. Details"
    }
    pub fn description() -> &'static str {
        "The name of this meter property."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            value_quantity: None,
            value_qualifier: None,
            name_code: None,
            value: None,
            name: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterPropertyArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<MeterPropertyArrayOfNameComponent> for MeterPropertyArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterPropertyArrayOfNameComponent> for MeterPropertyArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterPropertyArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterPropertyArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterPropertyArrayOfNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Name) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Name) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterPropertyArrayOfNameCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NameCode>,
}

impl AsMut<MeterPropertyArrayOfNameCodeComponent> for MeterPropertyArrayOfNameCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterPropertyArrayOfNameCodeComponent> for MeterPropertyArrayOfNameCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterPropertyArrayOfNameCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterPropertyArrayOfNameCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterPropertyArrayOfNameCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NameCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NameCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NameCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NameCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterPropertyArrayOfValueComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Value>,
}

impl AsMut<MeterPropertyArrayOfValueComponent> for MeterPropertyArrayOfValueComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterPropertyArrayOfValueComponent> for MeterPropertyArrayOfValueComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterPropertyArrayOfValueComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterPropertyArrayOfValueComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterPropertyArrayOfValueComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Value) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Value) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Value> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Value> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterPropertyArrayOfValueQualifierComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValueQualifier>,
}

impl AsMut<MeterPropertyArrayOfValueQualifierComponent> for MeterPropertyArrayOfValueQualifierComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterPropertyArrayOfValueQualifierComponent> for MeterPropertyArrayOfValueQualifierComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("MeterPropertyArrayOfValueQualifierComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterPropertyArrayOfValueQualifierComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValueQualifier) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValueQualifier) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValueQualifier> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValueQualifier> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterPropertyArrayOfValueQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValueQuantity>,
}

impl AsMut<MeterPropertyArrayOfValueQuantityComponent> for MeterPropertyArrayOfValueQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterPropertyArrayOfValueQuantityComponent> for MeterPropertyArrayOfValueQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterPropertyArrayOfValueQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterPropertyArrayOfValueQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterPropertyArrayOfValueQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValueQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValueQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValueQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValueQuantity> {
        self.items.iter()
    }
}


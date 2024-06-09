use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityProperty {
    #[serde(rename = "Name")]
    pub name: ActivityPropertyArrayOfNameComponent,
    #[serde(rename = "Value")]
    pub value: ActivityPropertyArrayOfValueComponent,
}

impl AsMut<ActivityProperty> for ActivityProperty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityProperty> for ActivityProperty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.value.validate() {
            return Err(UblError::component("ActivityProperty.value", e));
        }
        if let Err(e) = self.name.validate() {
            return Err(UblError::component("ActivityProperty.name", e));
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

impl ActivityProperty {
    pub fn title() -> &'static str {
        "Activity Property. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a name/value pair for a property of an inventory planning activity."
    }
    pub fn new(name: ActivityPropertyArrayOfNameComponent, value: ActivityPropertyArrayOfValueComponent) -> Component<Self> {
        Component(Self {
            value,
            name,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ActivityPropertyArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<ActivityPropertyArrayOfNameComponent> for ActivityPropertyArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityPropertyArrayOfNameComponent> for ActivityPropertyArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ActivityPropertyArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ActivityPropertyArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ActivityPropertyArrayOfNameComponent {
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
pub struct ActivityPropertyArrayOfValueComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Value>,
}

impl AsMut<ActivityPropertyArrayOfValueComponent> for ActivityPropertyArrayOfValueComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityPropertyArrayOfValueComponent> for ActivityPropertyArrayOfValueComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ActivityPropertyArrayOfValueComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ActivityPropertyArrayOfValueComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ActivityPropertyArrayOfValueComponent {
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


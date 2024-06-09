use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartyName {
    #[serde(rename = "Name")]
    pub name: PartyNameArrayOfNameComponent,
}

impl AsMut<PartyName> for PartyName {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyName> for PartyName {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.name.validate() {
            return Err(UblError::component("PartyName.name", e));
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

impl PartyName {
    pub fn title() -> &'static str {
        "Party Name. Details"
    }
    pub fn description() -> &'static str {
        "A class for defining the name of a party."
    }
    pub fn new(name: PartyNameArrayOfNameComponent) -> Component<Self> {
        Component(Self {
            name,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyNameArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<PartyNameArrayOfNameComponent> for PartyNameArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyNameArrayOfNameComponent> for PartyNameArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyNameArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyNameArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyNameArrayOfNameComponent {
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


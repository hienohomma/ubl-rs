use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddressLine {
    #[serde(rename = "Line")]
    pub line: AddressLineArrayOfLineComponent,
}

impl AsMut<AddressLine> for AddressLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressLine> for AddressLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.line.validate() {
            return Err(UblError::component("AddressLine.line", e));
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

impl AddressLine {
    pub fn title() -> &'static str {
        "Address Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define an unstructured address line."
    }
    pub fn new(line: AddressLineArrayOfLineComponent) -> Component<Self> {
        Component(Self {
            line,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressLineArrayOfLineComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Line>,
}

impl AsMut<AddressLineArrayOfLineComponent> for AddressLineArrayOfLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressLineArrayOfLineComponent> for AddressLineArrayOfLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressLineArrayOfLineComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressLineArrayOfLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressLineArrayOfLineComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Line) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Line) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Line> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Line> {
        self.items.iter()
    }
}


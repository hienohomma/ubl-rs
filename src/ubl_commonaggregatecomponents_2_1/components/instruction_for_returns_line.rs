use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstructionForReturnsLine {
    #[serde(rename = "ID")]
    pub id: InstructionForReturnsLineArrayOfIDComponent,
    #[serde(rename = "Item")]
    pub item: InstructionForReturnsLineArrayOfItemComponent,
    #[serde(rename = "ManufacturerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_party: Option<InstructionForReturnsLineArrayOfManufacturerPartyComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<InstructionForReturnsLineArrayOfNoteComponent>,
    #[serde(rename = "Quantity")]
    pub quantity: InstructionForReturnsLineArrayOfQuantityComponent,
}

impl AsMut<InstructionForReturnsLine> for InstructionForReturnsLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InstructionForReturnsLine> for InstructionForReturnsLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.quantity.validate() {
            return Err(UblError::component("InstructionForReturnsLine.quantity", e));
        }
        if let Some(v) = &self.manufacturer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InstructionForReturnsLine.manufacturer_party", e));
            }
        }
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("InstructionForReturnsLine.item", e));
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InstructionForReturnsLine.note", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("InstructionForReturnsLine.id", e));
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

impl InstructionForReturnsLine {
    pub fn title() -> &'static str {
        "Instruction For Returns Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in an Instruction for Returns."
    }
    pub fn new(id: InstructionForReturnsLineArrayOfIDComponent, item: InstructionForReturnsLineArrayOfItemComponent, quantity: InstructionForReturnsLineArrayOfQuantityComponent) -> Component<Self> {
        Component(Self {
            item,
            manufacturer_party: None,
            note: None,
            id,
            quantity,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InstructionForReturnsLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<InstructionForReturnsLineArrayOfIDComponent> for InstructionForReturnsLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InstructionForReturnsLineArrayOfIDComponent> for InstructionForReturnsLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InstructionForReturnsLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InstructionForReturnsLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InstructionForReturnsLineArrayOfIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InstructionForReturnsLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<InstructionForReturnsLineArrayOfItemComponent> for InstructionForReturnsLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InstructionForReturnsLineArrayOfItemComponent> for InstructionForReturnsLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InstructionForReturnsLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InstructionForReturnsLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InstructionForReturnsLineArrayOfItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Item) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Item) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InstructionForReturnsLineArrayOfManufacturerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ManufacturerParty>,
}

impl AsMut<InstructionForReturnsLineArrayOfManufacturerPartyComponent> for InstructionForReturnsLineArrayOfManufacturerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InstructionForReturnsLineArrayOfManufacturerPartyComponent> for InstructionForReturnsLineArrayOfManufacturerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InstructionForReturnsLineArrayOfManufacturerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InstructionForReturnsLineArrayOfManufacturerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InstructionForReturnsLineArrayOfManufacturerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ManufacturerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ManufacturerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ManufacturerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ManufacturerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InstructionForReturnsLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<InstructionForReturnsLineArrayOfNoteComponent> for InstructionForReturnsLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InstructionForReturnsLineArrayOfNoteComponent> for InstructionForReturnsLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InstructionForReturnsLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InstructionForReturnsLineArrayOfNoteComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Note) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Note) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InstructionForReturnsLineArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<InstructionForReturnsLineArrayOfQuantityComponent> for InstructionForReturnsLineArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InstructionForReturnsLineArrayOfQuantityComponent> for InstructionForReturnsLineArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InstructionForReturnsLineArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InstructionForReturnsLineArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InstructionForReturnsLineArrayOfQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Quantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Quantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter()
    }
}


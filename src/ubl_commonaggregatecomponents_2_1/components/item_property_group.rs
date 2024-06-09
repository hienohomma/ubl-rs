use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemPropertyGroup {
    #[serde(rename = "ID")]
    pub id: ItemPropertyGroupArrayOfIDComponent,
    #[serde(rename = "ImportanceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance_code: Option<ItemPropertyGroupArrayOfImportanceCodeComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ItemPropertyGroupArrayOfNameComponent>,
}

impl AsMut<ItemPropertyGroup> for ItemPropertyGroup {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyGroup> for ItemPropertyGroup {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemPropertyGroup.name", e));
            }
        }
        if let Some(v) = &self.importance_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemPropertyGroup.importance_code", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ItemPropertyGroup.id", e));
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

impl ItemPropertyGroup {
    pub fn title() -> &'static str {
        "Item Property Group. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a property group or classification."
    }
    pub fn new(id: ItemPropertyGroupArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            id,
            importance_code: None,
            name: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyGroupArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ItemPropertyGroupArrayOfIDComponent> for ItemPropertyGroupArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyGroupArrayOfIDComponent> for ItemPropertyGroupArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyGroupArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyGroupArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyGroupArrayOfIDComponent {
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
pub struct ItemPropertyGroupArrayOfImportanceCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ImportanceCode>,
}

impl AsMut<ItemPropertyGroupArrayOfImportanceCodeComponent> for ItemPropertyGroupArrayOfImportanceCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyGroupArrayOfImportanceCodeComponent> for ItemPropertyGroupArrayOfImportanceCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyGroupArrayOfImportanceCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyGroupArrayOfImportanceCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyGroupArrayOfImportanceCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ImportanceCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ImportanceCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ImportanceCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ImportanceCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyGroupArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<ItemPropertyGroupArrayOfNameComponent> for ItemPropertyGroupArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyGroupArrayOfNameComponent> for ItemPropertyGroupArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyGroupArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyGroupArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyGroupArrayOfNameComponent {
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


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassificationCategory {
    #[serde(rename = "CategorizesClassificationCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorizes_classification_category: Option<ClassificationCategoryArrayOfCategorizesClassificationCategoryComponent>,
    #[serde(rename = "CodeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_value: Option<ClassificationCategoryArrayOfCodeValueComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ClassificationCategoryArrayOfDescriptionComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ClassificationCategoryArrayOfNameComponent>,
}

impl AsMut<ClassificationCategory> for ClassificationCategory {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationCategory> for ClassificationCategory {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationCategory.name", e));
            }
        }
        if let Some(v) = &self.code_value {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationCategory.code_value", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationCategory.description", e));
            }
        }
        if let Some(v) = &self.categorizes_classification_category {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationCategory.categorizes_classification_category", e));
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

impl ClassificationCategory {
    pub fn title() -> &'static str {
        "Classification Category. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a category within a classification scheme."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            name: None,
            categorizes_classification_category: None,
            description: None,
            code_value: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationCategoryArrayOfCategorizesClassificationCategoryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CategorizesClassificationCategory>,
}

impl AsMut<ClassificationCategoryArrayOfCategorizesClassificationCategoryComponent> for ClassificationCategoryArrayOfCategorizesClassificationCategoryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationCategoryArrayOfCategorizesClassificationCategoryComponent> for ClassificationCategoryArrayOfCategorizesClassificationCategoryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ClassificationCategoryArrayOfCategorizesClassificationCategoryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationCategoryArrayOfCategorizesClassificationCategoryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CategorizesClassificationCategory) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CategorizesClassificationCategory) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CategorizesClassificationCategory> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CategorizesClassificationCategory> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationCategoryArrayOfCodeValueComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CodeValue>,
}

impl AsMut<ClassificationCategoryArrayOfCodeValueComponent> for ClassificationCategoryArrayOfCodeValueComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationCategoryArrayOfCodeValueComponent> for ClassificationCategoryArrayOfCodeValueComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationCategoryArrayOfCodeValueComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationCategoryArrayOfCodeValueComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationCategoryArrayOfCodeValueComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CodeValue) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CodeValue) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CodeValue> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CodeValue> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationCategoryArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ClassificationCategoryArrayOfDescriptionComponent> for ClassificationCategoryArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationCategoryArrayOfDescriptionComponent> for ClassificationCategoryArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ClassificationCategoryArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationCategoryArrayOfDescriptionComponent {
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
pub struct ClassificationCategoryArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<ClassificationCategoryArrayOfNameComponent> for ClassificationCategoryArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationCategoryArrayOfNameComponent> for ClassificationCategoryArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationCategoryArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationCategoryArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationCategoryArrayOfNameComponent {
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


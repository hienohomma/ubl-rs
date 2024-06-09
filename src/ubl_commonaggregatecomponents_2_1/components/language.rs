use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Language {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LanguageArrayOfIDComponent>,
    #[serde(rename = "LocaleCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_code: Option<LanguageArrayOfLocaleCodeComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<LanguageArrayOfNameComponent>,
}

impl AsMut<Language> for Language {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Language> for Language {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.locale_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Language.locale_code", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Language.id", e));
            }
        }
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Language.name", e));
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

impl Language {
    pub fn title() -> &'static str {
        "Language. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a language."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            id: None,
            locale_code: None,
            name: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LanguageArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<LanguageArrayOfIDComponent> for LanguageArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LanguageArrayOfIDComponent> for LanguageArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LanguageArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LanguageArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LanguageArrayOfIDComponent {
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
pub struct LanguageArrayOfLocaleCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LocaleCode>,
}

impl AsMut<LanguageArrayOfLocaleCodeComponent> for LanguageArrayOfLocaleCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LanguageArrayOfLocaleCodeComponent> for LanguageArrayOfLocaleCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LanguageArrayOfLocaleCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LanguageArrayOfLocaleCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LanguageArrayOfLocaleCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LocaleCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LocaleCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LocaleCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LocaleCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LanguageArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<LanguageArrayOfNameComponent> for LanguageArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LanguageArrayOfNameComponent> for LanguageArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LanguageArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LanguageArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LanguageArrayOfNameComponent {
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


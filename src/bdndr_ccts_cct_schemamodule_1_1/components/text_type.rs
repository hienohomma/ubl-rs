use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextType {
    #[serde(rename = "_")]
    pub _uc: String,
    #[serde(rename = "languageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<String>,
    #[serde(rename = "languageLocaleID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_locale_id: Option<String>,
}

impl AsMut<TextType> for TextType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TextType> for TextType {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.language_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("TextType.language_id"))
            }
        }
        if let Some(v) = &self.language_locale_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("TextType.language_locale_id"))
            }
        }
        if self._uc.is_empty() {
            return Err(UblError::empty("TextType._uc"))
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

impl TextType {
    pub fn title() -> &'static str {
        "Text. Type"
    }
    pub fn description() -> &'static str {
        "A character string (i.e. a finite set of characters) generally in the form of words of a language."
    }
    pub fn new<A>(_uc: A) -> Component<Self> where A: Into<String> {
        Component(Self {
            _uc: _uc.into(), // Generic type: A
            language_locale_id: None,
            language_id: None,
        })
    }
}


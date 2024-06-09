use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BinaryObjectType {
    #[serde(rename = "_")]
    pub _uc: String,
    #[serde(rename = "characterSetCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_code: Option<String>,
    #[serde(rename = "encodingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_code: Option<String>,
    #[serde(rename = "filename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "mimeCode")]
    pub mime_code: String,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl AsMut<BinaryObjectType> for BinaryObjectType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BinaryObjectType> for BinaryObjectType {
    fn validate(&self) -> Result<&Self, UblError> {
        if self.mime_code.is_empty() {
            return Err(UblError::empty("BinaryObjectType.mime_code"))
        }
        if let Some(v) = &self.encoding_code {
            if v.is_empty() {
                return Err(UblError::optional_empty("BinaryObjectType.encoding_code"))
            }
        }
        if let Some(v) = &self.filename {
            if v.is_empty() {
                return Err(UblError::optional_empty("BinaryObjectType.filename"))
            }
        }
        if let Some(v) = &self.character_set_code {
            if v.is_empty() {
                return Err(UblError::optional_empty("BinaryObjectType.character_set_code"))
            }
        }
        if let Some(v) = &self.format {
            if v.is_empty() {
                return Err(UblError::optional_empty("BinaryObjectType.format"))
            }
        }
        if let Some(v) = &self.uri {
            if v.is_empty() {
                return Err(UblError::optional_empty("BinaryObjectType.uri"))
            }
        }
        if self._uc.is_empty() {
            return Err(UblError::empty("BinaryObjectType._uc"))
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

impl BinaryObjectType {
    pub fn title() -> &'static str {
        "Binary Object. Type"
    }
    pub fn description() -> &'static str {
        "A set of finite-length sequences of binary octets."
    }
    pub fn new<B, A>(_uc: B, mime_code: A) -> Component<Self> where B: Into<String>, A: Into<String> {
        Component(Self {
            mime_code: mime_code.into(), // Generic type: A
            uri: None,
            filename: None,
            format: None,
            character_set_code: None,
            _uc: _uc.into(), // Generic type: B
            encoding_code: None,
        })
    }
}


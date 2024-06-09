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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_code: Option<String>,
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
        if let Some(v) = &self.character_set_code {
            if v.is_empty() {
                return Err(UblError::optional_empty("BinaryObjectType.character_set_code"))
            }
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
        if self._uc.is_empty() {
            return Err(UblError::empty("BinaryObjectType._uc"))
        }
        if let Some(v) = &self.format {
            if v.is_empty() {
                return Err(UblError::optional_empty("BinaryObjectType.format"))
            }
        }
        if let Some(v) = &self.mime_code {
            if v.is_empty() {
                return Err(UblError::optional_empty("BinaryObjectType.mime_code"))
            }
        }
        if let Some(v) = &self.uri {
            if v.is_empty() {
                return Err(UblError::optional_empty("BinaryObjectType.uri"))
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

impl BinaryObjectType {
    pub fn title() -> &'static str {
        "Binary Object. Type"
    }
    pub fn description() -> &'static str {
        "A set of finite-length sequences of binary octets."
    }
    pub fn new<A>(_uc: A) -> Component<Self> where A: Into<String> {
        Component(Self {
            _uc: _uc.into(), // Generic type: A
            uri: None,
            format: None,
            encoding_code: None,
            filename: None,
            mime_code: None,
            character_set_code: None,
        })
    }
}


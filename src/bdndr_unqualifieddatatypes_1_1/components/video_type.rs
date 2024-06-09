use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VideoType {
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

impl AsMut<VideoType> for VideoType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<VideoType> for VideoType {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.encoding_code {
            if v.is_empty() {
                return Err(UblError::optional_empty("VideoType.encoding_code"))
            }
        }
        if let Some(v) = &self.filename {
            if v.is_empty() {
                return Err(UblError::optional_empty("VideoType.filename"))
            }
        }
        if let Some(v) = &self.character_set_code {
            if v.is_empty() {
                return Err(UblError::optional_empty("VideoType.character_set_code"))
            }
        }
        if let Some(v) = &self.format {
            if v.is_empty() {
                return Err(UblError::optional_empty("VideoType.format"))
            }
        }
        if self.mime_code.is_empty() {
            return Err(UblError::empty("VideoType.mime_code"))
        }
        if let Some(v) = &self.uri {
            if v.is_empty() {
                return Err(UblError::optional_empty("VideoType.uri"))
            }
        }
        if self._uc.is_empty() {
            return Err(UblError::empty("VideoType._uc"))
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

impl VideoType {
    pub fn title() -> &'static str {
        "Video. Type"
    }
    pub fn description() -> &'static str {
        "A video representation."
    }
    pub fn new<B, A>(_uc: A, mime_code: B) -> Component<Self> where B: Into<String>, A: Into<String> {
        Component(Self {
            _uc: _uc.into(), // Generic type: A
            character_set_code: None,
            encoding_code: None,
            format: None,
            mime_code: mime_code.into(), // Generic type: B
            uri: None,
            filename: None,
        })
    }
}


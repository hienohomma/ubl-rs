use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GraphicType {
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

impl AsMut<GraphicType> for GraphicType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GraphicType> for GraphicType {
    fn validate(&self) -> Result<&Self, UblError> {
        if self._uc.is_empty() {
            return Err(UblError::empty("GraphicType._uc"))
        }
        if let Some(v) = &self.format {
            if v.is_empty() {
                return Err(UblError::optional_empty("GraphicType.format"))
            }
        }
        if self.mime_code.is_empty() {
            return Err(UblError::empty("GraphicType.mime_code"))
        }
        if let Some(v) = &self.uri {
            if v.is_empty() {
                return Err(UblError::optional_empty("GraphicType.uri"))
            }
        }
        if let Some(v) = &self.character_set_code {
            if v.is_empty() {
                return Err(UblError::optional_empty("GraphicType.character_set_code"))
            }
        }
        if let Some(v) = &self.filename {
            if v.is_empty() {
                return Err(UblError::optional_empty("GraphicType.filename"))
            }
        }
        if let Some(v) = &self.encoding_code {
            if v.is_empty() {
                return Err(UblError::optional_empty("GraphicType.encoding_code"))
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

impl GraphicType {
    pub fn title() -> &'static str {
        "Graphic. Type"
    }
    pub fn description() -> &'static str {
        "A diagram, graph, mathematical curve, or similar representation."
    }
    pub fn new<A, B>(_uc: A, mime_code: B) -> Component<Self> where A: Into<String>, B: Into<String> {
        Component(Self {
            _uc: _uc.into(), // Generic type: A
            format: None,
            mime_code: mime_code.into(), // Generic type: B
            character_set_code: None,
            filename: None,
            uri: None,
            encoding_code: None,
        })
    }
}


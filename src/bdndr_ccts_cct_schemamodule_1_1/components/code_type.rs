use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeType {
    #[serde(rename = "_")]
    pub _uc: String,
    #[serde(rename = "languageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<String>,
    #[serde(rename = "listAgencyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_agency_id: Option<String>,
    #[serde(rename = "listAgencyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_agency_name: Option<String>,
    #[serde(rename = "listID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    #[serde(rename = "listName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    #[serde(rename = "listSchemeURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_scheme_uri: Option<String>,
    #[serde(rename = "listURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_uri: Option<String>,
    #[serde(rename = "listVersionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_version_id: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AsMut<CodeType> for CodeType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CodeType> for CodeType {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.language_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("CodeType.language_id"))
            }
        }
        if let Some(v) = &self.list_scheme_uri {
            if v.is_empty() {
                return Err(UblError::optional_empty("CodeType.list_scheme_uri"))
            }
        }
        if let Some(v) = &self.list_uri {
            if v.is_empty() {
                return Err(UblError::optional_empty("CodeType.list_uri"))
            }
        }
        if let Some(v) = &self.list_version_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("CodeType.list_version_id"))
            }
        }
        if self._uc.is_empty() {
            return Err(UblError::empty("CodeType._uc"))
        }
        if let Some(v) = &self.name {
            if v.is_empty() {
                return Err(UblError::optional_empty("CodeType.name"))
            }
        }
        if let Some(v) = &self.list_agency_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("CodeType.list_agency_id"))
            }
        }
        if let Some(v) = &self.list_agency_name {
            if v.is_empty() {
                return Err(UblError::optional_empty("CodeType.list_agency_name"))
            }
        }
        if let Some(v) = &self.list_name {
            if v.is_empty() {
                return Err(UblError::optional_empty("CodeType.list_name"))
            }
        }
        if let Some(v) = &self.list_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("CodeType.list_id"))
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

impl CodeType {
    pub fn title() -> &'static str {
        "Code. Type"
    }
    pub fn description() -> &'static str {
        "A character string (letters, figures, or symbols) that for brevity and/or languange independence may be used to represent or replace a definitive value or text of an attribute together with relevant supplementary information."
    }
    pub fn new<A>(_uc: A) -> Component<Self> where A: Into<String> {
        Component(Self {
            list_uri: None,
            language_id: None,
            list_name: None,
            list_version_id: None,
            name: None,
            list_id: None,
            list_scheme_uri: None,
            _uc: _uc.into(), // Generic type: A
            list_agency_id: None,
            list_agency_name: None,
        })
    }
}


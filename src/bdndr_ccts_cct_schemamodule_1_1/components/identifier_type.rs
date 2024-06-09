use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdentifierType {
    #[serde(rename = "_")]
    pub _uc: String,
    #[serde(rename = "schemeAgencyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_agency_id: Option<String>,
    #[serde(rename = "schemeAgencyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_agency_name: Option<String>,
    #[serde(rename = "schemeDataURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_data_uri: Option<String>,
    #[serde(rename = "schemeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<String>,
    #[serde(rename = "schemeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_name: Option<String>,
    #[serde(rename = "schemeURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_uri: Option<String>,
    #[serde(rename = "schemeVersionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_version_id: Option<String>,
}

impl AsMut<IdentifierType> for IdentifierType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<IdentifierType> for IdentifierType {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.scheme_agency_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("IdentifierType.scheme_agency_id"))
            }
        }
        if let Some(v) = &self.scheme_name {
            if v.is_empty() {
                return Err(UblError::optional_empty("IdentifierType.scheme_name"))
            }
        }
        if let Some(v) = &self.scheme_data_uri {
            if v.is_empty() {
                return Err(UblError::optional_empty("IdentifierType.scheme_data_uri"))
            }
        }
        if let Some(v) = &self.scheme_uri {
            if v.is_empty() {
                return Err(UblError::optional_empty("IdentifierType.scheme_uri"))
            }
        }
        if let Some(v) = &self.scheme_agency_name {
            if v.is_empty() {
                return Err(UblError::optional_empty("IdentifierType.scheme_agency_name"))
            }
        }
        if self._uc.is_empty() {
            return Err(UblError::empty("IdentifierType._uc"))
        }
        if let Some(v) = &self.scheme_version_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("IdentifierType.scheme_version_id"))
            }
        }
        if let Some(v) = &self.scheme_id {
            if v.is_empty() {
                return Err(UblError::optional_empty("IdentifierType.scheme_id"))
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

impl IdentifierType {
    pub fn title() -> &'static str {
        "Identifier. Type"
    }
    pub fn description() -> &'static str {
        "A character string to identify and distinguish uniquely, one instance of an object in an identification scheme from all other objects in the same scheme together with relevant supplementary information."
    }
    pub fn new<A>(_uc: A) -> Component<Self> where A: Into<String> {
        Component(Self {
            scheme_data_uri: None,
            scheme_version_id: None,
            scheme_name: None,
            _uc: _uc.into(), // Generic type: A
            scheme_uri: None,
            scheme_agency_id: None,
            scheme_id: None,
            scheme_agency_name: None,
        })
    }
}


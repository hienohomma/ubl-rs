use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UBLExtension {
    #[serde(rename = "ExtensionAgencyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_agency_id: Option<UBLExtensionArrayOfExtensionAgencyIDComponent>,
    #[serde(rename = "ExtensionAgencyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_agency_name: Option<UBLExtensionArrayOfExtensionAgencyNameComponent>,
    #[serde(rename = "ExtensionAgencyURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_agency_uri: Option<UBLExtensionArrayOfExtensionAgencyURIComponent>,
    #[serde(rename = "ExtensionContent")]
    pub extension_content: UBLExtensionArrayOfExtensionContentComponent,
    #[serde(rename = "ExtensionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_reason: Option<UBLExtensionArrayOfExtensionReasonComponent>,
    #[serde(rename = "ExtensionReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_reason_code: Option<UBLExtensionArrayOfExtensionReasonCodeComponent>,
    #[serde(rename = "ExtensionURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_uri: Option<UBLExtensionArrayOfExtensionURIComponent>,
    #[serde(rename = "ExtensionVersionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_version_id: Option<UBLExtensionArrayOfExtensionVersionIDComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<UBLExtensionArrayOfIDComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UBLExtensionArrayOfNameComponent>,
}

impl AsMut<UBLExtension> for UBLExtension {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtension> for UBLExtension {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UBLExtension.name", e));
            }
        }
        if let Err(e) = self.extension_content.validate() {
            return Err(UblError::component("UBLExtension.extension_content", e));
        }
        if let Some(v) = &self.extension_agency_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UBLExtension.extension_agency_name", e));
            }
        }
        if let Some(v) = &self.extension_reason {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UBLExtension.extension_reason", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UBLExtension.id", e));
            }
        }
        if let Some(v) = &self.extension_agency_uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UBLExtension.extension_agency_uri", e));
            }
        }
        if let Some(v) = &self.extension_reason_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UBLExtension.extension_reason_code", e));
            }
        }
        if let Some(v) = &self.extension_version_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UBLExtension.extension_version_id", e));
            }
        }
        if let Some(v) = &self.extension_uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UBLExtension.extension_uri", e));
            }
        }
        if let Some(v) = &self.extension_agency_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UBLExtension.extension_agency_id", e));
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

impl UBLExtension {
    pub fn description() -> &'static str {
        "A single extension for private use."
    }
    pub fn new(extension_content: UBLExtensionArrayOfExtensionContentComponent) -> Component<Self> {
        Component(Self {
            extension_agency_name: None,
            extension_uri: None,
            extension_reason_code: None,
            extension_agency_id: None,
            extension_agency_uri: None,
            extension_content,
            extension_reason: None,
            extension_version_id: None,
            id: None,
            name: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UBLExtensionArrayOfExtensionAgencyIDComponent {
    pub items: Vec<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType>,
}

impl AsMut<UBLExtensionArrayOfExtensionAgencyIDComponent> for UBLExtensionArrayOfExtensionAgencyIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensionArrayOfExtensionAgencyIDComponent> for UBLExtensionArrayOfExtensionAgencyIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionAgencyIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionAgencyIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UBLExtensionArrayOfExtensionAgencyIDComponent {
    pub fn new(item: crate::bdndr_unqualifieddatatypes_1_1::IdentifierType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::bdndr_unqualifieddatatypes_1_1::IdentifierType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UBLExtensionArrayOfExtensionAgencyNameComponent {
    pub items: Vec<crate::bdndr_unqualifieddatatypes_1_1::TextType>,
}

impl AsMut<UBLExtensionArrayOfExtensionAgencyNameComponent> for UBLExtensionArrayOfExtensionAgencyNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensionArrayOfExtensionAgencyNameComponent> for UBLExtensionArrayOfExtensionAgencyNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionAgencyNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionAgencyNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UBLExtensionArrayOfExtensionAgencyNameComponent {
    pub fn new(item: crate::bdndr_unqualifieddatatypes_1_1::TextType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::bdndr_unqualifieddatatypes_1_1::TextType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::bdndr_unqualifieddatatypes_1_1::TextType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::bdndr_unqualifieddatatypes_1_1::TextType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UBLExtensionArrayOfExtensionAgencyURIComponent {
    pub items: Vec<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType>,
}

impl AsMut<UBLExtensionArrayOfExtensionAgencyURIComponent> for UBLExtensionArrayOfExtensionAgencyURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensionArrayOfExtensionAgencyURIComponent> for UBLExtensionArrayOfExtensionAgencyURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionAgencyURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionAgencyURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UBLExtensionArrayOfExtensionAgencyURIComponent {
    pub fn new(item: crate::bdndr_unqualifieddatatypes_1_1::IdentifierType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::bdndr_unqualifieddatatypes_1_1::IdentifierType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UBLExtensionArrayOfExtensionContentComponent {
    pub items: Vec<crate::ubl_extensioncontentdatatype_2_1::ExtensionContent>,
}

impl AsMut<UBLExtensionArrayOfExtensionContentComponent> for UBLExtensionArrayOfExtensionContentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensionArrayOfExtensionContentComponent> for UBLExtensionArrayOfExtensionContentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionContentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionContentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UBLExtensionArrayOfExtensionContentComponent {
    pub fn new(item: crate::ubl_extensioncontentdatatype_2_1::ExtensionContent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_extensioncontentdatatype_2_1::ExtensionContent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_extensioncontentdatatype_2_1::ExtensionContent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_extensioncontentdatatype_2_1::ExtensionContent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UBLExtensionArrayOfExtensionReasonComponent {
    pub items: Vec<crate::bdndr_unqualifieddatatypes_1_1::TextType>,
}

impl AsMut<UBLExtensionArrayOfExtensionReasonComponent> for UBLExtensionArrayOfExtensionReasonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensionArrayOfExtensionReasonComponent> for UBLExtensionArrayOfExtensionReasonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionReasonComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionReasonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UBLExtensionArrayOfExtensionReasonComponent {
    pub fn new(item: crate::bdndr_unqualifieddatatypes_1_1::TextType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::bdndr_unqualifieddatatypes_1_1::TextType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::bdndr_unqualifieddatatypes_1_1::TextType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::bdndr_unqualifieddatatypes_1_1::TextType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UBLExtensionArrayOfExtensionReasonCodeComponent {
    pub items: Vec<crate::bdndr_unqualifieddatatypes_1_1::CodeType>,
}

impl AsMut<UBLExtensionArrayOfExtensionReasonCodeComponent> for UBLExtensionArrayOfExtensionReasonCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensionArrayOfExtensionReasonCodeComponent> for UBLExtensionArrayOfExtensionReasonCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionReasonCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionReasonCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UBLExtensionArrayOfExtensionReasonCodeComponent {
    pub fn new(item: crate::bdndr_unqualifieddatatypes_1_1::CodeType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::bdndr_unqualifieddatatypes_1_1::CodeType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::bdndr_unqualifieddatatypes_1_1::CodeType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::bdndr_unqualifieddatatypes_1_1::CodeType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UBLExtensionArrayOfExtensionURIComponent {
    pub items: Vec<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType>,
}

impl AsMut<UBLExtensionArrayOfExtensionURIComponent> for UBLExtensionArrayOfExtensionURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensionArrayOfExtensionURIComponent> for UBLExtensionArrayOfExtensionURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UBLExtensionArrayOfExtensionURIComponent {
    pub fn new(item: crate::bdndr_unqualifieddatatypes_1_1::IdentifierType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::bdndr_unqualifieddatatypes_1_1::IdentifierType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UBLExtensionArrayOfExtensionVersionIDComponent {
    pub items: Vec<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType>,
}

impl AsMut<UBLExtensionArrayOfExtensionVersionIDComponent> for UBLExtensionArrayOfExtensionVersionIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensionArrayOfExtensionVersionIDComponent> for UBLExtensionArrayOfExtensionVersionIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionVersionIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfExtensionVersionIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UBLExtensionArrayOfExtensionVersionIDComponent {
    pub fn new(item: crate::bdndr_unqualifieddatatypes_1_1::IdentifierType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::bdndr_unqualifieddatatypes_1_1::IdentifierType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::bdndr_unqualifieddatatypes_1_1::IdentifierType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UBLExtensionArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<UBLExtensionArrayOfIDComponent> for UBLExtensionArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensionArrayOfIDComponent> for UBLExtensionArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UBLExtensionArrayOfIDComponent {
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
pub struct UBLExtensionArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<UBLExtensionArrayOfNameComponent> for UBLExtensionArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensionArrayOfNameComponent> for UBLExtensionArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UBLExtensionArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UBLExtensionArrayOfNameComponent {
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


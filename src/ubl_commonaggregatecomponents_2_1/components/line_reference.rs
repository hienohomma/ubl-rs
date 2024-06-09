use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineReference {
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<LineReferenceArrayOfDocumentReferenceComponent>,
    #[serde(rename = "LineID")]
    pub line_id: LineReferenceArrayOfLineIDComponent,
    #[serde(rename = "LineStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineReferenceArrayOfLineStatusCodeComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<LineReferenceArrayOfUUIDComponent>,
}

impl AsMut<LineReference> for LineReference {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineReference> for LineReference {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.line_id.validate() {
            return Err(UblError::component("LineReference.line_id", e));
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineReference.document_reference", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineReference.uuid", e));
            }
        }
        if let Some(v) = &self.line_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineReference.line_status_code", e));
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

impl LineReference {
    pub fn title() -> &'static str {
        "Line Reference. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a reference to a line in a document."
    }
    pub fn new(line_id: LineReferenceArrayOfLineIDComponent) -> Component<Self> {
        Component(Self {
            uuid: None,
            line_status_code: None,
            document_reference: None,
            line_id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineReferenceArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<LineReferenceArrayOfDocumentReferenceComponent> for LineReferenceArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineReferenceArrayOfDocumentReferenceComponent> for LineReferenceArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineReferenceArrayOfDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineReferenceArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineReferenceArrayOfDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineReferenceArrayOfLineIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineID>,
}

impl AsMut<LineReferenceArrayOfLineIDComponent> for LineReferenceArrayOfLineIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineReferenceArrayOfLineIDComponent> for LineReferenceArrayOfLineIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineReferenceArrayOfLineIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineReferenceArrayOfLineIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineReferenceArrayOfLineIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LineID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LineID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LineID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LineID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineReferenceArrayOfLineStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineStatusCode>,
}

impl AsMut<LineReferenceArrayOfLineStatusCodeComponent> for LineReferenceArrayOfLineStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineReferenceArrayOfLineStatusCodeComponent> for LineReferenceArrayOfLineStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineReferenceArrayOfLineStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineReferenceArrayOfLineStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineReferenceArrayOfLineStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LineStatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LineStatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LineStatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LineStatusCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineReferenceArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<LineReferenceArrayOfUUIDComponent> for LineReferenceArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineReferenceArrayOfUUIDComponent> for LineReferenceArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineReferenceArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineReferenceArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineReferenceArrayOfUUIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::UUID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::UUID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::UUID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::UUID> {
        self.items.iter()
    }
}


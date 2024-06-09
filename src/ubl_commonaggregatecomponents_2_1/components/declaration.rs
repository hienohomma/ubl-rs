use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Declaration {
    #[serde(rename = "DeclarationTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaration_type_code: Option<DeclarationArrayOfDeclarationTypeCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<DeclarationArrayOfDescriptionComponent>,
    #[serde(rename = "EvidenceSupplied")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_supplied: Option<DeclarationArrayOfEvidenceSuppliedComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<DeclarationArrayOfNameComponent>,
}

impl AsMut<Declaration> for Declaration {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Declaration> for Declaration {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Declaration.name", e));
            }
        }
        if let Some(v) = &self.evidence_supplied {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Declaration.evidence_supplied", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Declaration.description", e));
            }
        }
        if let Some(v) = &self.declaration_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Declaration.declaration_type_code", e));
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

impl Declaration {
    pub fn title() -> &'static str {
        "Declaration. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a declaration by an economic operator of certain characteristics or capabilities in fulfilment of requirements specified in a call for tenders."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            name: None,
            declaration_type_code: None,
            description: None,
            evidence_supplied: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeclarationArrayOfDeclarationTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeclarationTypeCode>,
}

impl AsMut<DeclarationArrayOfDeclarationTypeCodeComponent> for DeclarationArrayOfDeclarationTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeclarationArrayOfDeclarationTypeCodeComponent> for DeclarationArrayOfDeclarationTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeclarationArrayOfDeclarationTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeclarationArrayOfDeclarationTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeclarationArrayOfDeclarationTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DeclarationTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DeclarationTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DeclarationTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DeclarationTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeclarationArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<DeclarationArrayOfDescriptionComponent> for DeclarationArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeclarationArrayOfDescriptionComponent> for DeclarationArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DeclarationArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeclarationArrayOfDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Description) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Description) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeclarationArrayOfEvidenceSuppliedComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied>,
}

impl AsMut<DeclarationArrayOfEvidenceSuppliedComponent> for DeclarationArrayOfEvidenceSuppliedComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeclarationArrayOfEvidenceSuppliedComponent> for DeclarationArrayOfEvidenceSuppliedComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DeclarationArrayOfEvidenceSuppliedComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeclarationArrayOfEvidenceSuppliedComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeclarationArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<DeclarationArrayOfNameComponent> for DeclarationArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeclarationArrayOfNameComponent> for DeclarationArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DeclarationArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeclarationArrayOfNameComponent {
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


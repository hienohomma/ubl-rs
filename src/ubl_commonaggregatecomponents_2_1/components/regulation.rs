use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Regulation {
    #[serde(rename = "LegalReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_reference: Option<RegulationArrayOfLegalReferenceComponent>,
    #[serde(rename = "Name")]
    pub name: RegulationArrayOfNameComponent,
    #[serde(rename = "OntologyURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontology_uri: Option<RegulationArrayOfOntologyURIComponent>,
}

impl AsMut<Regulation> for Regulation {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Regulation> for Regulation {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.name.validate() {
            return Err(UblError::component("Regulation.name", e));
        }
        if let Some(v) = &self.ontology_uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Regulation.ontology_uri", e));
            }
        }
        if let Some(v) = &self.legal_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Regulation.legal_reference", e));
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

impl Regulation {
    pub fn title() -> &'static str {
        "Regulation. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a regulation."
    }
    pub fn new(name: RegulationArrayOfNameComponent) -> Component<Self> {
        Component(Self {
            legal_reference: None,
            name,
            ontology_uri: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RegulationArrayOfLegalReferenceComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LegalReference>,
}

impl AsMut<RegulationArrayOfLegalReferenceComponent> for RegulationArrayOfLegalReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RegulationArrayOfLegalReferenceComponent> for RegulationArrayOfLegalReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RegulationArrayOfLegalReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RegulationArrayOfLegalReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RegulationArrayOfLegalReferenceComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LegalReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LegalReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LegalReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LegalReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RegulationArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<RegulationArrayOfNameComponent> for RegulationArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RegulationArrayOfNameComponent> for RegulationArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RegulationArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RegulationArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RegulationArrayOfNameComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RegulationArrayOfOntologyURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OntologyURI>,
}

impl AsMut<RegulationArrayOfOntologyURIComponent> for RegulationArrayOfOntologyURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RegulationArrayOfOntologyURIComponent> for RegulationArrayOfOntologyURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RegulationArrayOfOntologyURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RegulationArrayOfOntologyURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RegulationArrayOfOntologyURIComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OntologyURI) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OntologyURI) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OntologyURI> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OntologyURI> {
        self.items.iter()
    }
}


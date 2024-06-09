use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenderRequirement {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TenderRequirementArrayOfDescriptionComponent>,
    #[serde(rename = "Name")]
    pub name: TenderRequirementArrayOfNameComponent,
    #[serde(rename = "TemplateDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_document_reference: Option<TenderRequirementArrayOfTemplateDocumentReferenceComponent>,
}

impl AsMut<TenderRequirement> for TenderRequirement {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderRequirement> for TenderRequirement {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.template_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderRequirement.template_document_reference", e));
            }
        }
        if let Err(e) = self.name.validate() {
            return Err(UblError::component("TenderRequirement.name", e));
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderRequirement.description", e));
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

impl TenderRequirement {
    pub fn title() -> &'static str {
        "Tender Requirement. Details"
    }
    pub fn description() -> &'static str {
        "A template for a required document in a tendering process."
    }
    pub fn new(name: TenderRequirementArrayOfNameComponent) -> Component<Self> {
        Component(Self {
            description: None,
            name,
            template_document_reference: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderRequirementArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<TenderRequirementArrayOfDescriptionComponent> for TenderRequirementArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderRequirementArrayOfDescriptionComponent> for TenderRequirementArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderRequirementArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderRequirementArrayOfDescriptionComponent {
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
pub struct TenderRequirementArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<TenderRequirementArrayOfNameComponent> for TenderRequirementArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderRequirementArrayOfNameComponent> for TenderRequirementArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderRequirementArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderRequirementArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderRequirementArrayOfNameComponent {
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
pub struct TenderRequirementArrayOfTemplateDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TemplateDocumentReference>,
}

impl AsMut<TenderRequirementArrayOfTemplateDocumentReferenceComponent> for TenderRequirementArrayOfTemplateDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderRequirementArrayOfTemplateDocumentReferenceComponent> for TenderRequirementArrayOfTemplateDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderRequirementArrayOfTemplateDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderRequirementArrayOfTemplateDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderRequirementArrayOfTemplateDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TemplateDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TemplateDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TemplateDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TemplateDocumentReference> {
        self.items.iter()
    }
}


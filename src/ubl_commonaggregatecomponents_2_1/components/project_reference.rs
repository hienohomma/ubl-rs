use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectReference {
    #[serde(rename = "ID")]
    pub id: ProjectReferenceArrayOfIDComponent,
    #[serde(rename = "IssueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<ProjectReferenceArrayOfIssueDateComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ProjectReferenceArrayOfUUIDComponent>,
    #[serde(rename = "WorkPhaseReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_phase_reference: Option<ProjectReferenceArrayOfWorkPhaseReferenceComponent>,
}

impl AsMut<ProjectReference> for ProjectReference {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProjectReference> for ProjectReference {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ProjectReference.id", e));
        }
        if let Some(v) = &self.issue_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProjectReference.issue_date", e));
            }
        }
        if let Some(v) = &self.work_phase_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProjectReference.work_phase_reference", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProjectReference.uuid", e));
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

impl ProjectReference {
    pub fn title() -> &'static str {
        "Project Reference. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a reference to a procurement project."
    }
    pub fn new(id: ProjectReferenceArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            issue_date: None,
            uuid: None,
            work_phase_reference: None,
            id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProjectReferenceArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ProjectReferenceArrayOfIDComponent> for ProjectReferenceArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProjectReferenceArrayOfIDComponent> for ProjectReferenceArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProjectReferenceArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProjectReferenceArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProjectReferenceArrayOfIDComponent {
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
pub struct ProjectReferenceArrayOfIssueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueDate>,
}

impl AsMut<ProjectReferenceArrayOfIssueDateComponent> for ProjectReferenceArrayOfIssueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProjectReferenceArrayOfIssueDateComponent> for ProjectReferenceArrayOfIssueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProjectReferenceArrayOfIssueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProjectReferenceArrayOfIssueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProjectReferenceArrayOfIssueDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IssueDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IssueDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IssueDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IssueDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProjectReferenceArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<ProjectReferenceArrayOfUUIDComponent> for ProjectReferenceArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProjectReferenceArrayOfUUIDComponent> for ProjectReferenceArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProjectReferenceArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProjectReferenceArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProjectReferenceArrayOfUUIDComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProjectReferenceArrayOfWorkPhaseReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WorkPhaseReference>,
}

impl AsMut<ProjectReferenceArrayOfWorkPhaseReferenceComponent> for ProjectReferenceArrayOfWorkPhaseReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProjectReferenceArrayOfWorkPhaseReferenceComponent> for ProjectReferenceArrayOfWorkPhaseReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ProjectReferenceArrayOfWorkPhaseReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProjectReferenceArrayOfWorkPhaseReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WorkPhaseReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WorkPhaseReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WorkPhaseReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WorkPhaseReference> {
        self.items.iter()
    }
}


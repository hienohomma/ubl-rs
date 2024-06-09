use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkPhaseReference {
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<WorkPhaseReferenceArrayOfEndDateComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<WorkPhaseReferenceArrayOfIDComponent>,
    #[serde(rename = "ProgressPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<WorkPhaseReferenceArrayOfProgressPercentComponent>,
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<WorkPhaseReferenceArrayOfStartDateComponent>,
    #[serde(rename = "WorkOrderDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_order_document_reference: Option<WorkPhaseReferenceArrayOfWorkOrderDocumentReferenceComponent>,
    #[serde(rename = "WorkPhase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_phase: Option<WorkPhaseReferenceArrayOfWorkPhaseComponent>,
    #[serde(rename = "WorkPhaseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_phase_code: Option<WorkPhaseReferenceArrayOfWorkPhaseCodeComponent>,
}

impl AsMut<WorkPhaseReference> for WorkPhaseReference {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WorkPhaseReference> for WorkPhaseReference {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("WorkPhaseReference.id", e));
            }
        }
        if let Some(v) = &self.work_order_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("WorkPhaseReference.work_order_document_reference", e));
            }
        }
        if let Some(v) = &self.work_phase {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("WorkPhaseReference.work_phase", e));
            }
        }
        if let Some(v) = &self.work_phase_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("WorkPhaseReference.work_phase_code", e));
            }
        }
        if let Some(v) = &self.progress_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("WorkPhaseReference.progress_percent", e));
            }
        }
        if let Some(v) = &self.end_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("WorkPhaseReference.end_date", e));
            }
        }
        if let Some(v) = &self.start_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("WorkPhaseReference.start_date", e));
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

impl WorkPhaseReference {
    pub fn title() -> &'static str {
        "Work Phase Reference. Details"
    }
    pub fn description() -> &'static str {
        "A class that refers to a phase of work. Used for instance to specify what part of the contract the billing is referring to."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            work_order_document_reference: None,
            work_phase: None,
            work_phase_code: None,
            id: None,
            progress_percent: None,
            end_date: None,
            start_date: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WorkPhaseReferenceArrayOfEndDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EndDate>,
}

impl AsMut<WorkPhaseReferenceArrayOfEndDateComponent> for WorkPhaseReferenceArrayOfEndDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WorkPhaseReferenceArrayOfEndDateComponent> for WorkPhaseReferenceArrayOfEndDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfEndDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfEndDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WorkPhaseReferenceArrayOfEndDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EndDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EndDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EndDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EndDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WorkPhaseReferenceArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<WorkPhaseReferenceArrayOfIDComponent> for WorkPhaseReferenceArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WorkPhaseReferenceArrayOfIDComponent> for WorkPhaseReferenceArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WorkPhaseReferenceArrayOfIDComponent {
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
pub struct WorkPhaseReferenceArrayOfProgressPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ProgressPercent>,
}

impl AsMut<WorkPhaseReferenceArrayOfProgressPercentComponent> for WorkPhaseReferenceArrayOfProgressPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WorkPhaseReferenceArrayOfProgressPercentComponent> for WorkPhaseReferenceArrayOfProgressPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfProgressPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfProgressPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WorkPhaseReferenceArrayOfProgressPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ProgressPercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ProgressPercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ProgressPercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ProgressPercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WorkPhaseReferenceArrayOfStartDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::StartDate>,
}

impl AsMut<WorkPhaseReferenceArrayOfStartDateComponent> for WorkPhaseReferenceArrayOfStartDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WorkPhaseReferenceArrayOfStartDateComponent> for WorkPhaseReferenceArrayOfStartDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfStartDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfStartDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WorkPhaseReferenceArrayOfStartDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::StartDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::StartDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::StartDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::StartDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WorkPhaseReferenceArrayOfWorkOrderDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WorkOrderDocumentReference>,
}

impl AsMut<WorkPhaseReferenceArrayOfWorkOrderDocumentReferenceComponent> for WorkPhaseReferenceArrayOfWorkOrderDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WorkPhaseReferenceArrayOfWorkOrderDocumentReferenceComponent> for WorkPhaseReferenceArrayOfWorkOrderDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfWorkOrderDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WorkPhaseReferenceArrayOfWorkOrderDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WorkOrderDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WorkOrderDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WorkOrderDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WorkOrderDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WorkPhaseReferenceArrayOfWorkPhaseComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::WorkPhase>,
}

impl AsMut<WorkPhaseReferenceArrayOfWorkPhaseComponent> for WorkPhaseReferenceArrayOfWorkPhaseComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WorkPhaseReferenceArrayOfWorkPhaseComponent> for WorkPhaseReferenceArrayOfWorkPhaseComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfWorkPhaseComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WorkPhaseReferenceArrayOfWorkPhaseComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::WorkPhase) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::WorkPhase) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::WorkPhase> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::WorkPhase> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WorkPhaseReferenceArrayOfWorkPhaseCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::WorkPhaseCode>,
}

impl AsMut<WorkPhaseReferenceArrayOfWorkPhaseCodeComponent> for WorkPhaseReferenceArrayOfWorkPhaseCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WorkPhaseReferenceArrayOfWorkPhaseCodeComponent> for WorkPhaseReferenceArrayOfWorkPhaseCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfWorkPhaseCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("WorkPhaseReferenceArrayOfWorkPhaseCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WorkPhaseReferenceArrayOfWorkPhaseCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::WorkPhaseCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::WorkPhaseCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::WorkPhaseCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::WorkPhaseCode> {
        self.items.iter()
    }
}


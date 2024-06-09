use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QualificationResolution {
    #[serde(rename = "AdmissionCode")]
    pub admission_code: QualificationResolutionArrayOfAdmissionCodeComponent,
    #[serde(rename = "ExclusionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_reason: Option<QualificationResolutionArrayOfExclusionReasonComponent>,
    #[serde(rename = "ProcurementProjectLot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procurement_project_lot: Option<QualificationResolutionArrayOfProcurementProjectLotComponent>,
    #[serde(rename = "Resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<QualificationResolutionArrayOfResolutionComponent>,
    #[serde(rename = "ResolutionDate")]
    pub resolution_date: QualificationResolutionArrayOfResolutionDateComponent,
    #[serde(rename = "ResolutionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_time: Option<QualificationResolutionArrayOfResolutionTimeComponent>,
}

impl AsMut<QualificationResolution> for QualificationResolution {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualificationResolution> for QualificationResolution {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.resolution {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualificationResolution.resolution", e));
            }
        }
        if let Some(v) = &self.resolution_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualificationResolution.resolution_time", e));
            }
        }
        if let Some(v) = &self.procurement_project_lot {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualificationResolution.procurement_project_lot", e));
            }
        }
        if let Err(e) = self.admission_code.validate() {
            return Err(UblError::component("QualificationResolution.admission_code", e));
        }
        if let Some(v) = &self.exclusion_reason {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualificationResolution.exclusion_reason", e));
            }
        }
        if let Err(e) = self.resolution_date.validate() {
            return Err(UblError::component("QualificationResolution.resolution_date", e));
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

impl QualificationResolution {
    pub fn title() -> &'static str {
        "Qualification Resolution. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the acceptance or rejection of an economic operator in a tendering process."
    }
    pub fn new(admission_code: QualificationResolutionArrayOfAdmissionCodeComponent, resolution_date: QualificationResolutionArrayOfResolutionDateComponent) -> Component<Self> {
        Component(Self {
            resolution_time: None,
            exclusion_reason: None,
            admission_code,
            resolution: None,
            procurement_project_lot: None,
            resolution_date,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualificationResolutionArrayOfAdmissionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AdmissionCode>,
}

impl AsMut<QualificationResolutionArrayOfAdmissionCodeComponent> for QualificationResolutionArrayOfAdmissionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualificationResolutionArrayOfAdmissionCodeComponent> for QualificationResolutionArrayOfAdmissionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualificationResolutionArrayOfAdmissionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualificationResolutionArrayOfAdmissionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualificationResolutionArrayOfAdmissionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AdmissionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AdmissionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AdmissionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AdmissionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualificationResolutionArrayOfExclusionReasonComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExclusionReason>,
}

impl AsMut<QualificationResolutionArrayOfExclusionReasonComponent> for QualificationResolutionArrayOfExclusionReasonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualificationResolutionArrayOfExclusionReasonComponent> for QualificationResolutionArrayOfExclusionReasonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("QualificationResolutionArrayOfExclusionReasonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualificationResolutionArrayOfExclusionReasonComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExclusionReason) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExclusionReason) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExclusionReason> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExclusionReason> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualificationResolutionArrayOfProcurementProjectLotComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ProcurementProjectLot>,
}

impl AsMut<QualificationResolutionArrayOfProcurementProjectLotComponent> for QualificationResolutionArrayOfProcurementProjectLotComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualificationResolutionArrayOfProcurementProjectLotComponent> for QualificationResolutionArrayOfProcurementProjectLotComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualificationResolutionArrayOfProcurementProjectLotComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualificationResolutionArrayOfProcurementProjectLotComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualificationResolutionArrayOfProcurementProjectLotComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ProcurementProjectLot) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ProcurementProjectLot) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ProcurementProjectLot> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ProcurementProjectLot> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualificationResolutionArrayOfResolutionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Resolution>,
}

impl AsMut<QualificationResolutionArrayOfResolutionComponent> for QualificationResolutionArrayOfResolutionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualificationResolutionArrayOfResolutionComponent> for QualificationResolutionArrayOfResolutionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("QualificationResolutionArrayOfResolutionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualificationResolutionArrayOfResolutionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Resolution) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Resolution) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Resolution> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Resolution> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualificationResolutionArrayOfResolutionDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ResolutionDate>,
}

impl AsMut<QualificationResolutionArrayOfResolutionDateComponent> for QualificationResolutionArrayOfResolutionDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualificationResolutionArrayOfResolutionDateComponent> for QualificationResolutionArrayOfResolutionDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualificationResolutionArrayOfResolutionDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualificationResolutionArrayOfResolutionDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualificationResolutionArrayOfResolutionDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ResolutionDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ResolutionDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ResolutionDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ResolutionDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualificationResolutionArrayOfResolutionTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ResolutionTime>,
}

impl AsMut<QualificationResolutionArrayOfResolutionTimeComponent> for QualificationResolutionArrayOfResolutionTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualificationResolutionArrayOfResolutionTimeComponent> for QualificationResolutionArrayOfResolutionTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualificationResolutionArrayOfResolutionTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualificationResolutionArrayOfResolutionTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualificationResolutionArrayOfResolutionTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ResolutionTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ResolutionTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ResolutionTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ResolutionTime> {
        self.items.iter()
    }
}


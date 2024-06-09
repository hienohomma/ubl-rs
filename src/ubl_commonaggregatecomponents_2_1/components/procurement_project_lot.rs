use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcurementProjectLot {
    #[serde(rename = "ID")]
    pub id: ProcurementProjectLotArrayOfIDComponent,
    #[serde(rename = "ProcurementProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProjectLotArrayOfProcurementProjectComponent>,
    #[serde(rename = "TenderingTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tendering_terms: Option<ProcurementProjectLotArrayOfTenderingTermsComponent>,
}

impl AsMut<ProcurementProjectLot> for ProcurementProjectLot {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectLot> for ProcurementProjectLot {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.tendering_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProjectLot.tendering_terms", e));
            }
        }
        if let Some(v) = &self.procurement_project {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProjectLot.procurement_project", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ProcurementProjectLot.id", e));
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

impl ProcurementProjectLot {
    pub fn title() -> &'static str {
        "Procurement Project Lot. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe one of the parts of a procurement project that is being subdivided to allow the contracting party to award different lots to different economic operators under different contracts."
    }
    pub fn new(id: ProcurementProjectLotArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            tendering_terms: None,
            procurement_project: None,
            id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectLotArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ProcurementProjectLotArrayOfIDComponent> for ProcurementProjectLotArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectLotArrayOfIDComponent> for ProcurementProjectLotArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectLotArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectLotArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectLotArrayOfIDComponent {
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
pub struct ProcurementProjectLotArrayOfProcurementProjectComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ProcurementProject>,
}

impl AsMut<ProcurementProjectLotArrayOfProcurementProjectComponent> for ProcurementProjectLotArrayOfProcurementProjectComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectLotArrayOfProcurementProjectComponent> for ProcurementProjectLotArrayOfProcurementProjectComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectLotArrayOfProcurementProjectComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectLotArrayOfProcurementProjectComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectLotArrayOfProcurementProjectComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ProcurementProject) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ProcurementProject) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ProcurementProject> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ProcurementProject> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectLotArrayOfTenderingTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TenderingTerms>,
}

impl AsMut<ProcurementProjectLotArrayOfTenderingTermsComponent> for ProcurementProjectLotArrayOfTenderingTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectLotArrayOfTenderingTermsComponent> for ProcurementProjectLotArrayOfTenderingTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectLotArrayOfTenderingTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectLotArrayOfTenderingTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectLotArrayOfTenderingTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TenderingTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TenderingTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TenderingTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TenderingTerms> {
        self.items.iter()
    }
}


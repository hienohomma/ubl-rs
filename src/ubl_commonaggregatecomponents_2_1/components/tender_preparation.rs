use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenderPreparation {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TenderPreparationArrayOfDescriptionComponent>,
    #[serde(rename = "DocumentTenderRequirement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_tender_requirement: Option<TenderPreparationArrayOfDocumentTenderRequirementComponent>,
    #[serde(rename = "OpenTenderID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_tender_id: Option<TenderPreparationArrayOfOpenTenderIDComponent>,
    #[serde(rename = "ProcurementProjectLot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procurement_project_lot: Option<TenderPreparationArrayOfProcurementProjectLotComponent>,
    #[serde(rename = "TenderEnvelopeID")]
    pub tender_envelope_id: TenderPreparationArrayOfTenderEnvelopeIDComponent,
    #[serde(rename = "TenderEnvelopeTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_envelope_type_code: Option<TenderPreparationArrayOfTenderEnvelopeTypeCodeComponent>,
}

impl AsMut<TenderPreparation> for TenderPreparation {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderPreparation> for TenderPreparation {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.document_tender_requirement {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderPreparation.document_tender_requirement", e));
            }
        }
        if let Some(v) = &self.procurement_project_lot {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderPreparation.procurement_project_lot", e));
            }
        }
        if let Some(v) = &self.tender_envelope_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderPreparation.tender_envelope_type_code", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderPreparation.description", e));
            }
        }
        if let Some(v) = &self.open_tender_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderPreparation.open_tender_id", e));
            }
        }
        if let Err(e) = self.tender_envelope_id.validate() {
            return Err(UblError::component("TenderPreparation.tender_envelope_id", e));
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

impl TenderPreparation {
    pub fn title() -> &'static str {
        "Tender Preparation. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe directions for preparing a tender."
    }
    pub fn new(tender_envelope_id: TenderPreparationArrayOfTenderEnvelopeIDComponent) -> Component<Self> {
        Component(Self {
            tender_envelope_id,
            description: None,
            procurement_project_lot: None,
            tender_envelope_type_code: None,
            open_tender_id: None,
            document_tender_requirement: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderPreparationArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<TenderPreparationArrayOfDescriptionComponent> for TenderPreparationArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderPreparationArrayOfDescriptionComponent> for TenderPreparationArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderPreparationArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderPreparationArrayOfDescriptionComponent {
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
pub struct TenderPreparationArrayOfDocumentTenderRequirementComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentTenderRequirement>,
}

impl AsMut<TenderPreparationArrayOfDocumentTenderRequirementComponent> for TenderPreparationArrayOfDocumentTenderRequirementComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderPreparationArrayOfDocumentTenderRequirementComponent> for TenderPreparationArrayOfDocumentTenderRequirementComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderPreparationArrayOfDocumentTenderRequirementComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderPreparationArrayOfDocumentTenderRequirementComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DocumentTenderRequirement) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DocumentTenderRequirement) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DocumentTenderRequirement> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DocumentTenderRequirement> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderPreparationArrayOfOpenTenderIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OpenTenderID>,
}

impl AsMut<TenderPreparationArrayOfOpenTenderIDComponent> for TenderPreparationArrayOfOpenTenderIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderPreparationArrayOfOpenTenderIDComponent> for TenderPreparationArrayOfOpenTenderIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderPreparationArrayOfOpenTenderIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderPreparationArrayOfOpenTenderIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderPreparationArrayOfOpenTenderIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OpenTenderID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OpenTenderID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OpenTenderID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OpenTenderID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderPreparationArrayOfProcurementProjectLotComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ProcurementProjectLot>,
}

impl AsMut<TenderPreparationArrayOfProcurementProjectLotComponent> for TenderPreparationArrayOfProcurementProjectLotComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderPreparationArrayOfProcurementProjectLotComponent> for TenderPreparationArrayOfProcurementProjectLotComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderPreparationArrayOfProcurementProjectLotComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderPreparationArrayOfProcurementProjectLotComponent {
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
pub struct TenderPreparationArrayOfTenderEnvelopeIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeID>,
}

impl AsMut<TenderPreparationArrayOfTenderEnvelopeIDComponent> for TenderPreparationArrayOfTenderEnvelopeIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderPreparationArrayOfTenderEnvelopeIDComponent> for TenderPreparationArrayOfTenderEnvelopeIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderPreparationArrayOfTenderEnvelopeIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderPreparationArrayOfTenderEnvelopeIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderPreparationArrayOfTenderEnvelopeIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderPreparationArrayOfTenderEnvelopeTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeTypeCode>,
}

impl AsMut<TenderPreparationArrayOfTenderEnvelopeTypeCodeComponent> for TenderPreparationArrayOfTenderEnvelopeTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderPreparationArrayOfTenderEnvelopeTypeCodeComponent> for TenderPreparationArrayOfTenderEnvelopeTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderPreparationArrayOfTenderEnvelopeTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderPreparationArrayOfTenderEnvelopeTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderPreparationArrayOfTenderEnvelopeTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeTypeCode> {
        self.items.iter()
    }
}


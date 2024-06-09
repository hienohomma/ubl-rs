use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PromotionalEvent {
    #[serde(rename = "FirstShipmentAvailibilityDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_shipment_availibility_date: Option<PromotionalEventArrayOfFirstShipmentAvailibilityDateComponent>,
    #[serde(rename = "LatestProposalAcceptanceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_proposal_acceptance_date: Option<PromotionalEventArrayOfLatestProposalAcceptanceDateComponent>,
    #[serde(rename = "PromotionalEventTypeCode")]
    pub promotional_event_type_code: PromotionalEventArrayOfPromotionalEventTypeCodeComponent,
    #[serde(rename = "PromotionalSpecification")]
    pub promotional_specification: PromotionalEventArrayOfPromotionalSpecificationComponent,
    #[serde(rename = "SubmissionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_date: Option<PromotionalEventArrayOfSubmissionDateComponent>,
}

impl AsMut<PromotionalEvent> for PromotionalEvent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalEvent> for PromotionalEvent {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.promotional_specification.validate() {
            return Err(UblError::component("PromotionalEvent.promotional_specification", e));
        }
        if let Some(v) = &self.latest_proposal_acceptance_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PromotionalEvent.latest_proposal_acceptance_date", e));
            }
        }
        if let Some(v) = &self.submission_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PromotionalEvent.submission_date", e));
            }
        }
        if let Err(e) = self.promotional_event_type_code.validate() {
            return Err(UblError::component("PromotionalEvent.promotional_event_type_code", e));
        }
        if let Some(v) = &self.first_shipment_availibility_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PromotionalEvent.first_shipment_availibility_date", e));
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

impl PromotionalEvent {
    pub fn title() -> &'static str {
        "Promotional Event. Details"
    }
    pub fn description() -> &'static str {
        "Agree can be renamed as PromotionalEvents"
    }
    pub fn new(promotional_event_type_code: PromotionalEventArrayOfPromotionalEventTypeCodeComponent, promotional_specification: PromotionalEventArrayOfPromotionalSpecificationComponent) -> Component<Self> {
        Component(Self {
            first_shipment_availibility_date: None,
            latest_proposal_acceptance_date: None,
            promotional_event_type_code,
            submission_date: None,
            promotional_specification,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PromotionalEventArrayOfFirstShipmentAvailibilityDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FirstShipmentAvailibilityDate>,
}

impl AsMut<PromotionalEventArrayOfFirstShipmentAvailibilityDateComponent> for PromotionalEventArrayOfFirstShipmentAvailibilityDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalEventArrayOfFirstShipmentAvailibilityDateComponent> for PromotionalEventArrayOfFirstShipmentAvailibilityDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PromotionalEventArrayOfFirstShipmentAvailibilityDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PromotionalEventArrayOfFirstShipmentAvailibilityDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PromotionalEventArrayOfFirstShipmentAvailibilityDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FirstShipmentAvailibilityDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FirstShipmentAvailibilityDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FirstShipmentAvailibilityDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FirstShipmentAvailibilityDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PromotionalEventArrayOfLatestProposalAcceptanceDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatestProposalAcceptanceDate>,
}

impl AsMut<PromotionalEventArrayOfLatestProposalAcceptanceDateComponent> for PromotionalEventArrayOfLatestProposalAcceptanceDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalEventArrayOfLatestProposalAcceptanceDateComponent> for PromotionalEventArrayOfLatestProposalAcceptanceDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PromotionalEventArrayOfLatestProposalAcceptanceDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PromotionalEventArrayOfLatestProposalAcceptanceDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PromotionalEventArrayOfLatestProposalAcceptanceDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatestProposalAcceptanceDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatestProposalAcceptanceDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatestProposalAcceptanceDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatestProposalAcceptanceDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PromotionalEventArrayOfPromotionalEventTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PromotionalEventTypeCode>,
}

impl AsMut<PromotionalEventArrayOfPromotionalEventTypeCodeComponent> for PromotionalEventArrayOfPromotionalEventTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalEventArrayOfPromotionalEventTypeCodeComponent> for PromotionalEventArrayOfPromotionalEventTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PromotionalEventArrayOfPromotionalEventTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PromotionalEventArrayOfPromotionalEventTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PromotionalEventArrayOfPromotionalEventTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PromotionalEventTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PromotionalEventTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PromotionalEventTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PromotionalEventTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PromotionalEventArrayOfPromotionalSpecificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PromotionalSpecification>,
}

impl AsMut<PromotionalEventArrayOfPromotionalSpecificationComponent> for PromotionalEventArrayOfPromotionalSpecificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalEventArrayOfPromotionalSpecificationComponent> for PromotionalEventArrayOfPromotionalSpecificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PromotionalEventArrayOfPromotionalSpecificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PromotionalEventArrayOfPromotionalSpecificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PromotionalSpecification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PromotionalSpecification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PromotionalSpecification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PromotionalSpecification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PromotionalEventArrayOfSubmissionDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SubmissionDate>,
}

impl AsMut<PromotionalEventArrayOfSubmissionDateComponent> for PromotionalEventArrayOfSubmissionDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PromotionalEventArrayOfSubmissionDateComponent> for PromotionalEventArrayOfSubmissionDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PromotionalEventArrayOfSubmissionDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PromotionalEventArrayOfSubmissionDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PromotionalEventArrayOfSubmissionDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SubmissionDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SubmissionDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SubmissionDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SubmissionDate> {
        self.items.iter()
    }
}


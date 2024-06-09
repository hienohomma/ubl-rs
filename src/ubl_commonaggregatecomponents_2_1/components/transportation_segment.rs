use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransportationSegment {
    #[serde(rename = "ReferencedConsignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_consignment: Option<TransportationSegmentArrayOfReferencedConsignmentComponent>,
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: TransportationSegmentArrayOfSequenceNumericComponent,
    #[serde(rename = "ShipmentStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_stage: Option<TransportationSegmentArrayOfShipmentStageComponent>,
    #[serde(rename = "TransportExecutionPlanReferenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_execution_plan_reference_id: Option<TransportationSegmentArrayOfTransportExecutionPlanReferenceIDComponent>,
    #[serde(rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: TransportationSegmentArrayOfTransportServiceProviderPartyComponent,
    #[serde(rename = "TransportationService")]
    pub transportation_service: TransportationSegmentArrayOfTransportationServiceComponent,
}

impl AsMut<TransportationSegment> for TransportationSegment {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationSegment> for TransportationSegment {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.shipment_stage {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationSegment.shipment_stage", e));
            }
        }
        if let Err(e) = self.sequence_numeric.validate() {
            return Err(UblError::component("TransportationSegment.sequence_numeric", e));
        }
        if let Some(v) = &self.transport_execution_plan_reference_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationSegment.transport_execution_plan_reference_id", e));
            }
        }
        if let Err(e) = self.transportation_service.validate() {
            return Err(UblError::component("TransportationSegment.transportation_service", e));
        }
        if let Err(e) = self.transport_service_provider_party.validate() {
            return Err(UblError::component("TransportationSegment.transport_service_provider_party", e));
        }
        if let Some(v) = &self.referenced_consignment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationSegment.referenced_consignment", e));
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

impl TransportationSegment {
    pub fn title() -> &'static str {
        "Transportation Segment. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe one segment or leg in a transportation service."
    }
    pub fn new(sequence_numeric: TransportationSegmentArrayOfSequenceNumericComponent, transport_service_provider_party: TransportationSegmentArrayOfTransportServiceProviderPartyComponent, transportation_service: TransportationSegmentArrayOfTransportationServiceComponent) -> Component<Self> {
        Component(Self {
            sequence_numeric,
            transport_service_provider_party,
            transportation_service,
            referenced_consignment: None,
            transport_execution_plan_reference_id: None,
            shipment_stage: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationSegmentArrayOfReferencedConsignmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReferencedConsignment>,
}

impl AsMut<TransportationSegmentArrayOfReferencedConsignmentComponent> for TransportationSegmentArrayOfReferencedConsignmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationSegmentArrayOfReferencedConsignmentComponent> for TransportationSegmentArrayOfReferencedConsignmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationSegmentArrayOfReferencedConsignmentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationSegmentArrayOfReferencedConsignmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportationSegmentArrayOfReferencedConsignmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReferencedConsignment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReferencedConsignment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReferencedConsignment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReferencedConsignment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationSegmentArrayOfSequenceNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric>,
}

impl AsMut<TransportationSegmentArrayOfSequenceNumericComponent> for TransportationSegmentArrayOfSequenceNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationSegmentArrayOfSequenceNumericComponent> for TransportationSegmentArrayOfSequenceNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationSegmentArrayOfSequenceNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationSegmentArrayOfSequenceNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportationSegmentArrayOfSequenceNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SequenceNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SequenceNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationSegmentArrayOfShipmentStageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ShipmentStage>,
}

impl AsMut<TransportationSegmentArrayOfShipmentStageComponent> for TransportationSegmentArrayOfShipmentStageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationSegmentArrayOfShipmentStageComponent> for TransportationSegmentArrayOfShipmentStageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationSegmentArrayOfShipmentStageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportationSegmentArrayOfShipmentStageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ShipmentStage) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ShipmentStage) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ShipmentStage> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ShipmentStage> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationSegmentArrayOfTransportExecutionPlanReferenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportExecutionPlanReferenceID>,
}

impl AsMut<TransportationSegmentArrayOfTransportExecutionPlanReferenceIDComponent> for TransportationSegmentArrayOfTransportExecutionPlanReferenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationSegmentArrayOfTransportExecutionPlanReferenceIDComponent> for TransportationSegmentArrayOfTransportExecutionPlanReferenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationSegmentArrayOfTransportExecutionPlanReferenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationSegmentArrayOfTransportExecutionPlanReferenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportationSegmentArrayOfTransportExecutionPlanReferenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportExecutionPlanReferenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportExecutionPlanReferenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportExecutionPlanReferenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportExecutionPlanReferenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationSegmentArrayOfTransportServiceProviderPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportServiceProviderParty>,
}

impl AsMut<TransportationSegmentArrayOfTransportServiceProviderPartyComponent> for TransportationSegmentArrayOfTransportServiceProviderPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationSegmentArrayOfTransportServiceProviderPartyComponent> for TransportationSegmentArrayOfTransportServiceProviderPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationSegmentArrayOfTransportServiceProviderPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationSegmentArrayOfTransportServiceProviderPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportationSegmentArrayOfTransportServiceProviderPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportServiceProviderParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportServiceProviderParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportServiceProviderParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportServiceProviderParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationSegmentArrayOfTransportationServiceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportationService>,
}

impl AsMut<TransportationSegmentArrayOfTransportationServiceComponent> for TransportationSegmentArrayOfTransportationServiceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationSegmentArrayOfTransportationServiceComponent> for TransportationSegmentArrayOfTransportationServiceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationSegmentArrayOfTransportationServiceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationSegmentArrayOfTransportationServiceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportationSegmentArrayOfTransportationServiceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportationService) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportationService) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportationService> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportationService> {
        self.items.iter()
    }
}


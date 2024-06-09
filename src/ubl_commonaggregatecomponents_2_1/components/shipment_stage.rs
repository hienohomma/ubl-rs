use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShipmentStage {
    #[serde(rename = "AcceptanceTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_transport_event: Option<ShipmentStageArrayOfAcceptanceTransportEventComponent>,
    #[serde(rename = "ActualArrivalTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_arrival_transport_event: Option<ShipmentStageArrayOfActualArrivalTransportEventComponent>,
    #[serde(rename = "ActualDepartureTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_departure_transport_event: Option<ShipmentStageArrayOfActualDepartureTransportEventComponent>,
    #[serde(rename = "ActualPickupTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_pickup_transport_event: Option<ShipmentStageArrayOfActualPickupTransportEventComponent>,
    #[serde(rename = "ActualWaypointTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_waypoint_transport_event: Option<ShipmentStageArrayOfActualWaypointTransportEventComponent>,
    #[serde(rename = "AvailabilityTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_transport_event: Option<ShipmentStageArrayOfAvailabilityTransportEventComponent>,
    #[serde(rename = "CarrierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_party: Option<ShipmentStageArrayOfCarrierPartyComponent>,
    #[serde(rename = "CrewMemberPerson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crew_member_person: Option<ShipmentStageArrayOfCrewMemberPersonComponent>,
    #[serde(rename = "CrewQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crew_quantity: Option<ShipmentStageArrayOfCrewQuantityComponent>,
    #[serde(rename = "CustomsAgentParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customs_agent_party: Option<ShipmentStageArrayOfCustomsAgentPartyComponent>,
    #[serde(rename = "DeliveryTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_transport_event: Option<ShipmentStageArrayOfDeliveryTransportEventComponent>,
    #[serde(rename = "DemurrageInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demurrage_instructions: Option<ShipmentStageArrayOfDemurrageInstructionsComponent>,
    #[serde(rename = "DetentionTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detention_transport_event: Option<ShipmentStageArrayOfDetentionTransportEventComponent>,
    #[serde(rename = "DischargeTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_transport_event: Option<ShipmentStageArrayOfDischargeTransportEventComponent>,
    #[serde(rename = "DriverPerson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_person: Option<ShipmentStageArrayOfDriverPersonComponent>,
    #[serde(rename = "DropoffTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropoff_transport_event: Option<ShipmentStageArrayOfDropoffTransportEventComponent>,
    #[serde(rename = "EstimatedArrivalTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_arrival_transport_event: Option<ShipmentStageArrayOfEstimatedArrivalTransportEventComponent>,
    #[serde(rename = "EstimatedDeliveryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_date: Option<ShipmentStageArrayOfEstimatedDeliveryDateComponent>,
    #[serde(rename = "EstimatedDeliveryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_time: Option<ShipmentStageArrayOfEstimatedDeliveryTimeComponent>,
    #[serde(rename = "EstimatedDepartureTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_departure_transport_event: Option<ShipmentStageArrayOfEstimatedDepartureTransportEventComponent>,
    #[serde(rename = "EstimatedTransitPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_transit_period: Option<ShipmentStageArrayOfEstimatedTransitPeriodComponent>,
    #[serde(rename = "ExaminationTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examination_transport_event: Option<ShipmentStageArrayOfExaminationTransportEventComponent>,
    #[serde(rename = "ExportationTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exportation_transport_event: Option<ShipmentStageArrayOfExportationTransportEventComponent>,
    #[serde(rename = "FreightAllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_allowance_charge: Option<ShipmentStageArrayOfFreightAllowanceChargeComponent>,
    #[serde(rename = "FreightChargeLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_charge_location: Option<ShipmentStageArrayOfFreightChargeLocationComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ShipmentStageArrayOfIDComponent>,
    #[serde(rename = "Instructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<ShipmentStageArrayOfInstructionsComponent>,
    #[serde(rename = "LoadingPortLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_port_location: Option<ShipmentStageArrayOfLoadingPortLocationComponent>,
    #[serde(rename = "LoadingSequenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_sequence_id: Option<ShipmentStageArrayOfLoadingSequenceIDComponent>,
    #[serde(rename = "LoadingTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_transport_event: Option<ShipmentStageArrayOfLoadingTransportEventComponent>,
    #[serde(rename = "MasterPerson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_person: Option<ShipmentStageArrayOfMasterPersonComponent>,
    #[serde(rename = "OnCarriageIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_carriage_indicator: Option<ShipmentStageArrayOfOnCarriageIndicatorComponent>,
    #[serde(rename = "OptionalTakeoverTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_takeover_transport_event: Option<ShipmentStageArrayOfOptionalTakeoverTransportEventComponent>,
    #[serde(rename = "PassengerPerson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_person: Option<ShipmentStageArrayOfPassengerPersonComponent>,
    #[serde(rename = "PassengerQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_quantity: Option<ShipmentStageArrayOfPassengerQuantityComponent>,
    #[serde(rename = "PlannedArrivalTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_arrival_transport_event: Option<ShipmentStageArrayOfPlannedArrivalTransportEventComponent>,
    #[serde(rename = "PlannedDepartureTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_departure_transport_event: Option<ShipmentStageArrayOfPlannedDepartureTransportEventComponent>,
    #[serde(rename = "PlannedWaypointTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_waypoint_transport_event: Option<ShipmentStageArrayOfPlannedWaypointTransportEventComponent>,
    #[serde(rename = "PreCarriageIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_carriage_indicator: Option<ShipmentStageArrayOfPreCarriageIndicatorComponent>,
    #[serde(rename = "ReceiptTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_transport_event: Option<ShipmentStageArrayOfReceiptTransportEventComponent>,
    #[serde(rename = "ReportingPerson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_person: Option<ShipmentStageArrayOfReportingPersonComponent>,
    #[serde(rename = "RequestedArrivalTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_arrival_transport_event: Option<ShipmentStageArrayOfRequestedArrivalTransportEventComponent>,
    #[serde(rename = "RequestedDepartureTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_departure_transport_event: Option<ShipmentStageArrayOfRequestedDepartureTransportEventComponent>,
    #[serde(rename = "RequestedWaypointTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_waypoint_transport_event: Option<ShipmentStageArrayOfRequestedWaypointTransportEventComponent>,
    #[serde(rename = "RequiredDeliveryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_delivery_date: Option<ShipmentStageArrayOfRequiredDeliveryDateComponent>,
    #[serde(rename = "RequiredDeliveryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_delivery_time: Option<ShipmentStageArrayOfRequiredDeliveryTimeComponent>,
    #[serde(rename = "SecurityOfficerPerson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_officer_person: Option<ShipmentStageArrayOfSecurityOfficerPersonComponent>,
    #[serde(rename = "ShipsSurgeonPerson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ships_surgeon_person: Option<ShipmentStageArrayOfShipsSurgeonPersonComponent>,
    #[serde(rename = "StorageTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_transport_event: Option<ShipmentStageArrayOfStorageTransportEventComponent>,
    #[serde(rename = "SuccessiveSequenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successive_sequence_id: Option<ShipmentStageArrayOfSuccessiveSequenceIDComponent>,
    #[serde(rename = "TakeoverTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub takeover_transport_event: Option<ShipmentStageArrayOfTakeoverTransportEventComponent>,
    #[serde(rename = "TerminalOperatorParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_operator_party: Option<ShipmentStageArrayOfTerminalOperatorPartyComponent>,
    #[serde(rename = "TransitDirectionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_direction_code: Option<ShipmentStageArrayOfTransitDirectionCodeComponent>,
    #[serde(rename = "TransitPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_period: Option<ShipmentStageArrayOfTransitPeriodComponent>,
    #[serde(rename = "TransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_event: Option<ShipmentStageArrayOfTransportEventComponent>,
    #[serde(rename = "TransportMeans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_means: Option<ShipmentStageArrayOfTransportMeansComponent>,
    #[serde(rename = "TransportMeansTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_means_type_code: Option<ShipmentStageArrayOfTransportMeansTypeCodeComponent>,
    #[serde(rename = "TransportModeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_mode_code: Option<ShipmentStageArrayOfTransportModeCodeComponent>,
    #[serde(rename = "TransshipPortLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transship_port_location: Option<ShipmentStageArrayOfTransshipPortLocationComponent>,
    #[serde(rename = "UnloadingPortLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unloading_port_location: Option<ShipmentStageArrayOfUnloadingPortLocationComponent>,
    #[serde(rename = "WarehousingTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehousing_transport_event: Option<ShipmentStageArrayOfWarehousingTransportEventComponent>,
}

impl AsMut<ShipmentStage> for ShipmentStage {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStage> for ShipmentStage {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.freight_allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.freight_allowance_charge", e));
            }
        }
        if let Some(v) = &self.exportation_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.exportation_transport_event", e));
            }
        }
        if let Some(v) = &self.carrier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.carrier_party", e));
            }
        }
        if let Some(v) = &self.master_person {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.master_person", e));
            }
        }
        if let Some(v) = &self.loading_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.loading_transport_event", e));
            }
        }
        if let Some(v) = &self.driver_person {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.driver_person", e));
            }
        }
        if let Some(v) = &self.security_officer_person {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.security_officer_person", e));
            }
        }
        if let Some(v) = &self.planned_waypoint_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.planned_waypoint_transport_event", e));
            }
        }
        if let Some(v) = &self.estimated_delivery_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.estimated_delivery_date", e));
            }
        }
        if let Some(v) = &self.takeover_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.takeover_transport_event", e));
            }
        }
        if let Some(v) = &self.actual_departure_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.actual_departure_transport_event", e));
            }
        }
        if let Some(v) = &self.instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.instructions", e));
            }
        }
        if let Some(v) = &self.passenger_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.passenger_quantity", e));
            }
        }
        if let Some(v) = &self.required_delivery_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.required_delivery_date", e));
            }
        }
        if let Some(v) = &self.freight_charge_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.freight_charge_location", e));
            }
        }
        if let Some(v) = &self.unloading_port_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.unloading_port_location", e));
            }
        }
        if let Some(v) = &self.reporting_person {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.reporting_person", e));
            }
        }
        if let Some(v) = &self.actual_pickup_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.actual_pickup_transport_event", e));
            }
        }
        if let Some(v) = &self.crew_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.crew_quantity", e));
            }
        }
        if let Some(v) = &self.optional_takeover_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.optional_takeover_transport_event", e));
            }
        }
        if let Some(v) = &self.actual_waypoint_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.actual_waypoint_transport_event", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.id", e));
            }
        }
        if let Some(v) = &self.ships_surgeon_person {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.ships_surgeon_person", e));
            }
        }
        if let Some(v) = &self.acceptance_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.acceptance_transport_event", e));
            }
        }
        if let Some(v) = &self.storage_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.storage_transport_event", e));
            }
        }
        if let Some(v) = &self.customs_agent_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.customs_agent_party", e));
            }
        }
        if let Some(v) = &self.receipt_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.receipt_transport_event", e));
            }
        }
        if let Some(v) = &self.estimated_arrival_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.estimated_arrival_transport_event", e));
            }
        }
        if let Some(v) = &self.planned_departure_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.planned_departure_transport_event", e));
            }
        }
        if let Some(v) = &self.demurrage_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.demurrage_instructions", e));
            }
        }
        if let Some(v) = &self.pre_carriage_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.pre_carriage_indicator", e));
            }
        }
        if let Some(v) = &self.transit_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.transit_period", e));
            }
        }
        if let Some(v) = &self.dropoff_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.dropoff_transport_event", e));
            }
        }
        if let Some(v) = &self.passenger_person {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.passenger_person", e));
            }
        }
        if let Some(v) = &self.transport_means {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.transport_means", e));
            }
        }
        if let Some(v) = &self.requested_arrival_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.requested_arrival_transport_event", e));
            }
        }
        if let Some(v) = &self.loading_sequence_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.loading_sequence_id", e));
            }
        }
        if let Some(v) = &self.requested_departure_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.requested_departure_transport_event", e));
            }
        }
        if let Some(v) = &self.estimated_departure_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.estimated_departure_transport_event", e));
            }
        }
        if let Some(v) = &self.successive_sequence_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.successive_sequence_id", e));
            }
        }
        if let Some(v) = &self.required_delivery_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.required_delivery_time", e));
            }
        }
        if let Some(v) = &self.transport_means_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.transport_means_type_code", e));
            }
        }
        if let Some(v) = &self.transship_port_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.transship_port_location", e));
            }
        }
        if let Some(v) = &self.availability_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.availability_transport_event", e));
            }
        }
        if let Some(v) = &self.transport_mode_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.transport_mode_code", e));
            }
        }
        if let Some(v) = &self.detention_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.detention_transport_event", e));
            }
        }
        if let Some(v) = &self.estimated_delivery_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.estimated_delivery_time", e));
            }
        }
        if let Some(v) = &self.warehousing_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.warehousing_transport_event", e));
            }
        }
        if let Some(v) = &self.planned_arrival_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.planned_arrival_transport_event", e));
            }
        }
        if let Some(v) = &self.transit_direction_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.transit_direction_code", e));
            }
        }
        if let Some(v) = &self.loading_port_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.loading_port_location", e));
            }
        }
        if let Some(v) = &self.transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.transport_event", e));
            }
        }
        if let Some(v) = &self.on_carriage_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.on_carriage_indicator", e));
            }
        }
        if let Some(v) = &self.requested_waypoint_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.requested_waypoint_transport_event", e));
            }
        }
        if let Some(v) = &self.crew_member_person {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.crew_member_person", e));
            }
        }
        if let Some(v) = &self.actual_arrival_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.actual_arrival_transport_event", e));
            }
        }
        if let Some(v) = &self.terminal_operator_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.terminal_operator_party", e));
            }
        }
        if let Some(v) = &self.discharge_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.discharge_transport_event", e));
            }
        }
        if let Some(v) = &self.estimated_transit_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.estimated_transit_period", e));
            }
        }
        if let Some(v) = &self.examination_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.examination_transport_event", e));
            }
        }
        if let Some(v) = &self.delivery_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShipmentStage.delivery_transport_event", e));
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

impl ShipmentStage {
    pub fn title() -> &'static str {
        "Shipment Stage. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe one stage of movement in a transport of goods."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            demurrage_instructions: None,
            examination_transport_event: None,
            loading_port_location: None,
            on_carriage_indicator: None,
            optional_takeover_transport_event: None,
            passenger_quantity: None,
            takeover_transport_event: None,
            detention_transport_event: None,
            availability_transport_event: None,
            pre_carriage_indicator: None,
            transport_event: None,
            transport_means_type_code: None,
            instructions: None,
            estimated_arrival_transport_event: None,
            requested_arrival_transport_event: None,
            successive_sequence_id: None,
            unloading_port_location: None,
            requested_waypoint_transport_event: None,
            crew_member_person: None,
            requested_departure_transport_event: None,
            carrier_party: None,
            estimated_transit_period: None,
            actual_pickup_transport_event: None,
            master_person: None,
            security_officer_person: None,
            crew_quantity: None,
            id: None,
            reporting_person: None,
            freight_charge_location: None,
            estimated_delivery_time: None,
            planned_arrival_transport_event: None,
            warehousing_transport_event: None,
            estimated_delivery_date: None,
            exportation_transport_event: None,
            customs_agent_party: None,
            terminal_operator_party: None,
            planned_departure_transport_event: None,
            actual_waypoint_transport_event: None,
            storage_transport_event: None,
            transit_direction_code: None,
            actual_arrival_transport_event: None,
            loading_sequence_id: None,
            receipt_transport_event: None,
            loading_transport_event: None,
            transship_port_location: None,
            transport_means: None,
            freight_allowance_charge: None,
            transport_mode_code: None,
            required_delivery_date: None,
            ships_surgeon_person: None,
            dropoff_transport_event: None,
            required_delivery_time: None,
            driver_person: None,
            delivery_transport_event: None,
            actual_departure_transport_event: None,
            passenger_person: None,
            transit_period: None,
            discharge_transport_event: None,
            estimated_departure_transport_event: None,
            acceptance_transport_event: None,
            planned_waypoint_transport_event: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfAcceptanceTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AcceptanceTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfAcceptanceTransportEventComponent> for ShipmentStageArrayOfAcceptanceTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfAcceptanceTransportEventComponent> for ShipmentStageArrayOfAcceptanceTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfAcceptanceTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfAcceptanceTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfAcceptanceTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AcceptanceTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AcceptanceTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AcceptanceTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AcceptanceTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfActualArrivalTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ActualArrivalTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfActualArrivalTransportEventComponent> for ShipmentStageArrayOfActualArrivalTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfActualArrivalTransportEventComponent> for ShipmentStageArrayOfActualArrivalTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfActualArrivalTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfActualArrivalTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfActualArrivalTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ActualArrivalTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ActualArrivalTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ActualArrivalTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ActualArrivalTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfActualDepartureTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ActualDepartureTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfActualDepartureTransportEventComponent> for ShipmentStageArrayOfActualDepartureTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfActualDepartureTransportEventComponent> for ShipmentStageArrayOfActualDepartureTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfActualDepartureTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfActualDepartureTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfActualDepartureTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ActualDepartureTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ActualDepartureTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ActualDepartureTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ActualDepartureTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfActualPickupTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ActualPickupTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfActualPickupTransportEventComponent> for ShipmentStageArrayOfActualPickupTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfActualPickupTransportEventComponent> for ShipmentStageArrayOfActualPickupTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfActualPickupTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfActualPickupTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfActualPickupTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ActualPickupTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ActualPickupTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ActualPickupTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ActualPickupTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfActualWaypointTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ActualWaypointTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfActualWaypointTransportEventComponent> for ShipmentStageArrayOfActualWaypointTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfActualWaypointTransportEventComponent> for ShipmentStageArrayOfActualWaypointTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfActualWaypointTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfActualWaypointTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfActualWaypointTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ActualWaypointTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ActualWaypointTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ActualWaypointTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ActualWaypointTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfAvailabilityTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AvailabilityTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfAvailabilityTransportEventComponent> for ShipmentStageArrayOfAvailabilityTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfAvailabilityTransportEventComponent> for ShipmentStageArrayOfAvailabilityTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfAvailabilityTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfAvailabilityTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfAvailabilityTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AvailabilityTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AvailabilityTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AvailabilityTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AvailabilityTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfCarrierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CarrierParty>,
}

impl AsMut<ShipmentStageArrayOfCarrierPartyComponent> for ShipmentStageArrayOfCarrierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfCarrierPartyComponent> for ShipmentStageArrayOfCarrierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfCarrierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfCarrierPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CarrierParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CarrierParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CarrierParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CarrierParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfCrewMemberPersonComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CrewMemberPerson>,
}

impl AsMut<ShipmentStageArrayOfCrewMemberPersonComponent> for ShipmentStageArrayOfCrewMemberPersonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfCrewMemberPersonComponent> for ShipmentStageArrayOfCrewMemberPersonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfCrewMemberPersonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfCrewMemberPersonComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CrewMemberPerson) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CrewMemberPerson) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CrewMemberPerson> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CrewMemberPerson> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfCrewQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CrewQuantity>,
}

impl AsMut<ShipmentStageArrayOfCrewQuantityComponent> for ShipmentStageArrayOfCrewQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfCrewQuantityComponent> for ShipmentStageArrayOfCrewQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfCrewQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfCrewQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfCrewQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CrewQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CrewQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CrewQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CrewQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfCustomsAgentPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CustomsAgentParty>,
}

impl AsMut<ShipmentStageArrayOfCustomsAgentPartyComponent> for ShipmentStageArrayOfCustomsAgentPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfCustomsAgentPartyComponent> for ShipmentStageArrayOfCustomsAgentPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfCustomsAgentPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfCustomsAgentPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfCustomsAgentPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CustomsAgentParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CustomsAgentParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CustomsAgentParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CustomsAgentParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfDeliveryTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfDeliveryTransportEventComponent> for ShipmentStageArrayOfDeliveryTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfDeliveryTransportEventComponent> for ShipmentStageArrayOfDeliveryTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfDeliveryTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfDeliveryTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfDeliveryTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfDemurrageInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DemurrageInstructions>,
}

impl AsMut<ShipmentStageArrayOfDemurrageInstructionsComponent> for ShipmentStageArrayOfDemurrageInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfDemurrageInstructionsComponent> for ShipmentStageArrayOfDemurrageInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfDemurrageInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfDemurrageInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DemurrageInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DemurrageInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DemurrageInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DemurrageInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfDetentionTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DetentionTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfDetentionTransportEventComponent> for ShipmentStageArrayOfDetentionTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfDetentionTransportEventComponent> for ShipmentStageArrayOfDetentionTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfDetentionTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfDetentionTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DetentionTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DetentionTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DetentionTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DetentionTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfDischargeTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DischargeTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfDischargeTransportEventComponent> for ShipmentStageArrayOfDischargeTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfDischargeTransportEventComponent> for ShipmentStageArrayOfDischargeTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfDischargeTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfDischargeTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfDischargeTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DischargeTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DischargeTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DischargeTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DischargeTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfDriverPersonComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DriverPerson>,
}

impl AsMut<ShipmentStageArrayOfDriverPersonComponent> for ShipmentStageArrayOfDriverPersonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfDriverPersonComponent> for ShipmentStageArrayOfDriverPersonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfDriverPersonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfDriverPersonComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DriverPerson) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DriverPerson) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DriverPerson> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DriverPerson> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfDropoffTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DropoffTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfDropoffTransportEventComponent> for ShipmentStageArrayOfDropoffTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfDropoffTransportEventComponent> for ShipmentStageArrayOfDropoffTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfDropoffTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfDropoffTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfDropoffTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DropoffTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DropoffTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DropoffTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DropoffTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfEstimatedArrivalTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EstimatedArrivalTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfEstimatedArrivalTransportEventComponent> for ShipmentStageArrayOfEstimatedArrivalTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfEstimatedArrivalTransportEventComponent> for ShipmentStageArrayOfEstimatedArrivalTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfEstimatedArrivalTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfEstimatedArrivalTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfEstimatedArrivalTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EstimatedArrivalTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EstimatedArrivalTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EstimatedArrivalTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EstimatedArrivalTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfEstimatedDeliveryDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EstimatedDeliveryDate>,
}

impl AsMut<ShipmentStageArrayOfEstimatedDeliveryDateComponent> for ShipmentStageArrayOfEstimatedDeliveryDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfEstimatedDeliveryDateComponent> for ShipmentStageArrayOfEstimatedDeliveryDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfEstimatedDeliveryDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfEstimatedDeliveryDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfEstimatedDeliveryDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EstimatedDeliveryDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EstimatedDeliveryDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EstimatedDeliveryDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EstimatedDeliveryDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfEstimatedDeliveryTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EstimatedDeliveryTime>,
}

impl AsMut<ShipmentStageArrayOfEstimatedDeliveryTimeComponent> for ShipmentStageArrayOfEstimatedDeliveryTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfEstimatedDeliveryTimeComponent> for ShipmentStageArrayOfEstimatedDeliveryTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfEstimatedDeliveryTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfEstimatedDeliveryTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfEstimatedDeliveryTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EstimatedDeliveryTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EstimatedDeliveryTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EstimatedDeliveryTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EstimatedDeliveryTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfEstimatedDepartureTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EstimatedDepartureTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfEstimatedDepartureTransportEventComponent> for ShipmentStageArrayOfEstimatedDepartureTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfEstimatedDepartureTransportEventComponent> for ShipmentStageArrayOfEstimatedDepartureTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfEstimatedDepartureTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfEstimatedDepartureTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfEstimatedDepartureTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EstimatedDepartureTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EstimatedDepartureTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EstimatedDepartureTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EstimatedDepartureTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfEstimatedTransitPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EstimatedTransitPeriod>,
}

impl AsMut<ShipmentStageArrayOfEstimatedTransitPeriodComponent> for ShipmentStageArrayOfEstimatedTransitPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfEstimatedTransitPeriodComponent> for ShipmentStageArrayOfEstimatedTransitPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfEstimatedTransitPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfEstimatedTransitPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfEstimatedTransitPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EstimatedTransitPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EstimatedTransitPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EstimatedTransitPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EstimatedTransitPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfExaminationTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExaminationTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfExaminationTransportEventComponent> for ShipmentStageArrayOfExaminationTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfExaminationTransportEventComponent> for ShipmentStageArrayOfExaminationTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfExaminationTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfExaminationTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfExaminationTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ExaminationTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ExaminationTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ExaminationTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ExaminationTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfExportationTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExportationTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfExportationTransportEventComponent> for ShipmentStageArrayOfExportationTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfExportationTransportEventComponent> for ShipmentStageArrayOfExportationTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfExportationTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfExportationTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfExportationTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ExportationTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ExportationTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ExportationTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ExportationTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfFreightAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge>,
}

impl AsMut<ShipmentStageArrayOfFreightAllowanceChargeComponent> for ShipmentStageArrayOfFreightAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfFreightAllowanceChargeComponent> for ShipmentStageArrayOfFreightAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfFreightAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfFreightAllowanceChargeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfFreightChargeLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FreightChargeLocation>,
}

impl AsMut<ShipmentStageArrayOfFreightChargeLocationComponent> for ShipmentStageArrayOfFreightChargeLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfFreightChargeLocationComponent> for ShipmentStageArrayOfFreightChargeLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfFreightChargeLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfFreightChargeLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfFreightChargeLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FreightChargeLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FreightChargeLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FreightChargeLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FreightChargeLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ShipmentStageArrayOfIDComponent> for ShipmentStageArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfIDComponent> for ShipmentStageArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfIDComponent {
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
pub struct ShipmentStageArrayOfInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Instructions>,
}

impl AsMut<ShipmentStageArrayOfInstructionsComponent> for ShipmentStageArrayOfInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfInstructionsComponent> for ShipmentStageArrayOfInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Instructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Instructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Instructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Instructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfLoadingPortLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LoadingPortLocation>,
}

impl AsMut<ShipmentStageArrayOfLoadingPortLocationComponent> for ShipmentStageArrayOfLoadingPortLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfLoadingPortLocationComponent> for ShipmentStageArrayOfLoadingPortLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfLoadingPortLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfLoadingPortLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfLoadingPortLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LoadingPortLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LoadingPortLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LoadingPortLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LoadingPortLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfLoadingSequenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LoadingSequenceID>,
}

impl AsMut<ShipmentStageArrayOfLoadingSequenceIDComponent> for ShipmentStageArrayOfLoadingSequenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfLoadingSequenceIDComponent> for ShipmentStageArrayOfLoadingSequenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfLoadingSequenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfLoadingSequenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfLoadingSequenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LoadingSequenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LoadingSequenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LoadingSequenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LoadingSequenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfLoadingTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LoadingTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfLoadingTransportEventComponent> for ShipmentStageArrayOfLoadingTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfLoadingTransportEventComponent> for ShipmentStageArrayOfLoadingTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfLoadingTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfLoadingTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfLoadingTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LoadingTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LoadingTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LoadingTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LoadingTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfMasterPersonComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MasterPerson>,
}

impl AsMut<ShipmentStageArrayOfMasterPersonComponent> for ShipmentStageArrayOfMasterPersonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfMasterPersonComponent> for ShipmentStageArrayOfMasterPersonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfMasterPersonComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfMasterPersonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfMasterPersonComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MasterPerson) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MasterPerson) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MasterPerson> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MasterPerson> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfOnCarriageIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OnCarriageIndicator>,
}

impl AsMut<ShipmentStageArrayOfOnCarriageIndicatorComponent> for ShipmentStageArrayOfOnCarriageIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfOnCarriageIndicatorComponent> for ShipmentStageArrayOfOnCarriageIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfOnCarriageIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfOnCarriageIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfOnCarriageIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OnCarriageIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OnCarriageIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OnCarriageIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OnCarriageIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfOptionalTakeoverTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OptionalTakeoverTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfOptionalTakeoverTransportEventComponent> for ShipmentStageArrayOfOptionalTakeoverTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfOptionalTakeoverTransportEventComponent> for ShipmentStageArrayOfOptionalTakeoverTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfOptionalTakeoverTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfOptionalTakeoverTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfOptionalTakeoverTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OptionalTakeoverTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OptionalTakeoverTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OptionalTakeoverTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OptionalTakeoverTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfPassengerPersonComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PassengerPerson>,
}

impl AsMut<ShipmentStageArrayOfPassengerPersonComponent> for ShipmentStageArrayOfPassengerPersonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfPassengerPersonComponent> for ShipmentStageArrayOfPassengerPersonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfPassengerPersonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfPassengerPersonComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PassengerPerson) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PassengerPerson) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PassengerPerson> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PassengerPerson> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfPassengerQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PassengerQuantity>,
}

impl AsMut<ShipmentStageArrayOfPassengerQuantityComponent> for ShipmentStageArrayOfPassengerQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfPassengerQuantityComponent> for ShipmentStageArrayOfPassengerQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfPassengerQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfPassengerQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfPassengerQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PassengerQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PassengerQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PassengerQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PassengerQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfPlannedArrivalTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PlannedArrivalTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfPlannedArrivalTransportEventComponent> for ShipmentStageArrayOfPlannedArrivalTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfPlannedArrivalTransportEventComponent> for ShipmentStageArrayOfPlannedArrivalTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfPlannedArrivalTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfPlannedArrivalTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfPlannedArrivalTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PlannedArrivalTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PlannedArrivalTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PlannedArrivalTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PlannedArrivalTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfPlannedDepartureTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PlannedDepartureTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfPlannedDepartureTransportEventComponent> for ShipmentStageArrayOfPlannedDepartureTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfPlannedDepartureTransportEventComponent> for ShipmentStageArrayOfPlannedDepartureTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfPlannedDepartureTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfPlannedDepartureTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfPlannedDepartureTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PlannedDepartureTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PlannedDepartureTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PlannedDepartureTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PlannedDepartureTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfPlannedWaypointTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PlannedWaypointTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfPlannedWaypointTransportEventComponent> for ShipmentStageArrayOfPlannedWaypointTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfPlannedWaypointTransportEventComponent> for ShipmentStageArrayOfPlannedWaypointTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfPlannedWaypointTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfPlannedWaypointTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PlannedWaypointTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PlannedWaypointTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PlannedWaypointTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PlannedWaypointTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfPreCarriageIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PreCarriageIndicator>,
}

impl AsMut<ShipmentStageArrayOfPreCarriageIndicatorComponent> for ShipmentStageArrayOfPreCarriageIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfPreCarriageIndicatorComponent> for ShipmentStageArrayOfPreCarriageIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfPreCarriageIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfPreCarriageIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfPreCarriageIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PreCarriageIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PreCarriageIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PreCarriageIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PreCarriageIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfReceiptTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReceiptTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfReceiptTransportEventComponent> for ShipmentStageArrayOfReceiptTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfReceiptTransportEventComponent> for ShipmentStageArrayOfReceiptTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfReceiptTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfReceiptTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfReceiptTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReceiptTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReceiptTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReceiptTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReceiptTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfReportingPersonComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReportingPerson>,
}

impl AsMut<ShipmentStageArrayOfReportingPersonComponent> for ShipmentStageArrayOfReportingPersonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfReportingPersonComponent> for ShipmentStageArrayOfReportingPersonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfReportingPersonComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfReportingPersonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfReportingPersonComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReportingPerson) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReportingPerson) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReportingPerson> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReportingPerson> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfRequestedArrivalTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequestedArrivalTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfRequestedArrivalTransportEventComponent> for ShipmentStageArrayOfRequestedArrivalTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfRequestedArrivalTransportEventComponent> for ShipmentStageArrayOfRequestedArrivalTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfRequestedArrivalTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfRequestedArrivalTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfRequestedArrivalTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequestedArrivalTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequestedArrivalTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequestedArrivalTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequestedArrivalTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfRequestedDepartureTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequestedDepartureTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfRequestedDepartureTransportEventComponent> for ShipmentStageArrayOfRequestedDepartureTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfRequestedDepartureTransportEventComponent> for ShipmentStageArrayOfRequestedDepartureTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfRequestedDepartureTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfRequestedDepartureTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfRequestedDepartureTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequestedDepartureTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequestedDepartureTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequestedDepartureTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequestedDepartureTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfRequestedWaypointTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequestedWaypointTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfRequestedWaypointTransportEventComponent> for ShipmentStageArrayOfRequestedWaypointTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfRequestedWaypointTransportEventComponent> for ShipmentStageArrayOfRequestedWaypointTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfRequestedWaypointTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfRequestedWaypointTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequestedWaypointTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequestedWaypointTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequestedWaypointTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequestedWaypointTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfRequiredDeliveryDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RequiredDeliveryDate>,
}

impl AsMut<ShipmentStageArrayOfRequiredDeliveryDateComponent> for ShipmentStageArrayOfRequiredDeliveryDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfRequiredDeliveryDateComponent> for ShipmentStageArrayOfRequiredDeliveryDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfRequiredDeliveryDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfRequiredDeliveryDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfRequiredDeliveryDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RequiredDeliveryDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RequiredDeliveryDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RequiredDeliveryDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RequiredDeliveryDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfRequiredDeliveryTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RequiredDeliveryTime>,
}

impl AsMut<ShipmentStageArrayOfRequiredDeliveryTimeComponent> for ShipmentStageArrayOfRequiredDeliveryTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfRequiredDeliveryTimeComponent> for ShipmentStageArrayOfRequiredDeliveryTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfRequiredDeliveryTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfRequiredDeliveryTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfRequiredDeliveryTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RequiredDeliveryTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RequiredDeliveryTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RequiredDeliveryTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RequiredDeliveryTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfSecurityOfficerPersonComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SecurityOfficerPerson>,
}

impl AsMut<ShipmentStageArrayOfSecurityOfficerPersonComponent> for ShipmentStageArrayOfSecurityOfficerPersonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfSecurityOfficerPersonComponent> for ShipmentStageArrayOfSecurityOfficerPersonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfSecurityOfficerPersonComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfSecurityOfficerPersonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfSecurityOfficerPersonComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SecurityOfficerPerson) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SecurityOfficerPerson) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SecurityOfficerPerson> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SecurityOfficerPerson> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfShipsSurgeonPersonComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ShipsSurgeonPerson>,
}

impl AsMut<ShipmentStageArrayOfShipsSurgeonPersonComponent> for ShipmentStageArrayOfShipsSurgeonPersonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfShipsSurgeonPersonComponent> for ShipmentStageArrayOfShipsSurgeonPersonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfShipsSurgeonPersonComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfShipsSurgeonPersonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfShipsSurgeonPersonComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ShipsSurgeonPerson) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ShipsSurgeonPerson) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ShipsSurgeonPerson> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ShipsSurgeonPerson> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfStorageTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::StorageTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfStorageTransportEventComponent> for ShipmentStageArrayOfStorageTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfStorageTransportEventComponent> for ShipmentStageArrayOfStorageTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfStorageTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfStorageTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfStorageTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::StorageTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::StorageTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::StorageTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::StorageTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfSuccessiveSequenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SuccessiveSequenceID>,
}

impl AsMut<ShipmentStageArrayOfSuccessiveSequenceIDComponent> for ShipmentStageArrayOfSuccessiveSequenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfSuccessiveSequenceIDComponent> for ShipmentStageArrayOfSuccessiveSequenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfSuccessiveSequenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfSuccessiveSequenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfSuccessiveSequenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SuccessiveSequenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SuccessiveSequenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SuccessiveSequenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SuccessiveSequenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfTakeoverTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TakeoverTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfTakeoverTransportEventComponent> for ShipmentStageArrayOfTakeoverTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfTakeoverTransportEventComponent> for ShipmentStageArrayOfTakeoverTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTakeoverTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTakeoverTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfTakeoverTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TakeoverTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TakeoverTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TakeoverTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TakeoverTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfTerminalOperatorPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TerminalOperatorParty>,
}

impl AsMut<ShipmentStageArrayOfTerminalOperatorPartyComponent> for ShipmentStageArrayOfTerminalOperatorPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfTerminalOperatorPartyComponent> for ShipmentStageArrayOfTerminalOperatorPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTerminalOperatorPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTerminalOperatorPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfTerminalOperatorPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TerminalOperatorParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TerminalOperatorParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TerminalOperatorParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TerminalOperatorParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfTransitDirectionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransitDirectionCode>,
}

impl AsMut<ShipmentStageArrayOfTransitDirectionCodeComponent> for ShipmentStageArrayOfTransitDirectionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfTransitDirectionCodeComponent> for ShipmentStageArrayOfTransitDirectionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransitDirectionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransitDirectionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfTransitDirectionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransitDirectionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransitDirectionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransitDirectionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransitDirectionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfTransitPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransitPeriod>,
}

impl AsMut<ShipmentStageArrayOfTransitPeriodComponent> for ShipmentStageArrayOfTransitPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfTransitPeriodComponent> for ShipmentStageArrayOfTransitPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransitPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransitPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfTransitPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransitPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransitPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransitPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransitPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportEvent>,
}

impl AsMut<ShipmentStageArrayOfTransportEventComponent> for ShipmentStageArrayOfTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfTransportEventComponent> for ShipmentStageArrayOfTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfTransportMeansComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportMeans>,
}

impl AsMut<ShipmentStageArrayOfTransportMeansComponent> for ShipmentStageArrayOfTransportMeansComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfTransportMeansComponent> for ShipmentStageArrayOfTransportMeansComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransportMeansComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransportMeansComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfTransportMeansComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportMeans) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportMeans) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportMeans> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportMeans> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfTransportMeansTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportMeansTypeCode>,
}

impl AsMut<ShipmentStageArrayOfTransportMeansTypeCodeComponent> for ShipmentStageArrayOfTransportMeansTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfTransportMeansTypeCodeComponent> for ShipmentStageArrayOfTransportMeansTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransportMeansTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransportMeansTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfTransportMeansTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportMeansTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportMeansTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportMeansTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportMeansTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfTransportModeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportModeCode>,
}

impl AsMut<ShipmentStageArrayOfTransportModeCodeComponent> for ShipmentStageArrayOfTransportModeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfTransportModeCodeComponent> for ShipmentStageArrayOfTransportModeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransportModeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransportModeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfTransportModeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportModeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportModeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportModeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportModeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfTransshipPortLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransshipPortLocation>,
}

impl AsMut<ShipmentStageArrayOfTransshipPortLocationComponent> for ShipmentStageArrayOfTransshipPortLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfTransshipPortLocationComponent> for ShipmentStageArrayOfTransshipPortLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransshipPortLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfTransshipPortLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfTransshipPortLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransshipPortLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransshipPortLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransshipPortLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransshipPortLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfUnloadingPortLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::UnloadingPortLocation>,
}

impl AsMut<ShipmentStageArrayOfUnloadingPortLocationComponent> for ShipmentStageArrayOfUnloadingPortLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfUnloadingPortLocationComponent> for ShipmentStageArrayOfUnloadingPortLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfUnloadingPortLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfUnloadingPortLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfUnloadingPortLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::UnloadingPortLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::UnloadingPortLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::UnloadingPortLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::UnloadingPortLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentStageArrayOfWarehousingTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WarehousingTransportEvent>,
}

impl AsMut<ShipmentStageArrayOfWarehousingTransportEventComponent> for ShipmentStageArrayOfWarehousingTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentStageArrayOfWarehousingTransportEventComponent> for ShipmentStageArrayOfWarehousingTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfWarehousingTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentStageArrayOfWarehousingTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentStageArrayOfWarehousingTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WarehousingTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WarehousingTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WarehousingTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WarehousingTransportEvent> {
        self.items.iter()
    }
}


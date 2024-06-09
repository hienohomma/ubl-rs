use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransportEquipment {
    #[serde(rename = "AirFlowPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_flow_percent: Option<TransportEquipmentArrayOfAirFlowPercentComponent>,
    #[serde(rename = "AnimalFoodApprovedIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animal_food_approved_indicator: Option<TransportEquipmentArrayOfAnimalFoodApprovedIndicatorComponent>,
    #[serde(rename = "ApplicableTransportMeans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_transport_means: Option<TransportEquipmentArrayOfApplicableTransportMeansComponent>,
    #[serde(rename = "AttachedTransportEquipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_transport_equipment: Option<TransportEquipmentArrayOfAttachedTransportEquipmentComponent>,
    #[serde(rename = "Characteristics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characteristics: Option<TransportEquipmentArrayOfCharacteristicsComponent>,
    #[serde(rename = "ContainedInTransportEquipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contained_in_transport_equipment: Option<TransportEquipmentArrayOfContainedInTransportEquipmentComponent>,
    #[serde(rename = "DamageRemarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_remarks: Option<TransportEquipmentArrayOfDamageRemarksComponent>,
    #[serde(rename = "DangerousGoodsApprovedIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangerous_goods_approved_indicator: Option<TransportEquipmentArrayOfDangerousGoodsApprovedIndicatorComponent>,
    #[serde(rename = "Delivery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<TransportEquipmentArrayOfDeliveryComponent>,
    #[serde(rename = "DeliveryTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_transport_event: Option<TransportEquipmentArrayOfDeliveryTransportEventComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TransportEquipmentArrayOfDescriptionComponent>,
    #[serde(rename = "Despatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch: Option<TransportEquipmentArrayOfDespatchComponent>,
    #[serde(rename = "DispositionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposition_code: Option<TransportEquipmentArrayOfDispositionCodeComponent>,
    #[serde(rename = "FreightAllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_allowance_charge: Option<TransportEquipmentArrayOfFreightAllowanceChargeComponent>,
    #[serde(rename = "FullnessIndicationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullness_indication_code: Option<TransportEquipmentArrayOfFullnessIndicationCodeComponent>,
    #[serde(rename = "GoodsItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_item: Option<TransportEquipmentArrayOfGoodsItemComponent>,
    #[serde(rename = "GrossVolumeMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_volume_measure: Option<TransportEquipmentArrayOfGrossVolumeMeasureComponent>,
    #[serde(rename = "GrossWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_weight_measure: Option<TransportEquipmentArrayOfGrossWeightMeasureComponent>,
    #[serde(rename = "HandlingTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handling_transport_event: Option<TransportEquipmentArrayOfHandlingTransportEventComponent>,
    #[serde(rename = "HaulageTradingTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haulage_trading_terms: Option<TransportEquipmentArrayOfHaulageTradingTermsComponent>,
    #[serde(rename = "HazardousGoodsTransit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_goods_transit: Option<TransportEquipmentArrayOfHazardousGoodsTransitComponent>,
    #[serde(rename = "HumanFoodApprovedIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_food_approved_indicator: Option<TransportEquipmentArrayOfHumanFoodApprovedIndicatorComponent>,
    #[serde(rename = "HumidityPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity_percent: Option<TransportEquipmentArrayOfHumidityPercentComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TransportEquipmentArrayOfIDComponent>,
    #[serde(rename = "Information")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<TransportEquipmentArrayOfInformationComponent>,
    #[serde(rename = "LegalStatusIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_status_indicator: Option<TransportEquipmentArrayOfLegalStatusIndicatorComponent>,
    #[serde(rename = "LoadingLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_location: Option<TransportEquipmentArrayOfLoadingLocationComponent>,
    #[serde(rename = "LoadingProofParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_proof_party: Option<TransportEquipmentArrayOfLoadingProofPartyComponent>,
    #[serde(rename = "LoadingTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_transport_event: Option<TransportEquipmentArrayOfLoadingTransportEventComponent>,
    #[serde(rename = "MaximumTemperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_temperature: Option<TransportEquipmentArrayOfMaximumTemperatureComponent>,
    #[serde(rename = "MeasurementDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_dimension: Option<TransportEquipmentArrayOfMeasurementDimensionComponent>,
    #[serde(rename = "MinimumTemperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_temperature: Option<TransportEquipmentArrayOfMinimumTemperatureComponent>,
    #[serde(rename = "OperatingParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_party: Option<TransportEquipmentArrayOfOperatingPartyComponent>,
    #[serde(rename = "OwnerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_party: Option<TransportEquipmentArrayOfOwnerPartyComponent>,
    #[serde(rename = "OwnerTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type_code: Option<TransportEquipmentArrayOfOwnerTypeCodeComponent>,
    #[serde(rename = "Package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<TransportEquipmentArrayOfPackageComponent>,
    #[serde(rename = "PackagedTransportHandlingUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaged_transport_handling_unit: Option<TransportEquipmentArrayOfPackagedTransportHandlingUnitComponent>,
    #[serde(rename = "Pickup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup: Option<TransportEquipmentArrayOfPickupComponent>,
    #[serde(rename = "PickupTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_transport_event: Option<TransportEquipmentArrayOfPickupTransportEventComponent>,
    #[serde(rename = "PositioningTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positioning_transport_event: Option<TransportEquipmentArrayOfPositioningTransportEventComponent>,
    #[serde(rename = "PowerIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_indicator: Option<TransportEquipmentArrayOfPowerIndicatorComponent>,
    #[serde(rename = "ProviderParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_party: Option<TransportEquipmentArrayOfProviderPartyComponent>,
    #[serde(rename = "ProviderTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type_code: Option<TransportEquipmentArrayOfProviderTypeCodeComponent>,
    #[serde(rename = "QuarantineTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarantine_transport_event: Option<TransportEquipmentArrayOfQuarantineTransportEventComponent>,
    #[serde(rename = "ReferencedConsignmentID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_consignment_id: Option<TransportEquipmentArrayOfReferencedConsignmentIDComponent>,
    #[serde(rename = "RefrigeratedIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refrigerated_indicator: Option<TransportEquipmentArrayOfRefrigeratedIndicatorComponent>,
    #[serde(rename = "RefrigerationOnIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refrigeration_on_indicator: Option<TransportEquipmentArrayOfRefrigerationOnIndicatorComponent>,
    #[serde(rename = "ReturnabilityIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnability_indicator: Option<TransportEquipmentArrayOfReturnabilityIndicatorComponent>,
    #[serde(rename = "ServiceAllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_allowance_charge: Option<TransportEquipmentArrayOfServiceAllowanceChargeComponent>,
    #[serde(rename = "ShipmentDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_document_reference: Option<TransportEquipmentArrayOfShipmentDocumentReferenceComponent>,
    #[serde(rename = "SizeTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_type_code: Option<TransportEquipmentArrayOfSizeTypeCodeComponent>,
    #[serde(rename = "SpecialTransportRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_transport_requirements: Option<TransportEquipmentArrayOfSpecialTransportRequirementsComponent>,
    #[serde(rename = "StorageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<TransportEquipmentArrayOfStorageLocationComponent>,
    #[serde(rename = "SupplierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_party: Option<TransportEquipmentArrayOfSupplierPartyComponent>,
    #[serde(rename = "TareWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tare_weight_measure: Option<TransportEquipmentArrayOfTareWeightMeasureComponent>,
    #[serde(rename = "TraceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<TransportEquipmentArrayOfTraceIDComponent>,
    #[serde(rename = "TrackingDeviceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_device_code: Option<TransportEquipmentArrayOfTrackingDeviceCodeComponent>,
    #[serde(rename = "TransportEquipmentSeal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_equipment_seal: Option<TransportEquipmentArrayOfTransportEquipmentSealComponent>,
    #[serde(rename = "TransportEquipmentTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_equipment_type_code: Option<TransportEquipmentArrayOfTransportEquipmentTypeCodeComponent>,
    #[serde(rename = "TransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_event: Option<TransportEquipmentArrayOfTransportEventComponent>,
    #[serde(rename = "UnloadingLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unloading_location: Option<TransportEquipmentArrayOfUnloadingLocationComponent>,
}

impl AsMut<TransportEquipment> for TransportEquipment {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipment> for TransportEquipment {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.power_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.power_indicator", e));
            }
        }
        if let Some(v) = &self.quarantine_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.quarantine_transport_event", e));
            }
        }
        if let Some(v) = &self.maximum_temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.maximum_temperature", e));
            }
        }
        if let Some(v) = &self.applicable_transport_means {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.applicable_transport_means", e));
            }
        }
        if let Some(v) = &self.service_allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.service_allowance_charge", e));
            }
        }
        if let Some(v) = &self.attached_transport_equipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.attached_transport_equipment", e));
            }
        }
        if let Some(v) = &self.information {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.information", e));
            }
        }
        if let Some(v) = &self.loading_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.loading_location", e));
            }
        }
        if let Some(v) = &self.minimum_temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.minimum_temperature", e));
            }
        }
        if let Some(v) = &self.damage_remarks {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.damage_remarks", e));
            }
        }
        if let Some(v) = &self.delivery {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.delivery", e));
            }
        }
        if let Some(v) = &self.returnability_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.returnability_indicator", e));
            }
        }
        if let Some(v) = &self.loading_proof_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.loading_proof_party", e));
            }
        }
        if let Some(v) = &self.loading_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.loading_transport_event", e));
            }
        }
        if let Some(v) = &self.legal_status_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.legal_status_indicator", e));
            }
        }
        if let Some(v) = &self.measurement_dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.measurement_dimension", e));
            }
        }
        if let Some(v) = &self.referenced_consignment_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.referenced_consignment_id", e));
            }
        }
        if let Some(v) = &self.haulage_trading_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.haulage_trading_terms", e));
            }
        }
        if let Some(v) = &self.positioning_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.positioning_transport_event", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.id", e));
            }
        }
        if let Some(v) = &self.operating_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.operating_party", e));
            }
        }
        if let Some(v) = &self.air_flow_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.air_flow_percent", e));
            }
        }
        if let Some(v) = &self.gross_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.gross_weight_measure", e));
            }
        }
        if let Some(v) = &self.characteristics {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.characteristics", e));
            }
        }
        if let Some(v) = &self.pickup {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.pickup", e));
            }
        }
        if let Some(v) = &self.tare_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.tare_weight_measure", e));
            }
        }
        if let Some(v) = &self.contained_in_transport_equipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.contained_in_transport_equipment", e));
            }
        }
        if let Some(v) = &self.provider_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.provider_party", e));
            }
        }
        if let Some(v) = &self.disposition_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.disposition_code", e));
            }
        }
        if let Some(v) = &self.transport_equipment_seal {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.transport_equipment_seal", e));
            }
        }
        if let Some(v) = &self.supplier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.supplier_party", e));
            }
        }
        if let Some(v) = &self.transport_equipment_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.transport_equipment_type_code", e));
            }
        }
        if let Some(v) = &self.fullness_indication_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.fullness_indication_code", e));
            }
        }
        if let Some(v) = &self.gross_volume_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.gross_volume_measure", e));
            }
        }
        if let Some(v) = &self.package {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.package", e));
            }
        }
        if let Some(v) = &self.animal_food_approved_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.animal_food_approved_indicator", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.description", e));
            }
        }
        if let Some(v) = &self.freight_allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.freight_allowance_charge", e));
            }
        }
        if let Some(v) = &self.humidity_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.humidity_percent", e));
            }
        }
        if let Some(v) = &self.shipment_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.shipment_document_reference", e));
            }
        }
        if let Some(v) = &self.human_food_approved_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.human_food_approved_indicator", e));
            }
        }
        if let Some(v) = &self.refrigerated_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.refrigerated_indicator", e));
            }
        }
        if let Some(v) = &self.trace_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.trace_id", e));
            }
        }
        if let Some(v) = &self.size_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.size_type_code", e));
            }
        }
        if let Some(v) = &self.despatch {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.despatch", e));
            }
        }
        if let Some(v) = &self.special_transport_requirements {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.special_transport_requirements", e));
            }
        }
        if let Some(v) = &self.handling_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.handling_transport_event", e));
            }
        }
        if let Some(v) = &self.goods_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.goods_item", e));
            }
        }
        if let Some(v) = &self.storage_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.storage_location", e));
            }
        }
        if let Some(v) = &self.transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.transport_event", e));
            }
        }
        if let Some(v) = &self.unloading_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.unloading_location", e));
            }
        }
        if let Some(v) = &self.dangerous_goods_approved_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.dangerous_goods_approved_indicator", e));
            }
        }
        if let Some(v) = &self.hazardous_goods_transit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.hazardous_goods_transit", e));
            }
        }
        if let Some(v) = &self.owner_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.owner_party", e));
            }
        }
        if let Some(v) = &self.packaged_transport_handling_unit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.packaged_transport_handling_unit", e));
            }
        }
        if let Some(v) = &self.pickup_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.pickup_transport_event", e));
            }
        }
        if let Some(v) = &self.provider_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.provider_type_code", e));
            }
        }
        if let Some(v) = &self.refrigeration_on_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.refrigeration_on_indicator", e));
            }
        }
        if let Some(v) = &self.delivery_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.delivery_transport_event", e));
            }
        }
        if let Some(v) = &self.owner_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.owner_type_code", e));
            }
        }
        if let Some(v) = &self.tracking_device_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipment.tracking_device_code", e));
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

impl TransportEquipment {
    pub fn title() -> &'static str {
        "Transport Equipment. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a piece of equipment used to transport goods."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            operating_party: None,
            loading_location: None,
            shipment_document_reference: None,
            unloading_location: None,
            tracking_device_code: None,
            tare_weight_measure: None,
            goods_item: None,
            description: None,
            maximum_temperature: None,
            haulage_trading_terms: None,
            hazardous_goods_transit: None,
            measurement_dimension: None,
            freight_allowance_charge: None,
            gross_weight_measure: None,
            loading_transport_event: None,
            minimum_temperature: None,
            owner_party: None,
            positioning_transport_event: None,
            transport_equipment_type_code: None,
            refrigeration_on_indicator: None,
            delivery: None,
            handling_transport_event: None,
            transport_event: None,
            referenced_consignment_id: None,
            fullness_indication_code: None,
            characteristics: None,
            disposition_code: None,
            human_food_approved_indicator: None,
            provider_type_code: None,
            contained_in_transport_equipment: None,
            pickup: None,
            supplier_party: None,
            trace_id: None,
            humidity_percent: None,
            legal_status_indicator: None,
            delivery_transport_event: None,
            package: None,
            service_allowance_charge: None,
            id: None,
            animal_food_approved_indicator: None,
            damage_remarks: None,
            provider_party: None,
            returnability_indicator: None,
            refrigerated_indicator: None,
            gross_volume_measure: None,
            dangerous_goods_approved_indicator: None,
            pickup_transport_event: None,
            transport_equipment_seal: None,
            size_type_code: None,
            packaged_transport_handling_unit: None,
            storage_location: None,
            owner_type_code: None,
            loading_proof_party: None,
            power_indicator: None,
            attached_transport_equipment: None,
            air_flow_percent: None,
            quarantine_transport_event: None,
            information: None,
            applicable_transport_means: None,
            despatch: None,
            special_transport_requirements: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfAirFlowPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AirFlowPercent>,
}

impl AsMut<TransportEquipmentArrayOfAirFlowPercentComponent> for TransportEquipmentArrayOfAirFlowPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfAirFlowPercentComponent> for TransportEquipmentArrayOfAirFlowPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfAirFlowPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfAirFlowPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfAirFlowPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AirFlowPercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AirFlowPercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AirFlowPercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AirFlowPercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfAnimalFoodApprovedIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AnimalFoodApprovedIndicator>,
}

impl AsMut<TransportEquipmentArrayOfAnimalFoodApprovedIndicatorComponent> for TransportEquipmentArrayOfAnimalFoodApprovedIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfAnimalFoodApprovedIndicatorComponent> for TransportEquipmentArrayOfAnimalFoodApprovedIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfAnimalFoodApprovedIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfAnimalFoodApprovedIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfAnimalFoodApprovedIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AnimalFoodApprovedIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AnimalFoodApprovedIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AnimalFoodApprovedIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AnimalFoodApprovedIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfApplicableTransportMeansComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ApplicableTransportMeans>,
}

impl AsMut<TransportEquipmentArrayOfApplicableTransportMeansComponent> for TransportEquipmentArrayOfApplicableTransportMeansComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfApplicableTransportMeansComponent> for TransportEquipmentArrayOfApplicableTransportMeansComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfApplicableTransportMeansComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfApplicableTransportMeansComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfApplicableTransportMeansComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ApplicableTransportMeans) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ApplicableTransportMeans) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ApplicableTransportMeans> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ApplicableTransportMeans> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfAttachedTransportEquipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AttachedTransportEquipment>,
}

impl AsMut<TransportEquipmentArrayOfAttachedTransportEquipmentComponent> for TransportEquipmentArrayOfAttachedTransportEquipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfAttachedTransportEquipmentComponent> for TransportEquipmentArrayOfAttachedTransportEquipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfAttachedTransportEquipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfAttachedTransportEquipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AttachedTransportEquipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AttachedTransportEquipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AttachedTransportEquipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AttachedTransportEquipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfCharacteristicsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Characteristics>,
}

impl AsMut<TransportEquipmentArrayOfCharacteristicsComponent> for TransportEquipmentArrayOfCharacteristicsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfCharacteristicsComponent> for TransportEquipmentArrayOfCharacteristicsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfCharacteristicsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfCharacteristicsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfCharacteristicsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Characteristics) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Characteristics) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Characteristics> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Characteristics> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfContainedInTransportEquipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContainedInTransportEquipment>,
}

impl AsMut<TransportEquipmentArrayOfContainedInTransportEquipmentComponent> for TransportEquipmentArrayOfContainedInTransportEquipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfContainedInTransportEquipmentComponent> for TransportEquipmentArrayOfContainedInTransportEquipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfContainedInTransportEquipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfContainedInTransportEquipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContainedInTransportEquipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContainedInTransportEquipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContainedInTransportEquipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContainedInTransportEquipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfDamageRemarksComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DamageRemarks>,
}

impl AsMut<TransportEquipmentArrayOfDamageRemarksComponent> for TransportEquipmentArrayOfDamageRemarksComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfDamageRemarksComponent> for TransportEquipmentArrayOfDamageRemarksComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfDamageRemarksComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfDamageRemarksComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DamageRemarks) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DamageRemarks) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DamageRemarks> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DamageRemarks> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfDangerousGoodsApprovedIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DangerousGoodsApprovedIndicator>,
}

impl AsMut<TransportEquipmentArrayOfDangerousGoodsApprovedIndicatorComponent> for TransportEquipmentArrayOfDangerousGoodsApprovedIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfDangerousGoodsApprovedIndicatorComponent> for TransportEquipmentArrayOfDangerousGoodsApprovedIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfDangerousGoodsApprovedIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfDangerousGoodsApprovedIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfDangerousGoodsApprovedIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DangerousGoodsApprovedIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DangerousGoodsApprovedIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DangerousGoodsApprovedIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DangerousGoodsApprovedIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfDeliveryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Delivery>,
}

impl AsMut<TransportEquipmentArrayOfDeliveryComponent> for TransportEquipmentArrayOfDeliveryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfDeliveryComponent> for TransportEquipmentArrayOfDeliveryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfDeliveryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfDeliveryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfDeliveryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Delivery) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Delivery) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Delivery> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Delivery> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfDeliveryTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryTransportEvent>,
}

impl AsMut<TransportEquipmentArrayOfDeliveryTransportEventComponent> for TransportEquipmentArrayOfDeliveryTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfDeliveryTransportEventComponent> for TransportEquipmentArrayOfDeliveryTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfDeliveryTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfDeliveryTransportEventComponent {
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
pub struct TransportEquipmentArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<TransportEquipmentArrayOfDescriptionComponent> for TransportEquipmentArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfDescriptionComponent> for TransportEquipmentArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfDescriptionComponent {
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
pub struct TransportEquipmentArrayOfDespatchComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Despatch>,
}

impl AsMut<TransportEquipmentArrayOfDespatchComponent> for TransportEquipmentArrayOfDespatchComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfDespatchComponent> for TransportEquipmentArrayOfDespatchComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfDespatchComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfDespatchComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfDespatchComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Despatch) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Despatch) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Despatch> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Despatch> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfDispositionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DispositionCode>,
}

impl AsMut<TransportEquipmentArrayOfDispositionCodeComponent> for TransportEquipmentArrayOfDispositionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfDispositionCodeComponent> for TransportEquipmentArrayOfDispositionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfDispositionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfDispositionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfDispositionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DispositionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DispositionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DispositionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DispositionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfFreightAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge>,
}

impl AsMut<TransportEquipmentArrayOfFreightAllowanceChargeComponent> for TransportEquipmentArrayOfFreightAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfFreightAllowanceChargeComponent> for TransportEquipmentArrayOfFreightAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfFreightAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfFreightAllowanceChargeComponent {
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
pub struct TransportEquipmentArrayOfFullnessIndicationCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FullnessIndicationCode>,
}

impl AsMut<TransportEquipmentArrayOfFullnessIndicationCodeComponent> for TransportEquipmentArrayOfFullnessIndicationCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfFullnessIndicationCodeComponent> for TransportEquipmentArrayOfFullnessIndicationCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfFullnessIndicationCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfFullnessIndicationCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfFullnessIndicationCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FullnessIndicationCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FullnessIndicationCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FullnessIndicationCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FullnessIndicationCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfGoodsItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::GoodsItem>,
}

impl AsMut<TransportEquipmentArrayOfGoodsItemComponent> for TransportEquipmentArrayOfGoodsItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfGoodsItemComponent> for TransportEquipmentArrayOfGoodsItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfGoodsItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfGoodsItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::GoodsItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::GoodsItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::GoodsItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::GoodsItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfGrossVolumeMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure>,
}

impl AsMut<TransportEquipmentArrayOfGrossVolumeMeasureComponent> for TransportEquipmentArrayOfGrossVolumeMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfGrossVolumeMeasureComponent> for TransportEquipmentArrayOfGrossVolumeMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfGrossVolumeMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfGrossVolumeMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfGrossVolumeMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfGrossWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure>,
}

impl AsMut<TransportEquipmentArrayOfGrossWeightMeasureComponent> for TransportEquipmentArrayOfGrossWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfGrossWeightMeasureComponent> for TransportEquipmentArrayOfGrossWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfGrossWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfGrossWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfGrossWeightMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfHandlingTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::HandlingTransportEvent>,
}

impl AsMut<TransportEquipmentArrayOfHandlingTransportEventComponent> for TransportEquipmentArrayOfHandlingTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfHandlingTransportEventComponent> for TransportEquipmentArrayOfHandlingTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfHandlingTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfHandlingTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::HandlingTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::HandlingTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::HandlingTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::HandlingTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfHaulageTradingTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::HaulageTradingTerms>,
}

impl AsMut<TransportEquipmentArrayOfHaulageTradingTermsComponent> for TransportEquipmentArrayOfHaulageTradingTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfHaulageTradingTermsComponent> for TransportEquipmentArrayOfHaulageTradingTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfHaulageTradingTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfHaulageTradingTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::HaulageTradingTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::HaulageTradingTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::HaulageTradingTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::HaulageTradingTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfHazardousGoodsTransitComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit>,
}

impl AsMut<TransportEquipmentArrayOfHazardousGoodsTransitComponent> for TransportEquipmentArrayOfHazardousGoodsTransitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfHazardousGoodsTransitComponent> for TransportEquipmentArrayOfHazardousGoodsTransitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfHazardousGoodsTransitComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfHazardousGoodsTransitComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfHumanFoodApprovedIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HumanFoodApprovedIndicator>,
}

impl AsMut<TransportEquipmentArrayOfHumanFoodApprovedIndicatorComponent> for TransportEquipmentArrayOfHumanFoodApprovedIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfHumanFoodApprovedIndicatorComponent> for TransportEquipmentArrayOfHumanFoodApprovedIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfHumanFoodApprovedIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfHumanFoodApprovedIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfHumanFoodApprovedIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HumanFoodApprovedIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HumanFoodApprovedIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HumanFoodApprovedIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HumanFoodApprovedIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfHumidityPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HumidityPercent>,
}

impl AsMut<TransportEquipmentArrayOfHumidityPercentComponent> for TransportEquipmentArrayOfHumidityPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfHumidityPercentComponent> for TransportEquipmentArrayOfHumidityPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfHumidityPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfHumidityPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfHumidityPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HumidityPercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HumidityPercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HumidityPercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HumidityPercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<TransportEquipmentArrayOfIDComponent> for TransportEquipmentArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfIDComponent> for TransportEquipmentArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfIDComponent {
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
pub struct TransportEquipmentArrayOfInformationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Information>,
}

impl AsMut<TransportEquipmentArrayOfInformationComponent> for TransportEquipmentArrayOfInformationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfInformationComponent> for TransportEquipmentArrayOfInformationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfInformationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfInformationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Information) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Information) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Information> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Information> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfLegalStatusIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LegalStatusIndicator>,
}

impl AsMut<TransportEquipmentArrayOfLegalStatusIndicatorComponent> for TransportEquipmentArrayOfLegalStatusIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfLegalStatusIndicatorComponent> for TransportEquipmentArrayOfLegalStatusIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfLegalStatusIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfLegalStatusIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfLegalStatusIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LegalStatusIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LegalStatusIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LegalStatusIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LegalStatusIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfLoadingLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LoadingLocation>,
}

impl AsMut<TransportEquipmentArrayOfLoadingLocationComponent> for TransportEquipmentArrayOfLoadingLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfLoadingLocationComponent> for TransportEquipmentArrayOfLoadingLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfLoadingLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfLoadingLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfLoadingLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LoadingLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LoadingLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LoadingLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LoadingLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfLoadingProofPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LoadingProofParty>,
}

impl AsMut<TransportEquipmentArrayOfLoadingProofPartyComponent> for TransportEquipmentArrayOfLoadingProofPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfLoadingProofPartyComponent> for TransportEquipmentArrayOfLoadingProofPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfLoadingProofPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfLoadingProofPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfLoadingProofPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LoadingProofParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LoadingProofParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LoadingProofParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LoadingProofParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfLoadingTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LoadingTransportEvent>,
}

impl AsMut<TransportEquipmentArrayOfLoadingTransportEventComponent> for TransportEquipmentArrayOfLoadingTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfLoadingTransportEventComponent> for TransportEquipmentArrayOfLoadingTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfLoadingTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfLoadingTransportEventComponent {
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
pub struct TransportEquipmentArrayOfMaximumTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature>,
}

impl AsMut<TransportEquipmentArrayOfMaximumTemperatureComponent> for TransportEquipmentArrayOfMaximumTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfMaximumTemperatureComponent> for TransportEquipmentArrayOfMaximumTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfMaximumTemperatureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfMaximumTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfMaximumTemperatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfMeasurementDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension>,
}

impl AsMut<TransportEquipmentArrayOfMeasurementDimensionComponent> for TransportEquipmentArrayOfMeasurementDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfMeasurementDimensionComponent> for TransportEquipmentArrayOfMeasurementDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfMeasurementDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfMeasurementDimensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfMinimumTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature>,
}

impl AsMut<TransportEquipmentArrayOfMinimumTemperatureComponent> for TransportEquipmentArrayOfMinimumTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfMinimumTemperatureComponent> for TransportEquipmentArrayOfMinimumTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfMinimumTemperatureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfMinimumTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfMinimumTemperatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfOperatingPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OperatingParty>,
}

impl AsMut<TransportEquipmentArrayOfOperatingPartyComponent> for TransportEquipmentArrayOfOperatingPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfOperatingPartyComponent> for TransportEquipmentArrayOfOperatingPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfOperatingPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfOperatingPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfOperatingPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OperatingParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OperatingParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OperatingParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OperatingParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfOwnerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OwnerParty>,
}

impl AsMut<TransportEquipmentArrayOfOwnerPartyComponent> for TransportEquipmentArrayOfOwnerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfOwnerPartyComponent> for TransportEquipmentArrayOfOwnerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfOwnerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfOwnerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfOwnerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OwnerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OwnerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OwnerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OwnerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfOwnerTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OwnerTypeCode>,
}

impl AsMut<TransportEquipmentArrayOfOwnerTypeCodeComponent> for TransportEquipmentArrayOfOwnerTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfOwnerTypeCodeComponent> for TransportEquipmentArrayOfOwnerTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfOwnerTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfOwnerTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfOwnerTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OwnerTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OwnerTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OwnerTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OwnerTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfPackageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Package>,
}

impl AsMut<TransportEquipmentArrayOfPackageComponent> for TransportEquipmentArrayOfPackageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfPackageComponent> for TransportEquipmentArrayOfPackageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfPackageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfPackageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Package) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Package) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Package> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Package> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfPackagedTransportHandlingUnitComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PackagedTransportHandlingUnit>,
}

impl AsMut<TransportEquipmentArrayOfPackagedTransportHandlingUnitComponent> for TransportEquipmentArrayOfPackagedTransportHandlingUnitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfPackagedTransportHandlingUnitComponent> for TransportEquipmentArrayOfPackagedTransportHandlingUnitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfPackagedTransportHandlingUnitComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfPackagedTransportHandlingUnitComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PackagedTransportHandlingUnit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PackagedTransportHandlingUnit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PackagedTransportHandlingUnit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PackagedTransportHandlingUnit> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfPickupComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Pickup>,
}

impl AsMut<TransportEquipmentArrayOfPickupComponent> for TransportEquipmentArrayOfPickupComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfPickupComponent> for TransportEquipmentArrayOfPickupComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfPickupComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfPickupComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfPickupComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Pickup) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Pickup) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Pickup> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Pickup> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfPickupTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PickupTransportEvent>,
}

impl AsMut<TransportEquipmentArrayOfPickupTransportEventComponent> for TransportEquipmentArrayOfPickupTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfPickupTransportEventComponent> for TransportEquipmentArrayOfPickupTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfPickupTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfPickupTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PickupTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PickupTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PickupTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PickupTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfPositioningTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PositioningTransportEvent>,
}

impl AsMut<TransportEquipmentArrayOfPositioningTransportEventComponent> for TransportEquipmentArrayOfPositioningTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfPositioningTransportEventComponent> for TransportEquipmentArrayOfPositioningTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfPositioningTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfPositioningTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PositioningTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PositioningTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PositioningTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PositioningTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfPowerIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PowerIndicator>,
}

impl AsMut<TransportEquipmentArrayOfPowerIndicatorComponent> for TransportEquipmentArrayOfPowerIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfPowerIndicatorComponent> for TransportEquipmentArrayOfPowerIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfPowerIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfPowerIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfPowerIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PowerIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PowerIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PowerIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PowerIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfProviderPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ProviderParty>,
}

impl AsMut<TransportEquipmentArrayOfProviderPartyComponent> for TransportEquipmentArrayOfProviderPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfProviderPartyComponent> for TransportEquipmentArrayOfProviderPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfProviderPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfProviderPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfProviderPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ProviderParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ProviderParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ProviderParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ProviderParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfProviderTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ProviderTypeCode>,
}

impl AsMut<TransportEquipmentArrayOfProviderTypeCodeComponent> for TransportEquipmentArrayOfProviderTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfProviderTypeCodeComponent> for TransportEquipmentArrayOfProviderTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfProviderTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfProviderTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfProviderTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ProviderTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ProviderTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ProviderTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ProviderTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfQuarantineTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::QuarantineTransportEvent>,
}

impl AsMut<TransportEquipmentArrayOfQuarantineTransportEventComponent> for TransportEquipmentArrayOfQuarantineTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfQuarantineTransportEventComponent> for TransportEquipmentArrayOfQuarantineTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfQuarantineTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfQuarantineTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::QuarantineTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::QuarantineTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::QuarantineTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::QuarantineTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfReferencedConsignmentIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReferencedConsignmentID>,
}

impl AsMut<TransportEquipmentArrayOfReferencedConsignmentIDComponent> for TransportEquipmentArrayOfReferencedConsignmentIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfReferencedConsignmentIDComponent> for TransportEquipmentArrayOfReferencedConsignmentIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfReferencedConsignmentIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfReferencedConsignmentIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReferencedConsignmentID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReferencedConsignmentID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReferencedConsignmentID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReferencedConsignmentID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfRefrigeratedIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RefrigeratedIndicator>,
}

impl AsMut<TransportEquipmentArrayOfRefrigeratedIndicatorComponent> for TransportEquipmentArrayOfRefrigeratedIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfRefrigeratedIndicatorComponent> for TransportEquipmentArrayOfRefrigeratedIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfRefrigeratedIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfRefrigeratedIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfRefrigeratedIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RefrigeratedIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RefrigeratedIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RefrigeratedIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RefrigeratedIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfRefrigerationOnIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RefrigerationOnIndicator>,
}

impl AsMut<TransportEquipmentArrayOfRefrigerationOnIndicatorComponent> for TransportEquipmentArrayOfRefrigerationOnIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfRefrigerationOnIndicatorComponent> for TransportEquipmentArrayOfRefrigerationOnIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfRefrigerationOnIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfRefrigerationOnIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfRefrigerationOnIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RefrigerationOnIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RefrigerationOnIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RefrigerationOnIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RefrigerationOnIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfReturnabilityIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReturnabilityIndicator>,
}

impl AsMut<TransportEquipmentArrayOfReturnabilityIndicatorComponent> for TransportEquipmentArrayOfReturnabilityIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfReturnabilityIndicatorComponent> for TransportEquipmentArrayOfReturnabilityIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfReturnabilityIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfReturnabilityIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfReturnabilityIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReturnabilityIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReturnabilityIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReturnabilityIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReturnabilityIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfServiceAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ServiceAllowanceCharge>,
}

impl AsMut<TransportEquipmentArrayOfServiceAllowanceChargeComponent> for TransportEquipmentArrayOfServiceAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfServiceAllowanceChargeComponent> for TransportEquipmentArrayOfServiceAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfServiceAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfServiceAllowanceChargeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ServiceAllowanceCharge) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ServiceAllowanceCharge) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ServiceAllowanceCharge> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ServiceAllowanceCharge> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfShipmentDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ShipmentDocumentReference>,
}

impl AsMut<TransportEquipmentArrayOfShipmentDocumentReferenceComponent> for TransportEquipmentArrayOfShipmentDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfShipmentDocumentReferenceComponent> for TransportEquipmentArrayOfShipmentDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfShipmentDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfShipmentDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ShipmentDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ShipmentDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ShipmentDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ShipmentDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfSizeTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SizeTypeCode>,
}

impl AsMut<TransportEquipmentArrayOfSizeTypeCodeComponent> for TransportEquipmentArrayOfSizeTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfSizeTypeCodeComponent> for TransportEquipmentArrayOfSizeTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfSizeTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfSizeTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfSizeTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SizeTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SizeTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SizeTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SizeTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfSpecialTransportRequirementsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SpecialTransportRequirements>,
}

impl AsMut<TransportEquipmentArrayOfSpecialTransportRequirementsComponent> for TransportEquipmentArrayOfSpecialTransportRequirementsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfSpecialTransportRequirementsComponent> for TransportEquipmentArrayOfSpecialTransportRequirementsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfSpecialTransportRequirementsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfSpecialTransportRequirementsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SpecialTransportRequirements) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SpecialTransportRequirements) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SpecialTransportRequirements> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SpecialTransportRequirements> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfStorageLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::StorageLocation>,
}

impl AsMut<TransportEquipmentArrayOfStorageLocationComponent> for TransportEquipmentArrayOfStorageLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfStorageLocationComponent> for TransportEquipmentArrayOfStorageLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfStorageLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfStorageLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfStorageLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::StorageLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::StorageLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::StorageLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::StorageLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfSupplierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SupplierParty>,
}

impl AsMut<TransportEquipmentArrayOfSupplierPartyComponent> for TransportEquipmentArrayOfSupplierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfSupplierPartyComponent> for TransportEquipmentArrayOfSupplierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfSupplierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfSupplierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfSupplierPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SupplierParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SupplierParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SupplierParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SupplierParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfTareWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TareWeightMeasure>,
}

impl AsMut<TransportEquipmentArrayOfTareWeightMeasureComponent> for TransportEquipmentArrayOfTareWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfTareWeightMeasureComponent> for TransportEquipmentArrayOfTareWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfTareWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfTareWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfTareWeightMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TareWeightMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TareWeightMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TareWeightMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TareWeightMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfTraceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TraceID>,
}

impl AsMut<TransportEquipmentArrayOfTraceIDComponent> for TransportEquipmentArrayOfTraceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfTraceIDComponent> for TransportEquipmentArrayOfTraceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfTraceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfTraceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfTraceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TraceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TraceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TraceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TraceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfTrackingDeviceCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TrackingDeviceCode>,
}

impl AsMut<TransportEquipmentArrayOfTrackingDeviceCodeComponent> for TransportEquipmentArrayOfTrackingDeviceCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfTrackingDeviceCodeComponent> for TransportEquipmentArrayOfTrackingDeviceCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfTrackingDeviceCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfTrackingDeviceCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfTrackingDeviceCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TrackingDeviceCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TrackingDeviceCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TrackingDeviceCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TrackingDeviceCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfTransportEquipmentSealComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportEquipmentSeal>,
}

impl AsMut<TransportEquipmentArrayOfTransportEquipmentSealComponent> for TransportEquipmentArrayOfTransportEquipmentSealComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfTransportEquipmentSealComponent> for TransportEquipmentArrayOfTransportEquipmentSealComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfTransportEquipmentSealComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfTransportEquipmentSealComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportEquipmentSeal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportEquipmentSeal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportEquipmentSeal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportEquipmentSeal> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfTransportEquipmentTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportEquipmentTypeCode>,
}

impl AsMut<TransportEquipmentArrayOfTransportEquipmentTypeCodeComponent> for TransportEquipmentArrayOfTransportEquipmentTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfTransportEquipmentTypeCodeComponent> for TransportEquipmentArrayOfTransportEquipmentTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfTransportEquipmentTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfTransportEquipmentTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfTransportEquipmentTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportEquipmentTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportEquipmentTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportEquipmentTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportEquipmentTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentArrayOfTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportEvent>,
}

impl AsMut<TransportEquipmentArrayOfTransportEventComponent> for TransportEquipmentArrayOfTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfTransportEventComponent> for TransportEquipmentArrayOfTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfTransportEventComponent {
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
pub struct TransportEquipmentArrayOfUnloadingLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::UnloadingLocation>,
}

impl AsMut<TransportEquipmentArrayOfUnloadingLocationComponent> for TransportEquipmentArrayOfUnloadingLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentArrayOfUnloadingLocationComponent> for TransportEquipmentArrayOfUnloadingLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfUnloadingLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentArrayOfUnloadingLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentArrayOfUnloadingLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::UnloadingLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::UnloadingLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::UnloadingLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::UnloadingLocation> {
        self.items.iter()
    }
}


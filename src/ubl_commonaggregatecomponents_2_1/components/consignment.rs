use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Consignment {
    #[serde(rename = "AnimalFoodIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animal_food_indicator: Option<ConsignmentArrayOfAnimalFoodIndicatorComponent>,
    #[serde(rename = "BillOfLadingHolderParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_of_lading_holder_party: Option<ConsignmentArrayOfBillOfLadingHolderPartyComponent>,
    #[serde(rename = "BrokerAssignedID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_assigned_id: Option<ConsignmentArrayOfBrokerAssignedIDComponent>,
    #[serde(rename = "BulkCargoIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_cargo_indicator: Option<ConsignmentArrayOfBulkCargoIndicatorComponent>,
    #[serde(rename = "CarrierAssignedID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_assigned_id: Option<ConsignmentArrayOfCarrierAssignedIDComponent>,
    #[serde(rename = "CarrierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_party: Option<ConsignmentArrayOfCarrierPartyComponent>,
    #[serde(rename = "CarrierServiceInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_service_instructions: Option<ConsignmentArrayOfCarrierServiceInstructionsComponent>,
    #[serde(rename = "ChargeableWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargeable_weight_measure: Option<ConsignmentArrayOfChargeableWeightMeasureComponent>,
    #[serde(rename = "ChildConsignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_consignment: Option<ConsignmentArrayOfChildConsignmentComponent>,
    #[serde(rename = "ChildConsignmentQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_consignment_quantity: Option<ConsignmentArrayOfChildConsignmentQuantityComponent>,
    #[serde(rename = "CollectPaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect_payment_terms: Option<ConsignmentArrayOfCollectPaymentTermsComponent>,
    #[serde(rename = "ConsigneeAssignedID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignee_assigned_id: Option<ConsignmentArrayOfConsigneeAssignedIDComponent>,
    #[serde(rename = "ConsigneeParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignee_party: Option<ConsignmentArrayOfConsigneePartyComponent>,
    #[serde(rename = "ConsignmentQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignment_quantity: Option<ConsignmentArrayOfConsignmentQuantityComponent>,
    #[serde(rename = "ConsignorAssignedID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignor_assigned_id: Option<ConsignmentArrayOfConsignorAssignedIDComponent>,
    #[serde(rename = "ConsignorParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignor_party: Option<ConsignmentArrayOfConsignorPartyComponent>,
    #[serde(rename = "ConsolidatableIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consolidatable_indicator: Option<ConsignmentArrayOfConsolidatableIndicatorComponent>,
    #[serde(rename = "ConsolidatedShipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consolidated_shipment: Option<ConsignmentArrayOfConsolidatedShipmentComponent>,
    #[serde(rename = "ContainerizedIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containerized_indicator: Option<ConsignmentArrayOfContainerizedIndicatorComponent>,
    #[serde(rename = "ContractedCarrierAssignedID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracted_carrier_assigned_id: Option<ConsignmentArrayOfContractedCarrierAssignedIDComponent>,
    #[serde(rename = "CustomsClearanceServiceInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customs_clearance_service_instructions: Option<ConsignmentArrayOfCustomsClearanceServiceInstructionsComponent>,
    #[serde(rename = "CustomsDeclaration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customs_declaration: Option<ConsignmentArrayOfCustomsDeclarationComponent>,
    #[serde(rename = "DeclaredCustomsValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_customs_value_amount: Option<ConsignmentArrayOfDeclaredCustomsValueAmountComponent>,
    #[serde(rename = "DeclaredForCarriageValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_for_carriage_value_amount: Option<ConsignmentArrayOfDeclaredForCarriageValueAmountComponent>,
    #[serde(rename = "DeclaredStatisticsValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_statistics_value_amount: Option<ConsignmentArrayOfDeclaredStatisticsValueAmountComponent>,
    #[serde(rename = "DeliveryInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_instructions: Option<ConsignmentArrayOfDeliveryInstructionsComponent>,
    #[serde(rename = "DeliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<ConsignmentArrayOfDeliveryTermsComponent>,
    #[serde(rename = "DisbursementPaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disbursement_payment_terms: Option<ConsignmentArrayOfDisbursementPaymentTermsComponent>,
    #[serde(rename = "ExporterParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exporter_party: Option<ConsignmentArrayOfExporterPartyComponent>,
    #[serde(rename = "ExtraAllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_allowance_charge: Option<ConsignmentArrayOfExtraAllowanceChargeComponent>,
    #[serde(rename = "FinalDeliveryParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_delivery_party: Option<ConsignmentArrayOfFinalDeliveryPartyComponent>,
    #[serde(rename = "FinalDeliveryTransportationService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_delivery_transportation_service: Option<ConsignmentArrayOfFinalDeliveryTransportationServiceComponent>,
    #[serde(rename = "FinalDestinationCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_destination_country: Option<ConsignmentArrayOfFinalDestinationCountryComponent>,
    #[serde(rename = "FirstArrivalPortLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_arrival_port_location: Option<ConsignmentArrayOfFirstArrivalPortLocationComponent>,
    #[serde(rename = "ForwarderServiceInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarder_service_instructions: Option<ConsignmentArrayOfForwarderServiceInstructionsComponent>,
    #[serde(rename = "FreeOnBoardValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_on_board_value_amount: Option<ConsignmentArrayOfFreeOnBoardValueAmountComponent>,
    #[serde(rename = "FreightAllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_allowance_charge: Option<ConsignmentArrayOfFreightAllowanceChargeComponent>,
    #[serde(rename = "FreightForwarderAssignedID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_forwarder_assigned_id: Option<ConsignmentArrayOfFreightForwarderAssignedIDComponent>,
    #[serde(rename = "FreightForwarderParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_forwarder_party: Option<ConsignmentArrayOfFreightForwarderPartyComponent>,
    #[serde(rename = "GeneralCargoIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_cargo_indicator: Option<ConsignmentArrayOfGeneralCargoIndicatorComponent>,
    #[serde(rename = "GrossVolumeMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_volume_measure: Option<ConsignmentArrayOfGrossVolumeMeasureComponent>,
    #[serde(rename = "GrossWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_weight_measure: Option<ConsignmentArrayOfGrossWeightMeasureComponent>,
    #[serde(rename = "HandlingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handling_code: Option<ConsignmentArrayOfHandlingCodeComponent>,
    #[serde(rename = "HandlingInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handling_instructions: Option<ConsignmentArrayOfHandlingInstructionsComponent>,
    #[serde(rename = "HaulageInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haulage_instructions: Option<ConsignmentArrayOfHaulageInstructionsComponent>,
    #[serde(rename = "HazardousItemNotificationParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_item_notification_party: Option<ConsignmentArrayOfHazardousItemNotificationPartyComponent>,
    #[serde(rename = "HazardousRiskIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_risk_indicator: Option<ConsignmentArrayOfHazardousRiskIndicatorComponent>,
    #[serde(rename = "HumanFoodIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_food_indicator: Option<ConsignmentArrayOfHumanFoodIndicatorComponent>,
    #[serde(rename = "ID")]
    pub id: ConsignmentArrayOfIDComponent,
    #[serde(rename = "ImporterParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importer_party: Option<ConsignmentArrayOfImporterPartyComponent>,
    #[serde(rename = "Information")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<ConsignmentArrayOfInformationComponent>,
    #[serde(rename = "InsuranceParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_party: Option<ConsignmentArrayOfInsurancePartyComponent>,
    #[serde(rename = "InsurancePremiumAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_premium_amount: Option<ConsignmentArrayOfInsurancePremiumAmountComponent>,
    #[serde(rename = "InsuranceValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_value_amount: Option<ConsignmentArrayOfInsuranceValueAmountComponent>,
    #[serde(rename = "LastExitPortLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_exit_port_location: Option<ConsignmentArrayOfLastExitPortLocationComponent>,
    #[serde(rename = "LivestockIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livestock_indicator: Option<ConsignmentArrayOfLivestockIndicatorComponent>,
    #[serde(rename = "LoadingLengthMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_length_measure: Option<ConsignmentArrayOfLoadingLengthMeasureComponent>,
    #[serde(rename = "LoadingSequenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_sequence_id: Option<ConsignmentArrayOfLoadingSequenceIDComponent>,
    #[serde(rename = "LogisticsOperatorParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logistics_operator_party: Option<ConsignmentArrayOfLogisticsOperatorPartyComponent>,
    #[serde(rename = "MainCarriageShipmentStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_carriage_shipment_stage: Option<ConsignmentArrayOfMainCarriageShipmentStageComponent>,
    #[serde(rename = "MortgageHolderParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mortgage_holder_party: Option<ConsignmentArrayOfMortgageHolderPartyComponent>,
    #[serde(rename = "NetNetWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_net_weight_measure: Option<ConsignmentArrayOfNetNetWeightMeasureComponent>,
    #[serde(rename = "NetVolumeMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_volume_measure: Option<ConsignmentArrayOfNetVolumeMeasureComponent>,
    #[serde(rename = "NetWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_weight_measure: Option<ConsignmentArrayOfNetWeightMeasureComponent>,
    #[serde(rename = "NotifyParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_party: Option<ConsignmentArrayOfNotifyPartyComponent>,
    #[serde(rename = "OnCarriageShipmentStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_carriage_shipment_stage: Option<ConsignmentArrayOfOnCarriageShipmentStageComponent>,
    #[serde(rename = "OriginalDepartureCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_departure_country: Option<ConsignmentArrayOfOriginalDepartureCountryComponent>,
    #[serde(rename = "OriginalDespatchParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_despatch_party: Option<ConsignmentArrayOfOriginalDespatchPartyComponent>,
    #[serde(rename = "OriginalDespatchTransportationService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_despatch_transportation_service: Option<ConsignmentArrayOfOriginalDespatchTransportationServiceComponent>,
    #[serde(rename = "PaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<ConsignmentArrayOfPaymentTermsComponent>,
    #[serde(rename = "PerformingCarrierAssignedID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performing_carrier_assigned_id: Option<ConsignmentArrayOfPerformingCarrierAssignedIDComponent>,
    #[serde(rename = "PerformingCarrierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performing_carrier_party: Option<ConsignmentArrayOfPerformingCarrierPartyComponent>,
    #[serde(rename = "PlannedDeliveryTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_delivery_transport_event: Option<ConsignmentArrayOfPlannedDeliveryTransportEventComponent>,
    #[serde(rename = "PlannedPickupTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_pickup_transport_event: Option<ConsignmentArrayOfPlannedPickupTransportEventComponent>,
    #[serde(rename = "PreCarriageShipmentStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_carriage_shipment_stage: Option<ConsignmentArrayOfPreCarriageShipmentStageComponent>,
    #[serde(rename = "PrepaidPaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_payment_terms: Option<ConsignmentArrayOfPrepaidPaymentTermsComponent>,
    #[serde(rename = "Remarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<ConsignmentArrayOfRemarksComponent>,
    #[serde(rename = "RequestedDeliveryTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_delivery_transport_event: Option<ConsignmentArrayOfRequestedDeliveryTransportEventComponent>,
    #[serde(rename = "RequestedPickupTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_pickup_transport_event: Option<ConsignmentArrayOfRequestedPickupTransportEventComponent>,
    #[serde(rename = "SequenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<ConsignmentArrayOfSequenceIDComponent>,
    #[serde(rename = "ShippingPriorityLevelCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_priority_level_code: Option<ConsignmentArrayOfShippingPriorityLevelCodeComponent>,
    #[serde(rename = "SpecialInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_instructions: Option<ConsignmentArrayOfSpecialInstructionsComponent>,
    #[serde(rename = "SpecialSecurityIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_security_indicator: Option<ConsignmentArrayOfSpecialSecurityIndicatorComponent>,
    #[serde(rename = "SpecialServiceInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_service_instructions: Option<ConsignmentArrayOfSpecialServiceInstructionsComponent>,
    #[serde(rename = "SplitConsignmentIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_consignment_indicator: Option<ConsignmentArrayOfSplitConsignmentIndicatorComponent>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ConsignmentArrayOfStatusComponent>,
    #[serde(rename = "SubstituteCarrierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitute_carrier_party: Option<ConsignmentArrayOfSubstituteCarrierPartyComponent>,
    #[serde(rename = "SummaryDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_description: Option<ConsignmentArrayOfSummaryDescriptionComponent>,
    #[serde(rename = "TariffCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff_code: Option<ConsignmentArrayOfTariffCodeComponent>,
    #[serde(rename = "TariffDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff_description: Option<ConsignmentArrayOfTariffDescriptionComponent>,
    #[serde(rename = "ThirdPartyPayerIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_payer_indicator: Option<ConsignmentArrayOfThirdPartyPayerIndicatorComponent>,
    #[serde(rename = "TotalGoodsItemQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_goods_item_quantity: Option<ConsignmentArrayOfTotalGoodsItemQuantityComponent>,
    #[serde(rename = "TotalInvoiceAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_invoice_amount: Option<ConsignmentArrayOfTotalInvoiceAmountComponent>,
    #[serde(rename = "TotalPackagesQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_packages_quantity: Option<ConsignmentArrayOfTotalPackagesQuantityComponent>,
    #[serde(rename = "TotalTransportHandlingUnitQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_transport_handling_unit_quantity: Option<ConsignmentArrayOfTotalTransportHandlingUnitQuantityComponent>,
    #[serde(rename = "TransitCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_country: Option<ConsignmentArrayOfTransitCountryComponent>,
    #[serde(rename = "TransportAdvisorParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_advisor_party: Option<ConsignmentArrayOfTransportAdvisorPartyComponent>,
    #[serde(rename = "TransportContract")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_contract: Option<ConsignmentArrayOfTransportContractComponent>,
    #[serde(rename = "TransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_event: Option<ConsignmentArrayOfTransportEventComponent>,
    #[serde(rename = "TransportHandlingUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_handling_unit: Option<ConsignmentArrayOfTransportHandlingUnitComponent>,
}

impl AsMut<Consignment> for Consignment {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Consignment> for Consignment {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.total_packages_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.total_packages_quantity", e));
            }
        }
        if let Some(v) = &self.extra_allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.extra_allowance_charge", e));
            }
        }
        if let Some(v) = &self.performing_carrier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.performing_carrier_party", e));
            }
        }
        if let Some(v) = &self.transport_advisor_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.transport_advisor_party", e));
            }
        }
        if let Some(v) = &self.transport_handling_unit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.transport_handling_unit", e));
            }
        }
        if let Some(v) = &self.bill_of_lading_holder_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.bill_of_lading_holder_party", e));
            }
        }
        if let Some(v) = &self.disbursement_payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.disbursement_payment_terms", e));
            }
        }
        if let Some(v) = &self.loading_length_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.loading_length_measure", e));
            }
        }
        if let Some(v) = &self.chargeable_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.chargeable_weight_measure", e));
            }
        }
        if let Some(v) = &self.containerized_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.containerized_indicator", e));
            }
        }
        if let Some(v) = &self.on_carriage_shipment_stage {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.on_carriage_shipment_stage", e));
            }
        }
        if let Some(v) = &self.remarks {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.remarks", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("Consignment.id", e));
        }
        if let Some(v) = &self.special_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.special_instructions", e));
            }
        }
        if let Some(v) = &self.logistics_operator_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.logistics_operator_party", e));
            }
        }
        if let Some(v) = &self.handling_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.handling_code", e));
            }
        }
        if let Some(v) = &self.final_destination_country {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.final_destination_country", e));
            }
        }
        if let Some(v) = &self.consignee_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.consignee_party", e));
            }
        }
        if let Some(v) = &self.consolidated_shipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.consolidated_shipment", e));
            }
        }
        if let Some(v) = &self.human_food_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.human_food_indicator", e));
            }
        }
        if let Some(v) = &self.carrier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.carrier_party", e));
            }
        }
        if let Some(v) = &self.net_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.net_weight_measure", e));
            }
        }
        if let Some(v) = &self.performing_carrier_assigned_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.performing_carrier_assigned_id", e));
            }
        }
        if let Some(v) = &self.status {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.status", e));
            }
        }
        if let Some(v) = &self.free_on_board_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.free_on_board_value_amount", e));
            }
        }
        if let Some(v) = &self.child_consignment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.child_consignment", e));
            }
        }
        if let Some(v) = &self.consignor_assigned_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.consignor_assigned_id", e));
            }
        }
        if let Some(v) = &self.carrier_assigned_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.carrier_assigned_id", e));
            }
        }
        if let Some(v) = &self.delivery_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.delivery_terms", e));
            }
        }
        if let Some(v) = &self.broker_assigned_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.broker_assigned_id", e));
            }
        }
        if let Some(v) = &self.consignment_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.consignment_quantity", e));
            }
        }
        if let Some(v) = &self.freight_forwarder_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.freight_forwarder_party", e));
            }
        }
        if let Some(v) = &self.mortgage_holder_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.mortgage_holder_party", e));
            }
        }
        if let Some(v) = &self.original_departure_country {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.original_departure_country", e));
            }
        }
        if let Some(v) = &self.animal_food_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.animal_food_indicator", e));
            }
        }
        if let Some(v) = &self.sequence_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.sequence_id", e));
            }
        }
        if let Some(v) = &self.shipping_priority_level_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.shipping_priority_level_code", e));
            }
        }
        if let Some(v) = &self.tariff_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.tariff_description", e));
            }
        }
        if let Some(v) = &self.insurance_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.insurance_party", e));
            }
        }
        if let Some(v) = &self.third_party_payer_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.third_party_payer_indicator", e));
            }
        }
        if let Some(v) = &self.carrier_service_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.carrier_service_instructions", e));
            }
        }
        if let Some(v) = &self.gross_volume_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.gross_volume_measure", e));
            }
        }
        if let Some(v) = &self.customs_clearance_service_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.customs_clearance_service_instructions", e));
            }
        }
        if let Some(v) = &self.hazardous_risk_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.hazardous_risk_indicator", e));
            }
        }
        if let Some(v) = &self.loading_sequence_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.loading_sequence_id", e));
            }
        }
        if let Some(v) = &self.consignee_assigned_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.consignee_assigned_id", e));
            }
        }
        if let Some(v) = &self.consolidatable_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.consolidatable_indicator", e));
            }
        }
        if let Some(v) = &self.general_cargo_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.general_cargo_indicator", e));
            }
        }
        if let Some(v) = &self.prepaid_payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.prepaid_payment_terms", e));
            }
        }
        if let Some(v) = &self.bulk_cargo_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.bulk_cargo_indicator", e));
            }
        }
        if let Some(v) = &self.insurance_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.insurance_value_amount", e));
            }
        }
        if let Some(v) = &self.freight_allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.freight_allowance_charge", e));
            }
        }
        if let Some(v) = &self.final_delivery_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.final_delivery_party", e));
            }
        }
        if let Some(v) = &self.original_despatch_transportation_service {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.original_despatch_transportation_service", e));
            }
        }
        if let Some(v) = &self.haulage_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.haulage_instructions", e));
            }
        }
        if let Some(v) = &self.total_invoice_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.total_invoice_amount", e));
            }
        }
        if let Some(v) = &self.net_net_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.net_net_weight_measure", e));
            }
        }
        if let Some(v) = &self.original_despatch_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.original_despatch_party", e));
            }
        }
        if let Some(v) = &self.declared_statistics_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.declared_statistics_value_amount", e));
            }
        }
        if let Some(v) = &self.total_transport_handling_unit_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.total_transport_handling_unit_quantity", e));
            }
        }
        if let Some(v) = &self.transit_country {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.transit_country", e));
            }
        }
        if let Some(v) = &self.first_arrival_port_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.first_arrival_port_location", e));
            }
        }
        if let Some(v) = &self.hazardous_item_notification_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.hazardous_item_notification_party", e));
            }
        }
        if let Some(v) = &self.information {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.information", e));
            }
        }
        if let Some(v) = &self.insurance_premium_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.insurance_premium_amount", e));
            }
        }
        if let Some(v) = &self.collect_payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.collect_payment_terms", e));
            }
        }
        if let Some(v) = &self.gross_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.gross_weight_measure", e));
            }
        }
        if let Some(v) = &self.notify_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.notify_party", e));
            }
        }
        if let Some(v) = &self.tariff_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.tariff_code", e));
            }
        }
        if let Some(v) = &self.transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.transport_event", e));
            }
        }
        if let Some(v) = &self.payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.payment_terms", e));
            }
        }
        if let Some(v) = &self.final_delivery_transportation_service {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.final_delivery_transportation_service", e));
            }
        }
        if let Some(v) = &self.importer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.importer_party", e));
            }
        }
        if let Some(v) = &self.planned_pickup_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.planned_pickup_transport_event", e));
            }
        }
        if let Some(v) = &self.split_consignment_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.split_consignment_indicator", e));
            }
        }
        if let Some(v) = &self.transport_contract {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.transport_contract", e));
            }
        }
        if let Some(v) = &self.total_goods_item_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.total_goods_item_quantity", e));
            }
        }
        if let Some(v) = &self.livestock_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.livestock_indicator", e));
            }
        }
        if let Some(v) = &self.exporter_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.exporter_party", e));
            }
        }
        if let Some(v) = &self.last_exit_port_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.last_exit_port_location", e));
            }
        }
        if let Some(v) = &self.pre_carriage_shipment_stage {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.pre_carriage_shipment_stage", e));
            }
        }
        if let Some(v) = &self.requested_delivery_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.requested_delivery_transport_event", e));
            }
        }
        if let Some(v) = &self.contracted_carrier_assigned_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.contracted_carrier_assigned_id", e));
            }
        }
        if let Some(v) = &self.child_consignment_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.child_consignment_quantity", e));
            }
        }
        if let Some(v) = &self.declared_customs_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.declared_customs_value_amount", e));
            }
        }
        if let Some(v) = &self.declared_for_carriage_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.declared_for_carriage_value_amount", e));
            }
        }
        if let Some(v) = &self.net_volume_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.net_volume_measure", e));
            }
        }
        if let Some(v) = &self.special_service_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.special_service_instructions", e));
            }
        }
        if let Some(v) = &self.planned_delivery_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.planned_delivery_transport_event", e));
            }
        }
        if let Some(v) = &self.customs_declaration {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.customs_declaration", e));
            }
        }
        if let Some(v) = &self.forwarder_service_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.forwarder_service_instructions", e));
            }
        }
        if let Some(v) = &self.special_security_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.special_security_indicator", e));
            }
        }
        if let Some(v) = &self.freight_forwarder_assigned_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.freight_forwarder_assigned_id", e));
            }
        }
        if let Some(v) = &self.substitute_carrier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.substitute_carrier_party", e));
            }
        }
        if let Some(v) = &self.handling_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.handling_instructions", e));
            }
        }
        if let Some(v) = &self.consignor_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.consignor_party", e));
            }
        }
        if let Some(v) = &self.delivery_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.delivery_instructions", e));
            }
        }
        if let Some(v) = &self.main_carriage_shipment_stage {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.main_carriage_shipment_stage", e));
            }
        }
        if let Some(v) = &self.summary_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.summary_description", e));
            }
        }
        if let Some(v) = &self.requested_pickup_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consignment.requested_pickup_transport_event", e));
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

impl Consignment {
    pub fn title() -> &'static str {
        "Consignment. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an identifiable collection of one or more goods items to be transported between the consignor and the consignee. This information may be defined within a transport contract. A consignment may comprise more than one shipment (e.g., when consolidated by a freight forwarder)."
    }
    pub fn new(id: ConsignmentArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            general_cargo_indicator: None,
            customs_declaration: None,
            loading_sequence_id: None,
            performing_carrier_party: None,
            shipping_priority_level_code: None,
            consignee_party: None,
            gross_volume_measure: None,
            status: None,
            transit_country: None,
            transport_contract: None,
            sequence_id: None,
            carrier_service_instructions: None,
            summary_description: None,
            handling_code: None,
            gross_weight_measure: None,
            total_transport_handling_unit_quantity: None,
            transport_advisor_party: None,
            bulk_cargo_indicator: None,
            freight_forwarder_assigned_id: None,
            insurance_party: None,
            insurance_premium_amount: None,
            prepaid_payment_terms: None,
            consolidated_shipment: None,
            freight_forwarder_party: None,
            original_departure_country: None,
            child_consignment: None,
            requested_delivery_transport_event: None,
            special_service_instructions: None,
            carrier_assigned_id: None,
            main_carriage_shipment_stage: None,
            hazardous_item_notification_party: None,
            delivery_terms: None,
            haulage_instructions: None,
            substitute_carrier_party: None,
            last_exit_port_location: None,
            insurance_value_amount: None,
            animal_food_indicator: None,
            freight_allowance_charge: None,
            id,
            chargeable_weight_measure: None,
            declared_customs_value_amount: None,
            logistics_operator_party: None,
            extra_allowance_charge: None,
            original_despatch_party: None,
            final_delivery_transportation_service: None,
            declared_for_carriage_value_amount: None,
            performing_carrier_assigned_id: None,
            final_delivery_party: None,
            collect_payment_terms: None,
            consignee_assigned_id: None,
            split_consignment_indicator: None,
            tariff_description: None,
            total_goods_item_quantity: None,
            transport_event: None,
            transport_handling_unit: None,
            remarks: None,
            disbursement_payment_terms: None,
            pre_carriage_shipment_stage: None,
            mortgage_holder_party: None,
            customs_clearance_service_instructions: None,
            importer_party: None,
            loading_length_measure: None,
            net_volume_measure: None,
            payment_terms: None,
            consignor_party: None,
            total_packages_quantity: None,
            livestock_indicator: None,
            tariff_code: None,
            consignment_quantity: None,
            requested_pickup_transport_event: None,
            total_invoice_amount: None,
            consignor_assigned_id: None,
            information: None,
            exporter_party: None,
            forwarder_service_instructions: None,
            handling_instructions: None,
            first_arrival_port_location: None,
            net_net_weight_measure: None,
            original_despatch_transportation_service: None,
            final_destination_country: None,
            special_instructions: None,
            notify_party: None,
            on_carriage_shipment_stage: None,
            planned_delivery_transport_event: None,
            consolidatable_indicator: None,
            free_on_board_value_amount: None,
            delivery_instructions: None,
            planned_pickup_transport_event: None,
            third_party_payer_indicator: None,
            special_security_indicator: None,
            human_food_indicator: None,
            carrier_party: None,
            broker_assigned_id: None,
            hazardous_risk_indicator: None,
            net_weight_measure: None,
            declared_statistics_value_amount: None,
            bill_of_lading_holder_party: None,
            contracted_carrier_assigned_id: None,
            child_consignment_quantity: None,
            containerized_indicator: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfAnimalFoodIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AnimalFoodIndicator>,
}

impl AsMut<ConsignmentArrayOfAnimalFoodIndicatorComponent> for ConsignmentArrayOfAnimalFoodIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfAnimalFoodIndicatorComponent> for ConsignmentArrayOfAnimalFoodIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfAnimalFoodIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfAnimalFoodIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfAnimalFoodIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AnimalFoodIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AnimalFoodIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AnimalFoodIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AnimalFoodIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfBillOfLadingHolderPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BillOfLadingHolderParty>,
}

impl AsMut<ConsignmentArrayOfBillOfLadingHolderPartyComponent> for ConsignmentArrayOfBillOfLadingHolderPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfBillOfLadingHolderPartyComponent> for ConsignmentArrayOfBillOfLadingHolderPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfBillOfLadingHolderPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfBillOfLadingHolderPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfBillOfLadingHolderPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BillOfLadingHolderParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BillOfLadingHolderParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BillOfLadingHolderParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BillOfLadingHolderParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfBrokerAssignedIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BrokerAssignedID>,
}

impl AsMut<ConsignmentArrayOfBrokerAssignedIDComponent> for ConsignmentArrayOfBrokerAssignedIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfBrokerAssignedIDComponent> for ConsignmentArrayOfBrokerAssignedIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfBrokerAssignedIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfBrokerAssignedIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfBrokerAssignedIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BrokerAssignedID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BrokerAssignedID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BrokerAssignedID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BrokerAssignedID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfBulkCargoIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BulkCargoIndicator>,
}

impl AsMut<ConsignmentArrayOfBulkCargoIndicatorComponent> for ConsignmentArrayOfBulkCargoIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfBulkCargoIndicatorComponent> for ConsignmentArrayOfBulkCargoIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfBulkCargoIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfBulkCargoIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfBulkCargoIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BulkCargoIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BulkCargoIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BulkCargoIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BulkCargoIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfCarrierAssignedIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CarrierAssignedID>,
}

impl AsMut<ConsignmentArrayOfCarrierAssignedIDComponent> for ConsignmentArrayOfCarrierAssignedIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfCarrierAssignedIDComponent> for ConsignmentArrayOfCarrierAssignedIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfCarrierAssignedIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfCarrierAssignedIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfCarrierAssignedIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CarrierAssignedID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CarrierAssignedID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CarrierAssignedID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CarrierAssignedID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfCarrierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CarrierParty>,
}

impl AsMut<ConsignmentArrayOfCarrierPartyComponent> for ConsignmentArrayOfCarrierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfCarrierPartyComponent> for ConsignmentArrayOfCarrierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfCarrierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfCarrierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfCarrierPartyComponent {
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
pub struct ConsignmentArrayOfCarrierServiceInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CarrierServiceInstructions>,
}

impl AsMut<ConsignmentArrayOfCarrierServiceInstructionsComponent> for ConsignmentArrayOfCarrierServiceInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfCarrierServiceInstructionsComponent> for ConsignmentArrayOfCarrierServiceInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfCarrierServiceInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfCarrierServiceInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CarrierServiceInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CarrierServiceInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CarrierServiceInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CarrierServiceInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfChargeableWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ChargeableWeightMeasure>,
}

impl AsMut<ConsignmentArrayOfChargeableWeightMeasureComponent> for ConsignmentArrayOfChargeableWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfChargeableWeightMeasureComponent> for ConsignmentArrayOfChargeableWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfChargeableWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfChargeableWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfChargeableWeightMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ChargeableWeightMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ChargeableWeightMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ChargeableWeightMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ChargeableWeightMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfChildConsignmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ChildConsignment>,
}

impl AsMut<ConsignmentArrayOfChildConsignmentComponent> for ConsignmentArrayOfChildConsignmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfChildConsignmentComponent> for ConsignmentArrayOfChildConsignmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfChildConsignmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfChildConsignmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ChildConsignment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ChildConsignment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ChildConsignment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ChildConsignment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfChildConsignmentQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ChildConsignmentQuantity>,
}

impl AsMut<ConsignmentArrayOfChildConsignmentQuantityComponent> for ConsignmentArrayOfChildConsignmentQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfChildConsignmentQuantityComponent> for ConsignmentArrayOfChildConsignmentQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfChildConsignmentQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfChildConsignmentQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfChildConsignmentQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ChildConsignmentQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ChildConsignmentQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ChildConsignmentQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ChildConsignmentQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfCollectPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CollectPaymentTerms>,
}

impl AsMut<ConsignmentArrayOfCollectPaymentTermsComponent> for ConsignmentArrayOfCollectPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfCollectPaymentTermsComponent> for ConsignmentArrayOfCollectPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfCollectPaymentTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfCollectPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfCollectPaymentTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CollectPaymentTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CollectPaymentTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CollectPaymentTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CollectPaymentTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfConsigneeAssignedIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsigneeAssignedID>,
}

impl AsMut<ConsignmentArrayOfConsigneeAssignedIDComponent> for ConsignmentArrayOfConsigneeAssignedIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfConsigneeAssignedIDComponent> for ConsignmentArrayOfConsigneeAssignedIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsigneeAssignedIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsigneeAssignedIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfConsigneeAssignedIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsigneeAssignedID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsigneeAssignedID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsigneeAssignedID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsigneeAssignedID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfConsigneePartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ConsigneeParty>,
}

impl AsMut<ConsignmentArrayOfConsigneePartyComponent> for ConsignmentArrayOfConsigneePartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfConsigneePartyComponent> for ConsignmentArrayOfConsigneePartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsigneePartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsigneePartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfConsigneePartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ConsigneeParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ConsigneeParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ConsigneeParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ConsigneeParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfConsignmentQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsignmentQuantity>,
}

impl AsMut<ConsignmentArrayOfConsignmentQuantityComponent> for ConsignmentArrayOfConsignmentQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfConsignmentQuantityComponent> for ConsignmentArrayOfConsignmentQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsignmentQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsignmentQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfConsignmentQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsignmentQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsignmentQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsignmentQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsignmentQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfConsignorAssignedIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsignorAssignedID>,
}

impl AsMut<ConsignmentArrayOfConsignorAssignedIDComponent> for ConsignmentArrayOfConsignorAssignedIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfConsignorAssignedIDComponent> for ConsignmentArrayOfConsignorAssignedIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsignorAssignedIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsignorAssignedIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfConsignorAssignedIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsignorAssignedID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsignorAssignedID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsignorAssignedID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsignorAssignedID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfConsignorPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ConsignorParty>,
}

impl AsMut<ConsignmentArrayOfConsignorPartyComponent> for ConsignmentArrayOfConsignorPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfConsignorPartyComponent> for ConsignmentArrayOfConsignorPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsignorPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsignorPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfConsignorPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ConsignorParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ConsignorParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ConsignorParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ConsignorParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfConsolidatableIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsolidatableIndicator>,
}

impl AsMut<ConsignmentArrayOfConsolidatableIndicatorComponent> for ConsignmentArrayOfConsolidatableIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfConsolidatableIndicatorComponent> for ConsignmentArrayOfConsolidatableIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsolidatableIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsolidatableIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfConsolidatableIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsolidatableIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsolidatableIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsolidatableIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsolidatableIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfConsolidatedShipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ConsolidatedShipment>,
}

impl AsMut<ConsignmentArrayOfConsolidatedShipmentComponent> for ConsignmentArrayOfConsolidatedShipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfConsolidatedShipmentComponent> for ConsignmentArrayOfConsolidatedShipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfConsolidatedShipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfConsolidatedShipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ConsolidatedShipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ConsolidatedShipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ConsolidatedShipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ConsolidatedShipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfContainerizedIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ContainerizedIndicator>,
}

impl AsMut<ConsignmentArrayOfContainerizedIndicatorComponent> for ConsignmentArrayOfContainerizedIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfContainerizedIndicatorComponent> for ConsignmentArrayOfContainerizedIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfContainerizedIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfContainerizedIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfContainerizedIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ContainerizedIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ContainerizedIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ContainerizedIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ContainerizedIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfContractedCarrierAssignedIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ContractedCarrierAssignedID>,
}

impl AsMut<ConsignmentArrayOfContractedCarrierAssignedIDComponent> for ConsignmentArrayOfContractedCarrierAssignedIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfContractedCarrierAssignedIDComponent> for ConsignmentArrayOfContractedCarrierAssignedIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfContractedCarrierAssignedIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfContractedCarrierAssignedIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfContractedCarrierAssignedIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ContractedCarrierAssignedID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ContractedCarrierAssignedID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ContractedCarrierAssignedID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ContractedCarrierAssignedID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfCustomsClearanceServiceInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CustomsClearanceServiceInstructions>,
}

impl AsMut<ConsignmentArrayOfCustomsClearanceServiceInstructionsComponent> for ConsignmentArrayOfCustomsClearanceServiceInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfCustomsClearanceServiceInstructionsComponent> for ConsignmentArrayOfCustomsClearanceServiceInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfCustomsClearanceServiceInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfCustomsClearanceServiceInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CustomsClearanceServiceInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CustomsClearanceServiceInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CustomsClearanceServiceInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CustomsClearanceServiceInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfCustomsDeclarationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CustomsDeclaration>,
}

impl AsMut<ConsignmentArrayOfCustomsDeclarationComponent> for ConsignmentArrayOfCustomsDeclarationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfCustomsDeclarationComponent> for ConsignmentArrayOfCustomsDeclarationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfCustomsDeclarationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfCustomsDeclarationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CustomsDeclaration) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CustomsDeclaration) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CustomsDeclaration> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CustomsDeclaration> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfDeclaredCustomsValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeclaredCustomsValueAmount>,
}

impl AsMut<ConsignmentArrayOfDeclaredCustomsValueAmountComponent> for ConsignmentArrayOfDeclaredCustomsValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfDeclaredCustomsValueAmountComponent> for ConsignmentArrayOfDeclaredCustomsValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfDeclaredCustomsValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfDeclaredCustomsValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfDeclaredCustomsValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DeclaredCustomsValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DeclaredCustomsValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DeclaredCustomsValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DeclaredCustomsValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfDeclaredForCarriageValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeclaredForCarriageValueAmount>,
}

impl AsMut<ConsignmentArrayOfDeclaredForCarriageValueAmountComponent> for ConsignmentArrayOfDeclaredForCarriageValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfDeclaredForCarriageValueAmountComponent> for ConsignmentArrayOfDeclaredForCarriageValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfDeclaredForCarriageValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfDeclaredForCarriageValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfDeclaredForCarriageValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DeclaredForCarriageValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DeclaredForCarriageValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DeclaredForCarriageValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DeclaredForCarriageValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfDeclaredStatisticsValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeclaredStatisticsValueAmount>,
}

impl AsMut<ConsignmentArrayOfDeclaredStatisticsValueAmountComponent> for ConsignmentArrayOfDeclaredStatisticsValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfDeclaredStatisticsValueAmountComponent> for ConsignmentArrayOfDeclaredStatisticsValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfDeclaredStatisticsValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfDeclaredStatisticsValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfDeclaredStatisticsValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DeclaredStatisticsValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DeclaredStatisticsValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DeclaredStatisticsValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DeclaredStatisticsValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfDeliveryInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeliveryInstructions>,
}

impl AsMut<ConsignmentArrayOfDeliveryInstructionsComponent> for ConsignmentArrayOfDeliveryInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfDeliveryInstructionsComponent> for ConsignmentArrayOfDeliveryInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfDeliveryInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfDeliveryInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DeliveryInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DeliveryInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DeliveryInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DeliveryInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfDeliveryTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms>,
}

impl AsMut<ConsignmentArrayOfDeliveryTermsComponent> for ConsignmentArrayOfDeliveryTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfDeliveryTermsComponent> for ConsignmentArrayOfDeliveryTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfDeliveryTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfDeliveryTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfDeliveryTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfDisbursementPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DisbursementPaymentTerms>,
}

impl AsMut<ConsignmentArrayOfDisbursementPaymentTermsComponent> for ConsignmentArrayOfDisbursementPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfDisbursementPaymentTermsComponent> for ConsignmentArrayOfDisbursementPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfDisbursementPaymentTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfDisbursementPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfDisbursementPaymentTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DisbursementPaymentTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DisbursementPaymentTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DisbursementPaymentTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DisbursementPaymentTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfExporterPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExporterParty>,
}

impl AsMut<ConsignmentArrayOfExporterPartyComponent> for ConsignmentArrayOfExporterPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfExporterPartyComponent> for ConsignmentArrayOfExporterPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfExporterPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfExporterPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfExporterPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ExporterParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ExporterParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ExporterParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ExporterParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfExtraAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExtraAllowanceCharge>,
}

impl AsMut<ConsignmentArrayOfExtraAllowanceChargeComponent> for ConsignmentArrayOfExtraAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfExtraAllowanceChargeComponent> for ConsignmentArrayOfExtraAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfExtraAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfExtraAllowanceChargeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ExtraAllowanceCharge) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ExtraAllowanceCharge) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ExtraAllowanceCharge> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ExtraAllowanceCharge> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfFinalDeliveryPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FinalDeliveryParty>,
}

impl AsMut<ConsignmentArrayOfFinalDeliveryPartyComponent> for ConsignmentArrayOfFinalDeliveryPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfFinalDeliveryPartyComponent> for ConsignmentArrayOfFinalDeliveryPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFinalDeliveryPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFinalDeliveryPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfFinalDeliveryPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FinalDeliveryParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FinalDeliveryParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FinalDeliveryParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FinalDeliveryParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfFinalDeliveryTransportationServiceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FinalDeliveryTransportationService>,
}

impl AsMut<ConsignmentArrayOfFinalDeliveryTransportationServiceComponent> for ConsignmentArrayOfFinalDeliveryTransportationServiceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfFinalDeliveryTransportationServiceComponent> for ConsignmentArrayOfFinalDeliveryTransportationServiceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFinalDeliveryTransportationServiceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFinalDeliveryTransportationServiceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfFinalDeliveryTransportationServiceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FinalDeliveryTransportationService) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FinalDeliveryTransportationService) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FinalDeliveryTransportationService> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FinalDeliveryTransportationService> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfFinalDestinationCountryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FinalDestinationCountry>,
}

impl AsMut<ConsignmentArrayOfFinalDestinationCountryComponent> for ConsignmentArrayOfFinalDestinationCountryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfFinalDestinationCountryComponent> for ConsignmentArrayOfFinalDestinationCountryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFinalDestinationCountryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFinalDestinationCountryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfFinalDestinationCountryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FinalDestinationCountry) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FinalDestinationCountry) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FinalDestinationCountry> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FinalDestinationCountry> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfFirstArrivalPortLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FirstArrivalPortLocation>,
}

impl AsMut<ConsignmentArrayOfFirstArrivalPortLocationComponent> for ConsignmentArrayOfFirstArrivalPortLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfFirstArrivalPortLocationComponent> for ConsignmentArrayOfFirstArrivalPortLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFirstArrivalPortLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFirstArrivalPortLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfFirstArrivalPortLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FirstArrivalPortLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FirstArrivalPortLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FirstArrivalPortLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FirstArrivalPortLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfForwarderServiceInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ForwarderServiceInstructions>,
}

impl AsMut<ConsignmentArrayOfForwarderServiceInstructionsComponent> for ConsignmentArrayOfForwarderServiceInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfForwarderServiceInstructionsComponent> for ConsignmentArrayOfForwarderServiceInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfForwarderServiceInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfForwarderServiceInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ForwarderServiceInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ForwarderServiceInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ForwarderServiceInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ForwarderServiceInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfFreeOnBoardValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FreeOnBoardValueAmount>,
}

impl AsMut<ConsignmentArrayOfFreeOnBoardValueAmountComponent> for ConsignmentArrayOfFreeOnBoardValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfFreeOnBoardValueAmountComponent> for ConsignmentArrayOfFreeOnBoardValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFreeOnBoardValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFreeOnBoardValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfFreeOnBoardValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FreeOnBoardValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FreeOnBoardValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FreeOnBoardValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FreeOnBoardValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfFreightAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge>,
}

impl AsMut<ConsignmentArrayOfFreightAllowanceChargeComponent> for ConsignmentArrayOfFreightAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfFreightAllowanceChargeComponent> for ConsignmentArrayOfFreightAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFreightAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfFreightAllowanceChargeComponent {
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
pub struct ConsignmentArrayOfFreightForwarderAssignedIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FreightForwarderAssignedID>,
}

impl AsMut<ConsignmentArrayOfFreightForwarderAssignedIDComponent> for ConsignmentArrayOfFreightForwarderAssignedIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfFreightForwarderAssignedIDComponent> for ConsignmentArrayOfFreightForwarderAssignedIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFreightForwarderAssignedIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFreightForwarderAssignedIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfFreightForwarderAssignedIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FreightForwarderAssignedID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FreightForwarderAssignedID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FreightForwarderAssignedID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FreightForwarderAssignedID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfFreightForwarderPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FreightForwarderParty>,
}

impl AsMut<ConsignmentArrayOfFreightForwarderPartyComponent> for ConsignmentArrayOfFreightForwarderPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfFreightForwarderPartyComponent> for ConsignmentArrayOfFreightForwarderPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFreightForwarderPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfFreightForwarderPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfFreightForwarderPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FreightForwarderParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FreightForwarderParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FreightForwarderParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FreightForwarderParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfGeneralCargoIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GeneralCargoIndicator>,
}

impl AsMut<ConsignmentArrayOfGeneralCargoIndicatorComponent> for ConsignmentArrayOfGeneralCargoIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfGeneralCargoIndicatorComponent> for ConsignmentArrayOfGeneralCargoIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfGeneralCargoIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfGeneralCargoIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfGeneralCargoIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GeneralCargoIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GeneralCargoIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GeneralCargoIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GeneralCargoIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfGrossVolumeMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure>,
}

impl AsMut<ConsignmentArrayOfGrossVolumeMeasureComponent> for ConsignmentArrayOfGrossVolumeMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfGrossVolumeMeasureComponent> for ConsignmentArrayOfGrossVolumeMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfGrossVolumeMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfGrossVolumeMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfGrossVolumeMeasureComponent {
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
pub struct ConsignmentArrayOfGrossWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure>,
}

impl AsMut<ConsignmentArrayOfGrossWeightMeasureComponent> for ConsignmentArrayOfGrossWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfGrossWeightMeasureComponent> for ConsignmentArrayOfGrossWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfGrossWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfGrossWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfGrossWeightMeasureComponent {
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
pub struct ConsignmentArrayOfHandlingCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HandlingCode>,
}

impl AsMut<ConsignmentArrayOfHandlingCodeComponent> for ConsignmentArrayOfHandlingCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfHandlingCodeComponent> for ConsignmentArrayOfHandlingCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfHandlingCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfHandlingCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfHandlingCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HandlingCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HandlingCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HandlingCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HandlingCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfHandlingInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HandlingInstructions>,
}

impl AsMut<ConsignmentArrayOfHandlingInstructionsComponent> for ConsignmentArrayOfHandlingInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfHandlingInstructionsComponent> for ConsignmentArrayOfHandlingInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfHandlingInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfHandlingInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HandlingInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HandlingInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HandlingInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HandlingInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfHaulageInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HaulageInstructions>,
}

impl AsMut<ConsignmentArrayOfHaulageInstructionsComponent> for ConsignmentArrayOfHaulageInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfHaulageInstructionsComponent> for ConsignmentArrayOfHaulageInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfHaulageInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfHaulageInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HaulageInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HaulageInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HaulageInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HaulageInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfHazardousItemNotificationPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::HazardousItemNotificationParty>,
}

impl AsMut<ConsignmentArrayOfHazardousItemNotificationPartyComponent> for ConsignmentArrayOfHazardousItemNotificationPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfHazardousItemNotificationPartyComponent> for ConsignmentArrayOfHazardousItemNotificationPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfHazardousItemNotificationPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfHazardousItemNotificationPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfHazardousItemNotificationPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::HazardousItemNotificationParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::HazardousItemNotificationParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::HazardousItemNotificationParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::HazardousItemNotificationParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfHazardousRiskIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator>,
}

impl AsMut<ConsignmentArrayOfHazardousRiskIndicatorComponent> for ConsignmentArrayOfHazardousRiskIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfHazardousRiskIndicatorComponent> for ConsignmentArrayOfHazardousRiskIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfHazardousRiskIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfHazardousRiskIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfHazardousRiskIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfHumanFoodIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HumanFoodIndicator>,
}

impl AsMut<ConsignmentArrayOfHumanFoodIndicatorComponent> for ConsignmentArrayOfHumanFoodIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfHumanFoodIndicatorComponent> for ConsignmentArrayOfHumanFoodIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfHumanFoodIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfHumanFoodIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfHumanFoodIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HumanFoodIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HumanFoodIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HumanFoodIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HumanFoodIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ConsignmentArrayOfIDComponent> for ConsignmentArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfIDComponent> for ConsignmentArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfIDComponent {
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
pub struct ConsignmentArrayOfImporterPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ImporterParty>,
}

impl AsMut<ConsignmentArrayOfImporterPartyComponent> for ConsignmentArrayOfImporterPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfImporterPartyComponent> for ConsignmentArrayOfImporterPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfImporterPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfImporterPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfImporterPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ImporterParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ImporterParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ImporterParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ImporterParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfInformationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Information>,
}

impl AsMut<ConsignmentArrayOfInformationComponent> for ConsignmentArrayOfInformationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfInformationComponent> for ConsignmentArrayOfInformationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfInformationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfInformationComponent {
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
pub struct ConsignmentArrayOfInsurancePartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::InsuranceParty>,
}

impl AsMut<ConsignmentArrayOfInsurancePartyComponent> for ConsignmentArrayOfInsurancePartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfInsurancePartyComponent> for ConsignmentArrayOfInsurancePartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfInsurancePartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfInsurancePartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfInsurancePartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::InsuranceParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::InsuranceParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::InsuranceParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::InsuranceParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfInsurancePremiumAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InsurancePremiumAmount>,
}

impl AsMut<ConsignmentArrayOfInsurancePremiumAmountComponent> for ConsignmentArrayOfInsurancePremiumAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfInsurancePremiumAmountComponent> for ConsignmentArrayOfInsurancePremiumAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfInsurancePremiumAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfInsurancePremiumAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfInsurancePremiumAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InsurancePremiumAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InsurancePremiumAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InsurancePremiumAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InsurancePremiumAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfInsuranceValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InsuranceValueAmount>,
}

impl AsMut<ConsignmentArrayOfInsuranceValueAmountComponent> for ConsignmentArrayOfInsuranceValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfInsuranceValueAmountComponent> for ConsignmentArrayOfInsuranceValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfInsuranceValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfInsuranceValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfInsuranceValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InsuranceValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InsuranceValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InsuranceValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InsuranceValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfLastExitPortLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LastExitPortLocation>,
}

impl AsMut<ConsignmentArrayOfLastExitPortLocationComponent> for ConsignmentArrayOfLastExitPortLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfLastExitPortLocationComponent> for ConsignmentArrayOfLastExitPortLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfLastExitPortLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfLastExitPortLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfLastExitPortLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LastExitPortLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LastExitPortLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LastExitPortLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LastExitPortLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfLivestockIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LivestockIndicator>,
}

impl AsMut<ConsignmentArrayOfLivestockIndicatorComponent> for ConsignmentArrayOfLivestockIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfLivestockIndicatorComponent> for ConsignmentArrayOfLivestockIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfLivestockIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfLivestockIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfLivestockIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LivestockIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LivestockIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LivestockIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LivestockIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfLoadingLengthMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LoadingLengthMeasure>,
}

impl AsMut<ConsignmentArrayOfLoadingLengthMeasureComponent> for ConsignmentArrayOfLoadingLengthMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfLoadingLengthMeasureComponent> for ConsignmentArrayOfLoadingLengthMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfLoadingLengthMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfLoadingLengthMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfLoadingLengthMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LoadingLengthMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LoadingLengthMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LoadingLengthMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LoadingLengthMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfLoadingSequenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LoadingSequenceID>,
}

impl AsMut<ConsignmentArrayOfLoadingSequenceIDComponent> for ConsignmentArrayOfLoadingSequenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfLoadingSequenceIDComponent> for ConsignmentArrayOfLoadingSequenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfLoadingSequenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfLoadingSequenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfLoadingSequenceIDComponent {
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
pub struct ConsignmentArrayOfLogisticsOperatorPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LogisticsOperatorParty>,
}

impl AsMut<ConsignmentArrayOfLogisticsOperatorPartyComponent> for ConsignmentArrayOfLogisticsOperatorPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfLogisticsOperatorPartyComponent> for ConsignmentArrayOfLogisticsOperatorPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfLogisticsOperatorPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfLogisticsOperatorPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfLogisticsOperatorPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LogisticsOperatorParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LogisticsOperatorParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LogisticsOperatorParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LogisticsOperatorParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfMainCarriageShipmentStageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MainCarriageShipmentStage>,
}

impl AsMut<ConsignmentArrayOfMainCarriageShipmentStageComponent> for ConsignmentArrayOfMainCarriageShipmentStageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfMainCarriageShipmentStageComponent> for ConsignmentArrayOfMainCarriageShipmentStageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfMainCarriageShipmentStageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfMainCarriageShipmentStageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MainCarriageShipmentStage) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MainCarriageShipmentStage) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MainCarriageShipmentStage> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MainCarriageShipmentStage> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfMortgageHolderPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MortgageHolderParty>,
}

impl AsMut<ConsignmentArrayOfMortgageHolderPartyComponent> for ConsignmentArrayOfMortgageHolderPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfMortgageHolderPartyComponent> for ConsignmentArrayOfMortgageHolderPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfMortgageHolderPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfMortgageHolderPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfMortgageHolderPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MortgageHolderParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MortgageHolderParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MortgageHolderParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MortgageHolderParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfNetNetWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetNetWeightMeasure>,
}

impl AsMut<ConsignmentArrayOfNetNetWeightMeasureComponent> for ConsignmentArrayOfNetNetWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfNetNetWeightMeasureComponent> for ConsignmentArrayOfNetNetWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfNetNetWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfNetNetWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfNetNetWeightMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NetNetWeightMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NetNetWeightMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NetNetWeightMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NetNetWeightMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfNetVolumeMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure>,
}

impl AsMut<ConsignmentArrayOfNetVolumeMeasureComponent> for ConsignmentArrayOfNetVolumeMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfNetVolumeMeasureComponent> for ConsignmentArrayOfNetVolumeMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfNetVolumeMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfNetVolumeMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfNetVolumeMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfNetWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure>,
}

impl AsMut<ConsignmentArrayOfNetWeightMeasureComponent> for ConsignmentArrayOfNetWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfNetWeightMeasureComponent> for ConsignmentArrayOfNetWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfNetWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfNetWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfNetWeightMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfNotifyPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::NotifyParty>,
}

impl AsMut<ConsignmentArrayOfNotifyPartyComponent> for ConsignmentArrayOfNotifyPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfNotifyPartyComponent> for ConsignmentArrayOfNotifyPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfNotifyPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfNotifyPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfNotifyPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::NotifyParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::NotifyParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::NotifyParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::NotifyParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfOnCarriageShipmentStageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OnCarriageShipmentStage>,
}

impl AsMut<ConsignmentArrayOfOnCarriageShipmentStageComponent> for ConsignmentArrayOfOnCarriageShipmentStageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfOnCarriageShipmentStageComponent> for ConsignmentArrayOfOnCarriageShipmentStageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfOnCarriageShipmentStageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfOnCarriageShipmentStageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OnCarriageShipmentStage) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OnCarriageShipmentStage) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OnCarriageShipmentStage> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OnCarriageShipmentStage> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfOriginalDepartureCountryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginalDepartureCountry>,
}

impl AsMut<ConsignmentArrayOfOriginalDepartureCountryComponent> for ConsignmentArrayOfOriginalDepartureCountryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfOriginalDepartureCountryComponent> for ConsignmentArrayOfOriginalDepartureCountryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfOriginalDepartureCountryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfOriginalDepartureCountryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfOriginalDepartureCountryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OriginalDepartureCountry) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OriginalDepartureCountry) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OriginalDepartureCountry> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OriginalDepartureCountry> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfOriginalDespatchPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginalDespatchParty>,
}

impl AsMut<ConsignmentArrayOfOriginalDespatchPartyComponent> for ConsignmentArrayOfOriginalDespatchPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfOriginalDespatchPartyComponent> for ConsignmentArrayOfOriginalDespatchPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfOriginalDespatchPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfOriginalDespatchPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfOriginalDespatchPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OriginalDespatchParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OriginalDespatchParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OriginalDespatchParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OriginalDespatchParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfOriginalDespatchTransportationServiceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginalDespatchTransportationService>,
}

impl AsMut<ConsignmentArrayOfOriginalDespatchTransportationServiceComponent> for ConsignmentArrayOfOriginalDespatchTransportationServiceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfOriginalDespatchTransportationServiceComponent> for ConsignmentArrayOfOriginalDespatchTransportationServiceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfOriginalDespatchTransportationServiceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfOriginalDespatchTransportationServiceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfOriginalDespatchTransportationServiceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OriginalDespatchTransportationService) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OriginalDespatchTransportationService) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OriginalDespatchTransportationService> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OriginalDespatchTransportationService> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms>,
}

impl AsMut<ConsignmentArrayOfPaymentTermsComponent> for ConsignmentArrayOfPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfPaymentTermsComponent> for ConsignmentArrayOfPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPaymentTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfPaymentTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PaymentTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PaymentTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfPerformingCarrierAssignedIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PerformingCarrierAssignedID>,
}

impl AsMut<ConsignmentArrayOfPerformingCarrierAssignedIDComponent> for ConsignmentArrayOfPerformingCarrierAssignedIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfPerformingCarrierAssignedIDComponent> for ConsignmentArrayOfPerformingCarrierAssignedIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPerformingCarrierAssignedIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPerformingCarrierAssignedIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfPerformingCarrierAssignedIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PerformingCarrierAssignedID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PerformingCarrierAssignedID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PerformingCarrierAssignedID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PerformingCarrierAssignedID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfPerformingCarrierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PerformingCarrierParty>,
}

impl AsMut<ConsignmentArrayOfPerformingCarrierPartyComponent> for ConsignmentArrayOfPerformingCarrierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfPerformingCarrierPartyComponent> for ConsignmentArrayOfPerformingCarrierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPerformingCarrierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPerformingCarrierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfPerformingCarrierPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PerformingCarrierParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PerformingCarrierParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PerformingCarrierParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PerformingCarrierParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfPlannedDeliveryTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PlannedDeliveryTransportEvent>,
}

impl AsMut<ConsignmentArrayOfPlannedDeliveryTransportEventComponent> for ConsignmentArrayOfPlannedDeliveryTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfPlannedDeliveryTransportEventComponent> for ConsignmentArrayOfPlannedDeliveryTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPlannedDeliveryTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPlannedDeliveryTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfPlannedDeliveryTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PlannedDeliveryTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PlannedDeliveryTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PlannedDeliveryTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PlannedDeliveryTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfPlannedPickupTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PlannedPickupTransportEvent>,
}

impl AsMut<ConsignmentArrayOfPlannedPickupTransportEventComponent> for ConsignmentArrayOfPlannedPickupTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfPlannedPickupTransportEventComponent> for ConsignmentArrayOfPlannedPickupTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPlannedPickupTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPlannedPickupTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfPlannedPickupTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PlannedPickupTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PlannedPickupTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PlannedPickupTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PlannedPickupTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfPreCarriageShipmentStageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PreCarriageShipmentStage>,
}

impl AsMut<ConsignmentArrayOfPreCarriageShipmentStageComponent> for ConsignmentArrayOfPreCarriageShipmentStageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfPreCarriageShipmentStageComponent> for ConsignmentArrayOfPreCarriageShipmentStageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPreCarriageShipmentStageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfPreCarriageShipmentStageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PreCarriageShipmentStage) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PreCarriageShipmentStage) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PreCarriageShipmentStage> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PreCarriageShipmentStage> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfPrepaidPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PrepaidPaymentTerms>,
}

impl AsMut<ConsignmentArrayOfPrepaidPaymentTermsComponent> for ConsignmentArrayOfPrepaidPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfPrepaidPaymentTermsComponent> for ConsignmentArrayOfPrepaidPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPrepaidPaymentTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfPrepaidPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfPrepaidPaymentTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PrepaidPaymentTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PrepaidPaymentTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PrepaidPaymentTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PrepaidPaymentTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfRemarksComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Remarks>,
}

impl AsMut<ConsignmentArrayOfRemarksComponent> for ConsignmentArrayOfRemarksComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfRemarksComponent> for ConsignmentArrayOfRemarksComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfRemarksComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfRemarksComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Remarks) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Remarks) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Remarks> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Remarks> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfRequestedDeliveryTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequestedDeliveryTransportEvent>,
}

impl AsMut<ConsignmentArrayOfRequestedDeliveryTransportEventComponent> for ConsignmentArrayOfRequestedDeliveryTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfRequestedDeliveryTransportEventComponent> for ConsignmentArrayOfRequestedDeliveryTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfRequestedDeliveryTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfRequestedDeliveryTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfRequestedDeliveryTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequestedDeliveryTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequestedDeliveryTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequestedDeliveryTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequestedDeliveryTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfRequestedPickupTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequestedPickupTransportEvent>,
}

impl AsMut<ConsignmentArrayOfRequestedPickupTransportEventComponent> for ConsignmentArrayOfRequestedPickupTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfRequestedPickupTransportEventComponent> for ConsignmentArrayOfRequestedPickupTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfRequestedPickupTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfRequestedPickupTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfRequestedPickupTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequestedPickupTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequestedPickupTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequestedPickupTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequestedPickupTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfSequenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SequenceID>,
}

impl AsMut<ConsignmentArrayOfSequenceIDComponent> for ConsignmentArrayOfSequenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfSequenceIDComponent> for ConsignmentArrayOfSequenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfSequenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfSequenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfSequenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SequenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SequenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SequenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SequenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfShippingPriorityLevelCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ShippingPriorityLevelCode>,
}

impl AsMut<ConsignmentArrayOfShippingPriorityLevelCodeComponent> for ConsignmentArrayOfShippingPriorityLevelCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfShippingPriorityLevelCodeComponent> for ConsignmentArrayOfShippingPriorityLevelCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfShippingPriorityLevelCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfShippingPriorityLevelCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfShippingPriorityLevelCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ShippingPriorityLevelCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ShippingPriorityLevelCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ShippingPriorityLevelCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ShippingPriorityLevelCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfSpecialInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SpecialInstructions>,
}

impl AsMut<ConsignmentArrayOfSpecialInstructionsComponent> for ConsignmentArrayOfSpecialInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfSpecialInstructionsComponent> for ConsignmentArrayOfSpecialInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfSpecialInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfSpecialInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SpecialInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SpecialInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SpecialInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SpecialInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfSpecialSecurityIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SpecialSecurityIndicator>,
}

impl AsMut<ConsignmentArrayOfSpecialSecurityIndicatorComponent> for ConsignmentArrayOfSpecialSecurityIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfSpecialSecurityIndicatorComponent> for ConsignmentArrayOfSpecialSecurityIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfSpecialSecurityIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfSpecialSecurityIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfSpecialSecurityIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SpecialSecurityIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SpecialSecurityIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SpecialSecurityIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SpecialSecurityIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfSpecialServiceInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SpecialServiceInstructions>,
}

impl AsMut<ConsignmentArrayOfSpecialServiceInstructionsComponent> for ConsignmentArrayOfSpecialServiceInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfSpecialServiceInstructionsComponent> for ConsignmentArrayOfSpecialServiceInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfSpecialServiceInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfSpecialServiceInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SpecialServiceInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SpecialServiceInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SpecialServiceInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SpecialServiceInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfSplitConsignmentIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SplitConsignmentIndicator>,
}

impl AsMut<ConsignmentArrayOfSplitConsignmentIndicatorComponent> for ConsignmentArrayOfSplitConsignmentIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfSplitConsignmentIndicatorComponent> for ConsignmentArrayOfSplitConsignmentIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfSplitConsignmentIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfSplitConsignmentIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfSplitConsignmentIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SplitConsignmentIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SplitConsignmentIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SplitConsignmentIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SplitConsignmentIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfStatusComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Status>,
}

impl AsMut<ConsignmentArrayOfStatusComponent> for ConsignmentArrayOfStatusComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfStatusComponent> for ConsignmentArrayOfStatusComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfStatusComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfStatusComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Status) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Status) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Status> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Status> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfSubstituteCarrierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubstituteCarrierParty>,
}

impl AsMut<ConsignmentArrayOfSubstituteCarrierPartyComponent> for ConsignmentArrayOfSubstituteCarrierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfSubstituteCarrierPartyComponent> for ConsignmentArrayOfSubstituteCarrierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfSubstituteCarrierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfSubstituteCarrierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfSubstituteCarrierPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubstituteCarrierParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubstituteCarrierParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubstituteCarrierParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubstituteCarrierParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfSummaryDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SummaryDescription>,
}

impl AsMut<ConsignmentArrayOfSummaryDescriptionComponent> for ConsignmentArrayOfSummaryDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfSummaryDescriptionComponent> for ConsignmentArrayOfSummaryDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfSummaryDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfSummaryDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SummaryDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SummaryDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SummaryDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SummaryDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfTariffCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TariffCode>,
}

impl AsMut<ConsignmentArrayOfTariffCodeComponent> for ConsignmentArrayOfTariffCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfTariffCodeComponent> for ConsignmentArrayOfTariffCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTariffCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTariffCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfTariffCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TariffCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TariffCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TariffCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TariffCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfTariffDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TariffDescription>,
}

impl AsMut<ConsignmentArrayOfTariffDescriptionComponent> for ConsignmentArrayOfTariffDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfTariffDescriptionComponent> for ConsignmentArrayOfTariffDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTariffDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfTariffDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TariffDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TariffDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TariffDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TariffDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfThirdPartyPayerIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ThirdPartyPayerIndicator>,
}

impl AsMut<ConsignmentArrayOfThirdPartyPayerIndicatorComponent> for ConsignmentArrayOfThirdPartyPayerIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfThirdPartyPayerIndicatorComponent> for ConsignmentArrayOfThirdPartyPayerIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfThirdPartyPayerIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfThirdPartyPayerIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfThirdPartyPayerIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ThirdPartyPayerIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ThirdPartyPayerIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ThirdPartyPayerIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ThirdPartyPayerIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfTotalGoodsItemQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalGoodsItemQuantity>,
}

impl AsMut<ConsignmentArrayOfTotalGoodsItemQuantityComponent> for ConsignmentArrayOfTotalGoodsItemQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfTotalGoodsItemQuantityComponent> for ConsignmentArrayOfTotalGoodsItemQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTotalGoodsItemQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTotalGoodsItemQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfTotalGoodsItemQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalGoodsItemQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalGoodsItemQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalGoodsItemQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalGoodsItemQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfTotalInvoiceAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalInvoiceAmount>,
}

impl AsMut<ConsignmentArrayOfTotalInvoiceAmountComponent> for ConsignmentArrayOfTotalInvoiceAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfTotalInvoiceAmountComponent> for ConsignmentArrayOfTotalInvoiceAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTotalInvoiceAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTotalInvoiceAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfTotalInvoiceAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalInvoiceAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalInvoiceAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalInvoiceAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalInvoiceAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfTotalPackagesQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalPackagesQuantity>,
}

impl AsMut<ConsignmentArrayOfTotalPackagesQuantityComponent> for ConsignmentArrayOfTotalPackagesQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfTotalPackagesQuantityComponent> for ConsignmentArrayOfTotalPackagesQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTotalPackagesQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTotalPackagesQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfTotalPackagesQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalPackagesQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalPackagesQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalPackagesQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalPackagesQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfTotalTransportHandlingUnitQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalTransportHandlingUnitQuantity>,
}

impl AsMut<ConsignmentArrayOfTotalTransportHandlingUnitQuantityComponent> for ConsignmentArrayOfTotalTransportHandlingUnitQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfTotalTransportHandlingUnitQuantityComponent> for ConsignmentArrayOfTotalTransportHandlingUnitQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTotalTransportHandlingUnitQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTotalTransportHandlingUnitQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfTotalTransportHandlingUnitQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalTransportHandlingUnitQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalTransportHandlingUnitQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalTransportHandlingUnitQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalTransportHandlingUnitQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfTransitCountryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransitCountry>,
}

impl AsMut<ConsignmentArrayOfTransitCountryComponent> for ConsignmentArrayOfTransitCountryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfTransitCountryComponent> for ConsignmentArrayOfTransitCountryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTransitCountryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfTransitCountryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransitCountry) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransitCountry) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransitCountry> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransitCountry> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfTransportAdvisorPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportAdvisorParty>,
}

impl AsMut<ConsignmentArrayOfTransportAdvisorPartyComponent> for ConsignmentArrayOfTransportAdvisorPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfTransportAdvisorPartyComponent> for ConsignmentArrayOfTransportAdvisorPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTransportAdvisorPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTransportAdvisorPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfTransportAdvisorPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportAdvisorParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportAdvisorParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportAdvisorParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportAdvisorParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfTransportContractComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportContract>,
}

impl AsMut<ConsignmentArrayOfTransportContractComponent> for ConsignmentArrayOfTransportContractComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfTransportContractComponent> for ConsignmentArrayOfTransportContractComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTransportContractComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTransportContractComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfTransportContractComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportContract) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportContract) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportContract> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportContract> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsignmentArrayOfTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportEvent>,
}

impl AsMut<ConsignmentArrayOfTransportEventComponent> for ConsignmentArrayOfTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfTransportEventComponent> for ConsignmentArrayOfTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfTransportEventComponent {
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
pub struct ConsignmentArrayOfTransportHandlingUnitComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportHandlingUnit>,
}

impl AsMut<ConsignmentArrayOfTransportHandlingUnitComponent> for ConsignmentArrayOfTransportHandlingUnitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsignmentArrayOfTransportHandlingUnitComponent> for ConsignmentArrayOfTransportHandlingUnitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsignmentArrayOfTransportHandlingUnitComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsignmentArrayOfTransportHandlingUnitComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportHandlingUnit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportHandlingUnit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportHandlingUnit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportHandlingUnit> {
        self.items.iter()
    }
}


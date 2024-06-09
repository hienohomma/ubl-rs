use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransportationService {
    #[serde(rename = "CommodityClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity_classification: Option<TransportationServiceArrayOfCommodityClassificationComponent>,
    #[serde(rename = "EnvironmentalEmission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environmental_emission: Option<TransportationServiceArrayOfEnvironmentalEmissionComponent>,
    #[serde(rename = "EstimatedDurationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_duration_period: Option<TransportationServiceArrayOfEstimatedDurationPeriodComponent>,
    #[serde(rename = "FreightRateClassCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_rate_class_code: Option<TransportationServiceArrayOfFreightRateClassCodeComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TransportationServiceArrayOfNameComponent>,
    #[serde(rename = "NominationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nomination_date: Option<TransportationServiceArrayOfNominationDateComponent>,
    #[serde(rename = "NominationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nomination_time: Option<TransportationServiceArrayOfNominationTimeComponent>,
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TransportationServiceArrayOfPriorityComponent>,
    #[serde(rename = "ResponsibleTransportServiceProviderParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsible_transport_service_provider_party: Option<TransportationServiceArrayOfResponsibleTransportServiceProviderPartyComponent>,
    #[serde(rename = "ScheduledServiceFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_service_frequency: Option<TransportationServiceArrayOfScheduledServiceFrequencyComponent>,
    #[serde(rename = "SequenceNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_numeric: Option<TransportationServiceArrayOfSequenceNumericComponent>,
    #[serde(rename = "ShipmentStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_stage: Option<TransportationServiceArrayOfShipmentStageComponent>,
    #[serde(rename = "SupportedCommodityClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_commodity_classification: Option<TransportationServiceArrayOfSupportedCommodityClassificationComponent>,
    #[serde(rename = "SupportedTransportEquipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_transport_equipment: Option<TransportationServiceArrayOfSupportedTransportEquipmentComponent>,
    #[serde(rename = "TariffClassCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff_class_code: Option<TransportationServiceArrayOfTariffClassCodeComponent>,
    #[serde(rename = "TotalCapacityDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_capacity_dimension: Option<TransportationServiceArrayOfTotalCapacityDimensionComponent>,
    #[serde(rename = "TransportEquipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_equipment: Option<TransportationServiceArrayOfTransportEquipmentComponent>,
    #[serde(rename = "TransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_event: Option<TransportationServiceArrayOfTransportEventComponent>,
    #[serde(rename = "TransportServiceCode")]
    pub transport_service_code: TransportationServiceArrayOfTransportServiceCodeComponent,
    #[serde(rename = "TransportationServiceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transportation_service_description: Option<TransportationServiceArrayOfTransportationServiceDescriptionComponent>,
    #[serde(rename = "TransportationServiceDetailsURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transportation_service_details_uri: Option<TransportationServiceArrayOfTransportationServiceDetailsURIComponent>,
    #[serde(rename = "UnsupportedCommodityClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsupported_commodity_classification: Option<TransportationServiceArrayOfUnsupportedCommodityClassificationComponent>,
    #[serde(rename = "UnsupportedTransportEquipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsupported_transport_equipment: Option<TransportationServiceArrayOfUnsupportedTransportEquipmentComponent>,
}

impl AsMut<TransportationService> for TransportationService {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationService> for TransportationService {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.commodity_classification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.commodity_classification", e));
            }
        }
        if let Some(v) = &self.scheduled_service_frequency {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.scheduled_service_frequency", e));
            }
        }
        if let Some(v) = &self.nomination_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.nomination_date", e));
            }
        }
        if let Some(v) = &self.estimated_duration_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.estimated_duration_period", e));
            }
        }
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.name", e));
            }
        }
        if let Some(v) = &self.transportation_service_details_uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.transportation_service_details_uri", e));
            }
        }
        if let Some(v) = &self.unsupported_commodity_classification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.unsupported_commodity_classification", e));
            }
        }
        if let Some(v) = &self.environmental_emission {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.environmental_emission", e));
            }
        }
        if let Some(v) = &self.freight_rate_class_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.freight_rate_class_code", e));
            }
        }
        if let Some(v) = &self.nomination_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.nomination_time", e));
            }
        }
        if let Some(v) = &self.sequence_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.sequence_numeric", e));
            }
        }
        if let Some(v) = &self.tariff_class_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.tariff_class_code", e));
            }
        }
        if let Some(v) = &self.transportation_service_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.transportation_service_description", e));
            }
        }
        if let Some(v) = &self.supported_commodity_classification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.supported_commodity_classification", e));
            }
        }
        if let Some(v) = &self.unsupported_transport_equipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.unsupported_transport_equipment", e));
            }
        }
        if let Some(v) = &self.transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.transport_event", e));
            }
        }
        if let Some(v) = &self.responsible_transport_service_provider_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.responsible_transport_service_provider_party", e));
            }
        }
        if let Some(v) = &self.total_capacity_dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.total_capacity_dimension", e));
            }
        }
        if let Some(v) = &self.transport_equipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.transport_equipment", e));
            }
        }
        if let Err(e) = self.transport_service_code.validate() {
            return Err(UblError::component("TransportationService.transport_service_code", e));
        }
        if let Some(v) = &self.supported_transport_equipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.supported_transport_equipment", e));
            }
        }
        if let Some(v) = &self.shipment_stage {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.shipment_stage", e));
            }
        }
        if let Some(v) = &self.priority {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportationService.priority", e));
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

impl TransportationService {
    pub fn title() -> &'static str {
        "Transportation Service. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a transportation service."
    }
    pub fn new(transport_service_code: TransportationServiceArrayOfTransportServiceCodeComponent) -> Component<Self> {
        Component(Self {
            unsupported_commodity_classification: None,
            transportation_service_details_uri: None,
            nomination_date: None,
            priority: None,
            transport_event: None,
            supported_transport_equipment: None,
            estimated_duration_period: None,
            transportation_service_description: None,
            unsupported_transport_equipment: None,
            nomination_time: None,
            scheduled_service_frequency: None,
            supported_commodity_classification: None,
            responsible_transport_service_provider_party: None,
            sequence_numeric: None,
            tariff_class_code: None,
            total_capacity_dimension: None,
            freight_rate_class_code: None,
            environmental_emission: None,
            name: None,
            shipment_stage: None,
            transport_equipment: None,
            commodity_classification: None,
            transport_service_code,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfCommodityClassificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CommodityClassification>,
}

impl AsMut<TransportationServiceArrayOfCommodityClassificationComponent> for TransportationServiceArrayOfCommodityClassificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfCommodityClassificationComponent> for TransportationServiceArrayOfCommodityClassificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfCommodityClassificationComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfCommodityClassificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CommodityClassification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CommodityClassification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CommodityClassification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CommodityClassification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfEnvironmentalEmissionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EnvironmentalEmission>,
}

impl AsMut<TransportationServiceArrayOfEnvironmentalEmissionComponent> for TransportationServiceArrayOfEnvironmentalEmissionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfEnvironmentalEmissionComponent> for TransportationServiceArrayOfEnvironmentalEmissionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfEnvironmentalEmissionComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfEnvironmentalEmissionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EnvironmentalEmission) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EnvironmentalEmission) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EnvironmentalEmission> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EnvironmentalEmission> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfEstimatedDurationPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EstimatedDurationPeriod>,
}

impl AsMut<TransportationServiceArrayOfEstimatedDurationPeriodComponent> for TransportationServiceArrayOfEstimatedDurationPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfEstimatedDurationPeriodComponent> for TransportationServiceArrayOfEstimatedDurationPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfEstimatedDurationPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfEstimatedDurationPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfEstimatedDurationPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EstimatedDurationPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EstimatedDurationPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EstimatedDurationPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EstimatedDurationPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfFreightRateClassCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FreightRateClassCode>,
}

impl AsMut<TransportationServiceArrayOfFreightRateClassCodeComponent> for TransportationServiceArrayOfFreightRateClassCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfFreightRateClassCodeComponent> for TransportationServiceArrayOfFreightRateClassCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfFreightRateClassCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfFreightRateClassCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfFreightRateClassCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FreightRateClassCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FreightRateClassCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FreightRateClassCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FreightRateClassCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<TransportationServiceArrayOfNameComponent> for TransportationServiceArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfNameComponent> for TransportationServiceArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Name) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Name) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfNominationDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NominationDate>,
}

impl AsMut<TransportationServiceArrayOfNominationDateComponent> for TransportationServiceArrayOfNominationDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfNominationDateComponent> for TransportationServiceArrayOfNominationDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfNominationDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfNominationDateComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfNominationDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NominationDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NominationDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NominationDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NominationDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfNominationTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NominationTime>,
}

impl AsMut<TransportationServiceArrayOfNominationTimeComponent> for TransportationServiceArrayOfNominationTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfNominationTimeComponent> for TransportationServiceArrayOfNominationTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfNominationTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfNominationTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfNominationTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NominationTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NominationTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NominationTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NominationTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfPriorityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Priority>,
}

impl AsMut<TransportationServiceArrayOfPriorityComponent> for TransportationServiceArrayOfPriorityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfPriorityComponent> for TransportationServiceArrayOfPriorityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfPriorityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfPriorityComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfPriorityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Priority) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Priority) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Priority> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Priority> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfResponsibleTransportServiceProviderPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ResponsibleTransportServiceProviderParty>,
}

impl AsMut<TransportationServiceArrayOfResponsibleTransportServiceProviderPartyComponent> for TransportationServiceArrayOfResponsibleTransportServiceProviderPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfResponsibleTransportServiceProviderPartyComponent> for TransportationServiceArrayOfResponsibleTransportServiceProviderPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfResponsibleTransportServiceProviderPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfResponsibleTransportServiceProviderPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfResponsibleTransportServiceProviderPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ResponsibleTransportServiceProviderParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ResponsibleTransportServiceProviderParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ResponsibleTransportServiceProviderParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ResponsibleTransportServiceProviderParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfScheduledServiceFrequencyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ScheduledServiceFrequency>,
}

impl AsMut<TransportationServiceArrayOfScheduledServiceFrequencyComponent> for TransportationServiceArrayOfScheduledServiceFrequencyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfScheduledServiceFrequencyComponent> for TransportationServiceArrayOfScheduledServiceFrequencyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfScheduledServiceFrequencyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfScheduledServiceFrequencyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ScheduledServiceFrequency) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ScheduledServiceFrequency) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ScheduledServiceFrequency> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ScheduledServiceFrequency> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfSequenceNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric>,
}

impl AsMut<TransportationServiceArrayOfSequenceNumericComponent> for TransportationServiceArrayOfSequenceNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfSequenceNumericComponent> for TransportationServiceArrayOfSequenceNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfSequenceNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfSequenceNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfSequenceNumericComponent {
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
pub struct TransportationServiceArrayOfShipmentStageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ShipmentStage>,
}

impl AsMut<TransportationServiceArrayOfShipmentStageComponent> for TransportationServiceArrayOfShipmentStageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfShipmentStageComponent> for TransportationServiceArrayOfShipmentStageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfShipmentStageComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfShipmentStageComponent {
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
pub struct TransportationServiceArrayOfSupportedCommodityClassificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SupportedCommodityClassification>,
}

impl AsMut<TransportationServiceArrayOfSupportedCommodityClassificationComponent> for TransportationServiceArrayOfSupportedCommodityClassificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfSupportedCommodityClassificationComponent> for TransportationServiceArrayOfSupportedCommodityClassificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfSupportedCommodityClassificationComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfSupportedCommodityClassificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SupportedCommodityClassification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SupportedCommodityClassification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SupportedCommodityClassification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SupportedCommodityClassification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfSupportedTransportEquipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SupportedTransportEquipment>,
}

impl AsMut<TransportationServiceArrayOfSupportedTransportEquipmentComponent> for TransportationServiceArrayOfSupportedTransportEquipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfSupportedTransportEquipmentComponent> for TransportationServiceArrayOfSupportedTransportEquipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfSupportedTransportEquipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfSupportedTransportEquipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SupportedTransportEquipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SupportedTransportEquipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SupportedTransportEquipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SupportedTransportEquipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfTariffClassCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TariffClassCode>,
}

impl AsMut<TransportationServiceArrayOfTariffClassCodeComponent> for TransportationServiceArrayOfTariffClassCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfTariffClassCodeComponent> for TransportationServiceArrayOfTariffClassCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfTariffClassCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfTariffClassCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfTariffClassCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TariffClassCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TariffClassCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TariffClassCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TariffClassCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfTotalCapacityDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TotalCapacityDimension>,
}

impl AsMut<TransportationServiceArrayOfTotalCapacityDimensionComponent> for TransportationServiceArrayOfTotalCapacityDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfTotalCapacityDimensionComponent> for TransportationServiceArrayOfTotalCapacityDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfTotalCapacityDimensionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfTotalCapacityDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfTotalCapacityDimensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TotalCapacityDimension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TotalCapacityDimension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TotalCapacityDimension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TotalCapacityDimension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfTransportEquipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportEquipment>,
}

impl AsMut<TransportationServiceArrayOfTransportEquipmentComponent> for TransportationServiceArrayOfTransportEquipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfTransportEquipmentComponent> for TransportationServiceArrayOfTransportEquipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfTransportEquipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfTransportEquipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportEquipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportEquipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportEquipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportEquipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportEvent>,
}

impl AsMut<TransportationServiceArrayOfTransportEventComponent> for TransportationServiceArrayOfTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfTransportEventComponent> for TransportationServiceArrayOfTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfTransportEventComponent {
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
pub struct TransportationServiceArrayOfTransportServiceCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportServiceCode>,
}

impl AsMut<TransportationServiceArrayOfTransportServiceCodeComponent> for TransportationServiceArrayOfTransportServiceCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfTransportServiceCodeComponent> for TransportationServiceArrayOfTransportServiceCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfTransportServiceCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfTransportServiceCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfTransportServiceCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportServiceCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportServiceCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportServiceCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportServiceCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfTransportationServiceDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportationServiceDescription>,
}

impl AsMut<TransportationServiceArrayOfTransportationServiceDescriptionComponent> for TransportationServiceArrayOfTransportationServiceDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfTransportationServiceDescriptionComponent> for TransportationServiceArrayOfTransportationServiceDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfTransportationServiceDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfTransportationServiceDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportationServiceDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportationServiceDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportationServiceDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportationServiceDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfTransportationServiceDetailsURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportationServiceDetailsURI>,
}

impl AsMut<TransportationServiceArrayOfTransportationServiceDetailsURIComponent> for TransportationServiceArrayOfTransportationServiceDetailsURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfTransportationServiceDetailsURIComponent> for TransportationServiceArrayOfTransportationServiceDetailsURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfTransportationServiceDetailsURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfTransportationServiceDetailsURIComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfTransportationServiceDetailsURIComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportationServiceDetailsURI) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportationServiceDetailsURI) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportationServiceDetailsURI> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportationServiceDetailsURI> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfUnsupportedCommodityClassificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::UnsupportedCommodityClassification>,
}

impl AsMut<TransportationServiceArrayOfUnsupportedCommodityClassificationComponent> for TransportationServiceArrayOfUnsupportedCommodityClassificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfUnsupportedCommodityClassificationComponent> for TransportationServiceArrayOfUnsupportedCommodityClassificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfUnsupportedCommodityClassificationComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfUnsupportedCommodityClassificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::UnsupportedCommodityClassification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::UnsupportedCommodityClassification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::UnsupportedCommodityClassification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::UnsupportedCommodityClassification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportationServiceArrayOfUnsupportedTransportEquipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::UnsupportedTransportEquipment>,
}

impl AsMut<TransportationServiceArrayOfUnsupportedTransportEquipmentComponent> for TransportationServiceArrayOfUnsupportedTransportEquipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportationServiceArrayOfUnsupportedTransportEquipmentComponent> for TransportationServiceArrayOfUnsupportedTransportEquipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportationServiceArrayOfUnsupportedTransportEquipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportationServiceArrayOfUnsupportedTransportEquipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::UnsupportedTransportEquipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::UnsupportedTransportEquipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::UnsupportedTransportEquipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::UnsupportedTransportEquipment> {
        self.items.iter()
    }
}


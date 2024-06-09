use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransportMeans {
    #[serde(rename = "AirTransport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_transport: Option<TransportMeansArrayOfAirTransportComponent>,
    #[serde(rename = "DirectionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_code: Option<TransportMeansArrayOfDirectionCodeComponent>,
    #[serde(rename = "JourneyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_id: Option<TransportMeansArrayOfJourneyIDComponent>,
    #[serde(rename = "MaritimeTransport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maritime_transport: Option<TransportMeansArrayOfMaritimeTransportComponent>,
    #[serde(rename = "MeasurementDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_dimension: Option<TransportMeansArrayOfMeasurementDimensionComponent>,
    #[serde(rename = "OwnerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_party: Option<TransportMeansArrayOfOwnerPartyComponent>,
    #[serde(rename = "RailTransport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rail_transport: Option<TransportMeansArrayOfRailTransportComponent>,
    #[serde(rename = "RegistrationNationality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_nationality: Option<TransportMeansArrayOfRegistrationNationalityComponent>,
    #[serde(rename = "RegistrationNationalityID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_nationality_id: Option<TransportMeansArrayOfRegistrationNationalityIDComponent>,
    #[serde(rename = "RoadTransport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub road_transport: Option<TransportMeansArrayOfRoadTransportComponent>,
    #[serde(rename = "Stowage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stowage: Option<TransportMeansArrayOfStowageComponent>,
    #[serde(rename = "TradeServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_service_code: Option<TransportMeansArrayOfTradeServiceCodeComponent>,
    #[serde(rename = "TransportMeansTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_means_type_code: Option<TransportMeansArrayOfTransportMeansTypeCodeComponent>,
}

impl AsMut<TransportMeans> for TransportMeans {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeans> for TransportMeans {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.owner_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.owner_party", e));
            }
        }
        if let Some(v) = &self.rail_transport {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.rail_transport", e));
            }
        }
        if let Some(v) = &self.measurement_dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.measurement_dimension", e));
            }
        }
        if let Some(v) = &self.air_transport {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.air_transport", e));
            }
        }
        if let Some(v) = &self.registration_nationality {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.registration_nationality", e));
            }
        }
        if let Some(v) = &self.road_transport {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.road_transport", e));
            }
        }
        if let Some(v) = &self.direction_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.direction_code", e));
            }
        }
        if let Some(v) = &self.maritime_transport {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.maritime_transport", e));
            }
        }
        if let Some(v) = &self.stowage {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.stowage", e));
            }
        }
        if let Some(v) = &self.trade_service_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.trade_service_code", e));
            }
        }
        if let Some(v) = &self.transport_means_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.transport_means_type_code", e));
            }
        }
        if let Some(v) = &self.journey_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.journey_id", e));
            }
        }
        if let Some(v) = &self.registration_nationality_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportMeans.registration_nationality_id", e));
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

impl TransportMeans {
    pub fn title() -> &'static str {
        "Transport Means. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a particular vehicle or vessel used for the conveyance of goods or persons."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            trade_service_code: None,
            measurement_dimension: None,
            owner_party: None,
            registration_nationality: None,
            air_transport: None,
            rail_transport: None,
            stowage: None,
            maritime_transport: None,
            transport_means_type_code: None,
            journey_id: None,
            direction_code: None,
            registration_nationality_id: None,
            road_transport: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportMeansArrayOfAirTransportComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AirTransport>,
}

impl AsMut<TransportMeansArrayOfAirTransportComponent> for TransportMeansArrayOfAirTransportComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfAirTransportComponent> for TransportMeansArrayOfAirTransportComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfAirTransportComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfAirTransportComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfAirTransportComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AirTransport) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AirTransport) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AirTransport> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AirTransport> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportMeansArrayOfDirectionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DirectionCode>,
}

impl AsMut<TransportMeansArrayOfDirectionCodeComponent> for TransportMeansArrayOfDirectionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfDirectionCodeComponent> for TransportMeansArrayOfDirectionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfDirectionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfDirectionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfDirectionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DirectionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DirectionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DirectionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DirectionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportMeansArrayOfJourneyIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::JourneyID>,
}

impl AsMut<TransportMeansArrayOfJourneyIDComponent> for TransportMeansArrayOfJourneyIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfJourneyIDComponent> for TransportMeansArrayOfJourneyIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfJourneyIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfJourneyIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfJourneyIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::JourneyID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::JourneyID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::JourneyID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::JourneyID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportMeansArrayOfMaritimeTransportComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MaritimeTransport>,
}

impl AsMut<TransportMeansArrayOfMaritimeTransportComponent> for TransportMeansArrayOfMaritimeTransportComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfMaritimeTransportComponent> for TransportMeansArrayOfMaritimeTransportComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfMaritimeTransportComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfMaritimeTransportComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfMaritimeTransportComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MaritimeTransport) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MaritimeTransport) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MaritimeTransport> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MaritimeTransport> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportMeansArrayOfMeasurementDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension>,
}

impl AsMut<TransportMeansArrayOfMeasurementDimensionComponent> for TransportMeansArrayOfMeasurementDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfMeasurementDimensionComponent> for TransportMeansArrayOfMeasurementDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfMeasurementDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfMeasurementDimensionComponent {
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
pub struct TransportMeansArrayOfOwnerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OwnerParty>,
}

impl AsMut<TransportMeansArrayOfOwnerPartyComponent> for TransportMeansArrayOfOwnerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfOwnerPartyComponent> for TransportMeansArrayOfOwnerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfOwnerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfOwnerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfOwnerPartyComponent {
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
pub struct TransportMeansArrayOfRailTransportComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RailTransport>,
}

impl AsMut<TransportMeansArrayOfRailTransportComponent> for TransportMeansArrayOfRailTransportComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfRailTransportComponent> for TransportMeansArrayOfRailTransportComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfRailTransportComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfRailTransportComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfRailTransportComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RailTransport) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RailTransport) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RailTransport> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RailTransport> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportMeansArrayOfRegistrationNationalityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RegistrationNationality>,
}

impl AsMut<TransportMeansArrayOfRegistrationNationalityComponent> for TransportMeansArrayOfRegistrationNationalityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfRegistrationNationalityComponent> for TransportMeansArrayOfRegistrationNationalityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfRegistrationNationalityComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfRegistrationNationalityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RegistrationNationality) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RegistrationNationality) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RegistrationNationality> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RegistrationNationality> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportMeansArrayOfRegistrationNationalityIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RegistrationNationalityID>,
}

impl AsMut<TransportMeansArrayOfRegistrationNationalityIDComponent> for TransportMeansArrayOfRegistrationNationalityIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfRegistrationNationalityIDComponent> for TransportMeansArrayOfRegistrationNationalityIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfRegistrationNationalityIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfRegistrationNationalityIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfRegistrationNationalityIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RegistrationNationalityID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RegistrationNationalityID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RegistrationNationalityID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RegistrationNationalityID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportMeansArrayOfRoadTransportComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RoadTransport>,
}

impl AsMut<TransportMeansArrayOfRoadTransportComponent> for TransportMeansArrayOfRoadTransportComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfRoadTransportComponent> for TransportMeansArrayOfRoadTransportComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfRoadTransportComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfRoadTransportComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfRoadTransportComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RoadTransport) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RoadTransport) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RoadTransport> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RoadTransport> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportMeansArrayOfStowageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Stowage>,
}

impl AsMut<TransportMeansArrayOfStowageComponent> for TransportMeansArrayOfStowageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfStowageComponent> for TransportMeansArrayOfStowageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfStowageComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfStowageComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfStowageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Stowage) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Stowage) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Stowage> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Stowage> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportMeansArrayOfTradeServiceCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TradeServiceCode>,
}

impl AsMut<TransportMeansArrayOfTradeServiceCodeComponent> for TransportMeansArrayOfTradeServiceCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfTradeServiceCodeComponent> for TransportMeansArrayOfTradeServiceCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfTradeServiceCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfTradeServiceCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfTradeServiceCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TradeServiceCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TradeServiceCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TradeServiceCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TradeServiceCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportMeansArrayOfTransportMeansTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportMeansTypeCode>,
}

impl AsMut<TransportMeansArrayOfTransportMeansTypeCodeComponent> for TransportMeansArrayOfTransportMeansTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportMeansArrayOfTransportMeansTypeCodeComponent> for TransportMeansArrayOfTransportMeansTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfTransportMeansTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportMeansArrayOfTransportMeansTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl TransportMeansArrayOfTransportMeansTypeCodeComponent {
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


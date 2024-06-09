use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AirTransport {
    #[serde(rename = "AircraftID")]
    pub aircraft_id: AirTransportArrayOfAircraftIDComponent,
}

impl AsMut<AirTransport> for AirTransport {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AirTransport> for AirTransport {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.aircraft_id.validate() {
            return Err(UblError::component("AirTransport.aircraft_id", e));
        }

        Ok(self)
    }

    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl AirTransport {
    pub fn title() -> &'static str {
        "Air Transport. Details"
    }
    pub fn description() -> &'static str {
        "A class to identify a specific aircraft used for transportation."
    }
    pub fn new(aircraft_id: AirTransportArrayOfAircraftIDComponent) -> Component<Self> {
        Component(Self {
            aircraft_id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AirTransportArrayOfAircraftIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AircraftID>,
}

impl AsMut<AirTransportArrayOfAircraftIDComponent> for AirTransportArrayOfAircraftIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AirTransportArrayOfAircraftIDComponent> for AirTransportArrayOfAircraftIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AirTransportArrayOfAircraftIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AirTransportArrayOfAircraftIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl AirTransportArrayOfAircraftIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AircraftID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AircraftID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AircraftID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AircraftID> {
        self.items.iter()
    }
}


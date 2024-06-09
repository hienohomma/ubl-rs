use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoadTransport {
    #[serde(rename = "LicensePlateID")]
    pub license_plate_id: RoadTransportArrayOfLicensePlateIDComponent,
}

impl AsMut<RoadTransport> for RoadTransport {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RoadTransport> for RoadTransport {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.license_plate_id.validate() {
            return Err(UblError::component("RoadTransport.license_plate_id", e));
        }

        Ok(self)
    }

    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl RoadTransport {
    pub fn title() -> &'static str {
        "Road Transport. Details"
    }
    pub fn description() -> &'static str {
        "A class for identifying a vehicle used for road transport."
    }
    pub fn new(license_plate_id: RoadTransportArrayOfLicensePlateIDComponent) -> Component<Self> {
        Component(Self {
            license_plate_id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RoadTransportArrayOfLicensePlateIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LicensePlateID>,
}

impl AsMut<RoadTransportArrayOfLicensePlateIDComponent> for RoadTransportArrayOfLicensePlateIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RoadTransportArrayOfLicensePlateIDComponent> for RoadTransportArrayOfLicensePlateIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RoadTransportArrayOfLicensePlateIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RoadTransportArrayOfLicensePlateIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl RoadTransportArrayOfLicensePlateIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LicensePlateID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LicensePlateID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LicensePlateID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LicensePlateID> {
        self.items.iter()
    }
}


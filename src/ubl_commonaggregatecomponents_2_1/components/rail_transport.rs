use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RailTransport {
    #[serde(rename = "RailCarID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rail_car_id: Option<RailTransportArrayOfRailCarIDComponent>,
    #[serde(rename = "TrainID")]
    pub train_id: RailTransportArrayOfTrainIDComponent,
}

impl AsMut<RailTransport> for RailTransport {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RailTransport> for RailTransport {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.train_id.validate() {
            return Err(UblError::component("RailTransport.train_id", e));
        }
        if let Some(v) = &self.rail_car_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RailTransport.rail_car_id", e));
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

impl RailTransport {
    pub fn title() -> &'static str {
        "Rail Transport. Details"
    }
    pub fn description() -> &'static str {
        "A class defining details about a train wagon used as a means of transport."
    }
    pub fn new(train_id: RailTransportArrayOfTrainIDComponent) -> Component<Self> {
        Component(Self {
            rail_car_id: None,
            train_id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RailTransportArrayOfRailCarIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RailCarID>,
}

impl AsMut<RailTransportArrayOfRailCarIDComponent> for RailTransportArrayOfRailCarIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RailTransportArrayOfRailCarIDComponent> for RailTransportArrayOfRailCarIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RailTransportArrayOfRailCarIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RailTransportArrayOfRailCarIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl RailTransportArrayOfRailCarIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RailCarID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RailCarID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RailCarID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RailCarID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RailTransportArrayOfTrainIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TrainID>,
}

impl AsMut<RailTransportArrayOfTrainIDComponent> for RailTransportArrayOfTrainIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RailTransportArrayOfTrainIDComponent> for RailTransportArrayOfTrainIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RailTransportArrayOfTrainIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RailTransportArrayOfTrainIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl RailTransportArrayOfTrainIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TrainID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TrainID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TrainID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TrainID> {
        self.items.iter()
    }
}


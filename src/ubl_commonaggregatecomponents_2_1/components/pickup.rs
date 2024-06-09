use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pickup {
    #[serde(rename = "ActualPickupDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_pickup_date: Option<PickupArrayOfActualPickupDateComponent>,
    #[serde(rename = "ActualPickupTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_pickup_time: Option<PickupArrayOfActualPickupTimeComponent>,
    #[serde(rename = "EarliestPickupDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_pickup_date: Option<PickupArrayOfEarliestPickupDateComponent>,
    #[serde(rename = "EarliestPickupTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_pickup_time: Option<PickupArrayOfEarliestPickupTimeComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PickupArrayOfIDComponent>,
    #[serde(rename = "LatestPickupDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_pickup_date: Option<PickupArrayOfLatestPickupDateComponent>,
    #[serde(rename = "LatestPickupTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_pickup_time: Option<PickupArrayOfLatestPickupTimeComponent>,
    #[serde(rename = "PickupLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_location: Option<PickupArrayOfPickupLocationComponent>,
    #[serde(rename = "PickupParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_party: Option<PickupArrayOfPickupPartyComponent>,
}

impl AsMut<Pickup> for Pickup {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Pickup> for Pickup {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.actual_pickup_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Pickup.actual_pickup_time", e));
            }
        }
        if let Some(v) = &self.latest_pickup_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Pickup.latest_pickup_date", e));
            }
        }
        if let Some(v) = &self.earliest_pickup_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Pickup.earliest_pickup_time", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Pickup.id", e));
            }
        }
        if let Some(v) = &self.earliest_pickup_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Pickup.earliest_pickup_date", e));
            }
        }
        if let Some(v) = &self.pickup_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Pickup.pickup_party", e));
            }
        }
        if let Some(v) = &self.latest_pickup_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Pickup.latest_pickup_time", e));
            }
        }
        if let Some(v) = &self.actual_pickup_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Pickup.actual_pickup_date", e));
            }
        }
        if let Some(v) = &self.pickup_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Pickup.pickup_location", e));
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

impl Pickup {
    pub fn title() -> &'static str {
        "Pickup. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a pickup for delivery."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            earliest_pickup_date: None,
            latest_pickup_date: None,
            actual_pickup_date: None,
            actual_pickup_time: None,
            earliest_pickup_time: None,
            latest_pickup_time: None,
            id: None,
            pickup_location: None,
            pickup_party: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PickupArrayOfActualPickupDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ActualPickupDate>,
}

impl AsMut<PickupArrayOfActualPickupDateComponent> for PickupArrayOfActualPickupDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PickupArrayOfActualPickupDateComponent> for PickupArrayOfActualPickupDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PickupArrayOfActualPickupDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PickupArrayOfActualPickupDateComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl PickupArrayOfActualPickupDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ActualPickupDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ActualPickupDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ActualPickupDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ActualPickupDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PickupArrayOfActualPickupTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ActualPickupTime>,
}

impl AsMut<PickupArrayOfActualPickupTimeComponent> for PickupArrayOfActualPickupTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PickupArrayOfActualPickupTimeComponent> for PickupArrayOfActualPickupTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PickupArrayOfActualPickupTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PickupArrayOfActualPickupTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl PickupArrayOfActualPickupTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ActualPickupTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ActualPickupTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ActualPickupTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ActualPickupTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PickupArrayOfEarliestPickupDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EarliestPickupDate>,
}

impl AsMut<PickupArrayOfEarliestPickupDateComponent> for PickupArrayOfEarliestPickupDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PickupArrayOfEarliestPickupDateComponent> for PickupArrayOfEarliestPickupDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PickupArrayOfEarliestPickupDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PickupArrayOfEarliestPickupDateComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl PickupArrayOfEarliestPickupDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EarliestPickupDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EarliestPickupDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EarliestPickupDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EarliestPickupDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PickupArrayOfEarliestPickupTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EarliestPickupTime>,
}

impl AsMut<PickupArrayOfEarliestPickupTimeComponent> for PickupArrayOfEarliestPickupTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PickupArrayOfEarliestPickupTimeComponent> for PickupArrayOfEarliestPickupTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PickupArrayOfEarliestPickupTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PickupArrayOfEarliestPickupTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl PickupArrayOfEarliestPickupTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EarliestPickupTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EarliestPickupTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EarliestPickupTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EarliestPickupTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PickupArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<PickupArrayOfIDComponent> for PickupArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PickupArrayOfIDComponent> for PickupArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PickupArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PickupArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl PickupArrayOfIDComponent {
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
pub struct PickupArrayOfLatestPickupDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatestPickupDate>,
}

impl AsMut<PickupArrayOfLatestPickupDateComponent> for PickupArrayOfLatestPickupDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PickupArrayOfLatestPickupDateComponent> for PickupArrayOfLatestPickupDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PickupArrayOfLatestPickupDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PickupArrayOfLatestPickupDateComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl PickupArrayOfLatestPickupDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatestPickupDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatestPickupDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatestPickupDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatestPickupDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PickupArrayOfLatestPickupTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatestPickupTime>,
}

impl AsMut<PickupArrayOfLatestPickupTimeComponent> for PickupArrayOfLatestPickupTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PickupArrayOfLatestPickupTimeComponent> for PickupArrayOfLatestPickupTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PickupArrayOfLatestPickupTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PickupArrayOfLatestPickupTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl PickupArrayOfLatestPickupTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatestPickupTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatestPickupTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatestPickupTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatestPickupTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PickupArrayOfPickupLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PickupLocation>,
}

impl AsMut<PickupArrayOfPickupLocationComponent> for PickupArrayOfPickupLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PickupArrayOfPickupLocationComponent> for PickupArrayOfPickupLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PickupArrayOfPickupLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PickupArrayOfPickupLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl PickupArrayOfPickupLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PickupLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PickupLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PickupLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PickupLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PickupArrayOfPickupPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PickupParty>,
}

impl AsMut<PickupArrayOfPickupPartyComponent> for PickupArrayOfPickupPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PickupArrayOfPickupPartyComponent> for PickupArrayOfPickupPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PickupArrayOfPickupPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PickupArrayOfPickupPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl PickupArrayOfPickupPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PickupParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PickupParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PickupParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PickupParty> {
        self.items.iter()
    }
}


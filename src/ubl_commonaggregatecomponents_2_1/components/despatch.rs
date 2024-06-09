use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Despatch {
    #[serde(rename = "ActualDespatchDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_despatch_date: Option<DespatchArrayOfActualDespatchDateComponent>,
    #[serde(rename = "ActualDespatchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_despatch_time: Option<DespatchArrayOfActualDespatchTimeComponent>,
    #[serde(rename = "CarrierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_party: Option<DespatchArrayOfCarrierPartyComponent>,
    #[serde(rename = "Contact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<DespatchArrayOfContactComponent>,
    #[serde(rename = "DespatchAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_address: Option<DespatchArrayOfDespatchAddressComponent>,
    #[serde(rename = "DespatchLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_location: Option<DespatchArrayOfDespatchLocationComponent>,
    #[serde(rename = "DespatchParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_party: Option<DespatchArrayOfDespatchPartyComponent>,
    #[serde(rename = "EstimatedDespatchDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_despatch_date: Option<DespatchArrayOfEstimatedDespatchDateComponent>,
    #[serde(rename = "EstimatedDespatchPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_despatch_period: Option<DespatchArrayOfEstimatedDespatchPeriodComponent>,
    #[serde(rename = "EstimatedDespatchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_despatch_time: Option<DespatchArrayOfEstimatedDespatchTimeComponent>,
    #[serde(rename = "GuaranteedDespatchDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guaranteed_despatch_date: Option<DespatchArrayOfGuaranteedDespatchDateComponent>,
    #[serde(rename = "GuaranteedDespatchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guaranteed_despatch_time: Option<DespatchArrayOfGuaranteedDespatchTimeComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<DespatchArrayOfIDComponent>,
    #[serde(rename = "Instructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<DespatchArrayOfInstructionsComponent>,
    #[serde(rename = "NotifyParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_party: Option<DespatchArrayOfNotifyPartyComponent>,
    #[serde(rename = "ReleaseID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_id: Option<DespatchArrayOfReleaseIDComponent>,
    #[serde(rename = "RequestedDespatchDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_despatch_date: Option<DespatchArrayOfRequestedDespatchDateComponent>,
    #[serde(rename = "RequestedDespatchPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_despatch_period: Option<DespatchArrayOfRequestedDespatchPeriodComponent>,
    #[serde(rename = "RequestedDespatchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_despatch_time: Option<DespatchArrayOfRequestedDespatchTimeComponent>,
}

impl AsMut<Despatch> for Despatch {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Despatch> for Despatch {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.guaranteed_despatch_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.guaranteed_despatch_time", e));
            }
        }
        if let Some(v) = &self.despatch_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.despatch_location", e));
            }
        }
        if let Some(v) = &self.requested_despatch_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.requested_despatch_date", e));
            }
        }
        if let Some(v) = &self.requested_despatch_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.requested_despatch_period", e));
            }
        }
        if let Some(v) = &self.actual_despatch_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.actual_despatch_time", e));
            }
        }
        if let Some(v) = &self.instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.instructions", e));
            }
        }
        if let Some(v) = &self.guaranteed_despatch_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.guaranteed_despatch_date", e));
            }
        }
        if let Some(v) = &self.carrier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.carrier_party", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.id", e));
            }
        }
        if let Some(v) = &self.despatch_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.despatch_address", e));
            }
        }
        if let Some(v) = &self.actual_despatch_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.actual_despatch_date", e));
            }
        }
        if let Some(v) = &self.contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.contact", e));
            }
        }
        if let Some(v) = &self.estimated_despatch_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.estimated_despatch_period", e));
            }
        }
        if let Some(v) = &self.notify_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.notify_party", e));
            }
        }
        if let Some(v) = &self.release_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.release_id", e));
            }
        }
        if let Some(v) = &self.estimated_despatch_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.estimated_despatch_date", e));
            }
        }
        if let Some(v) = &self.despatch_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.despatch_party", e));
            }
        }
        if let Some(v) = &self.requested_despatch_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.requested_despatch_time", e));
            }
        }
        if let Some(v) = &self.estimated_despatch_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Despatch.estimated_despatch_time", e));
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

impl Despatch {
    pub fn title() -> &'static str {
        "Despatch. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the despatching of goods (their pickup for delivery)."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            requested_despatch_period: None,
            despatch_party: None,
            guaranteed_despatch_date: None,
            requested_despatch_date: None,
            estimated_despatch_date: None,
            notify_party: None,
            carrier_party: None,
            instructions: None,
            despatch_location: None,
            estimated_despatch_time: None,
            guaranteed_despatch_time: None,
            release_id: None,
            contact: None,
            despatch_address: None,
            actual_despatch_time: None,
            id: None,
            requested_despatch_time: None,
            estimated_despatch_period: None,
            actual_despatch_date: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfActualDespatchDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ActualDespatchDate>,
}

impl AsMut<DespatchArrayOfActualDespatchDateComponent> for DespatchArrayOfActualDespatchDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfActualDespatchDateComponent> for DespatchArrayOfActualDespatchDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfActualDespatchDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfActualDespatchDateComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfActualDespatchDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ActualDespatchDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ActualDespatchDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ActualDespatchDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ActualDespatchDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfActualDespatchTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ActualDespatchTime>,
}

impl AsMut<DespatchArrayOfActualDespatchTimeComponent> for DespatchArrayOfActualDespatchTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfActualDespatchTimeComponent> for DespatchArrayOfActualDespatchTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfActualDespatchTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfActualDespatchTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfActualDespatchTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ActualDespatchTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ActualDespatchTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ActualDespatchTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ActualDespatchTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfCarrierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CarrierParty>,
}

impl AsMut<DespatchArrayOfCarrierPartyComponent> for DespatchArrayOfCarrierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfCarrierPartyComponent> for DespatchArrayOfCarrierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfCarrierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfCarrierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfCarrierPartyComponent {
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
pub struct DespatchArrayOfContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Contact>,
}

impl AsMut<DespatchArrayOfContactComponent> for DespatchArrayOfContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfContactComponent> for DespatchArrayOfContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfContactComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfContactComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Contact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Contact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Contact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Contact> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfDespatchAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DespatchAddress>,
}

impl AsMut<DespatchArrayOfDespatchAddressComponent> for DespatchArrayOfDespatchAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfDespatchAddressComponent> for DespatchArrayOfDespatchAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfDespatchAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfDespatchAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfDespatchAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DespatchAddress) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DespatchAddress) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DespatchAddress> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DespatchAddress> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfDespatchLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DespatchLocation>,
}

impl AsMut<DespatchArrayOfDespatchLocationComponent> for DespatchArrayOfDespatchLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfDespatchLocationComponent> for DespatchArrayOfDespatchLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfDespatchLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfDespatchLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfDespatchLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DespatchLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DespatchLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DespatchLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DespatchLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfDespatchPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DespatchParty>,
}

impl AsMut<DespatchArrayOfDespatchPartyComponent> for DespatchArrayOfDespatchPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfDespatchPartyComponent> for DespatchArrayOfDespatchPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfDespatchPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfDespatchPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfDespatchPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DespatchParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DespatchParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DespatchParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DespatchParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfEstimatedDespatchDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EstimatedDespatchDate>,
}

impl AsMut<DespatchArrayOfEstimatedDespatchDateComponent> for DespatchArrayOfEstimatedDespatchDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfEstimatedDespatchDateComponent> for DespatchArrayOfEstimatedDespatchDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfEstimatedDespatchDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfEstimatedDespatchDateComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfEstimatedDespatchDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EstimatedDespatchDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EstimatedDespatchDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EstimatedDespatchDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EstimatedDespatchDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfEstimatedDespatchPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EstimatedDespatchPeriod>,
}

impl AsMut<DespatchArrayOfEstimatedDespatchPeriodComponent> for DespatchArrayOfEstimatedDespatchPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfEstimatedDespatchPeriodComponent> for DespatchArrayOfEstimatedDespatchPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfEstimatedDespatchPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfEstimatedDespatchPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfEstimatedDespatchPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EstimatedDespatchPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EstimatedDespatchPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EstimatedDespatchPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EstimatedDespatchPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfEstimatedDespatchTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EstimatedDespatchTime>,
}

impl AsMut<DespatchArrayOfEstimatedDespatchTimeComponent> for DespatchArrayOfEstimatedDespatchTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfEstimatedDespatchTimeComponent> for DespatchArrayOfEstimatedDespatchTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfEstimatedDespatchTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfEstimatedDespatchTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfEstimatedDespatchTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EstimatedDespatchTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EstimatedDespatchTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EstimatedDespatchTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EstimatedDespatchTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfGuaranteedDespatchDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GuaranteedDespatchDate>,
}

impl AsMut<DespatchArrayOfGuaranteedDespatchDateComponent> for DespatchArrayOfGuaranteedDespatchDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfGuaranteedDespatchDateComponent> for DespatchArrayOfGuaranteedDespatchDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfGuaranteedDespatchDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfGuaranteedDespatchDateComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfGuaranteedDespatchDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GuaranteedDespatchDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GuaranteedDespatchDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GuaranteedDespatchDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GuaranteedDespatchDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfGuaranteedDespatchTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GuaranteedDespatchTime>,
}

impl AsMut<DespatchArrayOfGuaranteedDespatchTimeComponent> for DespatchArrayOfGuaranteedDespatchTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfGuaranteedDespatchTimeComponent> for DespatchArrayOfGuaranteedDespatchTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfGuaranteedDespatchTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfGuaranteedDespatchTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfGuaranteedDespatchTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GuaranteedDespatchTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GuaranteedDespatchTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GuaranteedDespatchTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GuaranteedDespatchTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<DespatchArrayOfIDComponent> for DespatchArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfIDComponent> for DespatchArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfIDComponent {
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
pub struct DespatchArrayOfInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Instructions>,
}

impl AsMut<DespatchArrayOfInstructionsComponent> for DespatchArrayOfInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfInstructionsComponent> for DespatchArrayOfInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfInstructionsComponent {
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
pub struct DespatchArrayOfNotifyPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::NotifyParty>,
}

impl AsMut<DespatchArrayOfNotifyPartyComponent> for DespatchArrayOfNotifyPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfNotifyPartyComponent> for DespatchArrayOfNotifyPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfNotifyPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfNotifyPartyComponent {
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
pub struct DespatchArrayOfReleaseIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReleaseID>,
}

impl AsMut<DespatchArrayOfReleaseIDComponent> for DespatchArrayOfReleaseIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfReleaseIDComponent> for DespatchArrayOfReleaseIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfReleaseIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfReleaseIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfReleaseIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReleaseID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReleaseID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReleaseID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReleaseID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfRequestedDespatchDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RequestedDespatchDate>,
}

impl AsMut<DespatchArrayOfRequestedDespatchDateComponent> for DespatchArrayOfRequestedDespatchDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfRequestedDespatchDateComponent> for DespatchArrayOfRequestedDespatchDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfRequestedDespatchDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfRequestedDespatchDateComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfRequestedDespatchDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RequestedDespatchDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RequestedDespatchDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RequestedDespatchDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RequestedDespatchDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfRequestedDespatchPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequestedDespatchPeriod>,
}

impl AsMut<DespatchArrayOfRequestedDespatchPeriodComponent> for DespatchArrayOfRequestedDespatchPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfRequestedDespatchPeriodComponent> for DespatchArrayOfRequestedDespatchPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfRequestedDespatchPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfRequestedDespatchPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfRequestedDespatchPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequestedDespatchPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequestedDespatchPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequestedDespatchPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequestedDespatchPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchArrayOfRequestedDespatchTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RequestedDespatchTime>,
}

impl AsMut<DespatchArrayOfRequestedDespatchTimeComponent> for DespatchArrayOfRequestedDespatchTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchArrayOfRequestedDespatchTimeComponent> for DespatchArrayOfRequestedDespatchTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchArrayOfRequestedDespatchTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchArrayOfRequestedDespatchTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DespatchArrayOfRequestedDespatchTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RequestedDespatchTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RequestedDespatchTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RequestedDespatchTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RequestedDespatchTime> {
        self.items.iter()
    }
}


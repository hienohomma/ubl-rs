use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Delivery {
    #[serde(rename = "ActualDeliveryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_delivery_date: Option<DeliveryArrayOfActualDeliveryDateComponent>,
    #[serde(rename = "ActualDeliveryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_delivery_time: Option<DeliveryArrayOfActualDeliveryTimeComponent>,
    #[serde(rename = "AlternativeDeliveryLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_delivery_location: Option<DeliveryArrayOfAlternativeDeliveryLocationComponent>,
    #[serde(rename = "CarrierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_party: Option<DeliveryArrayOfCarrierPartyComponent>,
    #[serde(rename = "DeliveryAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_address: Option<DeliveryArrayOfDeliveryAddressComponent>,
    #[serde(rename = "DeliveryLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_location: Option<DeliveryArrayOfDeliveryLocationComponent>,
    #[serde(rename = "DeliveryParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_party: Option<DeliveryArrayOfDeliveryPartyComponent>,
    #[serde(rename = "DeliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<DeliveryArrayOfDeliveryTermsComponent>,
    #[serde(rename = "Despatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch: Option<DeliveryArrayOfDespatchComponent>,
    #[serde(rename = "EstimatedDeliveryPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_period: Option<DeliveryArrayOfEstimatedDeliveryPeriodComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<DeliveryArrayOfIDComponent>,
    #[serde(rename = "LatestDeliveryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_date: Option<DeliveryArrayOfLatestDeliveryDateComponent>,
    #[serde(rename = "LatestDeliveryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_time: Option<DeliveryArrayOfLatestDeliveryTimeComponent>,
    #[serde(rename = "MaximumDeliveryUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_delivery_unit: Option<DeliveryArrayOfMaximumDeliveryUnitComponent>,
    #[serde(rename = "MaximumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_quantity: Option<DeliveryArrayOfMaximumQuantityComponent>,
    #[serde(rename = "MinimumDeliveryUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_delivery_unit: Option<DeliveryArrayOfMinimumDeliveryUnitComponent>,
    #[serde(rename = "MinimumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_quantity: Option<DeliveryArrayOfMinimumQuantityComponent>,
    #[serde(rename = "NotifyParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_party: Option<DeliveryArrayOfNotifyPartyComponent>,
    #[serde(rename = "PromisedDeliveryPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promised_delivery_period: Option<DeliveryArrayOfPromisedDeliveryPeriodComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<DeliveryArrayOfQuantityComponent>,
    #[serde(rename = "ReleaseID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_id: Option<DeliveryArrayOfReleaseIDComponent>,
    #[serde(rename = "RequestedDeliveryPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_delivery_period: Option<DeliveryArrayOfRequestedDeliveryPeriodComponent>,
    #[serde(rename = "Shipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment: Option<DeliveryArrayOfShipmentComponent>,
    #[serde(rename = "TrackingID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<DeliveryArrayOfTrackingIDComponent>,
}

impl AsMut<Delivery> for Delivery {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Delivery> for Delivery {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.latest_delivery_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.latest_delivery_date", e));
            }
        }
        if let Some(v) = &self.delivery_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.delivery_address", e));
            }
        }
        if let Some(v) = &self.delivery_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.delivery_location", e));
            }
        }
        if let Some(v) = &self.delivery_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.delivery_terms", e));
            }
        }
        if let Some(v) = &self.minimum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.minimum_quantity", e));
            }
        }
        if let Some(v) = &self.notify_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.notify_party", e));
            }
        }
        if let Some(v) = &self.requested_delivery_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.requested_delivery_period", e));
            }
        }
        if let Some(v) = &self.carrier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.carrier_party", e));
            }
        }
        if let Some(v) = &self.alternative_delivery_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.alternative_delivery_location", e));
            }
        }
        if let Some(v) = &self.maximum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.maximum_quantity", e));
            }
        }
        if let Some(v) = &self.actual_delivery_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.actual_delivery_date", e));
            }
        }
        if let Some(v) = &self.delivery_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.delivery_party", e));
            }
        }
        if let Some(v) = &self.latest_delivery_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.latest_delivery_time", e));
            }
        }
        if let Some(v) = &self.minimum_delivery_unit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.minimum_delivery_unit", e));
            }
        }
        if let Some(v) = &self.promised_delivery_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.promised_delivery_period", e));
            }
        }
        if let Some(v) = &self.maximum_delivery_unit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.maximum_delivery_unit", e));
            }
        }
        if let Some(v) = &self.shipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.shipment", e));
            }
        }
        if let Some(v) = &self.release_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.release_id", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.quantity", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.id", e));
            }
        }
        if let Some(v) = &self.despatch {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.despatch", e));
            }
        }
        if let Some(v) = &self.tracking_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.tracking_id", e));
            }
        }
        if let Some(v) = &self.actual_delivery_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.actual_delivery_time", e));
            }
        }
        if let Some(v) = &self.estimated_delivery_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Delivery.estimated_delivery_period", e));
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

impl Delivery {
    pub fn title() -> &'static str {
        "Delivery. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a delivery."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            maximum_delivery_unit: None,
            carrier_party: None,
            maximum_quantity: None,
            latest_delivery_date: None,
            alternative_delivery_location: None,
            tracking_id: None,
            estimated_delivery_period: None,
            minimum_delivery_unit: None,
            despatch: None,
            actual_delivery_date: None,
            actual_delivery_time: None,
            delivery_party: None,
            notify_party: None,
            quantity: None,
            latest_delivery_time: None,
            delivery_terms: None,
            promised_delivery_period: None,
            requested_delivery_period: None,
            shipment: None,
            delivery_address: None,
            delivery_location: None,
            id: None,
            minimum_quantity: None,
            release_id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfActualDeliveryDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ActualDeliveryDate>,
}

impl AsMut<DeliveryArrayOfActualDeliveryDateComponent> for DeliveryArrayOfActualDeliveryDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfActualDeliveryDateComponent> for DeliveryArrayOfActualDeliveryDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfActualDeliveryDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfActualDeliveryDateComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfActualDeliveryDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ActualDeliveryDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ActualDeliveryDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ActualDeliveryDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ActualDeliveryDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfActualDeliveryTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ActualDeliveryTime>,
}

impl AsMut<DeliveryArrayOfActualDeliveryTimeComponent> for DeliveryArrayOfActualDeliveryTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfActualDeliveryTimeComponent> for DeliveryArrayOfActualDeliveryTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfActualDeliveryTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfActualDeliveryTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfActualDeliveryTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ActualDeliveryTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ActualDeliveryTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ActualDeliveryTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ActualDeliveryTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfAlternativeDeliveryLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AlternativeDeliveryLocation>,
}

impl AsMut<DeliveryArrayOfAlternativeDeliveryLocationComponent> for DeliveryArrayOfAlternativeDeliveryLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfAlternativeDeliveryLocationComponent> for DeliveryArrayOfAlternativeDeliveryLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfAlternativeDeliveryLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfAlternativeDeliveryLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfAlternativeDeliveryLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AlternativeDeliveryLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AlternativeDeliveryLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AlternativeDeliveryLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AlternativeDeliveryLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfCarrierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CarrierParty>,
}

impl AsMut<DeliveryArrayOfCarrierPartyComponent> for DeliveryArrayOfCarrierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfCarrierPartyComponent> for DeliveryArrayOfCarrierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfCarrierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfCarrierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfCarrierPartyComponent {
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
pub struct DeliveryArrayOfDeliveryAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryAddress>,
}

impl AsMut<DeliveryArrayOfDeliveryAddressComponent> for DeliveryArrayOfDeliveryAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfDeliveryAddressComponent> for DeliveryArrayOfDeliveryAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfDeliveryAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfDeliveryAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfDeliveryAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryAddress) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryAddress) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryAddress> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryAddress> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfDeliveryLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryLocation>,
}

impl AsMut<DeliveryArrayOfDeliveryLocationComponent> for DeliveryArrayOfDeliveryLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfDeliveryLocationComponent> for DeliveryArrayOfDeliveryLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfDeliveryLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfDeliveryLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfDeliveryLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfDeliveryPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryParty>,
}

impl AsMut<DeliveryArrayOfDeliveryPartyComponent> for DeliveryArrayOfDeliveryPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfDeliveryPartyComponent> for DeliveryArrayOfDeliveryPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfDeliveryPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfDeliveryPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfDeliveryPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfDeliveryTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms>,
}

impl AsMut<DeliveryArrayOfDeliveryTermsComponent> for DeliveryArrayOfDeliveryTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfDeliveryTermsComponent> for DeliveryArrayOfDeliveryTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfDeliveryTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfDeliveryTermsComponent {
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
pub struct DeliveryArrayOfDespatchComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Despatch>,
}

impl AsMut<DeliveryArrayOfDespatchComponent> for DeliveryArrayOfDespatchComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfDespatchComponent> for DeliveryArrayOfDespatchComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfDespatchComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfDespatchComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfDespatchComponent {
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
pub struct DeliveryArrayOfEstimatedDeliveryPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EstimatedDeliveryPeriod>,
}

impl AsMut<DeliveryArrayOfEstimatedDeliveryPeriodComponent> for DeliveryArrayOfEstimatedDeliveryPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfEstimatedDeliveryPeriodComponent> for DeliveryArrayOfEstimatedDeliveryPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfEstimatedDeliveryPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfEstimatedDeliveryPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfEstimatedDeliveryPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EstimatedDeliveryPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EstimatedDeliveryPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EstimatedDeliveryPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EstimatedDeliveryPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<DeliveryArrayOfIDComponent> for DeliveryArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfIDComponent> for DeliveryArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfIDComponent {
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
pub struct DeliveryArrayOfLatestDeliveryDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatestDeliveryDate>,
}

impl AsMut<DeliveryArrayOfLatestDeliveryDateComponent> for DeliveryArrayOfLatestDeliveryDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfLatestDeliveryDateComponent> for DeliveryArrayOfLatestDeliveryDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfLatestDeliveryDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfLatestDeliveryDateComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfLatestDeliveryDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatestDeliveryDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatestDeliveryDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatestDeliveryDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatestDeliveryDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfLatestDeliveryTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatestDeliveryTime>,
}

impl AsMut<DeliveryArrayOfLatestDeliveryTimeComponent> for DeliveryArrayOfLatestDeliveryTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfLatestDeliveryTimeComponent> for DeliveryArrayOfLatestDeliveryTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfLatestDeliveryTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfLatestDeliveryTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfLatestDeliveryTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatestDeliveryTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatestDeliveryTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatestDeliveryTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatestDeliveryTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfMaximumDeliveryUnitComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MaximumDeliveryUnit>,
}

impl AsMut<DeliveryArrayOfMaximumDeliveryUnitComponent> for DeliveryArrayOfMaximumDeliveryUnitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfMaximumDeliveryUnitComponent> for DeliveryArrayOfMaximumDeliveryUnitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfMaximumDeliveryUnitComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfMaximumDeliveryUnitComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfMaximumDeliveryUnitComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MaximumDeliveryUnit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MaximumDeliveryUnit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MaximumDeliveryUnit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MaximumDeliveryUnit> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfMaximumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity>,
}

impl AsMut<DeliveryArrayOfMaximumQuantityComponent> for DeliveryArrayOfMaximumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfMaximumQuantityComponent> for DeliveryArrayOfMaximumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfMaximumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfMaximumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfMaximumQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfMinimumDeliveryUnitComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MinimumDeliveryUnit>,
}

impl AsMut<DeliveryArrayOfMinimumDeliveryUnitComponent> for DeliveryArrayOfMinimumDeliveryUnitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfMinimumDeliveryUnitComponent> for DeliveryArrayOfMinimumDeliveryUnitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfMinimumDeliveryUnitComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfMinimumDeliveryUnitComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfMinimumDeliveryUnitComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MinimumDeliveryUnit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MinimumDeliveryUnit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MinimumDeliveryUnit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MinimumDeliveryUnit> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfMinimumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity>,
}

impl AsMut<DeliveryArrayOfMinimumQuantityComponent> for DeliveryArrayOfMinimumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfMinimumQuantityComponent> for DeliveryArrayOfMinimumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfMinimumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfMinimumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfMinimumQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfNotifyPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::NotifyParty>,
}

impl AsMut<DeliveryArrayOfNotifyPartyComponent> for DeliveryArrayOfNotifyPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfNotifyPartyComponent> for DeliveryArrayOfNotifyPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfNotifyPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfNotifyPartyComponent {
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
pub struct DeliveryArrayOfPromisedDeliveryPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PromisedDeliveryPeriod>,
}

impl AsMut<DeliveryArrayOfPromisedDeliveryPeriodComponent> for DeliveryArrayOfPromisedDeliveryPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfPromisedDeliveryPeriodComponent> for DeliveryArrayOfPromisedDeliveryPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfPromisedDeliveryPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfPromisedDeliveryPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfPromisedDeliveryPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PromisedDeliveryPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PromisedDeliveryPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PromisedDeliveryPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PromisedDeliveryPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<DeliveryArrayOfQuantityComponent> for DeliveryArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfQuantityComponent> for DeliveryArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Quantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Quantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfReleaseIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReleaseID>,
}

impl AsMut<DeliveryArrayOfReleaseIDComponent> for DeliveryArrayOfReleaseIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfReleaseIDComponent> for DeliveryArrayOfReleaseIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfReleaseIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfReleaseIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfReleaseIDComponent {
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
pub struct DeliveryArrayOfRequestedDeliveryPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequestedDeliveryPeriod>,
}

impl AsMut<DeliveryArrayOfRequestedDeliveryPeriodComponent> for DeliveryArrayOfRequestedDeliveryPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfRequestedDeliveryPeriodComponent> for DeliveryArrayOfRequestedDeliveryPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfRequestedDeliveryPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfRequestedDeliveryPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfRequestedDeliveryPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequestedDeliveryPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequestedDeliveryPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequestedDeliveryPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequestedDeliveryPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfShipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Shipment>,
}

impl AsMut<DeliveryArrayOfShipmentComponent> for DeliveryArrayOfShipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfShipmentComponent> for DeliveryArrayOfShipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfShipmentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfShipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfShipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Shipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Shipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Shipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Shipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryArrayOfTrackingIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TrackingID>,
}

impl AsMut<DeliveryArrayOfTrackingIDComponent> for DeliveryArrayOfTrackingIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryArrayOfTrackingIDComponent> for DeliveryArrayOfTrackingIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryArrayOfTrackingIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryArrayOfTrackingIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl DeliveryArrayOfTrackingIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TrackingID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TrackingID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TrackingID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TrackingID> {
        self.items.iter()
    }
}


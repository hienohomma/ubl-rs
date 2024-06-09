use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsumptionPoint {
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ConsumptionPointArrayOfAddressComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ConsumptionPointArrayOfDescriptionComponent>,
    #[serde(rename = "ID")]
    pub id: ConsumptionPointArrayOfIDComponent,
    #[serde(rename = "SubscriberID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_id: Option<ConsumptionPointArrayOfSubscriberIDComponent>,
    #[serde(rename = "SubscriberType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_type: Option<ConsumptionPointArrayOfSubscriberTypeComponent>,
    #[serde(rename = "SubscriberTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_type_code: Option<ConsumptionPointArrayOfSubscriberTypeCodeComponent>,
    #[serde(rename = "TotalDeliveredQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_delivered_quantity: Option<ConsumptionPointArrayOfTotalDeliveredQuantityComponent>,
    #[serde(rename = "UtilityMeter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_meter: Option<ConsumptionPointArrayOfUtilityMeterComponent>,
    #[serde(rename = "WebSiteAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_site_access: Option<ConsumptionPointArrayOfWebSiteAccessComponent>,
}

impl AsMut<ConsumptionPoint> for ConsumptionPoint {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionPoint> for ConsumptionPoint {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.web_site_access {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionPoint.web_site_access", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionPoint.description", e));
            }
        }
        if let Some(v) = &self.subscriber_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionPoint.subscriber_id", e));
            }
        }
        if let Some(v) = &self.subscriber_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionPoint.subscriber_type", e));
            }
        }
        if let Some(v) = &self.subscriber_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionPoint.subscriber_type_code", e));
            }
        }
        if let Some(v) = &self.total_delivered_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionPoint.total_delivered_quantity", e));
            }
        }
        if let Some(v) = &self.address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionPoint.address", e));
            }
        }
        if let Some(v) = &self.utility_meter {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionPoint.utility_meter", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ConsumptionPoint.id", e));
        }

        Ok(self)
    }

    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl ConsumptionPoint {
    pub fn title() -> &'static str {
        "Consumption Point. Details"
    }
    pub fn description() -> &'static str {
        "A class to define the point of consumption for a utility, such as a meter."
    }
    pub fn new(id: ConsumptionPointArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            description: None,
            id,
            total_delivered_quantity: None,
            subscriber_type_code: None,
            subscriber_type: None,
            utility_meter: None,
            web_site_access: None,
            address: None,
            subscriber_id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionPointArrayOfAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Address>,
}

impl AsMut<ConsumptionPointArrayOfAddressComponent> for ConsumptionPointArrayOfAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionPointArrayOfAddressComponent> for ConsumptionPointArrayOfAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl ConsumptionPointArrayOfAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Address) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Address) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Address> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Address> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionPointArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ConsumptionPointArrayOfDescriptionComponent> for ConsumptionPointArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionPointArrayOfDescriptionComponent> for ConsumptionPointArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl ConsumptionPointArrayOfDescriptionComponent {
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
pub struct ConsumptionPointArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ConsumptionPointArrayOfIDComponent> for ConsumptionPointArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionPointArrayOfIDComponent> for ConsumptionPointArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl ConsumptionPointArrayOfIDComponent {
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
pub struct ConsumptionPointArrayOfSubscriberIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SubscriberID>,
}

impl AsMut<ConsumptionPointArrayOfSubscriberIDComponent> for ConsumptionPointArrayOfSubscriberIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionPointArrayOfSubscriberIDComponent> for ConsumptionPointArrayOfSubscriberIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfSubscriberIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfSubscriberIDComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl ConsumptionPointArrayOfSubscriberIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SubscriberID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SubscriberID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SubscriberID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SubscriberID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionPointArrayOfSubscriberTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SubscriberType>,
}

impl AsMut<ConsumptionPointArrayOfSubscriberTypeComponent> for ConsumptionPointArrayOfSubscriberTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionPointArrayOfSubscriberTypeComponent> for ConsumptionPointArrayOfSubscriberTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfSubscriberTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfSubscriberTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl ConsumptionPointArrayOfSubscriberTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SubscriberType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SubscriberType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SubscriberType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SubscriberType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionPointArrayOfSubscriberTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SubscriberTypeCode>,
}

impl AsMut<ConsumptionPointArrayOfSubscriberTypeCodeComponent> for ConsumptionPointArrayOfSubscriberTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionPointArrayOfSubscriberTypeCodeComponent> for ConsumptionPointArrayOfSubscriberTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfSubscriberTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfSubscriberTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl ConsumptionPointArrayOfSubscriberTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SubscriberTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SubscriberTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SubscriberTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SubscriberTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionPointArrayOfTotalDeliveredQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalDeliveredQuantity>,
}

impl AsMut<ConsumptionPointArrayOfTotalDeliveredQuantityComponent> for ConsumptionPointArrayOfTotalDeliveredQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionPointArrayOfTotalDeliveredQuantityComponent> for ConsumptionPointArrayOfTotalDeliveredQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfTotalDeliveredQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfTotalDeliveredQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl ConsumptionPointArrayOfTotalDeliveredQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalDeliveredQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalDeliveredQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalDeliveredQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalDeliveredQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionPointArrayOfUtilityMeterComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::UtilityMeter>,
}

impl AsMut<ConsumptionPointArrayOfUtilityMeterComponent> for ConsumptionPointArrayOfUtilityMeterComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionPointArrayOfUtilityMeterComponent> for ConsumptionPointArrayOfUtilityMeterComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfUtilityMeterComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl ConsumptionPointArrayOfUtilityMeterComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::UtilityMeter) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::UtilityMeter) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::UtilityMeter> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::UtilityMeter> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionPointArrayOfWebSiteAccessComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WebSiteAccess>,
}

impl AsMut<ConsumptionPointArrayOfWebSiteAccessComponent> for ConsumptionPointArrayOfWebSiteAccessComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionPointArrayOfWebSiteAccessComponent> for ConsumptionPointArrayOfWebSiteAccessComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfWebSiteAccessComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionPointArrayOfWebSiteAccessComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl ConsumptionPointArrayOfWebSiteAccessComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WebSiteAccess) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WebSiteAccess) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WebSiteAccess> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WebSiteAccess> {
        self.items.iter()
    }
}


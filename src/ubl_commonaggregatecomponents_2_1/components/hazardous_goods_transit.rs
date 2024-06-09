use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HazardousGoodsTransit {
    #[serde(rename = "HazardousRegulationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_regulation_code: Option<HazardousGoodsTransitArrayOfHazardousRegulationCodeComponent>,
    #[serde(rename = "InhalationToxicityZoneCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inhalation_toxicity_zone_code: Option<HazardousGoodsTransitArrayOfInhalationToxicityZoneCodeComponent>,
    #[serde(rename = "MaximumTemperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_temperature: Option<HazardousGoodsTransitArrayOfMaximumTemperatureComponent>,
    #[serde(rename = "MinimumTemperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_temperature: Option<HazardousGoodsTransitArrayOfMinimumTemperatureComponent>,
    #[serde(rename = "PackingCriteriaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packing_criteria_code: Option<HazardousGoodsTransitArrayOfPackingCriteriaCodeComponent>,
    #[serde(rename = "TransportAuthorizationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_authorization_code: Option<HazardousGoodsTransitArrayOfTransportAuthorizationCodeComponent>,
    #[serde(rename = "TransportEmergencyCardCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_emergency_card_code: Option<HazardousGoodsTransitArrayOfTransportEmergencyCardCodeComponent>,
}

impl AsMut<HazardousGoodsTransit> for HazardousGoodsTransit {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousGoodsTransit> for HazardousGoodsTransit {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.transport_emergency_card_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousGoodsTransit.transport_emergency_card_code", e));
            }
        }
        if let Some(v) = &self.maximum_temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousGoodsTransit.maximum_temperature", e));
            }
        }
        if let Some(v) = &self.minimum_temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousGoodsTransit.minimum_temperature", e));
            }
        }
        if let Some(v) = &self.packing_criteria_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousGoodsTransit.packing_criteria_code", e));
            }
        }
        if let Some(v) = &self.hazardous_regulation_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousGoodsTransit.hazardous_regulation_code", e));
            }
        }
        if let Some(v) = &self.inhalation_toxicity_zone_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousGoodsTransit.inhalation_toxicity_zone_code", e));
            }
        }
        if let Some(v) = &self.transport_authorization_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousGoodsTransit.transport_authorization_code", e));
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

impl HazardousGoodsTransit {
    pub fn title() -> &'static str {
        "Hazardous Goods Transit. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe hazardous goods in transit."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            transport_emergency_card_code: None,
            hazardous_regulation_code: None,
            transport_authorization_code: None,
            minimum_temperature: None,
            inhalation_toxicity_zone_code: None,
            packing_criteria_code: None,
            maximum_temperature: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousGoodsTransitArrayOfHazardousRegulationCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HazardousRegulationCode>,
}

impl AsMut<HazardousGoodsTransitArrayOfHazardousRegulationCodeComponent> for HazardousGoodsTransitArrayOfHazardousRegulationCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousGoodsTransitArrayOfHazardousRegulationCodeComponent> for HazardousGoodsTransitArrayOfHazardousRegulationCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfHazardousRegulationCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfHazardousRegulationCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousGoodsTransitArrayOfHazardousRegulationCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HazardousRegulationCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HazardousRegulationCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HazardousRegulationCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HazardousRegulationCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousGoodsTransitArrayOfInhalationToxicityZoneCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InhalationToxicityZoneCode>,
}

impl AsMut<HazardousGoodsTransitArrayOfInhalationToxicityZoneCodeComponent> for HazardousGoodsTransitArrayOfInhalationToxicityZoneCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousGoodsTransitArrayOfInhalationToxicityZoneCodeComponent> for HazardousGoodsTransitArrayOfInhalationToxicityZoneCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfInhalationToxicityZoneCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfInhalationToxicityZoneCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousGoodsTransitArrayOfInhalationToxicityZoneCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InhalationToxicityZoneCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InhalationToxicityZoneCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InhalationToxicityZoneCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InhalationToxicityZoneCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousGoodsTransitArrayOfMaximumTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature>,
}

impl AsMut<HazardousGoodsTransitArrayOfMaximumTemperatureComponent> for HazardousGoodsTransitArrayOfMaximumTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousGoodsTransitArrayOfMaximumTemperatureComponent> for HazardousGoodsTransitArrayOfMaximumTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfMaximumTemperatureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfMaximumTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousGoodsTransitArrayOfMaximumTemperatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousGoodsTransitArrayOfMinimumTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature>,
}

impl AsMut<HazardousGoodsTransitArrayOfMinimumTemperatureComponent> for HazardousGoodsTransitArrayOfMinimumTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousGoodsTransitArrayOfMinimumTemperatureComponent> for HazardousGoodsTransitArrayOfMinimumTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfMinimumTemperatureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfMinimumTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousGoodsTransitArrayOfMinimumTemperatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousGoodsTransitArrayOfPackingCriteriaCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PackingCriteriaCode>,
}

impl AsMut<HazardousGoodsTransitArrayOfPackingCriteriaCodeComponent> for HazardousGoodsTransitArrayOfPackingCriteriaCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousGoodsTransitArrayOfPackingCriteriaCodeComponent> for HazardousGoodsTransitArrayOfPackingCriteriaCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfPackingCriteriaCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfPackingCriteriaCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousGoodsTransitArrayOfPackingCriteriaCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PackingCriteriaCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PackingCriteriaCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PackingCriteriaCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PackingCriteriaCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousGoodsTransitArrayOfTransportAuthorizationCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportAuthorizationCode>,
}

impl AsMut<HazardousGoodsTransitArrayOfTransportAuthorizationCodeComponent> for HazardousGoodsTransitArrayOfTransportAuthorizationCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousGoodsTransitArrayOfTransportAuthorizationCodeComponent> for HazardousGoodsTransitArrayOfTransportAuthorizationCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfTransportAuthorizationCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfTransportAuthorizationCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousGoodsTransitArrayOfTransportAuthorizationCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportAuthorizationCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportAuthorizationCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportAuthorizationCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportAuthorizationCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousGoodsTransitArrayOfTransportEmergencyCardCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportEmergencyCardCode>,
}

impl AsMut<HazardousGoodsTransitArrayOfTransportEmergencyCardCodeComponent> for HazardousGoodsTransitArrayOfTransportEmergencyCardCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousGoodsTransitArrayOfTransportEmergencyCardCodeComponent> for HazardousGoodsTransitArrayOfTransportEmergencyCardCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfTransportEmergencyCardCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousGoodsTransitArrayOfTransportEmergencyCardCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousGoodsTransitArrayOfTransportEmergencyCardCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportEmergencyCardCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportEmergencyCardCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportEmergencyCardCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportEmergencyCardCode> {
        self.items.iter()
    }
}


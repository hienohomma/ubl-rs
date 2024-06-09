use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MaritimeTransport {
    #[serde(rename = "GrossTonnageMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_tonnage_measure: Option<MaritimeTransportArrayOfGrossTonnageMeasureComponent>,
    #[serde(rename = "NetTonnageMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_tonnage_measure: Option<MaritimeTransportArrayOfNetTonnageMeasureComponent>,
    #[serde(rename = "RadioCallSignID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radio_call_sign_id: Option<MaritimeTransportArrayOfRadioCallSignIDComponent>,
    #[serde(rename = "RegistryCertificateDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_certificate_document_reference: Option<MaritimeTransportArrayOfRegistryCertificateDocumentReferenceComponent>,
    #[serde(rename = "RegistryPortLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_port_location: Option<MaritimeTransportArrayOfRegistryPortLocationComponent>,
    #[serde(rename = "ShipsRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ships_requirements: Option<MaritimeTransportArrayOfShipsRequirementsComponent>,
    #[serde(rename = "VesselID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vessel_id: Option<MaritimeTransportArrayOfVesselIDComponent>,
    #[serde(rename = "VesselName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vessel_name: Option<MaritimeTransportArrayOfVesselNameComponent>,
}

impl AsMut<MaritimeTransport> for MaritimeTransport {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MaritimeTransport> for MaritimeTransport {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.radio_call_sign_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MaritimeTransport.radio_call_sign_id", e));
            }
        }
        if let Some(v) = &self.net_tonnage_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MaritimeTransport.net_tonnage_measure", e));
            }
        }
        if let Some(v) = &self.gross_tonnage_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MaritimeTransport.gross_tonnage_measure", e));
            }
        }
        if let Some(v) = &self.registry_port_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MaritimeTransport.registry_port_location", e));
            }
        }
        if let Some(v) = &self.vessel_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MaritimeTransport.vessel_name", e));
            }
        }
        if let Some(v) = &self.registry_certificate_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MaritimeTransport.registry_certificate_document_reference", e));
            }
        }
        if let Some(v) = &self.vessel_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MaritimeTransport.vessel_id", e));
            }
        }
        if let Some(v) = &self.ships_requirements {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MaritimeTransport.ships_requirements", e));
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

impl MaritimeTransport {
    pub fn title() -> &'static str {
        "Maritime Transport. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a vessel used for transport by water (including sea, river, and canal)."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            gross_tonnage_measure: None,
            registry_certificate_document_reference: None,
            vessel_id: None,
            ships_requirements: None,
            radio_call_sign_id: None,
            net_tonnage_measure: None,
            registry_port_location: None,
            vessel_name: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MaritimeTransportArrayOfGrossTonnageMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GrossTonnageMeasure>,
}

impl AsMut<MaritimeTransportArrayOfGrossTonnageMeasureComponent> for MaritimeTransportArrayOfGrossTonnageMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MaritimeTransportArrayOfGrossTonnageMeasureComponent> for MaritimeTransportArrayOfGrossTonnageMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfGrossTonnageMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfGrossTonnageMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MaritimeTransportArrayOfGrossTonnageMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GrossTonnageMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GrossTonnageMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GrossTonnageMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GrossTonnageMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MaritimeTransportArrayOfNetTonnageMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetTonnageMeasure>,
}

impl AsMut<MaritimeTransportArrayOfNetTonnageMeasureComponent> for MaritimeTransportArrayOfNetTonnageMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MaritimeTransportArrayOfNetTonnageMeasureComponent> for MaritimeTransportArrayOfNetTonnageMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfNetTonnageMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfNetTonnageMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MaritimeTransportArrayOfNetTonnageMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NetTonnageMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NetTonnageMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NetTonnageMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NetTonnageMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MaritimeTransportArrayOfRadioCallSignIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RadioCallSignID>,
}

impl AsMut<MaritimeTransportArrayOfRadioCallSignIDComponent> for MaritimeTransportArrayOfRadioCallSignIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MaritimeTransportArrayOfRadioCallSignIDComponent> for MaritimeTransportArrayOfRadioCallSignIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfRadioCallSignIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfRadioCallSignIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MaritimeTransportArrayOfRadioCallSignIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RadioCallSignID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RadioCallSignID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RadioCallSignID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RadioCallSignID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MaritimeTransportArrayOfRegistryCertificateDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RegistryCertificateDocumentReference>,
}

impl AsMut<MaritimeTransportArrayOfRegistryCertificateDocumentReferenceComponent> for MaritimeTransportArrayOfRegistryCertificateDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MaritimeTransportArrayOfRegistryCertificateDocumentReferenceComponent> for MaritimeTransportArrayOfRegistryCertificateDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfRegistryCertificateDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfRegistryCertificateDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MaritimeTransportArrayOfRegistryCertificateDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RegistryCertificateDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RegistryCertificateDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RegistryCertificateDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RegistryCertificateDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MaritimeTransportArrayOfRegistryPortLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RegistryPortLocation>,
}

impl AsMut<MaritimeTransportArrayOfRegistryPortLocationComponent> for MaritimeTransportArrayOfRegistryPortLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MaritimeTransportArrayOfRegistryPortLocationComponent> for MaritimeTransportArrayOfRegistryPortLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfRegistryPortLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfRegistryPortLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MaritimeTransportArrayOfRegistryPortLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RegistryPortLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RegistryPortLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RegistryPortLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RegistryPortLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MaritimeTransportArrayOfShipsRequirementsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ShipsRequirements>,
}

impl AsMut<MaritimeTransportArrayOfShipsRequirementsComponent> for MaritimeTransportArrayOfShipsRequirementsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MaritimeTransportArrayOfShipsRequirementsComponent> for MaritimeTransportArrayOfShipsRequirementsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfShipsRequirementsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MaritimeTransportArrayOfShipsRequirementsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ShipsRequirements) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ShipsRequirements) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ShipsRequirements> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ShipsRequirements> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MaritimeTransportArrayOfVesselIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::VesselID>,
}

impl AsMut<MaritimeTransportArrayOfVesselIDComponent> for MaritimeTransportArrayOfVesselIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MaritimeTransportArrayOfVesselIDComponent> for MaritimeTransportArrayOfVesselIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfVesselIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfVesselIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MaritimeTransportArrayOfVesselIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::VesselID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::VesselID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::VesselID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::VesselID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MaritimeTransportArrayOfVesselNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::VesselName>,
}

impl AsMut<MaritimeTransportArrayOfVesselNameComponent> for MaritimeTransportArrayOfVesselNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MaritimeTransportArrayOfVesselNameComponent> for MaritimeTransportArrayOfVesselNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfVesselNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MaritimeTransportArrayOfVesselNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MaritimeTransportArrayOfVesselNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::VesselName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::VesselName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::VesselName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::VesselName> {
        self.items.iter()
    }
}


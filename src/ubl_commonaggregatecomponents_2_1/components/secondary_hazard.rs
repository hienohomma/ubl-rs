use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecondaryHazard {
    #[serde(rename = "EmergencyProceduresCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_procedures_code: Option<SecondaryHazardArrayOfEmergencyProceduresCodeComponent>,
    #[serde(rename = "Extension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<SecondaryHazardArrayOfExtensionComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SecondaryHazardArrayOfIDComponent>,
    #[serde(rename = "PlacardEndorsement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placard_endorsement: Option<SecondaryHazardArrayOfPlacardEndorsementComponent>,
    #[serde(rename = "PlacardNotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placard_notation: Option<SecondaryHazardArrayOfPlacardNotationComponent>,
}

impl AsMut<SecondaryHazard> for SecondaryHazard {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SecondaryHazard> for SecondaryHazard {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.placard_notation {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SecondaryHazard.placard_notation", e));
            }
        }
        if let Some(v) = &self.extension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SecondaryHazard.extension", e));
            }
        }
        if let Some(v) = &self.placard_endorsement {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SecondaryHazard.placard_endorsement", e));
            }
        }
        if let Some(v) = &self.emergency_procedures_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SecondaryHazard.emergency_procedures_code", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SecondaryHazard.id", e));
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

impl SecondaryHazard {
    pub fn title() -> &'static str {
        "Secondary Hazard. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a secondary hazard associated with a hazardous item."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            id: None,
            extension: None,
            placard_notation: None,
            emergency_procedures_code: None,
            placard_endorsement: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SecondaryHazardArrayOfEmergencyProceduresCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EmergencyProceduresCode>,
}

impl AsMut<SecondaryHazardArrayOfEmergencyProceduresCodeComponent> for SecondaryHazardArrayOfEmergencyProceduresCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SecondaryHazardArrayOfEmergencyProceduresCodeComponent> for SecondaryHazardArrayOfEmergencyProceduresCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SecondaryHazardArrayOfEmergencyProceduresCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SecondaryHazardArrayOfEmergencyProceduresCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SecondaryHazardArrayOfEmergencyProceduresCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EmergencyProceduresCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EmergencyProceduresCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EmergencyProceduresCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EmergencyProceduresCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SecondaryHazardArrayOfExtensionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Extension>,
}

impl AsMut<SecondaryHazardArrayOfExtensionComponent> for SecondaryHazardArrayOfExtensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SecondaryHazardArrayOfExtensionComponent> for SecondaryHazardArrayOfExtensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SecondaryHazardArrayOfExtensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SecondaryHazardArrayOfExtensionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Extension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Extension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Extension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Extension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SecondaryHazardArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<SecondaryHazardArrayOfIDComponent> for SecondaryHazardArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SecondaryHazardArrayOfIDComponent> for SecondaryHazardArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SecondaryHazardArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SecondaryHazardArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SecondaryHazardArrayOfIDComponent {
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
pub struct SecondaryHazardArrayOfPlacardEndorsementComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PlacardEndorsement>,
}

impl AsMut<SecondaryHazardArrayOfPlacardEndorsementComponent> for SecondaryHazardArrayOfPlacardEndorsementComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SecondaryHazardArrayOfPlacardEndorsementComponent> for SecondaryHazardArrayOfPlacardEndorsementComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SecondaryHazardArrayOfPlacardEndorsementComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SecondaryHazardArrayOfPlacardEndorsementComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SecondaryHazardArrayOfPlacardEndorsementComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PlacardEndorsement) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PlacardEndorsement) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PlacardEndorsement> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PlacardEndorsement> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SecondaryHazardArrayOfPlacardNotationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PlacardNotation>,
}

impl AsMut<SecondaryHazardArrayOfPlacardNotationComponent> for SecondaryHazardArrayOfPlacardNotationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SecondaryHazardArrayOfPlacardNotationComponent> for SecondaryHazardArrayOfPlacardNotationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SecondaryHazardArrayOfPlacardNotationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SecondaryHazardArrayOfPlacardNotationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SecondaryHazardArrayOfPlacardNotationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PlacardNotation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PlacardNotation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PlacardNotation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PlacardNotation> {
        self.items.iter()
    }
}


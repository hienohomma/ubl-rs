use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransportEquipmentSeal {
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<TransportEquipmentSealArrayOfConditionComponent>,
    #[serde(rename = "ID")]
    pub id: TransportEquipmentSealArrayOfIDComponent,
    #[serde(rename = "SealIssuerTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seal_issuer_type_code: Option<TransportEquipmentSealArrayOfSealIssuerTypeCodeComponent>,
    #[serde(rename = "SealStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seal_status_code: Option<TransportEquipmentSealArrayOfSealStatusCodeComponent>,
    #[serde(rename = "SealingPartyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sealing_party_type: Option<TransportEquipmentSealArrayOfSealingPartyTypeComponent>,
}

impl AsMut<TransportEquipmentSeal> for TransportEquipmentSeal {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentSeal> for TransportEquipmentSeal {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("TransportEquipmentSeal.id", e));
        }
        if let Some(v) = &self.seal_issuer_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipmentSeal.seal_issuer_type_code", e));
            }
        }
        if let Some(v) = &self.condition {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipmentSeal.condition", e));
            }
        }
        if let Some(v) = &self.sealing_party_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipmentSeal.sealing_party_type", e));
            }
        }
        if let Some(v) = &self.seal_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEquipmentSeal.seal_status_code", e));
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

impl TransportEquipmentSeal {
    pub fn title() -> &'static str {
        "Transport Equipment Seal. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a device (a transport equipment seal) for securing the doors of a shipping container."
    }
    pub fn new(id: TransportEquipmentSealArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            seal_status_code: None,
            sealing_party_type: None,
            id,
            condition: None,
            seal_issuer_type_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentSealArrayOfConditionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Condition>,
}

impl AsMut<TransportEquipmentSealArrayOfConditionComponent> for TransportEquipmentSealArrayOfConditionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentSealArrayOfConditionComponent> for TransportEquipmentSealArrayOfConditionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentSealArrayOfConditionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentSealArrayOfConditionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentSealArrayOfConditionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Condition) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Condition) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Condition> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Condition> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentSealArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<TransportEquipmentSealArrayOfIDComponent> for TransportEquipmentSealArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentSealArrayOfIDComponent> for TransportEquipmentSealArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentSealArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentSealArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentSealArrayOfIDComponent {
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
pub struct TransportEquipmentSealArrayOfSealIssuerTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SealIssuerTypeCode>,
}

impl AsMut<TransportEquipmentSealArrayOfSealIssuerTypeCodeComponent> for TransportEquipmentSealArrayOfSealIssuerTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentSealArrayOfSealIssuerTypeCodeComponent> for TransportEquipmentSealArrayOfSealIssuerTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentSealArrayOfSealIssuerTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentSealArrayOfSealIssuerTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentSealArrayOfSealIssuerTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SealIssuerTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SealIssuerTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SealIssuerTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SealIssuerTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentSealArrayOfSealStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SealStatusCode>,
}

impl AsMut<TransportEquipmentSealArrayOfSealStatusCodeComponent> for TransportEquipmentSealArrayOfSealStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentSealArrayOfSealStatusCodeComponent> for TransportEquipmentSealArrayOfSealStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentSealArrayOfSealStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentSealArrayOfSealStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentSealArrayOfSealStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SealStatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SealStatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SealStatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SealStatusCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEquipmentSealArrayOfSealingPartyTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SealingPartyType>,
}

impl AsMut<TransportEquipmentSealArrayOfSealingPartyTypeComponent> for TransportEquipmentSealArrayOfSealingPartyTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEquipmentSealArrayOfSealingPartyTypeComponent> for TransportEquipmentSealArrayOfSealingPartyTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEquipmentSealArrayOfSealingPartyTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEquipmentSealArrayOfSealingPartyTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEquipmentSealArrayOfSealingPartyTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SealingPartyType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SealingPartyType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SealingPartyType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SealingPartyType> {
        self.items.iter()
    }
}


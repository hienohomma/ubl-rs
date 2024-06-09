use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LotIdentification {
    #[serde(rename = "AdditionalItemProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_item_property: Option<LotIdentificationArrayOfAdditionalItemPropertyComponent>,
    #[serde(rename = "ExpiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<LotIdentificationArrayOfExpiryDateComponent>,
    #[serde(rename = "LotNumberID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number_id: Option<LotIdentificationArrayOfLotNumberIDComponent>,
}

impl AsMut<LotIdentification> for LotIdentification {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LotIdentification> for LotIdentification {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.lot_number_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LotIdentification.lot_number_id", e));
            }
        }
        if let Some(v) = &self.additional_item_property {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LotIdentification.additional_item_property", e));
            }
        }
        if let Some(v) = &self.expiry_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LotIdentification.expiry_date", e));
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

impl LotIdentification {
    pub fn title() -> &'static str {
        "Lot Identification. Details"
    }
    pub fn description() -> &'static str {
        "A class for defining a lot identifier (the identifier of a set of item instances that would be used in case of a recall of that item)."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            lot_number_id: None,
            additional_item_property: None,
            expiry_date: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LotIdentificationArrayOfAdditionalItemPropertyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AdditionalItemProperty>,
}

impl AsMut<LotIdentificationArrayOfAdditionalItemPropertyComponent> for LotIdentificationArrayOfAdditionalItemPropertyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LotIdentificationArrayOfAdditionalItemPropertyComponent> for LotIdentificationArrayOfAdditionalItemPropertyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LotIdentificationArrayOfAdditionalItemPropertyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LotIdentificationArrayOfAdditionalItemPropertyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AdditionalItemProperty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AdditionalItemProperty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AdditionalItemProperty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AdditionalItemProperty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LotIdentificationArrayOfExpiryDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExpiryDate>,
}

impl AsMut<LotIdentificationArrayOfExpiryDateComponent> for LotIdentificationArrayOfExpiryDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LotIdentificationArrayOfExpiryDateComponent> for LotIdentificationArrayOfExpiryDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LotIdentificationArrayOfExpiryDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LotIdentificationArrayOfExpiryDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LotIdentificationArrayOfExpiryDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExpiryDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExpiryDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExpiryDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExpiryDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LotIdentificationArrayOfLotNumberIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LotNumberID>,
}

impl AsMut<LotIdentificationArrayOfLotNumberIDComponent> for LotIdentificationArrayOfLotNumberIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LotIdentificationArrayOfLotNumberIDComponent> for LotIdentificationArrayOfLotNumberIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LotIdentificationArrayOfLotNumberIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LotIdentificationArrayOfLotNumberIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LotIdentificationArrayOfLotNumberIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LotNumberID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LotNumberID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LotNumberID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LotNumberID> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemInstance {
    #[serde(rename = "AdditionalItemProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_item_property: Option<ItemInstanceArrayOfAdditionalItemPropertyComponent>,
    #[serde(rename = "BestBeforeDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_before_date: Option<ItemInstanceArrayOfBestBeforeDateComponent>,
    #[serde(rename = "LotIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_identification: Option<ItemInstanceArrayOfLotIdentificationComponent>,
    #[serde(rename = "ManufactureDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacture_date: Option<ItemInstanceArrayOfManufactureDateComponent>,
    #[serde(rename = "ManufactureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacture_time: Option<ItemInstanceArrayOfManufactureTimeComponent>,
    #[serde(rename = "ProductTraceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_trace_id: Option<ItemInstanceArrayOfProductTraceIDComponent>,
    #[serde(rename = "RegistrationID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<ItemInstanceArrayOfRegistrationIDComponent>,
    #[serde(rename = "SerialID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_id: Option<ItemInstanceArrayOfSerialIDComponent>,
}

impl AsMut<ItemInstance> for ItemInstance {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInstance> for ItemInstance {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.best_before_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInstance.best_before_date", e));
            }
        }
        if let Some(v) = &self.manufacture_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInstance.manufacture_time", e));
            }
        }
        if let Some(v) = &self.serial_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInstance.serial_id", e));
            }
        }
        if let Some(v) = &self.registration_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInstance.registration_id", e));
            }
        }
        if let Some(v) = &self.additional_item_property {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInstance.additional_item_property", e));
            }
        }
        if let Some(v) = &self.lot_identification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInstance.lot_identification", e));
            }
        }
        if let Some(v) = &self.manufacture_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInstance.manufacture_date", e));
            }
        }
        if let Some(v) = &self.product_trace_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInstance.product_trace_id", e));
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

impl ItemInstance {
    pub fn title() -> &'static str {
        "Item Instance. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a specific, trackable instance of an item."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            manufacture_time: None,
            product_trace_id: None,
            serial_id: None,
            best_before_date: None,
            additional_item_property: None,
            manufacture_date: None,
            registration_id: None,
            lot_identification: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInstanceArrayOfAdditionalItemPropertyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AdditionalItemProperty>,
}

impl AsMut<ItemInstanceArrayOfAdditionalItemPropertyComponent> for ItemInstanceArrayOfAdditionalItemPropertyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInstanceArrayOfAdditionalItemPropertyComponent> for ItemInstanceArrayOfAdditionalItemPropertyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfAdditionalItemPropertyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInstanceArrayOfAdditionalItemPropertyComponent {
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
pub struct ItemInstanceArrayOfBestBeforeDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BestBeforeDate>,
}

impl AsMut<ItemInstanceArrayOfBestBeforeDateComponent> for ItemInstanceArrayOfBestBeforeDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInstanceArrayOfBestBeforeDateComponent> for ItemInstanceArrayOfBestBeforeDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfBestBeforeDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfBestBeforeDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInstanceArrayOfBestBeforeDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BestBeforeDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BestBeforeDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BestBeforeDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BestBeforeDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInstanceArrayOfLotIdentificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LotIdentification>,
}

impl AsMut<ItemInstanceArrayOfLotIdentificationComponent> for ItemInstanceArrayOfLotIdentificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInstanceArrayOfLotIdentificationComponent> for ItemInstanceArrayOfLotIdentificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfLotIdentificationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfLotIdentificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInstanceArrayOfLotIdentificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LotIdentification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LotIdentification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LotIdentification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LotIdentification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInstanceArrayOfManufactureDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ManufactureDate>,
}

impl AsMut<ItemInstanceArrayOfManufactureDateComponent> for ItemInstanceArrayOfManufactureDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInstanceArrayOfManufactureDateComponent> for ItemInstanceArrayOfManufactureDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfManufactureDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfManufactureDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInstanceArrayOfManufactureDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ManufactureDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ManufactureDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ManufactureDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ManufactureDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInstanceArrayOfManufactureTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ManufactureTime>,
}

impl AsMut<ItemInstanceArrayOfManufactureTimeComponent> for ItemInstanceArrayOfManufactureTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInstanceArrayOfManufactureTimeComponent> for ItemInstanceArrayOfManufactureTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfManufactureTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfManufactureTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInstanceArrayOfManufactureTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ManufactureTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ManufactureTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ManufactureTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ManufactureTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInstanceArrayOfProductTraceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ProductTraceID>,
}

impl AsMut<ItemInstanceArrayOfProductTraceIDComponent> for ItemInstanceArrayOfProductTraceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInstanceArrayOfProductTraceIDComponent> for ItemInstanceArrayOfProductTraceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfProductTraceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfProductTraceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInstanceArrayOfProductTraceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ProductTraceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ProductTraceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ProductTraceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ProductTraceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInstanceArrayOfRegistrationIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RegistrationID>,
}

impl AsMut<ItemInstanceArrayOfRegistrationIDComponent> for ItemInstanceArrayOfRegistrationIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInstanceArrayOfRegistrationIDComponent> for ItemInstanceArrayOfRegistrationIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfRegistrationIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfRegistrationIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInstanceArrayOfRegistrationIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RegistrationID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RegistrationID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RegistrationID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RegistrationID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInstanceArrayOfSerialIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SerialID>,
}

impl AsMut<ItemInstanceArrayOfSerialIDComponent> for ItemInstanceArrayOfSerialIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInstanceArrayOfSerialIDComponent> for ItemInstanceArrayOfSerialIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfSerialIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemInstanceArrayOfSerialIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInstanceArrayOfSerialIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SerialID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SerialID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SerialID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SerialID> {
        self.items.iter()
    }
}


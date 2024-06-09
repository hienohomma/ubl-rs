use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeterReading {
    #[serde(rename = "DeliveredQuantity")]
    pub delivered_quantity: MeterReadingArrayOfDeliveredQuantityComponent,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<MeterReadingArrayOfIDComponent>,
    #[serde(rename = "LatestMeterQuantity")]
    pub latest_meter_quantity: MeterReadingArrayOfLatestMeterQuantityComponent,
    #[serde(rename = "LatestMeterReadingDate")]
    pub latest_meter_reading_date: MeterReadingArrayOfLatestMeterReadingDateComponent,
    #[serde(rename = "LatestMeterReadingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_meter_reading_method: Option<MeterReadingArrayOfLatestMeterReadingMethodComponent>,
    #[serde(rename = "LatestMeterReadingMethodCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_meter_reading_method_code: Option<MeterReadingArrayOfLatestMeterReadingMethodCodeComponent>,
    #[serde(rename = "MeterReadingComments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_reading_comments: Option<MeterReadingArrayOfMeterReadingCommentsComponent>,
    #[serde(rename = "MeterReadingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_reading_type: Option<MeterReadingArrayOfMeterReadingTypeComponent>,
    #[serde(rename = "MeterReadingTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_reading_type_code: Option<MeterReadingArrayOfMeterReadingTypeCodeComponent>,
    #[serde(rename = "PreviousMeterQuantity")]
    pub previous_meter_quantity: MeterReadingArrayOfPreviousMeterQuantityComponent,
    #[serde(rename = "PreviousMeterReadingDate")]
    pub previous_meter_reading_date: MeterReadingArrayOfPreviousMeterReadingDateComponent,
    #[serde(rename = "PreviousMeterReadingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_meter_reading_method: Option<MeterReadingArrayOfPreviousMeterReadingMethodComponent>,
    #[serde(rename = "PreviousMeterReadingMethodCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_meter_reading_method_code: Option<MeterReadingArrayOfPreviousMeterReadingMethodCodeComponent>,
}

impl AsMut<MeterReading> for MeterReading {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReading> for MeterReading {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.previous_meter_reading_method {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterReading.previous_meter_reading_method", e));
            }
        }
        if let Err(e) = self.delivered_quantity.validate() {
            return Err(UblError::component("MeterReading.delivered_quantity", e));
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterReading.id", e));
            }
        }
        if let Some(v) = &self.previous_meter_reading_method_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterReading.previous_meter_reading_method_code", e));
            }
        }
        if let Err(e) = self.latest_meter_quantity.validate() {
            return Err(UblError::component("MeterReading.latest_meter_quantity", e));
        }
        if let Err(e) = self.previous_meter_reading_date.validate() {
            return Err(UblError::component("MeterReading.previous_meter_reading_date", e));
        }
        if let Some(v) = &self.latest_meter_reading_method_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterReading.latest_meter_reading_method_code", e));
            }
        }
        if let Err(e) = self.latest_meter_reading_date.validate() {
            return Err(UblError::component("MeterReading.latest_meter_reading_date", e));
        }
        if let Some(v) = &self.latest_meter_reading_method {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterReading.latest_meter_reading_method", e));
            }
        }
        if let Some(v) = &self.meter_reading_comments {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterReading.meter_reading_comments", e));
            }
        }
        if let Some(v) = &self.meter_reading_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterReading.meter_reading_type", e));
            }
        }
        if let Some(v) = &self.meter_reading_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MeterReading.meter_reading_type_code", e));
            }
        }
        if let Err(e) = self.previous_meter_quantity.validate() {
            return Err(UblError::component("MeterReading.previous_meter_quantity", e));
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

impl MeterReading {
    pub fn title() -> &'static str {
        "Meter Reading. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a meter reading."
    }
    pub fn new(delivered_quantity: MeterReadingArrayOfDeliveredQuantityComponent, latest_meter_quantity: MeterReadingArrayOfLatestMeterQuantityComponent, latest_meter_reading_date: MeterReadingArrayOfLatestMeterReadingDateComponent, previous_meter_quantity: MeterReadingArrayOfPreviousMeterQuantityComponent, previous_meter_reading_date: MeterReadingArrayOfPreviousMeterReadingDateComponent) -> Component<Self> {
        Component(Self {
            delivered_quantity,
            meter_reading_type: None,
            meter_reading_type_code: None,
            previous_meter_reading_method_code: None,
            latest_meter_reading_method: None,
            previous_meter_reading_date,
            previous_meter_reading_method: None,
            meter_reading_comments: None,
            previous_meter_quantity,
            latest_meter_quantity,
            latest_meter_reading_method_code: None,
            latest_meter_reading_date,
            id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfDeliveredQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeliveredQuantity>,
}

impl AsMut<MeterReadingArrayOfDeliveredQuantityComponent> for MeterReadingArrayOfDeliveredQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfDeliveredQuantityComponent> for MeterReadingArrayOfDeliveredQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfDeliveredQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfDeliveredQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfDeliveredQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DeliveredQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DeliveredQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DeliveredQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DeliveredQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<MeterReadingArrayOfIDComponent> for MeterReadingArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfIDComponent> for MeterReadingArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfIDComponent {
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
pub struct MeterReadingArrayOfLatestMeterQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatestMeterQuantity>,
}

impl AsMut<MeterReadingArrayOfLatestMeterQuantityComponent> for MeterReadingArrayOfLatestMeterQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfLatestMeterQuantityComponent> for MeterReadingArrayOfLatestMeterQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfLatestMeterQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfLatestMeterQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfLatestMeterQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatestMeterQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatestMeterQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatestMeterQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatestMeterQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfLatestMeterReadingDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingDate>,
}

impl AsMut<MeterReadingArrayOfLatestMeterReadingDateComponent> for MeterReadingArrayOfLatestMeterReadingDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfLatestMeterReadingDateComponent> for MeterReadingArrayOfLatestMeterReadingDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfLatestMeterReadingDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfLatestMeterReadingDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfLatestMeterReadingDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfLatestMeterReadingMethodComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingMethod>,
}

impl AsMut<MeterReadingArrayOfLatestMeterReadingMethodComponent> for MeterReadingArrayOfLatestMeterReadingMethodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfLatestMeterReadingMethodComponent> for MeterReadingArrayOfLatestMeterReadingMethodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfLatestMeterReadingMethodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfLatestMeterReadingMethodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfLatestMeterReadingMethodComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingMethod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingMethod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingMethod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingMethod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfLatestMeterReadingMethodCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingMethodCode>,
}

impl AsMut<MeterReadingArrayOfLatestMeterReadingMethodCodeComponent> for MeterReadingArrayOfLatestMeterReadingMethodCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfLatestMeterReadingMethodCodeComponent> for MeterReadingArrayOfLatestMeterReadingMethodCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfLatestMeterReadingMethodCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfLatestMeterReadingMethodCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfLatestMeterReadingMethodCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingMethodCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingMethodCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingMethodCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatestMeterReadingMethodCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfMeterReadingCommentsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MeterReadingComments>,
}

impl AsMut<MeterReadingArrayOfMeterReadingCommentsComponent> for MeterReadingArrayOfMeterReadingCommentsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfMeterReadingCommentsComponent> for MeterReadingArrayOfMeterReadingCommentsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfMeterReadingCommentsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfMeterReadingCommentsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MeterReadingComments) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MeterReadingComments) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MeterReadingComments> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MeterReadingComments> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfMeterReadingTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MeterReadingType>,
}

impl AsMut<MeterReadingArrayOfMeterReadingTypeComponent> for MeterReadingArrayOfMeterReadingTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfMeterReadingTypeComponent> for MeterReadingArrayOfMeterReadingTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfMeterReadingTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfMeterReadingTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfMeterReadingTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MeterReadingType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MeterReadingType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MeterReadingType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MeterReadingType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfMeterReadingTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MeterReadingTypeCode>,
}

impl AsMut<MeterReadingArrayOfMeterReadingTypeCodeComponent> for MeterReadingArrayOfMeterReadingTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfMeterReadingTypeCodeComponent> for MeterReadingArrayOfMeterReadingTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfMeterReadingTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfMeterReadingTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfMeterReadingTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MeterReadingTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MeterReadingTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MeterReadingTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MeterReadingTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfPreviousMeterQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PreviousMeterQuantity>,
}

impl AsMut<MeterReadingArrayOfPreviousMeterQuantityComponent> for MeterReadingArrayOfPreviousMeterQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfPreviousMeterQuantityComponent> for MeterReadingArrayOfPreviousMeterQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfPreviousMeterQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfPreviousMeterQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfPreviousMeterQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PreviousMeterQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PreviousMeterQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PreviousMeterQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PreviousMeterQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfPreviousMeterReadingDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingDate>,
}

impl AsMut<MeterReadingArrayOfPreviousMeterReadingDateComponent> for MeterReadingArrayOfPreviousMeterReadingDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfPreviousMeterReadingDateComponent> for MeterReadingArrayOfPreviousMeterReadingDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfPreviousMeterReadingDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfPreviousMeterReadingDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfPreviousMeterReadingDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfPreviousMeterReadingMethodComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingMethod>,
}

impl AsMut<MeterReadingArrayOfPreviousMeterReadingMethodComponent> for MeterReadingArrayOfPreviousMeterReadingMethodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfPreviousMeterReadingMethodComponent> for MeterReadingArrayOfPreviousMeterReadingMethodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfPreviousMeterReadingMethodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfPreviousMeterReadingMethodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfPreviousMeterReadingMethodComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingMethod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingMethod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingMethod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingMethod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterReadingArrayOfPreviousMeterReadingMethodCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingMethodCode>,
}

impl AsMut<MeterReadingArrayOfPreviousMeterReadingMethodCodeComponent> for MeterReadingArrayOfPreviousMeterReadingMethodCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterReadingArrayOfPreviousMeterReadingMethodCodeComponent> for MeterReadingArrayOfPreviousMeterReadingMethodCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfPreviousMeterReadingMethodCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterReadingArrayOfPreviousMeterReadingMethodCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterReadingArrayOfPreviousMeterReadingMethodCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingMethodCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingMethodCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingMethodCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PreviousMeterReadingMethodCode> {
        self.items.iter()
    }
}


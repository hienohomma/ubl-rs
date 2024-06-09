use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Meter {
    #[serde(rename = "MeterConstant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_constant: Option<MeterArrayOfMeterConstantComponent>,
    #[serde(rename = "MeterConstantCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_constant_code: Option<MeterArrayOfMeterConstantCodeComponent>,
    #[serde(rename = "MeterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_name: Option<MeterArrayOfMeterNameComponent>,
    #[serde(rename = "MeterNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_number: Option<MeterArrayOfMeterNumberComponent>,
    #[serde(rename = "MeterProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_property: Option<MeterArrayOfMeterPropertyComponent>,
    #[serde(rename = "MeterReading")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_reading: Option<MeterArrayOfMeterReadingComponent>,
    #[serde(rename = "TotalDeliveredQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_delivered_quantity: Option<MeterArrayOfTotalDeliveredQuantityComponent>,
}

impl AsMut<Meter> for Meter {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Meter> for Meter {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.meter_number {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Meter.meter_number", e));
            }
        }
        if let Some(v) = &self.total_delivered_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Meter.total_delivered_quantity", e));
            }
        }
        if let Some(v) = &self.meter_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Meter.meter_name", e));
            }
        }
        if let Some(v) = &self.meter_property {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Meter.meter_property", e));
            }
        }
        if let Some(v) = &self.meter_constant {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Meter.meter_constant", e));
            }
        }
        if let Some(v) = &self.meter_reading {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Meter.meter_reading", e));
            }
        }
        if let Some(v) = &self.meter_constant_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Meter.meter_constant_code", e));
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

impl Meter {
    pub fn title() -> &'static str {
        "Meter. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a meter and its readings."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            meter_constant_code: None,
            meter_name: None,
            meter_property: None,
            meter_reading: None,
            total_delivered_quantity: None,
            meter_constant: None,
            meter_number: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterArrayOfMeterConstantComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MeterConstant>,
}

impl AsMut<MeterArrayOfMeterConstantComponent> for MeterArrayOfMeterConstantComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterArrayOfMeterConstantComponent> for MeterArrayOfMeterConstantComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterArrayOfMeterConstantComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterArrayOfMeterConstantComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterArrayOfMeterConstantComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MeterConstant) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MeterConstant) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MeterConstant> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MeterConstant> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterArrayOfMeterConstantCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MeterConstantCode>,
}

impl AsMut<MeterArrayOfMeterConstantCodeComponent> for MeterArrayOfMeterConstantCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterArrayOfMeterConstantCodeComponent> for MeterArrayOfMeterConstantCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterArrayOfMeterConstantCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterArrayOfMeterConstantCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterArrayOfMeterConstantCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MeterConstantCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MeterConstantCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MeterConstantCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MeterConstantCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterArrayOfMeterNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MeterName>,
}

impl AsMut<MeterArrayOfMeterNameComponent> for MeterArrayOfMeterNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterArrayOfMeterNameComponent> for MeterArrayOfMeterNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterArrayOfMeterNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterArrayOfMeterNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterArrayOfMeterNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MeterName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MeterName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MeterName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MeterName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterArrayOfMeterNumberComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MeterNumber>,
}

impl AsMut<MeterArrayOfMeterNumberComponent> for MeterArrayOfMeterNumberComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterArrayOfMeterNumberComponent> for MeterArrayOfMeterNumberComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterArrayOfMeterNumberComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterArrayOfMeterNumberComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterArrayOfMeterNumberComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MeterNumber) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MeterNumber) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MeterNumber> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MeterNumber> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterArrayOfMeterPropertyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MeterProperty>,
}

impl AsMut<MeterArrayOfMeterPropertyComponent> for MeterArrayOfMeterPropertyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterArrayOfMeterPropertyComponent> for MeterArrayOfMeterPropertyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("MeterArrayOfMeterPropertyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterArrayOfMeterPropertyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MeterProperty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MeterProperty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MeterProperty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MeterProperty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterArrayOfMeterReadingComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MeterReading>,
}

impl AsMut<MeterArrayOfMeterReadingComponent> for MeterArrayOfMeterReadingComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterArrayOfMeterReadingComponent> for MeterArrayOfMeterReadingComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("MeterArrayOfMeterReadingComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterArrayOfMeterReadingComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MeterReading) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MeterReading) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MeterReading> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MeterReading> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MeterArrayOfTotalDeliveredQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalDeliveredQuantity>,
}

impl AsMut<MeterArrayOfTotalDeliveredQuantityComponent> for MeterArrayOfTotalDeliveredQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MeterArrayOfTotalDeliveredQuantityComponent> for MeterArrayOfTotalDeliveredQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MeterArrayOfTotalDeliveredQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MeterArrayOfTotalDeliveredQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MeterArrayOfTotalDeliveredQuantityComponent {
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


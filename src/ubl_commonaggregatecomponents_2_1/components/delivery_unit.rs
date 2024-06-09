use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeliveryUnit {
    #[serde(rename = "BatchQuantity")]
    pub batch_quantity: DeliveryUnitArrayOfBatchQuantityComponent,
    #[serde(rename = "ConsumerUnitQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_unit_quantity: Option<DeliveryUnitArrayOfConsumerUnitQuantityComponent>,
    #[serde(rename = "HazardousRiskIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_risk_indicator: Option<DeliveryUnitArrayOfHazardousRiskIndicatorComponent>,
}

impl AsMut<DeliveryUnit> for DeliveryUnit {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryUnit> for DeliveryUnit {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.batch_quantity.validate() {
            return Err(UblError::component("DeliveryUnit.batch_quantity", e));
        }
        if let Some(v) = &self.consumer_unit_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DeliveryUnit.consumer_unit_quantity", e));
            }
        }
        if let Some(v) = &self.hazardous_risk_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DeliveryUnit.hazardous_risk_indicator", e));
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

impl DeliveryUnit {
    pub fn title() -> &'static str {
        "Delivery Unit. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a delivery unit."
    }
    pub fn new(batch_quantity: DeliveryUnitArrayOfBatchQuantityComponent) -> Component<Self> {
        Component(Self {
            hazardous_risk_indicator: None,
            batch_quantity,
            consumer_unit_quantity: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryUnitArrayOfBatchQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BatchQuantity>,
}

impl AsMut<DeliveryUnitArrayOfBatchQuantityComponent> for DeliveryUnitArrayOfBatchQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryUnitArrayOfBatchQuantityComponent> for DeliveryUnitArrayOfBatchQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryUnitArrayOfBatchQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryUnitArrayOfBatchQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeliveryUnitArrayOfBatchQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BatchQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BatchQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BatchQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BatchQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryUnitArrayOfConsumerUnitQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumerUnitQuantity>,
}

impl AsMut<DeliveryUnitArrayOfConsumerUnitQuantityComponent> for DeliveryUnitArrayOfConsumerUnitQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryUnitArrayOfConsumerUnitQuantityComponent> for DeliveryUnitArrayOfConsumerUnitQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryUnitArrayOfConsumerUnitQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryUnitArrayOfConsumerUnitQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeliveryUnitArrayOfConsumerUnitQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumerUnitQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumerUnitQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumerUnitQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumerUnitQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryUnitArrayOfHazardousRiskIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator>,
}

impl AsMut<DeliveryUnitArrayOfHazardousRiskIndicatorComponent> for DeliveryUnitArrayOfHazardousRiskIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryUnitArrayOfHazardousRiskIndicatorComponent> for DeliveryUnitArrayOfHazardousRiskIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryUnitArrayOfHazardousRiskIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryUnitArrayOfHazardousRiskIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeliveryUnitArrayOfHazardousRiskIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator> {
        self.items.iter()
    }
}


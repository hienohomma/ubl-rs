use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderedShipment {
    #[serde(rename = "Package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<OrderedShipmentArrayOfPackageComponent>,
    #[serde(rename = "Shipment")]
    pub shipment: OrderedShipmentArrayOfShipmentComponent,
}

impl AsMut<OrderedShipment> for OrderedShipment {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderedShipment> for OrderedShipment {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.shipment.validate() {
            return Err(UblError::component("OrderedShipment.shipment", e));
        }
        if let Some(v) = &self.package {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderedShipment.package", e));
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

impl OrderedShipment {
    pub fn title() -> &'static str {
        "Ordered Shipment. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an ordered shipment."
    }
    pub fn new(shipment: OrderedShipmentArrayOfShipmentComponent) -> Component<Self> {
        Component(Self {
            package: None,
            shipment,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderedShipmentArrayOfPackageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Package>,
}

impl AsMut<OrderedShipmentArrayOfPackageComponent> for OrderedShipmentArrayOfPackageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderedShipmentArrayOfPackageComponent> for OrderedShipmentArrayOfPackageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("OrderedShipmentArrayOfPackageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderedShipmentArrayOfPackageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Package) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Package) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Package> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Package> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderedShipmentArrayOfShipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Shipment>,
}

impl AsMut<OrderedShipmentArrayOfShipmentComponent> for OrderedShipmentArrayOfShipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderedShipmentArrayOfShipmentComponent> for OrderedShipmentArrayOfShipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderedShipmentArrayOfShipmentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderedShipmentArrayOfShipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderedShipmentArrayOfShipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Shipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Shipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Shipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Shipment> {
        self.items.iter()
    }
}


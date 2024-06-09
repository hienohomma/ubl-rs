use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GoodsItemContainer {
    #[serde(rename = "ID")]
    pub id: GoodsItemContainerArrayOfIDComponent,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<GoodsItemContainerArrayOfQuantityComponent>,
    #[serde(rename = "TransportEquipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_equipment: Option<GoodsItemContainerArrayOfTransportEquipmentComponent>,
}

impl AsMut<GoodsItemContainer> for GoodsItemContainer {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemContainer> for GoodsItemContainer {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItemContainer.quantity", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("GoodsItemContainer.id", e));
        }
        if let Some(v) = &self.transport_equipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItemContainer.transport_equipment", e));
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

impl GoodsItemContainer {
    pub fn title() -> &'static str {
        "Goods Item Container. Details"
    }
    pub fn description() -> &'static str {
        "A class defining how goods items are split across transport equipment."
    }
    pub fn new(id: GoodsItemContainerArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            transport_equipment: None,
            id,
            quantity: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemContainerArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<GoodsItemContainerArrayOfIDComponent> for GoodsItemContainerArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemContainerArrayOfIDComponent> for GoodsItemContainerArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemContainerArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemContainerArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemContainerArrayOfIDComponent {
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
pub struct GoodsItemContainerArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<GoodsItemContainerArrayOfQuantityComponent> for GoodsItemContainerArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemContainerArrayOfQuantityComponent> for GoodsItemContainerArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemContainerArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemContainerArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemContainerArrayOfQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Quantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Quantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemContainerArrayOfTransportEquipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportEquipment>,
}

impl AsMut<GoodsItemContainerArrayOfTransportEquipmentComponent> for GoodsItemContainerArrayOfTransportEquipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemContainerArrayOfTransportEquipmentComponent> for GoodsItemContainerArrayOfTransportEquipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("GoodsItemContainerArrayOfTransportEquipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemContainerArrayOfTransportEquipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportEquipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportEquipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportEquipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportEquipment> {
        self.items.iter()
    }
}


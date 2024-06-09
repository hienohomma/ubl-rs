use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderLineReference {
    #[serde(rename = "LineID")]
    pub line_id: OrderLineReferenceArrayOfLineIDComponent,
    #[serde(rename = "LineStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<OrderLineReferenceArrayOfLineStatusCodeComponent>,
    #[serde(rename = "OrderReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_reference: Option<OrderLineReferenceArrayOfOrderReferenceComponent>,
    #[serde(rename = "SalesOrderLineID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_order_line_id: Option<OrderLineReferenceArrayOfSalesOrderLineIDComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<OrderLineReferenceArrayOfUUIDComponent>,
}

impl AsMut<OrderLineReference> for OrderLineReference {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineReference> for OrderLineReference {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLineReference.uuid", e));
            }
        }
        if let Some(v) = &self.line_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLineReference.line_status_code", e));
            }
        }
        if let Some(v) = &self.order_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLineReference.order_reference", e));
            }
        }
        if let Some(v) = &self.sales_order_line_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLineReference.sales_order_line_id", e));
            }
        }
        if let Err(e) = self.line_id.validate() {
            return Err(UblError::component("OrderLineReference.line_id", e));
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

impl OrderLineReference {
    pub fn title() -> &'static str {
        "Order Line Reference. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a reference to an order line."
    }
    pub fn new(line_id: OrderLineReferenceArrayOfLineIDComponent) -> Component<Self> {
        Component(Self {
            order_reference: None,
            uuid: None,
            line_id,
            line_status_code: None,
            sales_order_line_id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineReferenceArrayOfLineIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineID>,
}

impl AsMut<OrderLineReferenceArrayOfLineIDComponent> for OrderLineReferenceArrayOfLineIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineReferenceArrayOfLineIDComponent> for OrderLineReferenceArrayOfLineIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderLineReferenceArrayOfLineIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderLineReferenceArrayOfLineIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineReferenceArrayOfLineIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LineID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LineID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LineID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LineID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineReferenceArrayOfLineStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineStatusCode>,
}

impl AsMut<OrderLineReferenceArrayOfLineStatusCodeComponent> for OrderLineReferenceArrayOfLineStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineReferenceArrayOfLineStatusCodeComponent> for OrderLineReferenceArrayOfLineStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderLineReferenceArrayOfLineStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderLineReferenceArrayOfLineStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineReferenceArrayOfLineStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LineStatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LineStatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LineStatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LineStatusCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineReferenceArrayOfOrderReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OrderReference>,
}

impl AsMut<OrderLineReferenceArrayOfOrderReferenceComponent> for OrderLineReferenceArrayOfOrderReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineReferenceArrayOfOrderReferenceComponent> for OrderLineReferenceArrayOfOrderReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderLineReferenceArrayOfOrderReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderLineReferenceArrayOfOrderReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineReferenceArrayOfOrderReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OrderReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OrderReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OrderReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OrderReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineReferenceArrayOfSalesOrderLineIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SalesOrderLineID>,
}

impl AsMut<OrderLineReferenceArrayOfSalesOrderLineIDComponent> for OrderLineReferenceArrayOfSalesOrderLineIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineReferenceArrayOfSalesOrderLineIDComponent> for OrderLineReferenceArrayOfSalesOrderLineIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderLineReferenceArrayOfSalesOrderLineIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderLineReferenceArrayOfSalesOrderLineIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineReferenceArrayOfSalesOrderLineIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SalesOrderLineID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SalesOrderLineID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SalesOrderLineID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SalesOrderLineID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineReferenceArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<OrderLineReferenceArrayOfUUIDComponent> for OrderLineReferenceArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineReferenceArrayOfUUIDComponent> for OrderLineReferenceArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderLineReferenceArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderLineReferenceArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineReferenceArrayOfUUIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::UUID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::UUID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::UUID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::UUID> {
        self.items.iter()
    }
}


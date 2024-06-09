use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderReference {
    #[serde(rename = "CopyIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<OrderReferenceArrayOfCopyIndicatorComponent>,
    #[serde(rename = "CustomerReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<OrderReferenceArrayOfCustomerReferenceComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<OrderReferenceArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ID")]
    pub id: OrderReferenceArrayOfIDComponent,
    #[serde(rename = "IssueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<OrderReferenceArrayOfIssueDateComponent>,
    #[serde(rename = "IssueTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<OrderReferenceArrayOfIssueTimeComponent>,
    #[serde(rename = "OrderTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type_code: Option<OrderReferenceArrayOfOrderTypeCodeComponent>,
    #[serde(rename = "SalesOrderID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_order_id: Option<OrderReferenceArrayOfSalesOrderIDComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<OrderReferenceArrayOfUUIDComponent>,
}

impl AsMut<OrderReference> for OrderReference {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderReference> for OrderReference {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.order_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderReference.order_type_code", e));
            }
        }
        if let Some(v) = &self.copy_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderReference.copy_indicator", e));
            }
        }
        if let Some(v) = &self.customer_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderReference.customer_reference", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("OrderReference.id", e));
        }
        if let Some(v) = &self.issue_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderReference.issue_date", e));
            }
        }
        if let Some(v) = &self.issue_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderReference.issue_time", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderReference.uuid", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderReference.document_reference", e));
            }
        }
        if let Some(v) = &self.sales_order_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderReference.sales_order_id", e));
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

impl OrderReference {
    pub fn title() -> &'static str {
        "Order Reference. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a reference to an Order."
    }
    pub fn new(id: OrderReferenceArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            order_type_code: None,
            issue_time: None,
            copy_indicator: None,
            customer_reference: None,
            document_reference: None,
            issue_date: None,
            id,
            sales_order_id: None,
            uuid: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderReferenceArrayOfCopyIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CopyIndicator>,
}

impl AsMut<OrderReferenceArrayOfCopyIndicatorComponent> for OrderReferenceArrayOfCopyIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderReferenceArrayOfCopyIndicatorComponent> for OrderReferenceArrayOfCopyIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfCopyIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfCopyIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderReferenceArrayOfCopyIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CopyIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CopyIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CopyIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CopyIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderReferenceArrayOfCustomerReferenceComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CustomerReference>,
}

impl AsMut<OrderReferenceArrayOfCustomerReferenceComponent> for OrderReferenceArrayOfCustomerReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderReferenceArrayOfCustomerReferenceComponent> for OrderReferenceArrayOfCustomerReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfCustomerReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfCustomerReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderReferenceArrayOfCustomerReferenceComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CustomerReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CustomerReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CustomerReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CustomerReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderReferenceArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<OrderReferenceArrayOfDocumentReferenceComponent> for OrderReferenceArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderReferenceArrayOfDocumentReferenceComponent> for OrderReferenceArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderReferenceArrayOfDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderReferenceArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<OrderReferenceArrayOfIDComponent> for OrderReferenceArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderReferenceArrayOfIDComponent> for OrderReferenceArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderReferenceArrayOfIDComponent {
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
pub struct OrderReferenceArrayOfIssueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueDate>,
}

impl AsMut<OrderReferenceArrayOfIssueDateComponent> for OrderReferenceArrayOfIssueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderReferenceArrayOfIssueDateComponent> for OrderReferenceArrayOfIssueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfIssueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfIssueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderReferenceArrayOfIssueDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IssueDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IssueDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IssueDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IssueDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderReferenceArrayOfIssueTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueTime>,
}

impl AsMut<OrderReferenceArrayOfIssueTimeComponent> for OrderReferenceArrayOfIssueTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderReferenceArrayOfIssueTimeComponent> for OrderReferenceArrayOfIssueTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfIssueTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfIssueTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderReferenceArrayOfIssueTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IssueTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IssueTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IssueTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IssueTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderReferenceArrayOfOrderTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OrderTypeCode>,
}

impl AsMut<OrderReferenceArrayOfOrderTypeCodeComponent> for OrderReferenceArrayOfOrderTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderReferenceArrayOfOrderTypeCodeComponent> for OrderReferenceArrayOfOrderTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfOrderTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfOrderTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderReferenceArrayOfOrderTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OrderTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OrderTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OrderTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OrderTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderReferenceArrayOfSalesOrderIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SalesOrderID>,
}

impl AsMut<OrderReferenceArrayOfSalesOrderIDComponent> for OrderReferenceArrayOfSalesOrderIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderReferenceArrayOfSalesOrderIDComponent> for OrderReferenceArrayOfSalesOrderIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfSalesOrderIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfSalesOrderIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderReferenceArrayOfSalesOrderIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SalesOrderID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SalesOrderID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SalesOrderID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SalesOrderID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderReferenceArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<OrderReferenceArrayOfUUIDComponent> for OrderReferenceArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderReferenceArrayOfUUIDComponent> for OrderReferenceArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderReferenceArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderReferenceArrayOfUUIDComponent {
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


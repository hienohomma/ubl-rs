use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DespatchLine {
    #[serde(rename = "BackorderQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backorder_quantity: Option<DespatchLineArrayOfBackorderQuantityComponent>,
    #[serde(rename = "BackorderReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backorder_reason: Option<DespatchLineArrayOfBackorderReasonComponent>,
    #[serde(rename = "DeliveredQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_quantity: Option<DespatchLineArrayOfDeliveredQuantityComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<DespatchLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ID")]
    pub id: DespatchLineArrayOfIDComponent,
    #[serde(rename = "Item")]
    pub item: DespatchLineArrayOfItemComponent,
    #[serde(rename = "LineStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<DespatchLineArrayOfLineStatusCodeComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<DespatchLineArrayOfNoteComponent>,
    #[serde(rename = "OrderLineReference")]
    pub order_line_reference: DespatchLineArrayOfOrderLineReferenceComponent,
    #[serde(rename = "OutstandingQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outstanding_quantity: Option<DespatchLineArrayOfOutstandingQuantityComponent>,
    #[serde(rename = "OutstandingReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outstanding_reason: Option<DespatchLineArrayOfOutstandingReasonComponent>,
    #[serde(rename = "OversupplyQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oversupply_quantity: Option<DespatchLineArrayOfOversupplyQuantityComponent>,
    #[serde(rename = "Shipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment: Option<DespatchLineArrayOfShipmentComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<DespatchLineArrayOfUUIDComponent>,
}

impl AsMut<DespatchLine> for DespatchLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLine> for DespatchLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DespatchLine.document_reference", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DespatchLine.uuid", e));
            }
        }
        if let Some(v) = &self.backorder_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DespatchLine.backorder_quantity", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DespatchLine.note", e));
            }
        }
        if let Some(v) = &self.outstanding_reason {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DespatchLine.outstanding_reason", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("DespatchLine.id", e));
        }
        if let Some(v) = &self.backorder_reason {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DespatchLine.backorder_reason", e));
            }
        }
        if let Some(v) = &self.delivered_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DespatchLine.delivered_quantity", e));
            }
        }
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("DespatchLine.item", e));
        }
        if let Some(v) = &self.line_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DespatchLine.line_status_code", e));
            }
        }
        if let Err(e) = self.order_line_reference.validate() {
            return Err(UblError::component("DespatchLine.order_line_reference", e));
        }
        if let Some(v) = &self.oversupply_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DespatchLine.oversupply_quantity", e));
            }
        }
        if let Some(v) = &self.shipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DespatchLine.shipment", e));
            }
        }
        if let Some(v) = &self.outstanding_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DespatchLine.outstanding_quantity", e));
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

impl DespatchLine {
    pub fn title() -> &'static str {
        "Despatch Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Despatch Advice."
    }
    pub fn new(id: DespatchLineArrayOfIDComponent, item: DespatchLineArrayOfItemComponent, order_line_reference: DespatchLineArrayOfOrderLineReferenceComponent) -> Component<Self> {
        Component(Self {
            line_status_code: None,
            note: None,
            uuid: None,
            oversupply_quantity: None,
            backorder_quantity: None,
            shipment: None,
            delivered_quantity: None,
            document_reference: None,
            id,
            item,
            order_line_reference,
            outstanding_quantity: None,
            outstanding_reason: None,
            backorder_reason: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchLineArrayOfBackorderQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BackorderQuantity>,
}

impl AsMut<DespatchLineArrayOfBackorderQuantityComponent> for DespatchLineArrayOfBackorderQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfBackorderQuantityComponent> for DespatchLineArrayOfBackorderQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfBackorderQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfBackorderQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfBackorderQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BackorderQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BackorderQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BackorderQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BackorderQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchLineArrayOfBackorderReasonComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BackorderReason>,
}

impl AsMut<DespatchLineArrayOfBackorderReasonComponent> for DespatchLineArrayOfBackorderReasonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfBackorderReasonComponent> for DespatchLineArrayOfBackorderReasonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfBackorderReasonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfBackorderReasonComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BackorderReason) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BackorderReason) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BackorderReason> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BackorderReason> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchLineArrayOfDeliveredQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeliveredQuantity>,
}

impl AsMut<DespatchLineArrayOfDeliveredQuantityComponent> for DespatchLineArrayOfDeliveredQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfDeliveredQuantityComponent> for DespatchLineArrayOfDeliveredQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfDeliveredQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfDeliveredQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfDeliveredQuantityComponent {
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
pub struct DespatchLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<DespatchLineArrayOfDocumentReferenceComponent> for DespatchLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfDocumentReferenceComponent> for DespatchLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfDocumentReferenceComponent {
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
pub struct DespatchLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<DespatchLineArrayOfIDComponent> for DespatchLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfIDComponent> for DespatchLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfIDComponent {
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
pub struct DespatchLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<DespatchLineArrayOfItemComponent> for DespatchLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfItemComponent> for DespatchLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Item) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Item) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchLineArrayOfLineStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineStatusCode>,
}

impl AsMut<DespatchLineArrayOfLineStatusCodeComponent> for DespatchLineArrayOfLineStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfLineStatusCodeComponent> for DespatchLineArrayOfLineStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfLineStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfLineStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfLineStatusCodeComponent {
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
pub struct DespatchLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<DespatchLineArrayOfNoteComponent> for DespatchLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfNoteComponent> for DespatchLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfNoteComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Note) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Note) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchLineArrayOfOrderLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OrderLineReference>,
}

impl AsMut<DespatchLineArrayOfOrderLineReferenceComponent> for DespatchLineArrayOfOrderLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfOrderLineReferenceComponent> for DespatchLineArrayOfOrderLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfOrderLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfOrderLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OrderLineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OrderLineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OrderLineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OrderLineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchLineArrayOfOutstandingQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OutstandingQuantity>,
}

impl AsMut<DespatchLineArrayOfOutstandingQuantityComponent> for DespatchLineArrayOfOutstandingQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfOutstandingQuantityComponent> for DespatchLineArrayOfOutstandingQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfOutstandingQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfOutstandingQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfOutstandingQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OutstandingQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OutstandingQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OutstandingQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OutstandingQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchLineArrayOfOutstandingReasonComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OutstandingReason>,
}

impl AsMut<DespatchLineArrayOfOutstandingReasonComponent> for DespatchLineArrayOfOutstandingReasonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfOutstandingReasonComponent> for DespatchLineArrayOfOutstandingReasonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfOutstandingReasonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfOutstandingReasonComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OutstandingReason) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OutstandingReason) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OutstandingReason> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OutstandingReason> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchLineArrayOfOversupplyQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OversupplyQuantity>,
}

impl AsMut<DespatchLineArrayOfOversupplyQuantityComponent> for DespatchLineArrayOfOversupplyQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfOversupplyQuantityComponent> for DespatchLineArrayOfOversupplyQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfOversupplyQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfOversupplyQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfOversupplyQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OversupplyQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OversupplyQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OversupplyQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OversupplyQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchLineArrayOfShipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Shipment>,
}

impl AsMut<DespatchLineArrayOfShipmentComponent> for DespatchLineArrayOfShipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfShipmentComponent> for DespatchLineArrayOfShipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfShipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfShipmentComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DespatchLineArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<DespatchLineArrayOfUUIDComponent> for DespatchLineArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DespatchLineArrayOfUUIDComponent> for DespatchLineArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DespatchLineArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DespatchLineArrayOfUUIDComponent {
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


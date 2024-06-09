use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReceiptLine {
    #[serde(rename = "DespatchLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_line_reference: Option<ReceiptLineArrayOfDespatchLineReferenceComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ReceiptLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ID")]
    pub id: ReceiptLineArrayOfIDComponent,
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<ReceiptLineArrayOfItemComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<ReceiptLineArrayOfNoteComponent>,
    #[serde(rename = "OrderLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_line_reference: Option<ReceiptLineArrayOfOrderLineReferenceComponent>,
    #[serde(rename = "OversupplyQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oversupply_quantity: Option<ReceiptLineArrayOfOversupplyQuantityComponent>,
    #[serde(rename = "QuantityDiscrepancyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_discrepancy_code: Option<ReceiptLineArrayOfQuantityDiscrepancyCodeComponent>,
    #[serde(rename = "ReceivedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_date: Option<ReceiptLineArrayOfReceivedDateComponent>,
    #[serde(rename = "ReceivedQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_quantity: Option<ReceiptLineArrayOfReceivedQuantityComponent>,
    #[serde(rename = "RejectActionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_action_code: Option<ReceiptLineArrayOfRejectActionCodeComponent>,
    #[serde(rename = "RejectReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<ReceiptLineArrayOfRejectReasonComponent>,
    #[serde(rename = "RejectReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason_code: Option<ReceiptLineArrayOfRejectReasonCodeComponent>,
    #[serde(rename = "RejectedQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_quantity: Option<ReceiptLineArrayOfRejectedQuantityComponent>,
    #[serde(rename = "Shipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment: Option<ReceiptLineArrayOfShipmentComponent>,
    #[serde(rename = "ShortQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_quantity: Option<ReceiptLineArrayOfShortQuantityComponent>,
    #[serde(rename = "ShortageActionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortage_action_code: Option<ReceiptLineArrayOfShortageActionCodeComponent>,
    #[serde(rename = "TimingComplaint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_complaint: Option<ReceiptLineArrayOfTimingComplaintComponent>,
    #[serde(rename = "TimingComplaintCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_complaint_code: Option<ReceiptLineArrayOfTimingComplaintCodeComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ReceiptLineArrayOfUUIDComponent>,
}

impl AsMut<ReceiptLine> for ReceiptLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLine> for ReceiptLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.reject_reason_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.reject_reason_code", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ReceiptLine.id", e));
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.document_reference", e));
            }
        }
        if let Some(v) = &self.shipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.shipment", e));
            }
        }
        if let Some(v) = &self.short_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.short_quantity", e));
            }
        }
        if let Some(v) = &self.timing_complaint_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.timing_complaint_code", e));
            }
        }
        if let Some(v) = &self.shortage_action_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.shortage_action_code", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.note", e));
            }
        }
        if let Some(v) = &self.received_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.received_date", e));
            }
        }
        if let Some(v) = &self.reject_reason {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.reject_reason", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.uuid", e));
            }
        }
        if let Some(v) = &self.order_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.order_line_reference", e));
            }
        }
        if let Some(v) = &self.oversupply_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.oversupply_quantity", e));
            }
        }
        if let Some(v) = &self.item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.item", e));
            }
        }
        if let Some(v) = &self.reject_action_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.reject_action_code", e));
            }
        }
        if let Some(v) = &self.timing_complaint {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.timing_complaint", e));
            }
        }
        if let Some(v) = &self.quantity_discrepancy_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.quantity_discrepancy_code", e));
            }
        }
        if let Some(v) = &self.despatch_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.despatch_line_reference", e));
            }
        }
        if let Some(v) = &self.received_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.received_quantity", e));
            }
        }
        if let Some(v) = &self.rejected_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ReceiptLine.rejected_quantity", e));
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

impl ReceiptLine {
    pub fn title() -> &'static str {
        "Receipt Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Receipt Advice."
    }
    pub fn new(id: ReceiptLineArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            shipment: None,
            received_date: None,
            despatch_line_reference: None,
            reject_action_code: None,
            item: None,
            quantity_discrepancy_code: None,
            short_quantity: None,
            timing_complaint: None,
            reject_reason_code: None,
            document_reference: None,
            shortage_action_code: None,
            uuid: None,
            reject_reason: None,
            rejected_quantity: None,
            received_quantity: None,
            note: None,
            order_line_reference: None,
            oversupply_quantity: None,
            id,
            timing_complaint_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfDespatchLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference>,
}

impl AsMut<ReceiptLineArrayOfDespatchLineReferenceComponent> for ReceiptLineArrayOfDespatchLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfDespatchLineReferenceComponent> for ReceiptLineArrayOfDespatchLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfDespatchLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfDespatchLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DespatchLineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<ReceiptLineArrayOfDocumentReferenceComponent> for ReceiptLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfDocumentReferenceComponent> for ReceiptLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfDocumentReferenceComponent {
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
pub struct ReceiptLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ReceiptLineArrayOfIDComponent> for ReceiptLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfIDComponent> for ReceiptLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfIDComponent {
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
pub struct ReceiptLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<ReceiptLineArrayOfItemComponent> for ReceiptLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfItemComponent> for ReceiptLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfItemComponent {
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
pub struct ReceiptLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<ReceiptLineArrayOfNoteComponent> for ReceiptLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfNoteComponent> for ReceiptLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfNoteComponent {
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
pub struct ReceiptLineArrayOfOrderLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OrderLineReference>,
}

impl AsMut<ReceiptLineArrayOfOrderLineReferenceComponent> for ReceiptLineArrayOfOrderLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfOrderLineReferenceComponent> for ReceiptLineArrayOfOrderLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfOrderLineReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfOrderLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfOrderLineReferenceComponent {
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
pub struct ReceiptLineArrayOfOversupplyQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OversupplyQuantity>,
}

impl AsMut<ReceiptLineArrayOfOversupplyQuantityComponent> for ReceiptLineArrayOfOversupplyQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfOversupplyQuantityComponent> for ReceiptLineArrayOfOversupplyQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfOversupplyQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfOversupplyQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfOversupplyQuantityComponent {
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
pub struct ReceiptLineArrayOfQuantityDiscrepancyCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::QuantityDiscrepancyCode>,
}

impl AsMut<ReceiptLineArrayOfQuantityDiscrepancyCodeComponent> for ReceiptLineArrayOfQuantityDiscrepancyCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfQuantityDiscrepancyCodeComponent> for ReceiptLineArrayOfQuantityDiscrepancyCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfQuantityDiscrepancyCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfQuantityDiscrepancyCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfQuantityDiscrepancyCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::QuantityDiscrepancyCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::QuantityDiscrepancyCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::QuantityDiscrepancyCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::QuantityDiscrepancyCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfReceivedDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReceivedDate>,
}

impl AsMut<ReceiptLineArrayOfReceivedDateComponent> for ReceiptLineArrayOfReceivedDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfReceivedDateComponent> for ReceiptLineArrayOfReceivedDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfReceivedDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfReceivedDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfReceivedDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReceivedDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReceivedDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReceivedDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReceivedDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfReceivedQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReceivedQuantity>,
}

impl AsMut<ReceiptLineArrayOfReceivedQuantityComponent> for ReceiptLineArrayOfReceivedQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfReceivedQuantityComponent> for ReceiptLineArrayOfReceivedQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfReceivedQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfReceivedQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfReceivedQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReceivedQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReceivedQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReceivedQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReceivedQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfRejectActionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RejectActionCode>,
}

impl AsMut<ReceiptLineArrayOfRejectActionCodeComponent> for ReceiptLineArrayOfRejectActionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfRejectActionCodeComponent> for ReceiptLineArrayOfRejectActionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfRejectActionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfRejectActionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfRejectActionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RejectActionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RejectActionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RejectActionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RejectActionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfRejectReasonComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RejectReason>,
}

impl AsMut<ReceiptLineArrayOfRejectReasonComponent> for ReceiptLineArrayOfRejectReasonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfRejectReasonComponent> for ReceiptLineArrayOfRejectReasonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfRejectReasonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfRejectReasonComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RejectReason) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RejectReason) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RejectReason> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RejectReason> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfRejectReasonCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RejectReasonCode>,
}

impl AsMut<ReceiptLineArrayOfRejectReasonCodeComponent> for ReceiptLineArrayOfRejectReasonCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfRejectReasonCodeComponent> for ReceiptLineArrayOfRejectReasonCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfRejectReasonCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfRejectReasonCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfRejectReasonCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RejectReasonCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RejectReasonCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RejectReasonCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RejectReasonCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfRejectedQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RejectedQuantity>,
}

impl AsMut<ReceiptLineArrayOfRejectedQuantityComponent> for ReceiptLineArrayOfRejectedQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfRejectedQuantityComponent> for ReceiptLineArrayOfRejectedQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfRejectedQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfRejectedQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfRejectedQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RejectedQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RejectedQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RejectedQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RejectedQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfShipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Shipment>,
}

impl AsMut<ReceiptLineArrayOfShipmentComponent> for ReceiptLineArrayOfShipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfShipmentComponent> for ReceiptLineArrayOfShipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfShipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfShipmentComponent {
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
pub struct ReceiptLineArrayOfShortQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ShortQuantity>,
}

impl AsMut<ReceiptLineArrayOfShortQuantityComponent> for ReceiptLineArrayOfShortQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfShortQuantityComponent> for ReceiptLineArrayOfShortQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfShortQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfShortQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfShortQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ShortQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ShortQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ShortQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ShortQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfShortageActionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ShortageActionCode>,
}

impl AsMut<ReceiptLineArrayOfShortageActionCodeComponent> for ReceiptLineArrayOfShortageActionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfShortageActionCodeComponent> for ReceiptLineArrayOfShortageActionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfShortageActionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfShortageActionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfShortageActionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ShortageActionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ShortageActionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ShortageActionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ShortageActionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfTimingComplaintComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TimingComplaint>,
}

impl AsMut<ReceiptLineArrayOfTimingComplaintComponent> for ReceiptLineArrayOfTimingComplaintComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfTimingComplaintComponent> for ReceiptLineArrayOfTimingComplaintComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfTimingComplaintComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfTimingComplaintComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfTimingComplaintComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TimingComplaint) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TimingComplaint) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TimingComplaint> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TimingComplaint> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfTimingComplaintCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TimingComplaintCode>,
}

impl AsMut<ReceiptLineArrayOfTimingComplaintCodeComponent> for ReceiptLineArrayOfTimingComplaintCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfTimingComplaintCodeComponent> for ReceiptLineArrayOfTimingComplaintCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfTimingComplaintCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfTimingComplaintCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfTimingComplaintCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TimingComplaintCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TimingComplaintCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TimingComplaintCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TimingComplaintCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReceiptLineArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<ReceiptLineArrayOfUUIDComponent> for ReceiptLineArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ReceiptLineArrayOfUUIDComponent> for ReceiptLineArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ReceiptLineArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ReceiptLineArrayOfUUIDComponent {
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


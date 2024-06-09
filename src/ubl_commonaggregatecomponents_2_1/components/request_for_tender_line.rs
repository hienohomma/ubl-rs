use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestForTenderLine {
    #[serde(rename = "DeliveryPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_period: Option<RequestForTenderLineArrayOfDeliveryPeriodComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<RequestForTenderLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "EstimatedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_amount: Option<RequestForTenderLineArrayOfEstimatedAmountComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestForTenderLineArrayOfIDComponent>,
    #[serde(rename = "Item")]
    pub item: RequestForTenderLineArrayOfItemComponent,
    #[serde(rename = "MaximumAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_amount: Option<RequestForTenderLineArrayOfMaximumAmountComponent>,
    #[serde(rename = "MaximumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_quantity: Option<RequestForTenderLineArrayOfMaximumQuantityComponent>,
    #[serde(rename = "MinimumAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<RequestForTenderLineArrayOfMinimumAmountComponent>,
    #[serde(rename = "MinimumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_quantity: Option<RequestForTenderLineArrayOfMinimumQuantityComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<RequestForTenderLineArrayOfNoteComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<RequestForTenderLineArrayOfQuantityComponent>,
    #[serde(rename = "RequiredItemLocationQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_item_location_quantity: Option<RequestForTenderLineArrayOfRequiredItemLocationQuantityComponent>,
    #[serde(rename = "SubRequestForTenderLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_request_for_tender_line: Option<RequestForTenderLineArrayOfSubRequestForTenderLineComponent>,
    #[serde(rename = "TaxIncludedIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_included_indicator: Option<RequestForTenderLineArrayOfTaxIncludedIndicatorComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<RequestForTenderLineArrayOfUUIDComponent>,
    #[serde(rename = "WarrantyValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_validity_period: Option<RequestForTenderLineArrayOfWarrantyValidityPeriodComponent>,
}

impl AsMut<RequestForTenderLine> for RequestForTenderLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLine> for RequestForTenderLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.delivery_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.delivery_period", e));
            }
        }
        if let Some(v) = &self.minimum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.minimum_quantity", e));
            }
        }
        if let Some(v) = &self.minimum_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.minimum_amount", e));
            }
        }
        if let Some(v) = &self.maximum_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.maximum_amount", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.document_reference", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.uuid", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.note", e));
            }
        }
        if let Some(v) = &self.tax_included_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.tax_included_indicator", e));
            }
        }
        if let Some(v) = &self.sub_request_for_tender_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.sub_request_for_tender_line", e));
            }
        }
        if let Some(v) = &self.estimated_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.estimated_amount", e));
            }
        }
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("RequestForTenderLine.item", e));
        }
        if let Some(v) = &self.required_item_location_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.required_item_location_quantity", e));
            }
        }
        if let Some(v) = &self.maximum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.maximum_quantity", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.quantity", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.id", e));
            }
        }
        if let Some(v) = &self.warranty_validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestForTenderLine.warranty_validity_period", e));
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

impl RequestForTenderLine {
    pub fn title() -> &'static str {
        "Request For Tender Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Request for Tender describing an item of goods or a service solicited in the Request for Tender."
    }
    pub fn new(item: RequestForTenderLineArrayOfItemComponent) -> Component<Self> {
        Component(Self {
            tax_included_indicator: None,
            minimum_quantity: None,
            warranty_validity_period: None,
            maximum_amount: None,
            minimum_amount: None,
            delivery_period: None,
            document_reference: None,
            maximum_quantity: None,
            note: None,
            quantity: None,
            required_item_location_quantity: None,
            sub_request_for_tender_line: None,
            id: None,
            uuid: None,
            estimated_amount: None,
            item,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForTenderLineArrayOfDeliveryPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryPeriod>,
}

impl AsMut<RequestForTenderLineArrayOfDeliveryPeriodComponent> for RequestForTenderLineArrayOfDeliveryPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfDeliveryPeriodComponent> for RequestForTenderLineArrayOfDeliveryPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfDeliveryPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfDeliveryPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForTenderLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<RequestForTenderLineArrayOfDocumentReferenceComponent> for RequestForTenderLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfDocumentReferenceComponent> for RequestForTenderLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfDocumentReferenceComponent {
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
pub struct RequestForTenderLineArrayOfEstimatedAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EstimatedAmount>,
}

impl AsMut<RequestForTenderLineArrayOfEstimatedAmountComponent> for RequestForTenderLineArrayOfEstimatedAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfEstimatedAmountComponent> for RequestForTenderLineArrayOfEstimatedAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfEstimatedAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfEstimatedAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfEstimatedAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EstimatedAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EstimatedAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EstimatedAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EstimatedAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForTenderLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<RequestForTenderLineArrayOfIDComponent> for RequestForTenderLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfIDComponent> for RequestForTenderLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfIDComponent {
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
pub struct RequestForTenderLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<RequestForTenderLineArrayOfItemComponent> for RequestForTenderLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfItemComponent> for RequestForTenderLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfItemComponent {
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
pub struct RequestForTenderLineArrayOfMaximumAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumAmount>,
}

impl AsMut<RequestForTenderLineArrayOfMaximumAmountComponent> for RequestForTenderLineArrayOfMaximumAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfMaximumAmountComponent> for RequestForTenderLineArrayOfMaximumAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfMaximumAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfMaximumAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfMaximumAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForTenderLineArrayOfMaximumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity>,
}

impl AsMut<RequestForTenderLineArrayOfMaximumQuantityComponent> for RequestForTenderLineArrayOfMaximumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfMaximumQuantityComponent> for RequestForTenderLineArrayOfMaximumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfMaximumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfMaximumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfMaximumQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForTenderLineArrayOfMinimumAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumAmount>,
}

impl AsMut<RequestForTenderLineArrayOfMinimumAmountComponent> for RequestForTenderLineArrayOfMinimumAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfMinimumAmountComponent> for RequestForTenderLineArrayOfMinimumAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfMinimumAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfMinimumAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfMinimumAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForTenderLineArrayOfMinimumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity>,
}

impl AsMut<RequestForTenderLineArrayOfMinimumQuantityComponent> for RequestForTenderLineArrayOfMinimumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfMinimumQuantityComponent> for RequestForTenderLineArrayOfMinimumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfMinimumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfMinimumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfMinimumQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForTenderLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<RequestForTenderLineArrayOfNoteComponent> for RequestForTenderLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfNoteComponent> for RequestForTenderLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfNoteComponent {
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
pub struct RequestForTenderLineArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<RequestForTenderLineArrayOfQuantityComponent> for RequestForTenderLineArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfQuantityComponent> for RequestForTenderLineArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfQuantityComponent {
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
pub struct RequestForTenderLineArrayOfRequiredItemLocationQuantityComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity>,
}

impl AsMut<RequestForTenderLineArrayOfRequiredItemLocationQuantityComponent> for RequestForTenderLineArrayOfRequiredItemLocationQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfRequiredItemLocationQuantityComponent> for RequestForTenderLineArrayOfRequiredItemLocationQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfRequiredItemLocationQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfRequiredItemLocationQuantityComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForTenderLineArrayOfSubRequestForTenderLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubRequestForTenderLine>,
}

impl AsMut<RequestForTenderLineArrayOfSubRequestForTenderLineComponent> for RequestForTenderLineArrayOfSubRequestForTenderLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfSubRequestForTenderLineComponent> for RequestForTenderLineArrayOfSubRequestForTenderLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfSubRequestForTenderLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfSubRequestForTenderLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubRequestForTenderLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubRequestForTenderLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubRequestForTenderLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubRequestForTenderLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForTenderLineArrayOfTaxIncludedIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxIncludedIndicator>,
}

impl AsMut<RequestForTenderLineArrayOfTaxIncludedIndicatorComponent> for RequestForTenderLineArrayOfTaxIncludedIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfTaxIncludedIndicatorComponent> for RequestForTenderLineArrayOfTaxIncludedIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfTaxIncludedIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfTaxIncludedIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfTaxIncludedIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxIncludedIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxIncludedIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxIncludedIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxIncludedIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForTenderLineArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<RequestForTenderLineArrayOfUUIDComponent> for RequestForTenderLineArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfUUIDComponent> for RequestForTenderLineArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfUUIDComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestForTenderLineArrayOfWarrantyValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod>,
}

impl AsMut<RequestForTenderLineArrayOfWarrantyValidityPeriodComponent> for RequestForTenderLineArrayOfWarrantyValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestForTenderLineArrayOfWarrantyValidityPeriodComponent> for RequestForTenderLineArrayOfWarrantyValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfWarrantyValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestForTenderLineArrayOfWarrantyValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestForTenderLineArrayOfWarrantyValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod> {
        self.items.iter()
    }
}


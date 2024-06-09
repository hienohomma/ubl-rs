use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenderLine {
    #[serde(rename = "CallForTendersDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_for_tenders_document_reference: Option<TenderLineArrayOfCallForTendersDocumentReferenceComponent>,
    #[serde(rename = "CallForTendersLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_for_tenders_line_reference: Option<TenderLineArrayOfCallForTendersLineReferenceComponent>,
    #[serde(rename = "ContentUnitQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_unit_quantity: Option<TenderLineArrayOfContentUnitQuantityComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<TenderLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TenderLineArrayOfIDComponent>,
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<TenderLineArrayOfItemComponent>,
    #[serde(rename = "LineExtensionAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_extension_amount: Option<TenderLineArrayOfLineExtensionAmountComponent>,
    #[serde(rename = "MaximumOrderQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_order_quantity: Option<TenderLineArrayOfMaximumOrderQuantityComponent>,
    #[serde(rename = "MinimumOrderQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_order_quantity: Option<TenderLineArrayOfMinimumOrderQuantityComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<TenderLineArrayOfNoteComponent>,
    #[serde(rename = "OfferedItemLocationQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offered_item_location_quantity: Option<TenderLineArrayOfOfferedItemLocationQuantityComponent>,
    #[serde(rename = "OrderQuantityIncrementNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_quantity_increment_numeric: Option<TenderLineArrayOfOrderQuantityIncrementNumericComponent>,
    #[serde(rename = "OrderableUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable_unit: Option<TenderLineArrayOfOrderableUnitComponent>,
    #[serde(rename = "PackLevelCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_level_code: Option<TenderLineArrayOfPackLevelCodeComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<TenderLineArrayOfQuantityComponent>,
    #[serde(rename = "ReplacementRelatedItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_related_item: Option<TenderLineArrayOfReplacementRelatedItemComponent>,
    #[serde(rename = "SubTenderLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_tender_line: Option<TenderLineArrayOfSubTenderLineComponent>,
    #[serde(rename = "TotalTaxAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_amount: Option<TenderLineArrayOfTotalTaxAmountComponent>,
    #[serde(rename = "WarrantyInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_information: Option<TenderLineArrayOfWarrantyInformationComponent>,
    #[serde(rename = "WarrantyParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_party: Option<TenderLineArrayOfWarrantyPartyComponent>,
    #[serde(rename = "WarrantyValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_validity_period: Option<TenderLineArrayOfWarrantyValidityPeriodComponent>,
}

impl AsMut<TenderLine> for TenderLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLine> for TenderLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.line_extension_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.line_extension_amount", e));
            }
        }
        if let Some(v) = &self.offered_item_location_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.offered_item_location_quantity", e));
            }
        }
        if let Some(v) = &self.warranty_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.warranty_party", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.id", e));
            }
        }
        if let Some(v) = &self.maximum_order_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.maximum_order_quantity", e));
            }
        }
        if let Some(v) = &self.pack_level_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.pack_level_code", e));
            }
        }
        if let Some(v) = &self.sub_tender_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.sub_tender_line", e));
            }
        }
        if let Some(v) = &self.orderable_unit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.orderable_unit", e));
            }
        }
        if let Some(v) = &self.content_unit_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.content_unit_quantity", e));
            }
        }
        if let Some(v) = &self.call_for_tenders_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.call_for_tenders_document_reference", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.note", e));
            }
        }
        if let Some(v) = &self.call_for_tenders_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.call_for_tenders_line_reference", e));
            }
        }
        if let Some(v) = &self.order_quantity_increment_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.order_quantity_increment_numeric", e));
            }
        }
        if let Some(v) = &self.minimum_order_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.minimum_order_quantity", e));
            }
        }
        if let Some(v) = &self.warranty_validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.warranty_validity_period", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.quantity", e));
            }
        }
        if let Some(v) = &self.item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.item", e));
            }
        }
        if let Some(v) = &self.warranty_information {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.warranty_information", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.document_reference", e));
            }
        }
        if let Some(v) = &self.replacement_related_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.replacement_related_item", e));
            }
        }
        if let Some(v) = &self.total_tax_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderLine.total_tax_amount", e));
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

impl TenderLine {
    pub fn title() -> &'static str {
        "Tender Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Tender."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            offered_item_location_quantity: None,
            total_tax_amount: None,
            replacement_related_item: None,
            line_extension_amount: None,
            quantity: None,
            item: None,
            warranty_information: None,
            content_unit_quantity: None,
            sub_tender_line: None,
            order_quantity_increment_numeric: None,
            minimum_order_quantity: None,
            warranty_validity_period: None,
            call_for_tenders_line_reference: None,
            maximum_order_quantity: None,
            note: None,
            pack_level_code: None,
            orderable_unit: None,
            warranty_party: None,
            id: None,
            document_reference: None,
            call_for_tenders_document_reference: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfCallForTendersDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference>,
}

impl AsMut<TenderLineArrayOfCallForTendersDocumentReferenceComponent> for TenderLineArrayOfCallForTendersDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfCallForTendersDocumentReferenceComponent> for TenderLineArrayOfCallForTendersDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfCallForTendersDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfCallForTendersDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfCallForTendersDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfCallForTendersLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CallForTendersLineReference>,
}

impl AsMut<TenderLineArrayOfCallForTendersLineReferenceComponent> for TenderLineArrayOfCallForTendersLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfCallForTendersLineReferenceComponent> for TenderLineArrayOfCallForTendersLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfCallForTendersLineReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfCallForTendersLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfCallForTendersLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CallForTendersLineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CallForTendersLineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CallForTendersLineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CallForTendersLineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfContentUnitQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ContentUnitQuantity>,
}

impl AsMut<TenderLineArrayOfContentUnitQuantityComponent> for TenderLineArrayOfContentUnitQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfContentUnitQuantityComponent> for TenderLineArrayOfContentUnitQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfContentUnitQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfContentUnitQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfContentUnitQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ContentUnitQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ContentUnitQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ContentUnitQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ContentUnitQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<TenderLineArrayOfDocumentReferenceComponent> for TenderLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfDocumentReferenceComponent> for TenderLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfDocumentReferenceComponent {
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
pub struct TenderLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<TenderLineArrayOfIDComponent> for TenderLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfIDComponent> for TenderLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfIDComponent {
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
pub struct TenderLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<TenderLineArrayOfItemComponent> for TenderLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfItemComponent> for TenderLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfItemComponent {
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
pub struct TenderLineArrayOfLineExtensionAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount>,
}

impl AsMut<TenderLineArrayOfLineExtensionAmountComponent> for TenderLineArrayOfLineExtensionAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfLineExtensionAmountComponent> for TenderLineArrayOfLineExtensionAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfLineExtensionAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfLineExtensionAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfLineExtensionAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfMaximumOrderQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumOrderQuantity>,
}

impl AsMut<TenderLineArrayOfMaximumOrderQuantityComponent> for TenderLineArrayOfMaximumOrderQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfMaximumOrderQuantityComponent> for TenderLineArrayOfMaximumOrderQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfMaximumOrderQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfMaximumOrderQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfMaximumOrderQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumOrderQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumOrderQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumOrderQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumOrderQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfMinimumOrderQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumOrderQuantity>,
}

impl AsMut<TenderLineArrayOfMinimumOrderQuantityComponent> for TenderLineArrayOfMinimumOrderQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfMinimumOrderQuantityComponent> for TenderLineArrayOfMinimumOrderQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfMinimumOrderQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfMinimumOrderQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfMinimumOrderQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumOrderQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumOrderQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumOrderQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumOrderQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<TenderLineArrayOfNoteComponent> for TenderLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfNoteComponent> for TenderLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfNoteComponent {
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
pub struct TenderLineArrayOfOfferedItemLocationQuantityComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OfferedItemLocationQuantity>,
}

impl AsMut<TenderLineArrayOfOfferedItemLocationQuantityComponent> for TenderLineArrayOfOfferedItemLocationQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfOfferedItemLocationQuantityComponent> for TenderLineArrayOfOfferedItemLocationQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfOfferedItemLocationQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfOfferedItemLocationQuantityComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OfferedItemLocationQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OfferedItemLocationQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OfferedItemLocationQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OfferedItemLocationQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfOrderQuantityIncrementNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OrderQuantityIncrementNumeric>,
}

impl AsMut<TenderLineArrayOfOrderQuantityIncrementNumericComponent> for TenderLineArrayOfOrderQuantityIncrementNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfOrderQuantityIncrementNumericComponent> for TenderLineArrayOfOrderQuantityIncrementNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfOrderQuantityIncrementNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfOrderQuantityIncrementNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfOrderQuantityIncrementNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OrderQuantityIncrementNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OrderQuantityIncrementNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OrderQuantityIncrementNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OrderQuantityIncrementNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfOrderableUnitComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OrderableUnit>,
}

impl AsMut<TenderLineArrayOfOrderableUnitComponent> for TenderLineArrayOfOrderableUnitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfOrderableUnitComponent> for TenderLineArrayOfOrderableUnitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfOrderableUnitComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfOrderableUnitComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfOrderableUnitComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OrderableUnit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OrderableUnit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OrderableUnit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OrderableUnit> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfPackLevelCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PackLevelCode>,
}

impl AsMut<TenderLineArrayOfPackLevelCodeComponent> for TenderLineArrayOfPackLevelCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfPackLevelCodeComponent> for TenderLineArrayOfPackLevelCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfPackLevelCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfPackLevelCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfPackLevelCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PackLevelCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PackLevelCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PackLevelCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PackLevelCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<TenderLineArrayOfQuantityComponent> for TenderLineArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfQuantityComponent> for TenderLineArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfQuantityComponent {
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
pub struct TenderLineArrayOfReplacementRelatedItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReplacementRelatedItem>,
}

impl AsMut<TenderLineArrayOfReplacementRelatedItemComponent> for TenderLineArrayOfReplacementRelatedItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfReplacementRelatedItemComponent> for TenderLineArrayOfReplacementRelatedItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfReplacementRelatedItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfReplacementRelatedItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReplacementRelatedItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReplacementRelatedItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReplacementRelatedItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReplacementRelatedItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfSubTenderLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubTenderLine>,
}

impl AsMut<TenderLineArrayOfSubTenderLineComponent> for TenderLineArrayOfSubTenderLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfSubTenderLineComponent> for TenderLineArrayOfSubTenderLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfSubTenderLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfSubTenderLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubTenderLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubTenderLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubTenderLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubTenderLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfTotalTaxAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalTaxAmount>,
}

impl AsMut<TenderLineArrayOfTotalTaxAmountComponent> for TenderLineArrayOfTotalTaxAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfTotalTaxAmountComponent> for TenderLineArrayOfTotalTaxAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfTotalTaxAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfTotalTaxAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfTotalTaxAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalTaxAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalTaxAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalTaxAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalTaxAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfWarrantyInformationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::WarrantyInformation>,
}

impl AsMut<TenderLineArrayOfWarrantyInformationComponent> for TenderLineArrayOfWarrantyInformationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfWarrantyInformationComponent> for TenderLineArrayOfWarrantyInformationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfWarrantyInformationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfWarrantyInformationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::WarrantyInformation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::WarrantyInformation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::WarrantyInformation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::WarrantyInformation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfWarrantyPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WarrantyParty>,
}

impl AsMut<TenderLineArrayOfWarrantyPartyComponent> for TenderLineArrayOfWarrantyPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfWarrantyPartyComponent> for TenderLineArrayOfWarrantyPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfWarrantyPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfWarrantyPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfWarrantyPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WarrantyParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WarrantyParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WarrantyParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WarrantyParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderLineArrayOfWarrantyValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod>,
}

impl AsMut<TenderLineArrayOfWarrantyValidityPeriodComponent> for TenderLineArrayOfWarrantyValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderLineArrayOfWarrantyValidityPeriodComponent> for TenderLineArrayOfWarrantyValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderLineArrayOfWarrantyValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderLineArrayOfWarrantyValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderLineArrayOfWarrantyValidityPeriodComponent {
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


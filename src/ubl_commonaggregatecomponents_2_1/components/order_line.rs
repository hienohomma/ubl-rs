use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderLine {
    #[serde(rename = "BuyerProposedSubstituteLineItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_proposed_substitute_line_item: Option<OrderLineArrayOfBuyerProposedSubstituteLineItemComponent>,
    #[serde(rename = "CatalogueLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogue_line_reference: Option<OrderLineArrayOfCatalogueLineReferenceComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<OrderLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "LineItem")]
    pub line_item: OrderLineArrayOfLineItemComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<OrderLineArrayOfNoteComponent>,
    #[serde(rename = "OrderLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_line_reference: Option<OrderLineArrayOfOrderLineReferenceComponent>,
    #[serde(rename = "QuotationLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotation_line_reference: Option<OrderLineArrayOfQuotationLineReferenceComponent>,
    #[serde(rename = "SellerProposedSubstituteLineItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_proposed_substitute_line_item: Option<OrderLineArrayOfSellerProposedSubstituteLineItemComponent>,
    #[serde(rename = "SellerSubstitutedLineItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_substituted_line_item: Option<OrderLineArrayOfSellerSubstitutedLineItemComponent>,
    #[serde(rename = "SubstitutionStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitution_status_code: Option<OrderLineArrayOfSubstitutionStatusCodeComponent>,
}

impl AsMut<OrderLine> for OrderLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLine> for OrderLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.order_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLine.order_line_reference", e));
            }
        }
        if let Some(v) = &self.seller_substituted_line_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLine.seller_substituted_line_item", e));
            }
        }
        if let Some(v) = &self.substitution_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLine.substitution_status_code", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLine.note", e));
            }
        }
        if let Some(v) = &self.catalogue_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLine.catalogue_line_reference", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLine.document_reference", e));
            }
        }
        if let Some(v) = &self.buyer_proposed_substitute_line_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLine.buyer_proposed_substitute_line_item", e));
            }
        }
        if let Err(e) = self.line_item.validate() {
            return Err(UblError::component("OrderLine.line_item", e));
        }
        if let Some(v) = &self.quotation_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLine.quotation_line_reference", e));
            }
        }
        if let Some(v) = &self.seller_proposed_substitute_line_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("OrderLine.seller_proposed_substitute_line_item", e));
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

impl OrderLine {
    pub fn title() -> &'static str {
        "Order Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in an order document (e.g., Order, Order Change, or Order Response) describing an item being ordered."
    }
    pub fn new(line_item: OrderLineArrayOfLineItemComponent) -> Component<Self> {
        Component(Self {
            buyer_proposed_substitute_line_item: None,
            note: None,
            quotation_line_reference: None,
            document_reference: None,
            seller_proposed_substitute_line_item: None,
            seller_substituted_line_item: None,
            substitution_status_code: None,
            order_line_reference: None,
            catalogue_line_reference: None,
            line_item,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineArrayOfBuyerProposedSubstituteLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BuyerProposedSubstituteLineItem>,
}

impl AsMut<OrderLineArrayOfBuyerProposedSubstituteLineItemComponent> for OrderLineArrayOfBuyerProposedSubstituteLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineArrayOfBuyerProposedSubstituteLineItemComponent> for OrderLineArrayOfBuyerProposedSubstituteLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("OrderLineArrayOfBuyerProposedSubstituteLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineArrayOfBuyerProposedSubstituteLineItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BuyerProposedSubstituteLineItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BuyerProposedSubstituteLineItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BuyerProposedSubstituteLineItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BuyerProposedSubstituteLineItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineArrayOfCatalogueLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CatalogueLineReference>,
}

impl AsMut<OrderLineArrayOfCatalogueLineReferenceComponent> for OrderLineArrayOfCatalogueLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineArrayOfCatalogueLineReferenceComponent> for OrderLineArrayOfCatalogueLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderLineArrayOfCatalogueLineReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderLineArrayOfCatalogueLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineArrayOfCatalogueLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CatalogueLineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CatalogueLineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CatalogueLineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CatalogueLineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<OrderLineArrayOfDocumentReferenceComponent> for OrderLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineArrayOfDocumentReferenceComponent> for OrderLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("OrderLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineArrayOfDocumentReferenceComponent {
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
pub struct OrderLineArrayOfLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LineItem>,
}

impl AsMut<OrderLineArrayOfLineItemComponent> for OrderLineArrayOfLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineArrayOfLineItemComponent> for OrderLineArrayOfLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderLineArrayOfLineItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderLineArrayOfLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineArrayOfLineItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LineItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LineItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LineItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LineItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<OrderLineArrayOfNoteComponent> for OrderLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineArrayOfNoteComponent> for OrderLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("OrderLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineArrayOfNoteComponent {
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
pub struct OrderLineArrayOfOrderLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OrderLineReference>,
}

impl AsMut<OrderLineArrayOfOrderLineReferenceComponent> for OrderLineArrayOfOrderLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineArrayOfOrderLineReferenceComponent> for OrderLineArrayOfOrderLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("OrderLineArrayOfOrderLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineArrayOfOrderLineReferenceComponent {
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
pub struct OrderLineArrayOfQuotationLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::QuotationLineReference>,
}

impl AsMut<OrderLineArrayOfQuotationLineReferenceComponent> for OrderLineArrayOfQuotationLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineArrayOfQuotationLineReferenceComponent> for OrderLineArrayOfQuotationLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderLineArrayOfQuotationLineReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderLineArrayOfQuotationLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineArrayOfQuotationLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::QuotationLineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::QuotationLineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::QuotationLineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::QuotationLineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineArrayOfSellerProposedSubstituteLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellerProposedSubstituteLineItem>,
}

impl AsMut<OrderLineArrayOfSellerProposedSubstituteLineItemComponent> for OrderLineArrayOfSellerProposedSubstituteLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineArrayOfSellerProposedSubstituteLineItemComponent> for OrderLineArrayOfSellerProposedSubstituteLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("OrderLineArrayOfSellerProposedSubstituteLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineArrayOfSellerProposedSubstituteLineItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SellerProposedSubstituteLineItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SellerProposedSubstituteLineItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SellerProposedSubstituteLineItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SellerProposedSubstituteLineItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineArrayOfSellerSubstitutedLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellerSubstitutedLineItem>,
}

impl AsMut<OrderLineArrayOfSellerSubstitutedLineItemComponent> for OrderLineArrayOfSellerSubstitutedLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineArrayOfSellerSubstitutedLineItemComponent> for OrderLineArrayOfSellerSubstitutedLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("OrderLineArrayOfSellerSubstitutedLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineArrayOfSellerSubstitutedLineItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SellerSubstitutedLineItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SellerSubstitutedLineItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SellerSubstitutedLineItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SellerSubstitutedLineItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrderLineArrayOfSubstitutionStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SubstitutionStatusCode>,
}

impl AsMut<OrderLineArrayOfSubstitutionStatusCodeComponent> for OrderLineArrayOfSubstitutionStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<OrderLineArrayOfSubstitutionStatusCodeComponent> for OrderLineArrayOfSubstitutionStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("OrderLineArrayOfSubstitutionStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("OrderLineArrayOfSubstitutionStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl OrderLineArrayOfSubstitutionStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SubstitutionStatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SubstitutionStatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SubstitutionStatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SubstitutionStatusCode> {
        self.items.iter()
    }
}


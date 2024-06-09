use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineItem {
    #[serde(rename = "AccountingCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<LineItemArrayOfAccountingCostComponent>,
    #[serde(rename = "AccountingCostCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<LineItemArrayOfAccountingCostCodeComponent>,
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<LineItemArrayOfAllowanceChargeComponent>,
    #[serde(rename = "BackOrderAllowedIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_order_allowed_indicator: Option<LineItemArrayOfBackOrderAllowedIndicatorComponent>,
    #[serde(rename = "Delivery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<LineItemArrayOfDeliveryComponent>,
    #[serde(rename = "DeliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<LineItemArrayOfDeliveryTermsComponent>,
    #[serde(rename = "ID")]
    pub id: LineItemArrayOfIDComponent,
    #[serde(rename = "InspectionMethodCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspection_method_code: Option<LineItemArrayOfInspectionMethodCodeComponent>,
    #[serde(rename = "Item")]
    pub item: LineItemArrayOfItemComponent,
    #[serde(rename = "ItemPriceExtension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_price_extension: Option<LineItemArrayOfItemPriceExtensionComponent>,
    #[serde(rename = "LineExtensionAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_extension_amount: Option<LineItemArrayOfLineExtensionAmountComponent>,
    #[serde(rename = "LineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_reference: Option<LineItemArrayOfLineReferenceComponent>,
    #[serde(rename = "LineStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineItemArrayOfLineStatusCodeComponent>,
    #[serde(rename = "MaximumBackorderQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_backorder_quantity: Option<LineItemArrayOfMaximumBackorderQuantityComponent>,
    #[serde(rename = "MaximumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_quantity: Option<LineItemArrayOfMaximumQuantityComponent>,
    #[serde(rename = "MinimumBackorderQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_backorder_quantity: Option<LineItemArrayOfMinimumBackorderQuantityComponent>,
    #[serde(rename = "MinimumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_quantity: Option<LineItemArrayOfMinimumQuantityComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<LineItemArrayOfNoteComponent>,
    #[serde(rename = "OrderedShipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered_shipment: Option<LineItemArrayOfOrderedShipmentComponent>,
    #[serde(rename = "OriginatorParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_party: Option<LineItemArrayOfOriginatorPartyComponent>,
    #[serde(rename = "PartialDeliveryIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_delivery_indicator: Option<LineItemArrayOfPartialDeliveryIndicatorComponent>,
    #[serde(rename = "Price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<LineItemArrayOfPriceComponent>,
    #[serde(rename = "PricingReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_reference: Option<LineItemArrayOfPricingReferenceComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<LineItemArrayOfQuantityComponent>,
    #[serde(rename = "SalesOrderID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_order_id: Option<LineItemArrayOfSalesOrderIDComponent>,
    #[serde(rename = "SubLineItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_line_item: Option<LineItemArrayOfSubLineItemComponent>,
    #[serde(rename = "TaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<LineItemArrayOfTaxTotalComponent>,
    #[serde(rename = "TotalTaxAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_amount: Option<LineItemArrayOfTotalTaxAmountComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<LineItemArrayOfUUIDComponent>,
    #[serde(rename = "WarrantyInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_information: Option<LineItemArrayOfWarrantyInformationComponent>,
    #[serde(rename = "WarrantyParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_party: Option<LineItemArrayOfWarrantyPartyComponent>,
    #[serde(rename = "WarrantyValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_validity_period: Option<LineItemArrayOfWarrantyValidityPeriodComponent>,
}

impl AsMut<LineItem> for LineItem {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItem> for LineItem {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.minimum_backorder_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.minimum_backorder_quantity", e));
            }
        }
        if let Some(v) = &self.warranty_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.warranty_party", e));
            }
        }
        if let Some(v) = &self.minimum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.minimum_quantity", e));
            }
        }
        if let Some(v) = &self.originator_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.originator_party", e));
            }
        }
        if let Some(v) = &self.accounting_cost_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.accounting_cost_code", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.uuid", e));
            }
        }
        if let Some(v) = &self.warranty_information {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.warranty_information", e));
            }
        }
        if let Some(v) = &self.accounting_cost {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.accounting_cost", e));
            }
        }
        if let Some(v) = &self.delivery_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.delivery_terms", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("LineItem.id", e));
        }
        if let Some(v) = &self.item_price_extension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.item_price_extension", e));
            }
        }
        if let Some(v) = &self.partial_delivery_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.partial_delivery_indicator", e));
            }
        }
        if let Some(v) = &self.back_order_allowed_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.back_order_allowed_indicator", e));
            }
        }
        if let Some(v) = &self.inspection_method_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.inspection_method_code", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.quantity", e));
            }
        }
        if let Some(v) = &self.sub_line_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.sub_line_item", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.note", e));
            }
        }
        if let Some(v) = &self.delivery {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.delivery", e));
            }
        }
        if let Some(v) = &self.line_extension_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.line_extension_amount", e));
            }
        }
        if let Some(v) = &self.line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.line_reference", e));
            }
        }
        if let Some(v) = &self.sales_order_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.sales_order_id", e));
            }
        }
        if let Some(v) = &self.price {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.price", e));
            }
        }
        if let Some(v) = &self.ordered_shipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.ordered_shipment", e));
            }
        }
        if let Some(v) = &self.tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.tax_total", e));
            }
        }
        if let Some(v) = &self.total_tax_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.total_tax_amount", e));
            }
        }
        if let Some(v) = &self.warranty_validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.warranty_validity_period", e));
            }
        }
        if let Some(v) = &self.pricing_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.pricing_reference", e));
            }
        }
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("LineItem.item", e));
        }
        if let Some(v) = &self.maximum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.maximum_quantity", e));
            }
        }
        if let Some(v) = &self.line_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.line_status_code", e));
            }
        }
        if let Some(v) = &self.maximum_backorder_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.maximum_backorder_quantity", e));
            }
        }
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LineItem.allowance_charge", e));
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

impl LineItem {
    pub fn title() -> &'static str {
        "Line Item. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a line item."
    }
    pub fn new(id: LineItemArrayOfIDComponent, item: LineItemArrayOfItemComponent) -> Component<Self> {
        Component(Self {
            accounting_cost: None,
            minimum_backorder_quantity: None,
            originator_party: None,
            pricing_reference: None,
            maximum_backorder_quantity: None,
            accounting_cost_code: None,
            warranty_validity_period: None,
            item_price_extension: None,
            line_reference: None,
            maximum_quantity: None,
            line_status_code: None,
            item,
            delivery: None,
            back_order_allowed_indicator: None,
            delivery_terms: None,
            ordered_shipment: None,
            total_tax_amount: None,
            uuid: None,
            line_extension_amount: None,
            partial_delivery_indicator: None,
            quantity: None,
            sales_order_id: None,
            warranty_party: None,
            allowance_charge: None,
            price: None,
            sub_line_item: None,
            minimum_quantity: None,
            inspection_method_code: None,
            tax_total: None,
            warranty_information: None,
            note: None,
            id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfAccountingCostComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCost>,
}

impl AsMut<LineItemArrayOfAccountingCostComponent> for LineItemArrayOfAccountingCostComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfAccountingCostComponent> for LineItemArrayOfAccountingCostComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfAccountingCostComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfAccountingCostComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfAccountingCostComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AccountingCost) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AccountingCost) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AccountingCost> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AccountingCost> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfAccountingCostCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCostCode>,
}

impl AsMut<LineItemArrayOfAccountingCostCodeComponent> for LineItemArrayOfAccountingCostCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfAccountingCostCodeComponent> for LineItemArrayOfAccountingCostCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfAccountingCostCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfAccountingCostCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfAccountingCostCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AccountingCostCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AccountingCostCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AccountingCostCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AccountingCostCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<LineItemArrayOfAllowanceChargeComponent> for LineItemArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfAllowanceChargeComponent> for LineItemArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfAllowanceChargeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfBackOrderAllowedIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BackOrderAllowedIndicator>,
}

impl AsMut<LineItemArrayOfBackOrderAllowedIndicatorComponent> for LineItemArrayOfBackOrderAllowedIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfBackOrderAllowedIndicatorComponent> for LineItemArrayOfBackOrderAllowedIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfBackOrderAllowedIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfBackOrderAllowedIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfBackOrderAllowedIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BackOrderAllowedIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BackOrderAllowedIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BackOrderAllowedIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BackOrderAllowedIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfDeliveryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Delivery>,
}

impl AsMut<LineItemArrayOfDeliveryComponent> for LineItemArrayOfDeliveryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfDeliveryComponent> for LineItemArrayOfDeliveryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfDeliveryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfDeliveryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Delivery) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Delivery) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Delivery> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Delivery> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfDeliveryTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms>,
}

impl AsMut<LineItemArrayOfDeliveryTermsComponent> for LineItemArrayOfDeliveryTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfDeliveryTermsComponent> for LineItemArrayOfDeliveryTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfDeliveryTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfDeliveryTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfDeliveryTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<LineItemArrayOfIDComponent> for LineItemArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfIDComponent> for LineItemArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfIDComponent {
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
pub struct LineItemArrayOfInspectionMethodCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InspectionMethodCode>,
}

impl AsMut<LineItemArrayOfInspectionMethodCodeComponent> for LineItemArrayOfInspectionMethodCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfInspectionMethodCodeComponent> for LineItemArrayOfInspectionMethodCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfInspectionMethodCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfInspectionMethodCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfInspectionMethodCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InspectionMethodCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InspectionMethodCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InspectionMethodCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InspectionMethodCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<LineItemArrayOfItemComponent> for LineItemArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfItemComponent> for LineItemArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfItemComponent {
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
pub struct LineItemArrayOfItemPriceExtensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ItemPriceExtension>,
}

impl AsMut<LineItemArrayOfItemPriceExtensionComponent> for LineItemArrayOfItemPriceExtensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfItemPriceExtensionComponent> for LineItemArrayOfItemPriceExtensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfItemPriceExtensionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfItemPriceExtensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfItemPriceExtensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ItemPriceExtension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ItemPriceExtension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ItemPriceExtension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ItemPriceExtension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfLineExtensionAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount>,
}

impl AsMut<LineItemArrayOfLineExtensionAmountComponent> for LineItemArrayOfLineExtensionAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfLineExtensionAmountComponent> for LineItemArrayOfLineExtensionAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfLineExtensionAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfLineExtensionAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfLineExtensionAmountComponent {
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
pub struct LineItemArrayOfLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LineReference>,
}

impl AsMut<LineItemArrayOfLineReferenceComponent> for LineItemArrayOfLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfLineReferenceComponent> for LineItemArrayOfLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfLineStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineStatusCode>,
}

impl AsMut<LineItemArrayOfLineStatusCodeComponent> for LineItemArrayOfLineStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfLineStatusCodeComponent> for LineItemArrayOfLineStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfLineStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfLineStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfLineStatusCodeComponent {
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
pub struct LineItemArrayOfMaximumBackorderQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumBackorderQuantity>,
}

impl AsMut<LineItemArrayOfMaximumBackorderQuantityComponent> for LineItemArrayOfMaximumBackorderQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfMaximumBackorderQuantityComponent> for LineItemArrayOfMaximumBackorderQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfMaximumBackorderQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfMaximumBackorderQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfMaximumBackorderQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumBackorderQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumBackorderQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumBackorderQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumBackorderQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfMaximumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity>,
}

impl AsMut<LineItemArrayOfMaximumQuantityComponent> for LineItemArrayOfMaximumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfMaximumQuantityComponent> for LineItemArrayOfMaximumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfMaximumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfMaximumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfMaximumQuantityComponent {
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
pub struct LineItemArrayOfMinimumBackorderQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumBackorderQuantity>,
}

impl AsMut<LineItemArrayOfMinimumBackorderQuantityComponent> for LineItemArrayOfMinimumBackorderQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfMinimumBackorderQuantityComponent> for LineItemArrayOfMinimumBackorderQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfMinimumBackorderQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfMinimumBackorderQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfMinimumBackorderQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumBackorderQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumBackorderQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumBackorderQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumBackorderQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfMinimumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity>,
}

impl AsMut<LineItemArrayOfMinimumQuantityComponent> for LineItemArrayOfMinimumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfMinimumQuantityComponent> for LineItemArrayOfMinimumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfMinimumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfMinimumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfMinimumQuantityComponent {
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
pub struct LineItemArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<LineItemArrayOfNoteComponent> for LineItemArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfNoteComponent> for LineItemArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfNoteComponent {
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
pub struct LineItemArrayOfOrderedShipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OrderedShipment>,
}

impl AsMut<LineItemArrayOfOrderedShipmentComponent> for LineItemArrayOfOrderedShipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfOrderedShipmentComponent> for LineItemArrayOfOrderedShipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfOrderedShipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfOrderedShipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OrderedShipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OrderedShipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OrderedShipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OrderedShipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfOriginatorPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginatorParty>,
}

impl AsMut<LineItemArrayOfOriginatorPartyComponent> for LineItemArrayOfOriginatorPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfOriginatorPartyComponent> for LineItemArrayOfOriginatorPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfOriginatorPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfOriginatorPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfOriginatorPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OriginatorParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OriginatorParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OriginatorParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OriginatorParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfPartialDeliveryIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PartialDeliveryIndicator>,
}

impl AsMut<LineItemArrayOfPartialDeliveryIndicatorComponent> for LineItemArrayOfPartialDeliveryIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfPartialDeliveryIndicatorComponent> for LineItemArrayOfPartialDeliveryIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfPartialDeliveryIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfPartialDeliveryIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfPartialDeliveryIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PartialDeliveryIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PartialDeliveryIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PartialDeliveryIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PartialDeliveryIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfPriceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Price>,
}

impl AsMut<LineItemArrayOfPriceComponent> for LineItemArrayOfPriceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfPriceComponent> for LineItemArrayOfPriceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfPriceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfPriceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfPriceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Price) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Price) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Price> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Price> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfPricingReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PricingReference>,
}

impl AsMut<LineItemArrayOfPricingReferenceComponent> for LineItemArrayOfPricingReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfPricingReferenceComponent> for LineItemArrayOfPricingReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfPricingReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfPricingReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfPricingReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PricingReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PricingReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PricingReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PricingReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<LineItemArrayOfQuantityComponent> for LineItemArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfQuantityComponent> for LineItemArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfQuantityComponent {
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
pub struct LineItemArrayOfSalesOrderIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SalesOrderID>,
}

impl AsMut<LineItemArrayOfSalesOrderIDComponent> for LineItemArrayOfSalesOrderIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfSalesOrderIDComponent> for LineItemArrayOfSalesOrderIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfSalesOrderIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfSalesOrderIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfSalesOrderIDComponent {
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
pub struct LineItemArrayOfSubLineItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubLineItem>,
}

impl AsMut<LineItemArrayOfSubLineItemComponent> for LineItemArrayOfSubLineItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfSubLineItemComponent> for LineItemArrayOfSubLineItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfSubLineItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfSubLineItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubLineItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubLineItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubLineItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubLineItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxTotal>,
}

impl AsMut<LineItemArrayOfTaxTotalComponent> for LineItemArrayOfTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfTaxTotalComponent> for LineItemArrayOfTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfTaxTotalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineItemArrayOfTotalTaxAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalTaxAmount>,
}

impl AsMut<LineItemArrayOfTotalTaxAmountComponent> for LineItemArrayOfTotalTaxAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfTotalTaxAmountComponent> for LineItemArrayOfTotalTaxAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfTotalTaxAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfTotalTaxAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfTotalTaxAmountComponent {
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
pub struct LineItemArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<LineItemArrayOfUUIDComponent> for LineItemArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfUUIDComponent> for LineItemArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfUUIDComponent {
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
pub struct LineItemArrayOfWarrantyInformationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::WarrantyInformation>,
}

impl AsMut<LineItemArrayOfWarrantyInformationComponent> for LineItemArrayOfWarrantyInformationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfWarrantyInformationComponent> for LineItemArrayOfWarrantyInformationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfWarrantyInformationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfWarrantyInformationComponent {
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
pub struct LineItemArrayOfWarrantyPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WarrantyParty>,
}

impl AsMut<LineItemArrayOfWarrantyPartyComponent> for LineItemArrayOfWarrantyPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfWarrantyPartyComponent> for LineItemArrayOfWarrantyPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfWarrantyPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfWarrantyPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfWarrantyPartyComponent {
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
pub struct LineItemArrayOfWarrantyValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod>,
}

impl AsMut<LineItemArrayOfWarrantyValidityPeriodComponent> for LineItemArrayOfWarrantyValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineItemArrayOfWarrantyValidityPeriodComponent> for LineItemArrayOfWarrantyValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineItemArrayOfWarrantyValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineItemArrayOfWarrantyValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineItemArrayOfWarrantyValidityPeriodComponent {
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


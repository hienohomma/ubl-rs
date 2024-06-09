use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsumptionLine {
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<ConsumptionLineArrayOfAllowanceChargeComponent>,
    #[serde(rename = "Delivery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<ConsumptionLineArrayOfDeliveryComponent>,
    #[serde(rename = "ID")]
    pub id: ConsumptionLineArrayOfIDComponent,
    #[serde(rename = "InvoicedQuantity")]
    pub invoiced_quantity: ConsumptionLineArrayOfInvoicedQuantityComponent,
    #[serde(rename = "LineExtensionAmount")]
    pub line_extension_amount: ConsumptionLineArrayOfLineExtensionAmountComponent,
    #[serde(rename = "ParentDocumentLineReferenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_line_reference_id: Option<ConsumptionLineArrayOfParentDocumentLineReferenceIDComponent>,
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<ConsumptionLineArrayOfPeriodComponent>,
    #[serde(rename = "Price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<ConsumptionLineArrayOfPriceComponent>,
    #[serde(rename = "TaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<ConsumptionLineArrayOfTaxTotalComponent>,
    #[serde(rename = "UnstructuredPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured_price: Option<ConsumptionLineArrayOfUnstructuredPriceComponent>,
    #[serde(rename = "UtilityItem")]
    pub utility_item: ConsumptionLineArrayOfUtilityItemComponent,
}

impl AsMut<ConsumptionLine> for ConsumptionLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLine> for ConsumptionLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.utility_item.validate() {
            return Err(UblError::component("ConsumptionLine.utility_item", e));
        }
        if let Some(v) = &self.parent_document_line_reference_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionLine.parent_document_line_reference_id", e));
            }
        }
        if let Some(v) = &self.period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionLine.period", e));
            }
        }
        if let Err(e) = self.invoiced_quantity.validate() {
            return Err(UblError::component("ConsumptionLine.invoiced_quantity", e));
        }
        if let Some(v) = &self.price {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionLine.price", e));
            }
        }
        if let Err(e) = self.line_extension_amount.validate() {
            return Err(UblError::component("ConsumptionLine.line_extension_amount", e));
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ConsumptionLine.id", e));
        }
        if let Some(v) = &self.tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionLine.tax_total", e));
            }
        }
        if let Some(v) = &self.unstructured_price {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionLine.unstructured_price", e));
            }
        }
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionLine.allowance_charge", e));
            }
        }
        if let Some(v) = &self.delivery {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionLine.delivery", e));
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

impl ConsumptionLine {
    pub fn title() -> &'static str {
        "Consumption Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a line item for utility consumption. To specify more than one utility item, use separate consumption lines."
    }
    pub fn new(id: ConsumptionLineArrayOfIDComponent, invoiced_quantity: ConsumptionLineArrayOfInvoicedQuantityComponent, line_extension_amount: ConsumptionLineArrayOfLineExtensionAmountComponent, utility_item: ConsumptionLineArrayOfUtilityItemComponent) -> Component<Self> {
        Component(Self {
            line_extension_amount,
            invoiced_quantity,
            parent_document_line_reference_id: None,
            price: None,
            tax_total: None,
            delivery: None,
            unstructured_price: None,
            utility_item,
            id,
            period: None,
            allowance_charge: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionLineArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<ConsumptionLineArrayOfAllowanceChargeComponent> for ConsumptionLineArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLineArrayOfAllowanceChargeComponent> for ConsumptionLineArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionLineArrayOfAllowanceChargeComponent {
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
pub struct ConsumptionLineArrayOfDeliveryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Delivery>,
}

impl AsMut<ConsumptionLineArrayOfDeliveryComponent> for ConsumptionLineArrayOfDeliveryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLineArrayOfDeliveryComponent> for ConsumptionLineArrayOfDeliveryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfDeliveryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionLineArrayOfDeliveryComponent {
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
pub struct ConsumptionLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ConsumptionLineArrayOfIDComponent> for ConsumptionLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLineArrayOfIDComponent> for ConsumptionLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionLineArrayOfIDComponent {
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
pub struct ConsumptionLineArrayOfInvoicedQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InvoicedQuantity>,
}

impl AsMut<ConsumptionLineArrayOfInvoicedQuantityComponent> for ConsumptionLineArrayOfInvoicedQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLineArrayOfInvoicedQuantityComponent> for ConsumptionLineArrayOfInvoicedQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfInvoicedQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfInvoicedQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionLineArrayOfInvoicedQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InvoicedQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InvoicedQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InvoicedQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InvoicedQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionLineArrayOfLineExtensionAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount>,
}

impl AsMut<ConsumptionLineArrayOfLineExtensionAmountComponent> for ConsumptionLineArrayOfLineExtensionAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLineArrayOfLineExtensionAmountComponent> for ConsumptionLineArrayOfLineExtensionAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfLineExtensionAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfLineExtensionAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionLineArrayOfLineExtensionAmountComponent {
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
pub struct ConsumptionLineArrayOfParentDocumentLineReferenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ParentDocumentLineReferenceID>,
}

impl AsMut<ConsumptionLineArrayOfParentDocumentLineReferenceIDComponent> for ConsumptionLineArrayOfParentDocumentLineReferenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLineArrayOfParentDocumentLineReferenceIDComponent> for ConsumptionLineArrayOfParentDocumentLineReferenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfParentDocumentLineReferenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfParentDocumentLineReferenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionLineArrayOfParentDocumentLineReferenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ParentDocumentLineReferenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ParentDocumentLineReferenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ParentDocumentLineReferenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ParentDocumentLineReferenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionLineArrayOfPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Period>,
}

impl AsMut<ConsumptionLineArrayOfPeriodComponent> for ConsumptionLineArrayOfPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLineArrayOfPeriodComponent> for ConsumptionLineArrayOfPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionLineArrayOfPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Period) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Period) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Period> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Period> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionLineArrayOfPriceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Price>,
}

impl AsMut<ConsumptionLineArrayOfPriceComponent> for ConsumptionLineArrayOfPriceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLineArrayOfPriceComponent> for ConsumptionLineArrayOfPriceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfPriceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfPriceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionLineArrayOfPriceComponent {
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
pub struct ConsumptionLineArrayOfTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxTotal>,
}

impl AsMut<ConsumptionLineArrayOfTaxTotalComponent> for ConsumptionLineArrayOfTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLineArrayOfTaxTotalComponent> for ConsumptionLineArrayOfTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionLineArrayOfTaxTotalComponent {
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
pub struct ConsumptionLineArrayOfUnstructuredPriceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::UnstructuredPrice>,
}

impl AsMut<ConsumptionLineArrayOfUnstructuredPriceComponent> for ConsumptionLineArrayOfUnstructuredPriceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLineArrayOfUnstructuredPriceComponent> for ConsumptionLineArrayOfUnstructuredPriceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfUnstructuredPriceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfUnstructuredPriceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionLineArrayOfUnstructuredPriceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::UnstructuredPrice) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::UnstructuredPrice) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::UnstructuredPrice> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::UnstructuredPrice> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionLineArrayOfUtilityItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::UtilityItem>,
}

impl AsMut<ConsumptionLineArrayOfUtilityItemComponent> for ConsumptionLineArrayOfUtilityItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionLineArrayOfUtilityItemComponent> for ConsumptionLineArrayOfUtilityItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfUtilityItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionLineArrayOfUtilityItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionLineArrayOfUtilityItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::UtilityItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::UtilityItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::UtilityItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::UtilityItem> {
        self.items.iter()
    }
}


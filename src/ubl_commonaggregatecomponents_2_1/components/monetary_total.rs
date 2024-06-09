use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonetaryTotal {
    #[serde(rename = "AllowanceTotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_total_amount: Option<MonetaryTotalArrayOfAllowanceTotalAmountComponent>,
    #[serde(rename = "ChargeTotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_total_amount: Option<MonetaryTotalArrayOfChargeTotalAmountComponent>,
    #[serde(rename = "LineExtensionAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_extension_amount: Option<MonetaryTotalArrayOfLineExtensionAmountComponent>,
    #[serde(rename = "PayableAlternativeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payable_alternative_amount: Option<MonetaryTotalArrayOfPayableAlternativeAmountComponent>,
    #[serde(rename = "PayableAmount")]
    pub payable_amount: MonetaryTotalArrayOfPayableAmountComponent,
    #[serde(rename = "PayableRoundingAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payable_rounding_amount: Option<MonetaryTotalArrayOfPayableRoundingAmountComponent>,
    #[serde(rename = "PrepaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_amount: Option<MonetaryTotalArrayOfPrepaidAmountComponent>,
    #[serde(rename = "TaxExclusiveAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exclusive_amount: Option<MonetaryTotalArrayOfTaxExclusiveAmountComponent>,
    #[serde(rename = "TaxInclusiveAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_inclusive_amount: Option<MonetaryTotalArrayOfTaxInclusiveAmountComponent>,
}

impl AsMut<MonetaryTotal> for MonetaryTotal {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MonetaryTotal> for MonetaryTotal {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.allowance_total_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MonetaryTotal.allowance_total_amount", e));
            }
        }
        if let Some(v) = &self.line_extension_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MonetaryTotal.line_extension_amount", e));
            }
        }
        if let Some(v) = &self.tax_inclusive_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MonetaryTotal.tax_inclusive_amount", e));
            }
        }
        if let Err(e) = self.payable_amount.validate() {
            return Err(UblError::component("MonetaryTotal.payable_amount", e));
        }
        if let Some(v) = &self.payable_alternative_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MonetaryTotal.payable_alternative_amount", e));
            }
        }
        if let Some(v) = &self.charge_total_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MonetaryTotal.charge_total_amount", e));
            }
        }
        if let Some(v) = &self.payable_rounding_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MonetaryTotal.payable_rounding_amount", e));
            }
        }
        if let Some(v) = &self.tax_exclusive_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MonetaryTotal.tax_exclusive_amount", e));
            }
        }
        if let Some(v) = &self.prepaid_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("MonetaryTotal.prepaid_amount", e));
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

impl MonetaryTotal {
    pub fn title() -> &'static str {
        "Monetary Total. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a monetary total."
    }
    pub fn new(payable_amount: MonetaryTotalArrayOfPayableAmountComponent) -> Component<Self> {
        Component(Self {
            payable_alternative_amount: None,
            charge_total_amount: None,
            prepaid_amount: None,
            tax_inclusive_amount: None,
            line_extension_amount: None,
            payable_rounding_amount: None,
            tax_exclusive_amount: None,
            allowance_total_amount: None,
            payable_amount,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MonetaryTotalArrayOfAllowanceTotalAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AllowanceTotalAmount>,
}

impl AsMut<MonetaryTotalArrayOfAllowanceTotalAmountComponent> for MonetaryTotalArrayOfAllowanceTotalAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MonetaryTotalArrayOfAllowanceTotalAmountComponent> for MonetaryTotalArrayOfAllowanceTotalAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfAllowanceTotalAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfAllowanceTotalAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MonetaryTotalArrayOfAllowanceTotalAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AllowanceTotalAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AllowanceTotalAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AllowanceTotalAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AllowanceTotalAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MonetaryTotalArrayOfChargeTotalAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ChargeTotalAmount>,
}

impl AsMut<MonetaryTotalArrayOfChargeTotalAmountComponent> for MonetaryTotalArrayOfChargeTotalAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MonetaryTotalArrayOfChargeTotalAmountComponent> for MonetaryTotalArrayOfChargeTotalAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfChargeTotalAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfChargeTotalAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MonetaryTotalArrayOfChargeTotalAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ChargeTotalAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ChargeTotalAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ChargeTotalAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ChargeTotalAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MonetaryTotalArrayOfLineExtensionAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount>,
}

impl AsMut<MonetaryTotalArrayOfLineExtensionAmountComponent> for MonetaryTotalArrayOfLineExtensionAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MonetaryTotalArrayOfLineExtensionAmountComponent> for MonetaryTotalArrayOfLineExtensionAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfLineExtensionAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfLineExtensionAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MonetaryTotalArrayOfLineExtensionAmountComponent {
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
pub struct MonetaryTotalArrayOfPayableAlternativeAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PayableAlternativeAmount>,
}

impl AsMut<MonetaryTotalArrayOfPayableAlternativeAmountComponent> for MonetaryTotalArrayOfPayableAlternativeAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MonetaryTotalArrayOfPayableAlternativeAmountComponent> for MonetaryTotalArrayOfPayableAlternativeAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfPayableAlternativeAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfPayableAlternativeAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MonetaryTotalArrayOfPayableAlternativeAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PayableAlternativeAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PayableAlternativeAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PayableAlternativeAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PayableAlternativeAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MonetaryTotalArrayOfPayableAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PayableAmount>,
}

impl AsMut<MonetaryTotalArrayOfPayableAmountComponent> for MonetaryTotalArrayOfPayableAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MonetaryTotalArrayOfPayableAmountComponent> for MonetaryTotalArrayOfPayableAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfPayableAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfPayableAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MonetaryTotalArrayOfPayableAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PayableAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PayableAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PayableAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PayableAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MonetaryTotalArrayOfPayableRoundingAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PayableRoundingAmount>,
}

impl AsMut<MonetaryTotalArrayOfPayableRoundingAmountComponent> for MonetaryTotalArrayOfPayableRoundingAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MonetaryTotalArrayOfPayableRoundingAmountComponent> for MonetaryTotalArrayOfPayableRoundingAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfPayableRoundingAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfPayableRoundingAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MonetaryTotalArrayOfPayableRoundingAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PayableRoundingAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PayableRoundingAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PayableRoundingAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PayableRoundingAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MonetaryTotalArrayOfPrepaidAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PrepaidAmount>,
}

impl AsMut<MonetaryTotalArrayOfPrepaidAmountComponent> for MonetaryTotalArrayOfPrepaidAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MonetaryTotalArrayOfPrepaidAmountComponent> for MonetaryTotalArrayOfPrepaidAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfPrepaidAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfPrepaidAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MonetaryTotalArrayOfPrepaidAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PrepaidAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PrepaidAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PrepaidAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PrepaidAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MonetaryTotalArrayOfTaxExclusiveAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxExclusiveAmount>,
}

impl AsMut<MonetaryTotalArrayOfTaxExclusiveAmountComponent> for MonetaryTotalArrayOfTaxExclusiveAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MonetaryTotalArrayOfTaxExclusiveAmountComponent> for MonetaryTotalArrayOfTaxExclusiveAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfTaxExclusiveAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfTaxExclusiveAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MonetaryTotalArrayOfTaxExclusiveAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxExclusiveAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxExclusiveAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxExclusiveAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxExclusiveAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MonetaryTotalArrayOfTaxInclusiveAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxInclusiveAmount>,
}

impl AsMut<MonetaryTotalArrayOfTaxInclusiveAmountComponent> for MonetaryTotalArrayOfTaxInclusiveAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<MonetaryTotalArrayOfTaxInclusiveAmountComponent> for MonetaryTotalArrayOfTaxInclusiveAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfTaxInclusiveAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("MonetaryTotalArrayOfTaxInclusiveAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl MonetaryTotalArrayOfTaxInclusiveAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxInclusiveAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxInclusiveAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxInclusiveAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxInclusiveAmount> {
        self.items.iter()
    }
}


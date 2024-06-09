use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaxSubtotal {
    #[serde(rename = "BaseUnitMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_unit_measure: Option<TaxSubtotalArrayOfBaseUnitMeasureComponent>,
    #[serde(rename = "CalculationSequenceNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_sequence_numeric: Option<TaxSubtotalArrayOfCalculationSequenceNumericComponent>,
    #[serde(rename = "PerUnitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_unit_amount: Option<TaxSubtotalArrayOfPerUnitAmountComponent>,
    #[serde(rename = "Percent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<TaxSubtotalArrayOfPercentComponent>,
    #[serde(rename = "TaxAmount")]
    pub tax_amount: TaxSubtotalArrayOfTaxAmountComponent,
    #[serde(rename = "TaxCategory")]
    pub tax_category: TaxSubtotalArrayOfTaxCategoryComponent,
    #[serde(rename = "TaxableAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<TaxSubtotalArrayOfTaxableAmountComponent>,
    #[serde(rename = "TierRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_range: Option<TaxSubtotalArrayOfTierRangeComponent>,
    #[serde(rename = "TierRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_rate_percent: Option<TaxSubtotalArrayOfTierRatePercentComponent>,
    #[serde(rename = "TransactionCurrencyTaxAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_currency_tax_amount: Option<TaxSubtotalArrayOfTransactionCurrencyTaxAmountComponent>,
}

impl AsMut<TaxSubtotal> for TaxSubtotal {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSubtotal> for TaxSubtotal {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.transaction_currency_tax_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxSubtotal.transaction_currency_tax_amount", e));
            }
        }
        if let Err(e) = self.tax_amount.validate() {
            return Err(UblError::component("TaxSubtotal.tax_amount", e));
        }
        if let Some(v) = &self.percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxSubtotal.percent", e));
            }
        }
        if let Some(v) = &self.tier_range {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxSubtotal.tier_range", e));
            }
        }
        if let Some(v) = &self.tier_rate_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxSubtotal.tier_rate_percent", e));
            }
        }
        if let Err(e) = self.tax_category.validate() {
            return Err(UblError::component("TaxSubtotal.tax_category", e));
        }
        if let Some(v) = &self.taxable_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxSubtotal.taxable_amount", e));
            }
        }
        if let Some(v) = &self.base_unit_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxSubtotal.base_unit_measure", e));
            }
        }
        if let Some(v) = &self.calculation_sequence_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxSubtotal.calculation_sequence_numeric", e));
            }
        }
        if let Some(v) = &self.per_unit_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxSubtotal.per_unit_amount", e));
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

impl TaxSubtotal {
    pub fn title() -> &'static str {
        "Tax Subtotal. Details"
    }
    pub fn description() -> &'static str {
        "A class to define the subtotal for a particular tax category within a particular taxation scheme, such as standard rate within VAT."
    }
    pub fn new(tax_amount: TaxSubtotalArrayOfTaxAmountComponent, tax_category: TaxSubtotalArrayOfTaxCategoryComponent) -> Component<Self> {
        Component(Self {
            base_unit_measure: None,
            percent: None,
            per_unit_amount: None,
            taxable_amount: None,
            tier_rate_percent: None,
            transaction_currency_tax_amount: None,
            tax_amount,
            calculation_sequence_numeric: None,
            tier_range: None,
            tax_category,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSubtotalArrayOfBaseUnitMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BaseUnitMeasure>,
}

impl AsMut<TaxSubtotalArrayOfBaseUnitMeasureComponent> for TaxSubtotalArrayOfBaseUnitMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSubtotalArrayOfBaseUnitMeasureComponent> for TaxSubtotalArrayOfBaseUnitMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfBaseUnitMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfBaseUnitMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSubtotalArrayOfBaseUnitMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BaseUnitMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BaseUnitMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BaseUnitMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BaseUnitMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSubtotalArrayOfCalculationSequenceNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CalculationSequenceNumeric>,
}

impl AsMut<TaxSubtotalArrayOfCalculationSequenceNumericComponent> for TaxSubtotalArrayOfCalculationSequenceNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSubtotalArrayOfCalculationSequenceNumericComponent> for TaxSubtotalArrayOfCalculationSequenceNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfCalculationSequenceNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfCalculationSequenceNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSubtotalArrayOfCalculationSequenceNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CalculationSequenceNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CalculationSequenceNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CalculationSequenceNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CalculationSequenceNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSubtotalArrayOfPerUnitAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PerUnitAmount>,
}

impl AsMut<TaxSubtotalArrayOfPerUnitAmountComponent> for TaxSubtotalArrayOfPerUnitAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSubtotalArrayOfPerUnitAmountComponent> for TaxSubtotalArrayOfPerUnitAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfPerUnitAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfPerUnitAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSubtotalArrayOfPerUnitAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PerUnitAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PerUnitAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PerUnitAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PerUnitAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSubtotalArrayOfPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Percent>,
}

impl AsMut<TaxSubtotalArrayOfPercentComponent> for TaxSubtotalArrayOfPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSubtotalArrayOfPercentComponent> for TaxSubtotalArrayOfPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSubtotalArrayOfPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Percent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Percent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Percent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Percent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSubtotalArrayOfTaxAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxAmount>,
}

impl AsMut<TaxSubtotalArrayOfTaxAmountComponent> for TaxSubtotalArrayOfTaxAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSubtotalArrayOfTaxAmountComponent> for TaxSubtotalArrayOfTaxAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTaxAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTaxAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSubtotalArrayOfTaxAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSubtotalArrayOfTaxCategoryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxCategory>,
}

impl AsMut<TaxSubtotalArrayOfTaxCategoryComponent> for TaxSubtotalArrayOfTaxCategoryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSubtotalArrayOfTaxCategoryComponent> for TaxSubtotalArrayOfTaxCategoryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTaxCategoryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTaxCategoryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSubtotalArrayOfTaxCategoryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxCategory) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxCategory) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxCategory> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxCategory> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSubtotalArrayOfTaxableAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxableAmount>,
}

impl AsMut<TaxSubtotalArrayOfTaxableAmountComponent> for TaxSubtotalArrayOfTaxableAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSubtotalArrayOfTaxableAmountComponent> for TaxSubtotalArrayOfTaxableAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTaxableAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTaxableAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSubtotalArrayOfTaxableAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxableAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxableAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxableAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxableAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSubtotalArrayOfTierRangeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TierRange>,
}

impl AsMut<TaxSubtotalArrayOfTierRangeComponent> for TaxSubtotalArrayOfTierRangeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSubtotalArrayOfTierRangeComponent> for TaxSubtotalArrayOfTierRangeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTierRangeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTierRangeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSubtotalArrayOfTierRangeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TierRange) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TierRange) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TierRange> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TierRange> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSubtotalArrayOfTierRatePercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TierRatePercent>,
}

impl AsMut<TaxSubtotalArrayOfTierRatePercentComponent> for TaxSubtotalArrayOfTierRatePercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSubtotalArrayOfTierRatePercentComponent> for TaxSubtotalArrayOfTierRatePercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTierRatePercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTierRatePercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSubtotalArrayOfTierRatePercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TierRatePercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TierRatePercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TierRatePercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TierRatePercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxSubtotalArrayOfTransactionCurrencyTaxAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransactionCurrencyTaxAmount>,
}

impl AsMut<TaxSubtotalArrayOfTransactionCurrencyTaxAmountComponent> for TaxSubtotalArrayOfTransactionCurrencyTaxAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxSubtotalArrayOfTransactionCurrencyTaxAmountComponent> for TaxSubtotalArrayOfTransactionCurrencyTaxAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTransactionCurrencyTaxAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxSubtotalArrayOfTransactionCurrencyTaxAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxSubtotalArrayOfTransactionCurrencyTaxAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransactionCurrencyTaxAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransactionCurrencyTaxAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransactionCurrencyTaxAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransactionCurrencyTaxAmount> {
        self.items.iter()
    }
}


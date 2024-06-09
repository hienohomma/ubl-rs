use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AllowanceCharge {
    #[serde(rename = "AccountingCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<AllowanceChargeArrayOfAccountingCostComponent>,
    #[serde(rename = "AccountingCostCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<AllowanceChargeArrayOfAccountingCostCodeComponent>,
    #[serde(rename = "AllowanceChargeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge_reason: Option<AllowanceChargeArrayOfAllowanceChargeReasonComponent>,
    #[serde(rename = "AllowanceChargeReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge_reason_code: Option<AllowanceChargeArrayOfAllowanceChargeReasonCodeComponent>,
    #[serde(rename = "Amount")]
    pub amount: AllowanceChargeArrayOfAmountComponent,
    #[serde(rename = "BaseAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_amount: Option<AllowanceChargeArrayOfBaseAmountComponent>,
    #[serde(rename = "ChargeIndicator")]
    pub charge_indicator: AllowanceChargeArrayOfChargeIndicatorComponent,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<AllowanceChargeArrayOfIDComponent>,
    #[serde(rename = "MultiplierFactorNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier_factor_numeric: Option<AllowanceChargeArrayOfMultiplierFactorNumericComponent>,
    #[serde(rename = "PaymentMeans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_means: Option<AllowanceChargeArrayOfPaymentMeansComponent>,
    #[serde(rename = "PerUnitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_unit_amount: Option<AllowanceChargeArrayOfPerUnitAmountComponent>,
    #[serde(rename = "PrepaidIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_indicator: Option<AllowanceChargeArrayOfPrepaidIndicatorComponent>,
    #[serde(rename = "SequenceNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_numeric: Option<AllowanceChargeArrayOfSequenceNumericComponent>,
    #[serde(rename = "TaxCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_category: Option<AllowanceChargeArrayOfTaxCategoryComponent>,
    #[serde(rename = "TaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<AllowanceChargeArrayOfTaxTotalComponent>,
}

impl AsMut<AllowanceCharge> for AllowanceCharge {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceCharge> for AllowanceCharge {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.accounting_cost_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.accounting_cost_code", e));
            }
        }
        if let Some(v) = &self.accounting_cost {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.accounting_cost", e));
            }
        }
        if let Some(v) = &self.prepaid_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.prepaid_indicator", e));
            }
        }
        if let Err(e) = self.charge_indicator.validate() {
            return Err(UblError::component("AllowanceCharge.charge_indicator", e));
        }
        if let Some(v) = &self.allowance_charge_reason {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.allowance_charge_reason", e));
            }
        }
        if let Some(v) = &self.sequence_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.sequence_numeric", e));
            }
        }
        if let Some(v) = &self.base_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.base_amount", e));
            }
        }
        if let Some(v) = &self.multiplier_factor_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.multiplier_factor_numeric", e));
            }
        }
        if let Some(v) = &self.per_unit_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.per_unit_amount", e));
            }
        }
        if let Some(v) = &self.payment_means {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.payment_means", e));
            }
        }
        if let Some(v) = &self.tax_category {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.tax_category", e));
            }
        }
        if let Some(v) = &self.allowance_charge_reason_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.allowance_charge_reason_code", e));
            }
        }
        if let Err(e) = self.amount.validate() {
            return Err(UblError::component("AllowanceCharge.amount", e));
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.id", e));
            }
        }
        if let Some(v) = &self.tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AllowanceCharge.tax_total", e));
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

impl AllowanceCharge {
    pub fn title() -> &'static str {
        "Allowance Charge. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe information about a charge or discount as applied to a price component."
    }
    pub fn new(amount: AllowanceChargeArrayOfAmountComponent, charge_indicator: AllowanceChargeArrayOfChargeIndicatorComponent) -> Component<Self> {
        Component(Self {
            amount,
            charge_indicator,
            tax_category: None,
            accounting_cost: None,
            accounting_cost_code: None,
            prepaid_indicator: None,
            multiplier_factor_numeric: None,
            payment_means: None,
            id: None,
            allowance_charge_reason: None,
            per_unit_amount: None,
            base_amount: None,
            sequence_numeric: None,
            allowance_charge_reason_code: None,
            tax_total: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AllowanceChargeArrayOfAccountingCostComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCost>,
}

impl AsMut<AllowanceChargeArrayOfAccountingCostComponent> for AllowanceChargeArrayOfAccountingCostComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfAccountingCostComponent> for AllowanceChargeArrayOfAccountingCostComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfAccountingCostComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfAccountingCostComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfAccountingCostComponent {
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
pub struct AllowanceChargeArrayOfAccountingCostCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountingCostCode>,
}

impl AsMut<AllowanceChargeArrayOfAccountingCostCodeComponent> for AllowanceChargeArrayOfAccountingCostCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfAccountingCostCodeComponent> for AllowanceChargeArrayOfAccountingCostCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfAccountingCostCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfAccountingCostCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfAccountingCostCodeComponent {
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
pub struct AllowanceChargeArrayOfAllowanceChargeReasonComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AllowanceChargeReason>,
}

impl AsMut<AllowanceChargeArrayOfAllowanceChargeReasonComponent> for AllowanceChargeArrayOfAllowanceChargeReasonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfAllowanceChargeReasonComponent> for AllowanceChargeArrayOfAllowanceChargeReasonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfAllowanceChargeReasonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfAllowanceChargeReasonComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AllowanceChargeReason) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AllowanceChargeReason) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AllowanceChargeReason> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AllowanceChargeReason> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AllowanceChargeArrayOfAllowanceChargeReasonCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AllowanceChargeReasonCode>,
}

impl AsMut<AllowanceChargeArrayOfAllowanceChargeReasonCodeComponent> for AllowanceChargeArrayOfAllowanceChargeReasonCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfAllowanceChargeReasonCodeComponent> for AllowanceChargeArrayOfAllowanceChargeReasonCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfAllowanceChargeReasonCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfAllowanceChargeReasonCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfAllowanceChargeReasonCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AllowanceChargeReasonCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AllowanceChargeReasonCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AllowanceChargeReasonCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AllowanceChargeReasonCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AllowanceChargeArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<AllowanceChargeArrayOfAmountComponent> for AllowanceChargeArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfAmountComponent> for AllowanceChargeArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Amount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Amount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Amount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Amount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AllowanceChargeArrayOfBaseAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BaseAmount>,
}

impl AsMut<AllowanceChargeArrayOfBaseAmountComponent> for AllowanceChargeArrayOfBaseAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfBaseAmountComponent> for AllowanceChargeArrayOfBaseAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfBaseAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfBaseAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfBaseAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BaseAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BaseAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BaseAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BaseAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AllowanceChargeArrayOfChargeIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ChargeIndicator>,
}

impl AsMut<AllowanceChargeArrayOfChargeIndicatorComponent> for AllowanceChargeArrayOfChargeIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfChargeIndicatorComponent> for AllowanceChargeArrayOfChargeIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfChargeIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfChargeIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfChargeIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ChargeIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ChargeIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ChargeIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ChargeIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AllowanceChargeArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<AllowanceChargeArrayOfIDComponent> for AllowanceChargeArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfIDComponent> for AllowanceChargeArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfIDComponent {
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
pub struct AllowanceChargeArrayOfMultiplierFactorNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MultiplierFactorNumeric>,
}

impl AsMut<AllowanceChargeArrayOfMultiplierFactorNumericComponent> for AllowanceChargeArrayOfMultiplierFactorNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfMultiplierFactorNumericComponent> for AllowanceChargeArrayOfMultiplierFactorNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfMultiplierFactorNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfMultiplierFactorNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfMultiplierFactorNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MultiplierFactorNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MultiplierFactorNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MultiplierFactorNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MultiplierFactorNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AllowanceChargeArrayOfPaymentMeansComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PaymentMeans>,
}

impl AsMut<AllowanceChargeArrayOfPaymentMeansComponent> for AllowanceChargeArrayOfPaymentMeansComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfPaymentMeansComponent> for AllowanceChargeArrayOfPaymentMeansComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfPaymentMeansComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfPaymentMeansComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PaymentMeans) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PaymentMeans) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PaymentMeans> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PaymentMeans> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AllowanceChargeArrayOfPerUnitAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PerUnitAmount>,
}

impl AsMut<AllowanceChargeArrayOfPerUnitAmountComponent> for AllowanceChargeArrayOfPerUnitAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfPerUnitAmountComponent> for AllowanceChargeArrayOfPerUnitAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfPerUnitAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfPerUnitAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfPerUnitAmountComponent {
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
pub struct AllowanceChargeArrayOfPrepaidIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PrepaidIndicator>,
}

impl AsMut<AllowanceChargeArrayOfPrepaidIndicatorComponent> for AllowanceChargeArrayOfPrepaidIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfPrepaidIndicatorComponent> for AllowanceChargeArrayOfPrepaidIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfPrepaidIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfPrepaidIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfPrepaidIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PrepaidIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PrepaidIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PrepaidIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PrepaidIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AllowanceChargeArrayOfSequenceNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric>,
}

impl AsMut<AllowanceChargeArrayOfSequenceNumericComponent> for AllowanceChargeArrayOfSequenceNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfSequenceNumericComponent> for AllowanceChargeArrayOfSequenceNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfSequenceNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfSequenceNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfSequenceNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SequenceNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SequenceNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AllowanceChargeArrayOfTaxCategoryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxCategory>,
}

impl AsMut<AllowanceChargeArrayOfTaxCategoryComponent> for AllowanceChargeArrayOfTaxCategoryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfTaxCategoryComponent> for AllowanceChargeArrayOfTaxCategoryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfTaxCategoryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfTaxCategoryComponent {
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
pub struct AllowanceChargeArrayOfTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxTotal>,
}

impl AsMut<AllowanceChargeArrayOfTaxTotalComponent> for AllowanceChargeArrayOfTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AllowanceChargeArrayOfTaxTotalComponent> for AllowanceChargeArrayOfTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfTaxTotalComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AllowanceChargeArrayOfTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AllowanceChargeArrayOfTaxTotalComponent {
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


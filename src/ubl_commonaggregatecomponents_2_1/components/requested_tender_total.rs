use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestedTenderTotal {
    #[serde(rename = "ApplicableTaxCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_tax_category: Option<RequestedTenderTotalArrayOfApplicableTaxCategoryComponent>,
    #[serde(rename = "AverageSubsequentContractAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_subsequent_contract_amount: Option<RequestedTenderTotalArrayOfAverageSubsequentContractAmountComponent>,
    #[serde(rename = "EstimatedOverallContractAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_overall_contract_amount: Option<RequestedTenderTotalArrayOfEstimatedOverallContractAmountComponent>,
    #[serde(rename = "MaximumAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_amount: Option<RequestedTenderTotalArrayOfMaximumAmountComponent>,
    #[serde(rename = "MinimumAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<RequestedTenderTotalArrayOfMinimumAmountComponent>,
    #[serde(rename = "MonetaryScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monetary_scope: Option<RequestedTenderTotalArrayOfMonetaryScopeComponent>,
    #[serde(rename = "TaxIncludedIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_included_indicator: Option<RequestedTenderTotalArrayOfTaxIncludedIndicatorComponent>,
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<RequestedTenderTotalArrayOfTotalAmountComponent>,
}

impl AsMut<RequestedTenderTotal> for RequestedTenderTotal {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestedTenderTotal> for RequestedTenderTotal {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.tax_included_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestedTenderTotal.tax_included_indicator", e));
            }
        }
        if let Some(v) = &self.estimated_overall_contract_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestedTenderTotal.estimated_overall_contract_amount", e));
            }
        }
        if let Some(v) = &self.minimum_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestedTenderTotal.minimum_amount", e));
            }
        }
        if let Some(v) = &self.monetary_scope {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestedTenderTotal.monetary_scope", e));
            }
        }
        if let Some(v) = &self.total_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestedTenderTotal.total_amount", e));
            }
        }
        if let Some(v) = &self.applicable_tax_category {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestedTenderTotal.applicable_tax_category", e));
            }
        }
        if let Some(v) = &self.average_subsequent_contract_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestedTenderTotal.average_subsequent_contract_amount", e));
            }
        }
        if let Some(v) = &self.maximum_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("RequestedTenderTotal.maximum_amount", e));
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

impl RequestedTenderTotal {
    pub fn title() -> &'static str {
        "Requested Tender Total. Details"
    }
    pub fn description() -> &'static str {
        "A class defining budgeted monetary amounts."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            average_subsequent_contract_amount: None,
            maximum_amount: None,
            tax_included_indicator: None,
            minimum_amount: None,
            total_amount: None,
            applicable_tax_category: None,
            estimated_overall_contract_amount: None,
            monetary_scope: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestedTenderTotalArrayOfApplicableTaxCategoryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ApplicableTaxCategory>,
}

impl AsMut<RequestedTenderTotalArrayOfApplicableTaxCategoryComponent> for RequestedTenderTotalArrayOfApplicableTaxCategoryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestedTenderTotalArrayOfApplicableTaxCategoryComponent> for RequestedTenderTotalArrayOfApplicableTaxCategoryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfApplicableTaxCategoryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestedTenderTotalArrayOfApplicableTaxCategoryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ApplicableTaxCategory) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ApplicableTaxCategory) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ApplicableTaxCategory> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ApplicableTaxCategory> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestedTenderTotalArrayOfAverageSubsequentContractAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AverageSubsequentContractAmount>,
}

impl AsMut<RequestedTenderTotalArrayOfAverageSubsequentContractAmountComponent> for RequestedTenderTotalArrayOfAverageSubsequentContractAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestedTenderTotalArrayOfAverageSubsequentContractAmountComponent> for RequestedTenderTotalArrayOfAverageSubsequentContractAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfAverageSubsequentContractAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfAverageSubsequentContractAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestedTenderTotalArrayOfAverageSubsequentContractAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AverageSubsequentContractAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AverageSubsequentContractAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AverageSubsequentContractAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AverageSubsequentContractAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestedTenderTotalArrayOfEstimatedOverallContractAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EstimatedOverallContractAmount>,
}

impl AsMut<RequestedTenderTotalArrayOfEstimatedOverallContractAmountComponent> for RequestedTenderTotalArrayOfEstimatedOverallContractAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestedTenderTotalArrayOfEstimatedOverallContractAmountComponent> for RequestedTenderTotalArrayOfEstimatedOverallContractAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfEstimatedOverallContractAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfEstimatedOverallContractAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestedTenderTotalArrayOfEstimatedOverallContractAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EstimatedOverallContractAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EstimatedOverallContractAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EstimatedOverallContractAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EstimatedOverallContractAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestedTenderTotalArrayOfMaximumAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumAmount>,
}

impl AsMut<RequestedTenderTotalArrayOfMaximumAmountComponent> for RequestedTenderTotalArrayOfMaximumAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestedTenderTotalArrayOfMaximumAmountComponent> for RequestedTenderTotalArrayOfMaximumAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfMaximumAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfMaximumAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestedTenderTotalArrayOfMaximumAmountComponent {
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
pub struct RequestedTenderTotalArrayOfMinimumAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumAmount>,
}

impl AsMut<RequestedTenderTotalArrayOfMinimumAmountComponent> for RequestedTenderTotalArrayOfMinimumAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestedTenderTotalArrayOfMinimumAmountComponent> for RequestedTenderTotalArrayOfMinimumAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfMinimumAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfMinimumAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestedTenderTotalArrayOfMinimumAmountComponent {
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
pub struct RequestedTenderTotalArrayOfMonetaryScopeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MonetaryScope>,
}

impl AsMut<RequestedTenderTotalArrayOfMonetaryScopeComponent> for RequestedTenderTotalArrayOfMonetaryScopeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestedTenderTotalArrayOfMonetaryScopeComponent> for RequestedTenderTotalArrayOfMonetaryScopeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfMonetaryScopeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestedTenderTotalArrayOfMonetaryScopeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MonetaryScope) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MonetaryScope) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MonetaryScope> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MonetaryScope> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestedTenderTotalArrayOfTaxIncludedIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxIncludedIndicator>,
}

impl AsMut<RequestedTenderTotalArrayOfTaxIncludedIndicatorComponent> for RequestedTenderTotalArrayOfTaxIncludedIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestedTenderTotalArrayOfTaxIncludedIndicatorComponent> for RequestedTenderTotalArrayOfTaxIncludedIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfTaxIncludedIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfTaxIncludedIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestedTenderTotalArrayOfTaxIncludedIndicatorComponent {
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
pub struct RequestedTenderTotalArrayOfTotalAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalAmount>,
}

impl AsMut<RequestedTenderTotalArrayOfTotalAmountComponent> for RequestedTenderTotalArrayOfTotalAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<RequestedTenderTotalArrayOfTotalAmountComponent> for RequestedTenderTotalArrayOfTotalAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfTotalAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("RequestedTenderTotalArrayOfTotalAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl RequestedTenderTotalArrayOfTotalAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalAmount> {
        self.items.iter()
    }
}


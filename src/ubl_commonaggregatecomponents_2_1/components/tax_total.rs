use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaxTotal {
    #[serde(rename = "RoundingAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rounding_amount: Option<TaxTotalArrayOfRoundingAmountComponent>,
    #[serde(rename = "TaxAmount")]
    pub tax_amount: TaxTotalArrayOfTaxAmountComponent,
    #[serde(rename = "TaxEvidenceIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_evidence_indicator: Option<TaxTotalArrayOfTaxEvidenceIndicatorComponent>,
    #[serde(rename = "TaxIncludedIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_included_indicator: Option<TaxTotalArrayOfTaxIncludedIndicatorComponent>,
    #[serde(rename = "TaxSubtotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_subtotal: Option<TaxTotalArrayOfTaxSubtotalComponent>,
}

impl AsMut<TaxTotal> for TaxTotal {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxTotal> for TaxTotal {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.rounding_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxTotal.rounding_amount", e));
            }
        }
        if let Err(e) = self.tax_amount.validate() {
            return Err(UblError::component("TaxTotal.tax_amount", e));
        }
        if let Some(v) = &self.tax_evidence_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxTotal.tax_evidence_indicator", e));
            }
        }
        if let Some(v) = &self.tax_subtotal {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxTotal.tax_subtotal", e));
            }
        }
        if let Some(v) = &self.tax_included_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxTotal.tax_included_indicator", e));
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

impl TaxTotal {
    pub fn title() -> &'static str {
        "Tax Total. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the total tax for a particular taxation scheme."
    }
    pub fn new(tax_amount: TaxTotalArrayOfTaxAmountComponent) -> Component<Self> {
        Component(Self {
            tax_subtotal: None,
            rounding_amount: None,
            tax_evidence_indicator: None,
            tax_amount,
            tax_included_indicator: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxTotalArrayOfRoundingAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RoundingAmount>,
}

impl AsMut<TaxTotalArrayOfRoundingAmountComponent> for TaxTotalArrayOfRoundingAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxTotalArrayOfRoundingAmountComponent> for TaxTotalArrayOfRoundingAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxTotalArrayOfRoundingAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxTotalArrayOfRoundingAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxTotalArrayOfRoundingAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RoundingAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RoundingAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RoundingAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RoundingAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxTotalArrayOfTaxAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxAmount>,
}

impl AsMut<TaxTotalArrayOfTaxAmountComponent> for TaxTotalArrayOfTaxAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxTotalArrayOfTaxAmountComponent> for TaxTotalArrayOfTaxAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxTotalArrayOfTaxAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxTotalArrayOfTaxAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxTotalArrayOfTaxAmountComponent {
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
pub struct TaxTotalArrayOfTaxEvidenceIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxEvidenceIndicator>,
}

impl AsMut<TaxTotalArrayOfTaxEvidenceIndicatorComponent> for TaxTotalArrayOfTaxEvidenceIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxTotalArrayOfTaxEvidenceIndicatorComponent> for TaxTotalArrayOfTaxEvidenceIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxTotalArrayOfTaxEvidenceIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxTotalArrayOfTaxEvidenceIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxTotalArrayOfTaxEvidenceIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxEvidenceIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxEvidenceIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxEvidenceIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxEvidenceIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxTotalArrayOfTaxIncludedIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxIncludedIndicator>,
}

impl AsMut<TaxTotalArrayOfTaxIncludedIndicatorComponent> for TaxTotalArrayOfTaxIncludedIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxTotalArrayOfTaxIncludedIndicatorComponent> for TaxTotalArrayOfTaxIncludedIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxTotalArrayOfTaxIncludedIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxTotalArrayOfTaxIncludedIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxTotalArrayOfTaxIncludedIndicatorComponent {
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
pub struct TaxTotalArrayOfTaxSubtotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxSubtotal>,
}

impl AsMut<TaxTotalArrayOfTaxSubtotalComponent> for TaxTotalArrayOfTaxSubtotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxTotalArrayOfTaxSubtotalComponent> for TaxTotalArrayOfTaxSubtotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TaxTotalArrayOfTaxSubtotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxTotalArrayOfTaxSubtotalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxSubtotal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxSubtotal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxSubtotal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxSubtotal> {
        self.items.iter()
    }
}


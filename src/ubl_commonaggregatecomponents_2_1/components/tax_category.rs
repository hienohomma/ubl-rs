use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaxCategory {
    #[serde(rename = "BaseUnitMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_unit_measure: Option<TaxCategoryArrayOfBaseUnitMeasureComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TaxCategoryArrayOfIDComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TaxCategoryArrayOfNameComponent>,
    #[serde(rename = "PerUnitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_unit_amount: Option<TaxCategoryArrayOfPerUnitAmountComponent>,
    #[serde(rename = "Percent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<TaxCategoryArrayOfPercentComponent>,
    #[serde(rename = "TaxExemptionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exemption_reason: Option<TaxCategoryArrayOfTaxExemptionReasonComponent>,
    #[serde(rename = "TaxExemptionReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exemption_reason_code: Option<TaxCategoryArrayOfTaxExemptionReasonCodeComponent>,
    #[serde(rename = "TaxScheme")]
    pub tax_scheme: TaxCategoryArrayOfTaxSchemeComponent,
    #[serde(rename = "TierRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_range: Option<TaxCategoryArrayOfTierRangeComponent>,
    #[serde(rename = "TierRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_rate_percent: Option<TaxCategoryArrayOfTierRatePercentComponent>,
}

impl AsMut<TaxCategory> for TaxCategory {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxCategory> for TaxCategory {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxCategory.id", e));
            }
        }
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxCategory.name", e));
            }
        }
        if let Some(v) = &self.tax_exemption_reason_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxCategory.tax_exemption_reason_code", e));
            }
        }
        if let Some(v) = &self.per_unit_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxCategory.per_unit_amount", e));
            }
        }
        if let Some(v) = &self.tax_exemption_reason {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxCategory.tax_exemption_reason", e));
            }
        }
        if let Some(v) = &self.base_unit_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxCategory.base_unit_measure", e));
            }
        }
        if let Some(v) = &self.tier_range {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxCategory.tier_range", e));
            }
        }
        if let Some(v) = &self.tier_rate_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxCategory.tier_rate_percent", e));
            }
        }
        if let Err(e) = self.tax_scheme.validate() {
            return Err(UblError::component("TaxCategory.tax_scheme", e));
        }
        if let Some(v) = &self.percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TaxCategory.percent", e));
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

impl TaxCategory {
    pub fn title() -> &'static str {
        "Tax Category. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe one of the tax categories within a taxation scheme (e.g., High Rate VAT, Low Rate VAT)."
    }
    pub fn new(tax_scheme: TaxCategoryArrayOfTaxSchemeComponent) -> Component<Self> {
        Component(Self {
            base_unit_measure: None,
            id: None,
            tax_scheme,
            tier_rate_percent: None,
            per_unit_amount: None,
            tax_exemption_reason: None,
            tax_exemption_reason_code: None,
            name: None,
            tier_range: None,
            percent: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxCategoryArrayOfBaseUnitMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BaseUnitMeasure>,
}

impl AsMut<TaxCategoryArrayOfBaseUnitMeasureComponent> for TaxCategoryArrayOfBaseUnitMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxCategoryArrayOfBaseUnitMeasureComponent> for TaxCategoryArrayOfBaseUnitMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfBaseUnitMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfBaseUnitMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxCategoryArrayOfBaseUnitMeasureComponent {
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
pub struct TaxCategoryArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<TaxCategoryArrayOfIDComponent> for TaxCategoryArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxCategoryArrayOfIDComponent> for TaxCategoryArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxCategoryArrayOfIDComponent {
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
pub struct TaxCategoryArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<TaxCategoryArrayOfNameComponent> for TaxCategoryArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxCategoryArrayOfNameComponent> for TaxCategoryArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxCategoryArrayOfNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Name) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Name) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxCategoryArrayOfPerUnitAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PerUnitAmount>,
}

impl AsMut<TaxCategoryArrayOfPerUnitAmountComponent> for TaxCategoryArrayOfPerUnitAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxCategoryArrayOfPerUnitAmountComponent> for TaxCategoryArrayOfPerUnitAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfPerUnitAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfPerUnitAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxCategoryArrayOfPerUnitAmountComponent {
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
pub struct TaxCategoryArrayOfPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Percent>,
}

impl AsMut<TaxCategoryArrayOfPercentComponent> for TaxCategoryArrayOfPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxCategoryArrayOfPercentComponent> for TaxCategoryArrayOfPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxCategoryArrayOfPercentComponent {
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
pub struct TaxCategoryArrayOfTaxExemptionReasonComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxExemptionReason>,
}

impl AsMut<TaxCategoryArrayOfTaxExemptionReasonComponent> for TaxCategoryArrayOfTaxExemptionReasonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxCategoryArrayOfTaxExemptionReasonComponent> for TaxCategoryArrayOfTaxExemptionReasonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfTaxExemptionReasonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxCategoryArrayOfTaxExemptionReasonComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxExemptionReason) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxExemptionReason) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxExemptionReason> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxExemptionReason> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxCategoryArrayOfTaxExemptionReasonCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxExemptionReasonCode>,
}

impl AsMut<TaxCategoryArrayOfTaxExemptionReasonCodeComponent> for TaxCategoryArrayOfTaxExemptionReasonCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxCategoryArrayOfTaxExemptionReasonCodeComponent> for TaxCategoryArrayOfTaxExemptionReasonCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfTaxExemptionReasonCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfTaxExemptionReasonCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxCategoryArrayOfTaxExemptionReasonCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxExemptionReasonCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxExemptionReasonCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxExemptionReasonCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxExemptionReasonCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxCategoryArrayOfTaxSchemeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxScheme>,
}

impl AsMut<TaxCategoryArrayOfTaxSchemeComponent> for TaxCategoryArrayOfTaxSchemeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxCategoryArrayOfTaxSchemeComponent> for TaxCategoryArrayOfTaxSchemeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfTaxSchemeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfTaxSchemeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxCategoryArrayOfTaxSchemeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxScheme) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxScheme) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxScheme> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxScheme> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TaxCategoryArrayOfTierRangeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TierRange>,
}

impl AsMut<TaxCategoryArrayOfTierRangeComponent> for TaxCategoryArrayOfTierRangeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxCategoryArrayOfTierRangeComponent> for TaxCategoryArrayOfTierRangeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfTierRangeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfTierRangeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxCategoryArrayOfTierRangeComponent {
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
pub struct TaxCategoryArrayOfTierRatePercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TierRatePercent>,
}

impl AsMut<TaxCategoryArrayOfTierRatePercentComponent> for TaxCategoryArrayOfTierRatePercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TaxCategoryArrayOfTierRatePercentComponent> for TaxCategoryArrayOfTierRatePercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfTierRatePercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TaxCategoryArrayOfTierRatePercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TaxCategoryArrayOfTierRatePercentComponent {
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


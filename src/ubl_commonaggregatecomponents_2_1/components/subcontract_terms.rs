use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubcontractTerms {
    #[serde(rename = "Amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<SubcontractTermsArrayOfAmountComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SubcontractTermsArrayOfDescriptionComponent>,
    #[serde(rename = "MaximumPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_percent: Option<SubcontractTermsArrayOfMaximumPercentComponent>,
    #[serde(rename = "MinimumPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_percent: Option<SubcontractTermsArrayOfMinimumPercentComponent>,
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<SubcontractTermsArrayOfRateComponent>,
    #[serde(rename = "SubcontractingConditionsCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcontracting_conditions_code: Option<SubcontractTermsArrayOfSubcontractingConditionsCodeComponent>,
    #[serde(rename = "UnknownPriceIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_price_indicator: Option<SubcontractTermsArrayOfUnknownPriceIndicatorComponent>,
}

impl AsMut<SubcontractTerms> for SubcontractTerms {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubcontractTerms> for SubcontractTerms {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.minimum_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubcontractTerms.minimum_percent", e));
            }
        }
        if let Some(v) = &self.maximum_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubcontractTerms.maximum_percent", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubcontractTerms.description", e));
            }
        }
        if let Some(v) = &self.rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubcontractTerms.rate", e));
            }
        }
        if let Some(v) = &self.unknown_price_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubcontractTerms.unknown_price_indicator", e));
            }
        }
        if let Some(v) = &self.subcontracting_conditions_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubcontractTerms.subcontracting_conditions_code", e));
            }
        }
        if let Some(v) = &self.amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubcontractTerms.amount", e));
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

impl SubcontractTerms {
    pub fn title() -> &'static str {
        "Subcontract Terms. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe subcontract terms for a tendering process."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            description: None,
            amount: None,
            maximum_percent: None,
            minimum_percent: None,
            rate: None,
            unknown_price_indicator: None,
            subcontracting_conditions_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubcontractTermsArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<SubcontractTermsArrayOfAmountComponent> for SubcontractTermsArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubcontractTermsArrayOfAmountComponent> for SubcontractTermsArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubcontractTermsArrayOfAmountComponent {
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
pub struct SubcontractTermsArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<SubcontractTermsArrayOfDescriptionComponent> for SubcontractTermsArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubcontractTermsArrayOfDescriptionComponent> for SubcontractTermsArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubcontractTermsArrayOfDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Description) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Description) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubcontractTermsArrayOfMaximumPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumPercent>,
}

impl AsMut<SubcontractTermsArrayOfMaximumPercentComponent> for SubcontractTermsArrayOfMaximumPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubcontractTermsArrayOfMaximumPercentComponent> for SubcontractTermsArrayOfMaximumPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfMaximumPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfMaximumPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubcontractTermsArrayOfMaximumPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumPercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumPercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumPercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumPercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubcontractTermsArrayOfMinimumPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumPercent>,
}

impl AsMut<SubcontractTermsArrayOfMinimumPercentComponent> for SubcontractTermsArrayOfMinimumPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubcontractTermsArrayOfMinimumPercentComponent> for SubcontractTermsArrayOfMinimumPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfMinimumPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfMinimumPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubcontractTermsArrayOfMinimumPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumPercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumPercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumPercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumPercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubcontractTermsArrayOfRateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Rate>,
}

impl AsMut<SubcontractTermsArrayOfRateComponent> for SubcontractTermsArrayOfRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubcontractTermsArrayOfRateComponent> for SubcontractTermsArrayOfRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfRateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubcontractTermsArrayOfRateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Rate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Rate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Rate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Rate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubcontractTermsArrayOfSubcontractingConditionsCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SubcontractingConditionsCode>,
}

impl AsMut<SubcontractTermsArrayOfSubcontractingConditionsCodeComponent> for SubcontractTermsArrayOfSubcontractingConditionsCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubcontractTermsArrayOfSubcontractingConditionsCodeComponent> for SubcontractTermsArrayOfSubcontractingConditionsCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfSubcontractingConditionsCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfSubcontractingConditionsCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubcontractTermsArrayOfSubcontractingConditionsCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SubcontractingConditionsCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SubcontractingConditionsCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SubcontractingConditionsCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SubcontractingConditionsCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubcontractTermsArrayOfUnknownPriceIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UnknownPriceIndicator>,
}

impl AsMut<SubcontractTermsArrayOfUnknownPriceIndicatorComponent> for SubcontractTermsArrayOfUnknownPriceIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubcontractTermsArrayOfUnknownPriceIndicatorComponent> for SubcontractTermsArrayOfUnknownPriceIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfUnknownPriceIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubcontractTermsArrayOfUnknownPriceIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubcontractTermsArrayOfUnknownPriceIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::UnknownPriceIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::UnknownPriceIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::UnknownPriceIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::UnknownPriceIndicator> {
        self.items.iter()
    }
}


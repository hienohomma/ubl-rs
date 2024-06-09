use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EconomicOperatorShortList {
    #[serde(rename = "ExpectedQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_quantity: Option<EconomicOperatorShortListArrayOfExpectedQuantityComponent>,
    #[serde(rename = "LimitationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitation_description: Option<EconomicOperatorShortListArrayOfLimitationDescriptionComponent>,
    #[serde(rename = "MaximumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_quantity: Option<EconomicOperatorShortListArrayOfMaximumQuantityComponent>,
    #[serde(rename = "MinimumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_quantity: Option<EconomicOperatorShortListArrayOfMinimumQuantityComponent>,
    #[serde(rename = "PreSelectedParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_selected_party: Option<EconomicOperatorShortListArrayOfPreSelectedPartyComponent>,
}

impl AsMut<EconomicOperatorShortList> for EconomicOperatorShortList {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EconomicOperatorShortList> for EconomicOperatorShortList {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.minimum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EconomicOperatorShortList.minimum_quantity", e));
            }
        }
        if let Some(v) = &self.expected_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EconomicOperatorShortList.expected_quantity", e));
            }
        }
        if let Some(v) = &self.limitation_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EconomicOperatorShortList.limitation_description", e));
            }
        }
        if let Some(v) = &self.maximum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EconomicOperatorShortList.maximum_quantity", e));
            }
        }
        if let Some(v) = &self.pre_selected_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EconomicOperatorShortList.pre_selected_party", e));
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

impl EconomicOperatorShortList {
    pub fn title() -> &'static str {
        "Economic Operator Short List. Details"
    }
    pub fn description() -> &'static str {
        "A class to provide information about the preselection of a short list of economic operators for consideration as possible candidates in a tendering process."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            minimum_quantity: None,
            limitation_description: None,
            maximum_quantity: None,
            expected_quantity: None,
            pre_selected_party: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EconomicOperatorShortListArrayOfExpectedQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExpectedQuantity>,
}

impl AsMut<EconomicOperatorShortListArrayOfExpectedQuantityComponent> for EconomicOperatorShortListArrayOfExpectedQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EconomicOperatorShortListArrayOfExpectedQuantityComponent> for EconomicOperatorShortListArrayOfExpectedQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EconomicOperatorShortListArrayOfExpectedQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EconomicOperatorShortListArrayOfExpectedQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EconomicOperatorShortListArrayOfExpectedQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExpectedQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExpectedQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExpectedQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExpectedQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EconomicOperatorShortListArrayOfLimitationDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LimitationDescription>,
}

impl AsMut<EconomicOperatorShortListArrayOfLimitationDescriptionComponent> for EconomicOperatorShortListArrayOfLimitationDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EconomicOperatorShortListArrayOfLimitationDescriptionComponent> for EconomicOperatorShortListArrayOfLimitationDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EconomicOperatorShortListArrayOfLimitationDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EconomicOperatorShortListArrayOfLimitationDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LimitationDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LimitationDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LimitationDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LimitationDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EconomicOperatorShortListArrayOfMaximumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity>,
}

impl AsMut<EconomicOperatorShortListArrayOfMaximumQuantityComponent> for EconomicOperatorShortListArrayOfMaximumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EconomicOperatorShortListArrayOfMaximumQuantityComponent> for EconomicOperatorShortListArrayOfMaximumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EconomicOperatorShortListArrayOfMaximumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EconomicOperatorShortListArrayOfMaximumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EconomicOperatorShortListArrayOfMaximumQuantityComponent {
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
pub struct EconomicOperatorShortListArrayOfMinimumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity>,
}

impl AsMut<EconomicOperatorShortListArrayOfMinimumQuantityComponent> for EconomicOperatorShortListArrayOfMinimumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EconomicOperatorShortListArrayOfMinimumQuantityComponent> for EconomicOperatorShortListArrayOfMinimumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EconomicOperatorShortListArrayOfMinimumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EconomicOperatorShortListArrayOfMinimumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EconomicOperatorShortListArrayOfMinimumQuantityComponent {
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
pub struct EconomicOperatorShortListArrayOfPreSelectedPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PreSelectedParty>,
}

impl AsMut<EconomicOperatorShortListArrayOfPreSelectedPartyComponent> for EconomicOperatorShortListArrayOfPreSelectedPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EconomicOperatorShortListArrayOfPreSelectedPartyComponent> for EconomicOperatorShortListArrayOfPreSelectedPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EconomicOperatorShortListArrayOfPreSelectedPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EconomicOperatorShortListArrayOfPreSelectedPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PreSelectedParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PreSelectedParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PreSelectedParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PreSelectedParty> {
        self.items.iter()
    }
}


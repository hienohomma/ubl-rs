use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AwardingCriterionResponse {
    #[serde(rename = "Amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<AwardingCriterionResponseArrayOfAmountComponent>,
    #[serde(rename = "AwardingCriterionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarding_criterion_description: Option<AwardingCriterionResponseArrayOfAwardingCriterionDescriptionComponent>,
    #[serde(rename = "AwardingCriterionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarding_criterion_id: Option<AwardingCriterionResponseArrayOfAwardingCriterionIDComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<AwardingCriterionResponseArrayOfDescriptionComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<AwardingCriterionResponseArrayOfIDComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<AwardingCriterionResponseArrayOfQuantityComponent>,
    #[serde(rename = "SubordinateAwardingCriterionResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subordinate_awarding_criterion_response: Option<AwardingCriterionResponseArrayOfSubordinateAwardingCriterionResponseComponent>,
}

impl AsMut<AwardingCriterionResponse> for AwardingCriterionResponse {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionResponse> for AwardingCriterionResponse {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterionResponse.id", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterionResponse.description", e));
            }
        }
        if let Some(v) = &self.amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterionResponse.amount", e));
            }
        }
        if let Some(v) = &self.awarding_criterion_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterionResponse.awarding_criterion_description", e));
            }
        }
        if let Some(v) = &self.awarding_criterion_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterionResponse.awarding_criterion_id", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterionResponse.quantity", e));
            }
        }
        if let Some(v) = &self.subordinate_awarding_criterion_response {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterionResponse.subordinate_awarding_criterion_response", e));
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

impl AwardingCriterionResponse {
    pub fn title() -> &'static str {
        "Awarding Criterion Response. Details"
    }
    pub fn description() -> &'static str {
        "Defines the response for an awarding criterion from the tendering party."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            subordinate_awarding_criterion_response: None,
            quantity: None,
            id: None,
            awarding_criterion_description: None,
            awarding_criterion_id: None,
            amount: None,
            description: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingCriterionResponseArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<AwardingCriterionResponseArrayOfAmountComponent> for AwardingCriterionResponseArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionResponseArrayOfAmountComponent> for AwardingCriterionResponseArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionResponseArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionResponseArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionResponseArrayOfAmountComponent {
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
pub struct AwardingCriterionResponseArrayOfAwardingCriterionDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AwardingCriterionDescription>,
}

impl AsMut<AwardingCriterionResponseArrayOfAwardingCriterionDescriptionComponent> for AwardingCriterionResponseArrayOfAwardingCriterionDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionResponseArrayOfAwardingCriterionDescriptionComponent> for AwardingCriterionResponseArrayOfAwardingCriterionDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionResponseArrayOfAwardingCriterionDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionResponseArrayOfAwardingCriterionDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AwardingCriterionDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AwardingCriterionDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AwardingCriterionDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AwardingCriterionDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingCriterionResponseArrayOfAwardingCriterionIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AwardingCriterionID>,
}

impl AsMut<AwardingCriterionResponseArrayOfAwardingCriterionIDComponent> for AwardingCriterionResponseArrayOfAwardingCriterionIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionResponseArrayOfAwardingCriterionIDComponent> for AwardingCriterionResponseArrayOfAwardingCriterionIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionResponseArrayOfAwardingCriterionIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionResponseArrayOfAwardingCriterionIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionResponseArrayOfAwardingCriterionIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AwardingCriterionID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AwardingCriterionID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AwardingCriterionID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AwardingCriterionID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingCriterionResponseArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<AwardingCriterionResponseArrayOfDescriptionComponent> for AwardingCriterionResponseArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionResponseArrayOfDescriptionComponent> for AwardingCriterionResponseArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionResponseArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionResponseArrayOfDescriptionComponent {
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
pub struct AwardingCriterionResponseArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<AwardingCriterionResponseArrayOfIDComponent> for AwardingCriterionResponseArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionResponseArrayOfIDComponent> for AwardingCriterionResponseArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionResponseArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionResponseArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionResponseArrayOfIDComponent {
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
pub struct AwardingCriterionResponseArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<AwardingCriterionResponseArrayOfQuantityComponent> for AwardingCriterionResponseArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionResponseArrayOfQuantityComponent> for AwardingCriterionResponseArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionResponseArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionResponseArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionResponseArrayOfQuantityComponent {
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
pub struct AwardingCriterionResponseArrayOfSubordinateAwardingCriterionResponseComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubordinateAwardingCriterionResponse>,
}

impl AsMut<AwardingCriterionResponseArrayOfSubordinateAwardingCriterionResponseComponent> for AwardingCriterionResponseArrayOfSubordinateAwardingCriterionResponseComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionResponseArrayOfSubordinateAwardingCriterionResponseComponent> for AwardingCriterionResponseArrayOfSubordinateAwardingCriterionResponseComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionResponseArrayOfSubordinateAwardingCriterionResponseComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionResponseArrayOfSubordinateAwardingCriterionResponseComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubordinateAwardingCriterionResponse) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubordinateAwardingCriterionResponse) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubordinateAwardingCriterionResponse> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubordinateAwardingCriterionResponse> {
        self.items.iter()
    }
}


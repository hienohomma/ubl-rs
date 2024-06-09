use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AwardingCriterion {
    #[serde(rename = "AwardingCriterionTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarding_criterion_type_code: Option<AwardingCriterionArrayOfAwardingCriterionTypeCodeComponent>,
    #[serde(rename = "CalculationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_expression: Option<AwardingCriterionArrayOfCalculationExpressionComponent>,
    #[serde(rename = "CalculationExpressionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_expression_code: Option<AwardingCriterionArrayOfCalculationExpressionCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<AwardingCriterionArrayOfDescriptionComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<AwardingCriterionArrayOfIDComponent>,
    #[serde(rename = "MaximumAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_amount: Option<AwardingCriterionArrayOfMaximumAmountComponent>,
    #[serde(rename = "MaximumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_quantity: Option<AwardingCriterionArrayOfMaximumQuantityComponent>,
    #[serde(rename = "MinimumAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<AwardingCriterionArrayOfMinimumAmountComponent>,
    #[serde(rename = "MinimumImprovementBid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_improvement_bid: Option<AwardingCriterionArrayOfMinimumImprovementBidComponent>,
    #[serde(rename = "MinimumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_quantity: Option<AwardingCriterionArrayOfMinimumQuantityComponent>,
    #[serde(rename = "SubordinateAwardingCriterion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subordinate_awarding_criterion: Option<AwardingCriterionArrayOfSubordinateAwardingCriterionComponent>,
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<AwardingCriterionArrayOfWeightComponent>,
    #[serde(rename = "WeightNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_numeric: Option<AwardingCriterionArrayOfWeightNumericComponent>,
}

impl AsMut<AwardingCriterion> for AwardingCriterion {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterion> for AwardingCriterion {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.maximum_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.maximum_amount", e));
            }
        }
        if let Some(v) = &self.maximum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.maximum_quantity", e));
            }
        }
        if let Some(v) = &self.subordinate_awarding_criterion {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.subordinate_awarding_criterion", e));
            }
        }
        if let Some(v) = &self.calculation_expression_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.calculation_expression_code", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.description", e));
            }
        }
        if let Some(v) = &self.minimum_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.minimum_amount", e));
            }
        }
        if let Some(v) = &self.minimum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.minimum_quantity", e));
            }
        }
        if let Some(v) = &self.minimum_improvement_bid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.minimum_improvement_bid", e));
            }
        }
        if let Some(v) = &self.weight {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.weight", e));
            }
        }
        if let Some(v) = &self.calculation_expression {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.calculation_expression", e));
            }
        }
        if let Some(v) = &self.weight_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.weight_numeric", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.id", e));
            }
        }
        if let Some(v) = &self.awarding_criterion_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingCriterion.awarding_criterion_type_code", e));
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

impl AwardingCriterion {
    pub fn title() -> &'static str {
        "Awarding Criterion. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a criterion from the contracting party that will be taken into account when awarding a contract. An awarding criterion can be objective, when it can be evaluated following a formula, or subjective, when human analysis is required."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            minimum_improvement_bid: None,
            id: None,
            calculation_expression: None,
            maximum_amount: None,
            minimum_amount: None,
            subordinate_awarding_criterion: None,
            calculation_expression_code: None,
            maximum_quantity: None,
            minimum_quantity: None,
            weight: None,
            weight_numeric: None,
            description: None,
            awarding_criterion_type_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingCriterionArrayOfAwardingCriterionTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AwardingCriterionTypeCode>,
}

impl AsMut<AwardingCriterionArrayOfAwardingCriterionTypeCodeComponent> for AwardingCriterionArrayOfAwardingCriterionTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfAwardingCriterionTypeCodeComponent> for AwardingCriterionArrayOfAwardingCriterionTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfAwardingCriterionTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfAwardingCriterionTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfAwardingCriterionTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AwardingCriterionTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AwardingCriterionTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AwardingCriterionTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AwardingCriterionTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingCriterionArrayOfCalculationExpressionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CalculationExpression>,
}

impl AsMut<AwardingCriterionArrayOfCalculationExpressionComponent> for AwardingCriterionArrayOfCalculationExpressionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfCalculationExpressionComponent> for AwardingCriterionArrayOfCalculationExpressionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfCalculationExpressionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfCalculationExpressionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CalculationExpression) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CalculationExpression) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CalculationExpression> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CalculationExpression> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingCriterionArrayOfCalculationExpressionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CalculationExpressionCode>,
}

impl AsMut<AwardingCriterionArrayOfCalculationExpressionCodeComponent> for AwardingCriterionArrayOfCalculationExpressionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfCalculationExpressionCodeComponent> for AwardingCriterionArrayOfCalculationExpressionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfCalculationExpressionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfCalculationExpressionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfCalculationExpressionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CalculationExpressionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CalculationExpressionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CalculationExpressionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CalculationExpressionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingCriterionArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<AwardingCriterionArrayOfDescriptionComponent> for AwardingCriterionArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfDescriptionComponent> for AwardingCriterionArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfDescriptionComponent {
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
pub struct AwardingCriterionArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<AwardingCriterionArrayOfIDComponent> for AwardingCriterionArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfIDComponent> for AwardingCriterionArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfIDComponent {
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
pub struct AwardingCriterionArrayOfMaximumAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumAmount>,
}

impl AsMut<AwardingCriterionArrayOfMaximumAmountComponent> for AwardingCriterionArrayOfMaximumAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfMaximumAmountComponent> for AwardingCriterionArrayOfMaximumAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfMaximumAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfMaximumAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfMaximumAmountComponent {
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
pub struct AwardingCriterionArrayOfMaximumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity>,
}

impl AsMut<AwardingCriterionArrayOfMaximumQuantityComponent> for AwardingCriterionArrayOfMaximumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfMaximumQuantityComponent> for AwardingCriterionArrayOfMaximumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfMaximumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfMaximumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfMaximumQuantityComponent {
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
pub struct AwardingCriterionArrayOfMinimumAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumAmount>,
}

impl AsMut<AwardingCriterionArrayOfMinimumAmountComponent> for AwardingCriterionArrayOfMinimumAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfMinimumAmountComponent> for AwardingCriterionArrayOfMinimumAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfMinimumAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfMinimumAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfMinimumAmountComponent {
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
pub struct AwardingCriterionArrayOfMinimumImprovementBidComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumImprovementBid>,
}

impl AsMut<AwardingCriterionArrayOfMinimumImprovementBidComponent> for AwardingCriterionArrayOfMinimumImprovementBidComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfMinimumImprovementBidComponent> for AwardingCriterionArrayOfMinimumImprovementBidComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfMinimumImprovementBidComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfMinimumImprovementBidComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumImprovementBid) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumImprovementBid) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumImprovementBid> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumImprovementBid> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingCriterionArrayOfMinimumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity>,
}

impl AsMut<AwardingCriterionArrayOfMinimumQuantityComponent> for AwardingCriterionArrayOfMinimumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfMinimumQuantityComponent> for AwardingCriterionArrayOfMinimumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfMinimumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfMinimumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfMinimumQuantityComponent {
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
pub struct AwardingCriterionArrayOfSubordinateAwardingCriterionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubordinateAwardingCriterion>,
}

impl AsMut<AwardingCriterionArrayOfSubordinateAwardingCriterionComponent> for AwardingCriterionArrayOfSubordinateAwardingCriterionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfSubordinateAwardingCriterionComponent> for AwardingCriterionArrayOfSubordinateAwardingCriterionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfSubordinateAwardingCriterionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfSubordinateAwardingCriterionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubordinateAwardingCriterion) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubordinateAwardingCriterion) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubordinateAwardingCriterion> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubordinateAwardingCriterion> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingCriterionArrayOfWeightComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Weight>,
}

impl AsMut<AwardingCriterionArrayOfWeightComponent> for AwardingCriterionArrayOfWeightComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfWeightComponent> for AwardingCriterionArrayOfWeightComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfWeightComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfWeightComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Weight) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Weight) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Weight> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Weight> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingCriterionArrayOfWeightNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::WeightNumeric>,
}

impl AsMut<AwardingCriterionArrayOfWeightNumericComponent> for AwardingCriterionArrayOfWeightNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingCriterionArrayOfWeightNumericComponent> for AwardingCriterionArrayOfWeightNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfWeightNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingCriterionArrayOfWeightNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingCriterionArrayOfWeightNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::WeightNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::WeightNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::WeightNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::WeightNumeric> {
        self.items.iter()
    }
}


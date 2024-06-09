use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvaluationCriterion {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<EvaluationCriterionArrayOfDescriptionComponent>,
    #[serde(rename = "DurationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_period: Option<EvaluationCriterionArrayOfDurationPeriodComponent>,
    #[serde(rename = "EvaluationCriterionTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_criterion_type_code: Option<EvaluationCriterionArrayOfEvaluationCriterionTypeCodeComponent>,
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<EvaluationCriterionArrayOfExpressionComponent>,
    #[serde(rename = "ExpressionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_code: Option<EvaluationCriterionArrayOfExpressionCodeComponent>,
    #[serde(rename = "SuggestedEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_evidence: Option<EvaluationCriterionArrayOfSuggestedEvidenceComponent>,
    #[serde(rename = "ThresholdAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_amount: Option<EvaluationCriterionArrayOfThresholdAmountComponent>,
    #[serde(rename = "ThresholdQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_quantity: Option<EvaluationCriterionArrayOfThresholdQuantityComponent>,
}

impl AsMut<EvaluationCriterion> for EvaluationCriterion {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvaluationCriterion> for EvaluationCriterion {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.threshold_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EvaluationCriterion.threshold_amount", e));
            }
        }
        if let Some(v) = &self.evaluation_criterion_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EvaluationCriterion.evaluation_criterion_type_code", e));
            }
        }
        if let Some(v) = &self.duration_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EvaluationCriterion.duration_period", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EvaluationCriterion.description", e));
            }
        }
        if let Some(v) = &self.suggested_evidence {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EvaluationCriterion.suggested_evidence", e));
            }
        }
        if let Some(v) = &self.threshold_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EvaluationCriterion.threshold_quantity", e));
            }
        }
        if let Some(v) = &self.expression_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EvaluationCriterion.expression_code", e));
            }
        }
        if let Some(v) = &self.expression {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EvaluationCriterion.expression", e));
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

impl EvaluationCriterion {
    pub fn title() -> &'static str {
        "Evaluation Criterion. Details"
    }
    pub fn description() -> &'static str {
        "A class defining the required criterion for a tenderer to be elligible in a tendering process. "
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            threshold_amount: None,
            expression: None,
            description: None,
            expression_code: None,
            suggested_evidence: None,
            evaluation_criterion_type_code: None,
            threshold_quantity: None,
            duration_period: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvaluationCriterionArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<EvaluationCriterionArrayOfDescriptionComponent> for EvaluationCriterionArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvaluationCriterionArrayOfDescriptionComponent> for EvaluationCriterionArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvaluationCriterionArrayOfDescriptionComponent {
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
pub struct EvaluationCriterionArrayOfDurationPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DurationPeriod>,
}

impl AsMut<EvaluationCriterionArrayOfDurationPeriodComponent> for EvaluationCriterionArrayOfDurationPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvaluationCriterionArrayOfDurationPeriodComponent> for EvaluationCriterionArrayOfDurationPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfDurationPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfDurationPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvaluationCriterionArrayOfDurationPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DurationPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DurationPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DurationPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DurationPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvaluationCriterionArrayOfEvaluationCriterionTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EvaluationCriterionTypeCode>,
}

impl AsMut<EvaluationCriterionArrayOfEvaluationCriterionTypeCodeComponent> for EvaluationCriterionArrayOfEvaluationCriterionTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvaluationCriterionArrayOfEvaluationCriterionTypeCodeComponent> for EvaluationCriterionArrayOfEvaluationCriterionTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfEvaluationCriterionTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfEvaluationCriterionTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvaluationCriterionArrayOfEvaluationCriterionTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EvaluationCriterionTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EvaluationCriterionTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EvaluationCriterionTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EvaluationCriterionTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvaluationCriterionArrayOfExpressionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Expression>,
}

impl AsMut<EvaluationCriterionArrayOfExpressionComponent> for EvaluationCriterionArrayOfExpressionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvaluationCriterionArrayOfExpressionComponent> for EvaluationCriterionArrayOfExpressionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfExpressionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvaluationCriterionArrayOfExpressionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Expression) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Expression) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Expression> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Expression> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvaluationCriterionArrayOfExpressionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExpressionCode>,
}

impl AsMut<EvaluationCriterionArrayOfExpressionCodeComponent> for EvaluationCriterionArrayOfExpressionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvaluationCriterionArrayOfExpressionCodeComponent> for EvaluationCriterionArrayOfExpressionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfExpressionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfExpressionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvaluationCriterionArrayOfExpressionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExpressionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExpressionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExpressionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExpressionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvaluationCriterionArrayOfSuggestedEvidenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SuggestedEvidence>,
}

impl AsMut<EvaluationCriterionArrayOfSuggestedEvidenceComponent> for EvaluationCriterionArrayOfSuggestedEvidenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvaluationCriterionArrayOfSuggestedEvidenceComponent> for EvaluationCriterionArrayOfSuggestedEvidenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfSuggestedEvidenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvaluationCriterionArrayOfSuggestedEvidenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SuggestedEvidence) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SuggestedEvidence) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SuggestedEvidence> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SuggestedEvidence> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvaluationCriterionArrayOfThresholdAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ThresholdAmount>,
}

impl AsMut<EvaluationCriterionArrayOfThresholdAmountComponent> for EvaluationCriterionArrayOfThresholdAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvaluationCriterionArrayOfThresholdAmountComponent> for EvaluationCriterionArrayOfThresholdAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfThresholdAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfThresholdAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvaluationCriterionArrayOfThresholdAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ThresholdAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ThresholdAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ThresholdAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ThresholdAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EvaluationCriterionArrayOfThresholdQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ThresholdQuantity>,
}

impl AsMut<EvaluationCriterionArrayOfThresholdQuantityComponent> for EvaluationCriterionArrayOfThresholdQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EvaluationCriterionArrayOfThresholdQuantityComponent> for EvaluationCriterionArrayOfThresholdQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfThresholdQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EvaluationCriterionArrayOfThresholdQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EvaluationCriterionArrayOfThresholdQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ThresholdQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ThresholdQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ThresholdQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ThresholdQuantity> {
        self.items.iter()
    }
}


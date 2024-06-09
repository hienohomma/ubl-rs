use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrameworkAgreement {
    #[serde(rename = "DurationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_period: Option<FrameworkAgreementArrayOfDurationPeriodComponent>,
    #[serde(rename = "ExpectedOperatorQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_operator_quantity: Option<FrameworkAgreementArrayOfExpectedOperatorQuantityComponent>,
    #[serde(rename = "Frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<FrameworkAgreementArrayOfFrequencyComponent>,
    #[serde(rename = "Justification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<FrameworkAgreementArrayOfJustificationComponent>,
    #[serde(rename = "MaximumOperatorQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_operator_quantity: Option<FrameworkAgreementArrayOfMaximumOperatorQuantityComponent>,
    #[serde(rename = "SubsequentProcessTenderRequirement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsequent_process_tender_requirement: Option<FrameworkAgreementArrayOfSubsequentProcessTenderRequirementComponent>,
}

impl AsMut<FrameworkAgreement> for FrameworkAgreement {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FrameworkAgreement> for FrameworkAgreement {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.expected_operator_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FrameworkAgreement.expected_operator_quantity", e));
            }
        }
        if let Some(v) = &self.frequency {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FrameworkAgreement.frequency", e));
            }
        }
        if let Some(v) = &self.duration_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FrameworkAgreement.duration_period", e));
            }
        }
        if let Some(v) = &self.maximum_operator_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FrameworkAgreement.maximum_operator_quantity", e));
            }
        }
        if let Some(v) = &self.justification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FrameworkAgreement.justification", e));
            }
        }
        if let Some(v) = &self.subsequent_process_tender_requirement {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FrameworkAgreement.subsequent_process_tender_requirement", e));
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

impl FrameworkAgreement {
    pub fn title() -> &'static str {
        "Framework Agreement. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a tendering framework agreement."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            maximum_operator_quantity: None,
            subsequent_process_tender_requirement: None,
            frequency: None,
            duration_period: None,
            expected_operator_quantity: None,
            justification: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FrameworkAgreementArrayOfDurationPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DurationPeriod>,
}

impl AsMut<FrameworkAgreementArrayOfDurationPeriodComponent> for FrameworkAgreementArrayOfDurationPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FrameworkAgreementArrayOfDurationPeriodComponent> for FrameworkAgreementArrayOfDurationPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FrameworkAgreementArrayOfDurationPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FrameworkAgreementArrayOfDurationPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FrameworkAgreementArrayOfDurationPeriodComponent {
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
pub struct FrameworkAgreementArrayOfExpectedOperatorQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExpectedOperatorQuantity>,
}

impl AsMut<FrameworkAgreementArrayOfExpectedOperatorQuantityComponent> for FrameworkAgreementArrayOfExpectedOperatorQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FrameworkAgreementArrayOfExpectedOperatorQuantityComponent> for FrameworkAgreementArrayOfExpectedOperatorQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FrameworkAgreementArrayOfExpectedOperatorQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FrameworkAgreementArrayOfExpectedOperatorQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FrameworkAgreementArrayOfExpectedOperatorQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExpectedOperatorQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExpectedOperatorQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExpectedOperatorQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExpectedOperatorQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FrameworkAgreementArrayOfFrequencyComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Frequency>,
}

impl AsMut<FrameworkAgreementArrayOfFrequencyComponent> for FrameworkAgreementArrayOfFrequencyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FrameworkAgreementArrayOfFrequencyComponent> for FrameworkAgreementArrayOfFrequencyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("FrameworkAgreementArrayOfFrequencyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FrameworkAgreementArrayOfFrequencyComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Frequency) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Frequency) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Frequency> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Frequency> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FrameworkAgreementArrayOfJustificationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Justification>,
}

impl AsMut<FrameworkAgreementArrayOfJustificationComponent> for FrameworkAgreementArrayOfJustificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FrameworkAgreementArrayOfJustificationComponent> for FrameworkAgreementArrayOfJustificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("FrameworkAgreementArrayOfJustificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FrameworkAgreementArrayOfJustificationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Justification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Justification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Justification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Justification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FrameworkAgreementArrayOfMaximumOperatorQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumOperatorQuantity>,
}

impl AsMut<FrameworkAgreementArrayOfMaximumOperatorQuantityComponent> for FrameworkAgreementArrayOfMaximumOperatorQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FrameworkAgreementArrayOfMaximumOperatorQuantityComponent> for FrameworkAgreementArrayOfMaximumOperatorQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FrameworkAgreementArrayOfMaximumOperatorQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FrameworkAgreementArrayOfMaximumOperatorQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FrameworkAgreementArrayOfMaximumOperatorQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumOperatorQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumOperatorQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumOperatorQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumOperatorQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FrameworkAgreementArrayOfSubsequentProcessTenderRequirementComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubsequentProcessTenderRequirement>,
}

impl AsMut<FrameworkAgreementArrayOfSubsequentProcessTenderRequirementComponent> for FrameworkAgreementArrayOfSubsequentProcessTenderRequirementComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FrameworkAgreementArrayOfSubsequentProcessTenderRequirementComponent> for FrameworkAgreementArrayOfSubsequentProcessTenderRequirementComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("FrameworkAgreementArrayOfSubsequentProcessTenderRequirementComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FrameworkAgreementArrayOfSubsequentProcessTenderRequirementComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubsequentProcessTenderRequirement) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubsequentProcessTenderRequirement) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubsequentProcessTenderRequirement> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubsequentProcessTenderRequirement> {
        self.items.iter()
    }
}


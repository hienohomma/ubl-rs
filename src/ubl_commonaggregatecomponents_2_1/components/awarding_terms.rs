use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AwardingTerms {
    #[serde(rename = "AwardingCriterion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarding_criterion: Option<AwardingTermsArrayOfAwardingCriterionComponent>,
    #[serde(rename = "BindingOnBuyerIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_on_buyer_indicator: Option<AwardingTermsArrayOfBindingOnBuyerIndicatorComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<AwardingTermsArrayOfDescriptionComponent>,
    #[serde(rename = "FollowupContractIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followup_contract_indicator: Option<AwardingTermsArrayOfFollowupContractIndicatorComponent>,
    #[serde(rename = "LowTendersDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_tenders_description: Option<AwardingTermsArrayOfLowTendersDescriptionComponent>,
    #[serde(rename = "PaymentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_description: Option<AwardingTermsArrayOfPaymentDescriptionComponent>,
    #[serde(rename = "PrizeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<AwardingTermsArrayOfPrizeDescriptionComponent>,
    #[serde(rename = "PrizeIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_indicator: Option<AwardingTermsArrayOfPrizeIndicatorComponent>,
    #[serde(rename = "TechnicalCommitteeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_committee_description: Option<AwardingTermsArrayOfTechnicalCommitteeDescriptionComponent>,
    #[serde(rename = "TechnicalCommitteePerson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_committee_person: Option<AwardingTermsArrayOfTechnicalCommitteePersonComponent>,
    #[serde(rename = "WeightingAlgorithmCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighting_algorithm_code: Option<AwardingTermsArrayOfWeightingAlgorithmCodeComponent>,
}

impl AsMut<AwardingTerms> for AwardingTerms {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTerms> for AwardingTerms {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.weighting_algorithm_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingTerms.weighting_algorithm_code", e));
            }
        }
        if let Some(v) = &self.binding_on_buyer_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingTerms.binding_on_buyer_indicator", e));
            }
        }
        if let Some(v) = &self.awarding_criterion {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingTerms.awarding_criterion", e));
            }
        }
        if let Some(v) = &self.low_tenders_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingTerms.low_tenders_description", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingTerms.description", e));
            }
        }
        if let Some(v) = &self.payment_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingTerms.payment_description", e));
            }
        }
        if let Some(v) = &self.technical_committee_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingTerms.technical_committee_description", e));
            }
        }
        if let Some(v) = &self.followup_contract_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingTerms.followup_contract_indicator", e));
            }
        }
        if let Some(v) = &self.prize_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingTerms.prize_indicator", e));
            }
        }
        if let Some(v) = &self.technical_committee_person {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingTerms.technical_committee_person", e));
            }
        }
        if let Some(v) = &self.prize_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AwardingTerms.prize_description", e));
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

impl AwardingTerms {
    pub fn title() -> &'static str {
        "Awarding Terms. Details"
    }
    pub fn description() -> &'static str {
        "A class to define the terms for awarding a contract."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            awarding_criterion: None,
            description: None,
            followup_contract_indicator: None,
            prize_indicator: None,
            binding_on_buyer_indicator: None,
            weighting_algorithm_code: None,
            technical_committee_person: None,
            low_tenders_description: None,
            technical_committee_description: None,
            prize_description: None,
            payment_description: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingTermsArrayOfAwardingCriterionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AwardingCriterion>,
}

impl AsMut<AwardingTermsArrayOfAwardingCriterionComponent> for AwardingTermsArrayOfAwardingCriterionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTermsArrayOfAwardingCriterionComponent> for AwardingTermsArrayOfAwardingCriterionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfAwardingCriterionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingTermsArrayOfAwardingCriterionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AwardingCriterion) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AwardingCriterion) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AwardingCriterion> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AwardingCriterion> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingTermsArrayOfBindingOnBuyerIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BindingOnBuyerIndicator>,
}

impl AsMut<AwardingTermsArrayOfBindingOnBuyerIndicatorComponent> for AwardingTermsArrayOfBindingOnBuyerIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTermsArrayOfBindingOnBuyerIndicatorComponent> for AwardingTermsArrayOfBindingOnBuyerIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfBindingOnBuyerIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfBindingOnBuyerIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingTermsArrayOfBindingOnBuyerIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BindingOnBuyerIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BindingOnBuyerIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BindingOnBuyerIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BindingOnBuyerIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingTermsArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<AwardingTermsArrayOfDescriptionComponent> for AwardingTermsArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTermsArrayOfDescriptionComponent> for AwardingTermsArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingTermsArrayOfDescriptionComponent {
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
pub struct AwardingTermsArrayOfFollowupContractIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FollowupContractIndicator>,
}

impl AsMut<AwardingTermsArrayOfFollowupContractIndicatorComponent> for AwardingTermsArrayOfFollowupContractIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTermsArrayOfFollowupContractIndicatorComponent> for AwardingTermsArrayOfFollowupContractIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfFollowupContractIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfFollowupContractIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingTermsArrayOfFollowupContractIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FollowupContractIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FollowupContractIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FollowupContractIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FollowupContractIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingTermsArrayOfLowTendersDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LowTendersDescription>,
}

impl AsMut<AwardingTermsArrayOfLowTendersDescriptionComponent> for AwardingTermsArrayOfLowTendersDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTermsArrayOfLowTendersDescriptionComponent> for AwardingTermsArrayOfLowTendersDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfLowTendersDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingTermsArrayOfLowTendersDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LowTendersDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LowTendersDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LowTendersDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LowTendersDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingTermsArrayOfPaymentDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentDescription>,
}

impl AsMut<AwardingTermsArrayOfPaymentDescriptionComponent> for AwardingTermsArrayOfPaymentDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTermsArrayOfPaymentDescriptionComponent> for AwardingTermsArrayOfPaymentDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfPaymentDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingTermsArrayOfPaymentDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaymentDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaymentDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaymentDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaymentDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingTermsArrayOfPrizeDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PrizeDescription>,
}

impl AsMut<AwardingTermsArrayOfPrizeDescriptionComponent> for AwardingTermsArrayOfPrizeDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTermsArrayOfPrizeDescriptionComponent> for AwardingTermsArrayOfPrizeDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfPrizeDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingTermsArrayOfPrizeDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PrizeDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PrizeDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PrizeDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PrizeDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingTermsArrayOfPrizeIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PrizeIndicator>,
}

impl AsMut<AwardingTermsArrayOfPrizeIndicatorComponent> for AwardingTermsArrayOfPrizeIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTermsArrayOfPrizeIndicatorComponent> for AwardingTermsArrayOfPrizeIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfPrizeIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfPrizeIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingTermsArrayOfPrizeIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PrizeIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PrizeIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PrizeIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PrizeIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingTermsArrayOfTechnicalCommitteeDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TechnicalCommitteeDescription>,
}

impl AsMut<AwardingTermsArrayOfTechnicalCommitteeDescriptionComponent> for AwardingTermsArrayOfTechnicalCommitteeDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTermsArrayOfTechnicalCommitteeDescriptionComponent> for AwardingTermsArrayOfTechnicalCommitteeDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfTechnicalCommitteeDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingTermsArrayOfTechnicalCommitteeDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TechnicalCommitteeDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TechnicalCommitteeDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TechnicalCommitteeDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TechnicalCommitteeDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingTermsArrayOfTechnicalCommitteePersonComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TechnicalCommitteePerson>,
}

impl AsMut<AwardingTermsArrayOfTechnicalCommitteePersonComponent> for AwardingTermsArrayOfTechnicalCommitteePersonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTermsArrayOfTechnicalCommitteePersonComponent> for AwardingTermsArrayOfTechnicalCommitteePersonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfTechnicalCommitteePersonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingTermsArrayOfTechnicalCommitteePersonComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TechnicalCommitteePerson) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TechnicalCommitteePerson) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TechnicalCommitteePerson> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TechnicalCommitteePerson> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AwardingTermsArrayOfWeightingAlgorithmCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::WeightingAlgorithmCode>,
}

impl AsMut<AwardingTermsArrayOfWeightingAlgorithmCodeComponent> for AwardingTermsArrayOfWeightingAlgorithmCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AwardingTermsArrayOfWeightingAlgorithmCodeComponent> for AwardingTermsArrayOfWeightingAlgorithmCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfWeightingAlgorithmCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AwardingTermsArrayOfWeightingAlgorithmCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AwardingTermsArrayOfWeightingAlgorithmCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::WeightingAlgorithmCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::WeightingAlgorithmCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::WeightingAlgorithmCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::WeightingAlgorithmCode> {
        self.items.iter()
    }
}


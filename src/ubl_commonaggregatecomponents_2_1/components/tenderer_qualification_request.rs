use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TendererQualificationRequest {
    #[serde(rename = "CompanyLegalForm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_legal_form: Option<TendererQualificationRequestArrayOfCompanyLegalFormComponent>,
    #[serde(rename = "CompanyLegalFormCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_legal_form_code: Option<TendererQualificationRequestArrayOfCompanyLegalFormCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TendererQualificationRequestArrayOfDescriptionComponent>,
    #[serde(rename = "EconomicOperatorRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_operator_role: Option<TendererQualificationRequestArrayOfEconomicOperatorRoleComponent>,
    #[serde(rename = "EmployeeQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_quantity: Option<TendererQualificationRequestArrayOfEmployeeQuantityComponent>,
    #[serde(rename = "FinancialEvaluationCriterion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_evaluation_criterion: Option<TendererQualificationRequestArrayOfFinancialEvaluationCriterionComponent>,
    #[serde(rename = "OperatingYearsQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_years_quantity: Option<TendererQualificationRequestArrayOfOperatingYearsQuantityComponent>,
    #[serde(rename = "PersonalSituation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_situation: Option<TendererQualificationRequestArrayOfPersonalSituationComponent>,
    #[serde(rename = "RequiredBusinessClassificationScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_business_classification_scheme: Option<TendererQualificationRequestArrayOfRequiredBusinessClassificationSchemeComponent>,
    #[serde(rename = "SpecificTendererRequirement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specific_tenderer_requirement: Option<TendererQualificationRequestArrayOfSpecificTendererRequirementComponent>,
    #[serde(rename = "TechnicalEvaluationCriterion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_evaluation_criterion: Option<TendererQualificationRequestArrayOfTechnicalEvaluationCriterionComponent>,
}

impl AsMut<TendererQualificationRequest> for TendererQualificationRequest {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequest> for TendererQualificationRequest {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.employee_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererQualificationRequest.employee_quantity", e));
            }
        }
        if let Some(v) = &self.financial_evaluation_criterion {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererQualificationRequest.financial_evaluation_criterion", e));
            }
        }
        if let Some(v) = &self.required_business_classification_scheme {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererQualificationRequest.required_business_classification_scheme", e));
            }
        }
        if let Some(v) = &self.personal_situation {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererQualificationRequest.personal_situation", e));
            }
        }
        if let Some(v) = &self.technical_evaluation_criterion {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererQualificationRequest.technical_evaluation_criterion", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererQualificationRequest.description", e));
            }
        }
        if let Some(v) = &self.company_legal_form_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererQualificationRequest.company_legal_form_code", e));
            }
        }
        if let Some(v) = &self.economic_operator_role {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererQualificationRequest.economic_operator_role", e));
            }
        }
        if let Some(v) = &self.operating_years_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererQualificationRequest.operating_years_quantity", e));
            }
        }
        if let Some(v) = &self.company_legal_form {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererQualificationRequest.company_legal_form", e));
            }
        }
        if let Some(v) = &self.specific_tenderer_requirement {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererQualificationRequest.specific_tenderer_requirement", e));
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

impl TendererQualificationRequest {
    pub fn title() -> &'static str {
        "Tenderer Qualification Request. Details"
    }
    pub fn description() -> &'static str {
        "The evaluation that the Contracting Authority party requests to fulfill to the tenderers."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            description: None,
            employee_quantity: None,
            company_legal_form_code: None,
            operating_years_quantity: None,
            specific_tenderer_requirement: None,
            personal_situation: None,
            economic_operator_role: None,
            required_business_classification_scheme: None,
            technical_evaluation_criterion: None,
            company_legal_form: None,
            financial_evaluation_criterion: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererQualificationRequestArrayOfCompanyLegalFormComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CompanyLegalForm>,
}

impl AsMut<TendererQualificationRequestArrayOfCompanyLegalFormComponent> for TendererQualificationRequestArrayOfCompanyLegalFormComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequestArrayOfCompanyLegalFormComponent> for TendererQualificationRequestArrayOfCompanyLegalFormComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfCompanyLegalFormComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfCompanyLegalFormComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererQualificationRequestArrayOfCompanyLegalFormComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CompanyLegalForm) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CompanyLegalForm) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CompanyLegalForm> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CompanyLegalForm> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererQualificationRequestArrayOfCompanyLegalFormCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CompanyLegalFormCode>,
}

impl AsMut<TendererQualificationRequestArrayOfCompanyLegalFormCodeComponent> for TendererQualificationRequestArrayOfCompanyLegalFormCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequestArrayOfCompanyLegalFormCodeComponent> for TendererQualificationRequestArrayOfCompanyLegalFormCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfCompanyLegalFormCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfCompanyLegalFormCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererQualificationRequestArrayOfCompanyLegalFormCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CompanyLegalFormCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CompanyLegalFormCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CompanyLegalFormCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CompanyLegalFormCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererQualificationRequestArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<TendererQualificationRequestArrayOfDescriptionComponent> for TendererQualificationRequestArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequestArrayOfDescriptionComponent> for TendererQualificationRequestArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererQualificationRequestArrayOfDescriptionComponent {
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
pub struct TendererQualificationRequestArrayOfEconomicOperatorRoleComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EconomicOperatorRole>,
}

impl AsMut<TendererQualificationRequestArrayOfEconomicOperatorRoleComponent> for TendererQualificationRequestArrayOfEconomicOperatorRoleComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequestArrayOfEconomicOperatorRoleComponent> for TendererQualificationRequestArrayOfEconomicOperatorRoleComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfEconomicOperatorRoleComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererQualificationRequestArrayOfEconomicOperatorRoleComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EconomicOperatorRole) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EconomicOperatorRole) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EconomicOperatorRole> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EconomicOperatorRole> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererQualificationRequestArrayOfEmployeeQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EmployeeQuantity>,
}

impl AsMut<TendererQualificationRequestArrayOfEmployeeQuantityComponent> for TendererQualificationRequestArrayOfEmployeeQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequestArrayOfEmployeeQuantityComponent> for TendererQualificationRequestArrayOfEmployeeQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfEmployeeQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfEmployeeQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererQualificationRequestArrayOfEmployeeQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EmployeeQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EmployeeQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EmployeeQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EmployeeQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererQualificationRequestArrayOfFinancialEvaluationCriterionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FinancialEvaluationCriterion>,
}

impl AsMut<TendererQualificationRequestArrayOfFinancialEvaluationCriterionComponent> for TendererQualificationRequestArrayOfFinancialEvaluationCriterionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequestArrayOfFinancialEvaluationCriterionComponent> for TendererQualificationRequestArrayOfFinancialEvaluationCriterionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfFinancialEvaluationCriterionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererQualificationRequestArrayOfFinancialEvaluationCriterionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FinancialEvaluationCriterion) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FinancialEvaluationCriterion) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FinancialEvaluationCriterion> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FinancialEvaluationCriterion> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererQualificationRequestArrayOfOperatingYearsQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OperatingYearsQuantity>,
}

impl AsMut<TendererQualificationRequestArrayOfOperatingYearsQuantityComponent> for TendererQualificationRequestArrayOfOperatingYearsQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequestArrayOfOperatingYearsQuantityComponent> for TendererQualificationRequestArrayOfOperatingYearsQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfOperatingYearsQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfOperatingYearsQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererQualificationRequestArrayOfOperatingYearsQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OperatingYearsQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OperatingYearsQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OperatingYearsQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OperatingYearsQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererQualificationRequestArrayOfPersonalSituationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PersonalSituation>,
}

impl AsMut<TendererQualificationRequestArrayOfPersonalSituationComponent> for TendererQualificationRequestArrayOfPersonalSituationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequestArrayOfPersonalSituationComponent> for TendererQualificationRequestArrayOfPersonalSituationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfPersonalSituationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererQualificationRequestArrayOfPersonalSituationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PersonalSituation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PersonalSituation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PersonalSituation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PersonalSituation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererQualificationRequestArrayOfRequiredBusinessClassificationSchemeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequiredBusinessClassificationScheme>,
}

impl AsMut<TendererQualificationRequestArrayOfRequiredBusinessClassificationSchemeComponent> for TendererQualificationRequestArrayOfRequiredBusinessClassificationSchemeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequestArrayOfRequiredBusinessClassificationSchemeComponent> for TendererQualificationRequestArrayOfRequiredBusinessClassificationSchemeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfRequiredBusinessClassificationSchemeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererQualificationRequestArrayOfRequiredBusinessClassificationSchemeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequiredBusinessClassificationScheme) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequiredBusinessClassificationScheme) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequiredBusinessClassificationScheme> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequiredBusinessClassificationScheme> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererQualificationRequestArrayOfSpecificTendererRequirementComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SpecificTendererRequirement>,
}

impl AsMut<TendererQualificationRequestArrayOfSpecificTendererRequirementComponent> for TendererQualificationRequestArrayOfSpecificTendererRequirementComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequestArrayOfSpecificTendererRequirementComponent> for TendererQualificationRequestArrayOfSpecificTendererRequirementComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfSpecificTendererRequirementComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererQualificationRequestArrayOfSpecificTendererRequirementComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SpecificTendererRequirement) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SpecificTendererRequirement) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SpecificTendererRequirement> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SpecificTendererRequirement> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererQualificationRequestArrayOfTechnicalEvaluationCriterionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TechnicalEvaluationCriterion>,
}

impl AsMut<TendererQualificationRequestArrayOfTechnicalEvaluationCriterionComponent> for TendererQualificationRequestArrayOfTechnicalEvaluationCriterionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererQualificationRequestArrayOfTechnicalEvaluationCriterionComponent> for TendererQualificationRequestArrayOfTechnicalEvaluationCriterionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererQualificationRequestArrayOfTechnicalEvaluationCriterionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererQualificationRequestArrayOfTechnicalEvaluationCriterionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TechnicalEvaluationCriterion) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TechnicalEvaluationCriterion) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TechnicalEvaluationCriterion> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TechnicalEvaluationCriterion> {
        self.items.iter()
    }
}


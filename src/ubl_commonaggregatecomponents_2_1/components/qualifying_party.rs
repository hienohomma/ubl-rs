use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QualifyingParty {
    #[serde(rename = "BusinessClassificationEvidenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_classification_evidence_id: Option<QualifyingPartyArrayOfBusinessClassificationEvidenceIDComponent>,
    #[serde(rename = "BusinessClassificationScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_classification_scheme: Option<QualifyingPartyArrayOfBusinessClassificationSchemeComponent>,
    #[serde(rename = "BusinessIdentityEvidenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_identity_evidence_id: Option<QualifyingPartyArrayOfBusinessIdentityEvidenceIDComponent>,
    #[serde(rename = "CompletedTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_task: Option<QualifyingPartyArrayOfCompletedTaskComponent>,
    #[serde(rename = "Declaration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaration: Option<QualifyingPartyArrayOfDeclarationComponent>,
    #[serde(rename = "EconomicOperatorRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_operator_role: Option<QualifyingPartyArrayOfEconomicOperatorRoleComponent>,
    #[serde(rename = "EmployeeQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_quantity: Option<QualifyingPartyArrayOfEmployeeQuantityComponent>,
    #[serde(rename = "FinancialCapability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_capability: Option<QualifyingPartyArrayOfFinancialCapabilityComponent>,
    #[serde(rename = "OperatingYearsQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_years_quantity: Option<QualifyingPartyArrayOfOperatingYearsQuantityComponent>,
    #[serde(rename = "ParticipationPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participation_percent: Option<QualifyingPartyArrayOfParticipationPercentComponent>,
    #[serde(rename = "Party")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<QualifyingPartyArrayOfPartyComponent>,
    #[serde(rename = "PersonalSituation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_situation: Option<QualifyingPartyArrayOfPersonalSituationComponent>,
    #[serde(rename = "TechnicalCapability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_capability: Option<QualifyingPartyArrayOfTechnicalCapabilityComponent>,
    #[serde(rename = "TendererRoleCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenderer_role_code: Option<QualifyingPartyArrayOfTendererRoleCodeComponent>,
}

impl AsMut<QualifyingParty> for QualifyingParty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingParty> for QualifyingParty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.declaration {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.declaration", e));
            }
        }
        if let Some(v) = &self.employee_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.employee_quantity", e));
            }
        }
        if let Some(v) = &self.business_identity_evidence_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.business_identity_evidence_id", e));
            }
        }
        if let Some(v) = &self.technical_capability {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.technical_capability", e));
            }
        }
        if let Some(v) = &self.party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.party", e));
            }
        }
        if let Some(v) = &self.financial_capability {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.financial_capability", e));
            }
        }
        if let Some(v) = &self.tenderer_role_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.tenderer_role_code", e));
            }
        }
        if let Some(v) = &self.business_classification_evidence_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.business_classification_evidence_id", e));
            }
        }
        if let Some(v) = &self.business_classification_scheme {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.business_classification_scheme", e));
            }
        }
        if let Some(v) = &self.economic_operator_role {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.economic_operator_role", e));
            }
        }
        if let Some(v) = &self.personal_situation {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.personal_situation", e));
            }
        }
        if let Some(v) = &self.participation_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.participation_percent", e));
            }
        }
        if let Some(v) = &self.operating_years_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.operating_years_quantity", e));
            }
        }
        if let Some(v) = &self.completed_task {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("QualifyingParty.completed_task", e));
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

impl QualifyingParty {
    pub fn title() -> &'static str {
        "Qualifying Party. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the distinctive features or characteristics qualifying an economic operator to be a party in a tendering process (e.g., number of employees, number of operating units, type of business, technical and financial capabilities, completed projects)."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            participation_percent: None,
            declaration: None,
            personal_situation: None,
            employee_quantity: None,
            economic_operator_role: None,
            completed_task: None,
            party: None,
            tenderer_role_code: None,
            business_classification_scheme: None,
            business_identity_evidence_id: None,
            technical_capability: None,
            operating_years_quantity: None,
            financial_capability: None,
            business_classification_evidence_id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualifyingPartyArrayOfBusinessClassificationEvidenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BusinessClassificationEvidenceID>,
}

impl AsMut<QualifyingPartyArrayOfBusinessClassificationEvidenceIDComponent> for QualifyingPartyArrayOfBusinessClassificationEvidenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfBusinessClassificationEvidenceIDComponent> for QualifyingPartyArrayOfBusinessClassificationEvidenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfBusinessClassificationEvidenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfBusinessClassificationEvidenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfBusinessClassificationEvidenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BusinessClassificationEvidenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BusinessClassificationEvidenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BusinessClassificationEvidenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BusinessClassificationEvidenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualifyingPartyArrayOfBusinessClassificationSchemeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BusinessClassificationScheme>,
}

impl AsMut<QualifyingPartyArrayOfBusinessClassificationSchemeComponent> for QualifyingPartyArrayOfBusinessClassificationSchemeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfBusinessClassificationSchemeComponent> for QualifyingPartyArrayOfBusinessClassificationSchemeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfBusinessClassificationSchemeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfBusinessClassificationSchemeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfBusinessClassificationSchemeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BusinessClassificationScheme) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BusinessClassificationScheme) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BusinessClassificationScheme> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BusinessClassificationScheme> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualifyingPartyArrayOfBusinessIdentityEvidenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BusinessIdentityEvidenceID>,
}

impl AsMut<QualifyingPartyArrayOfBusinessIdentityEvidenceIDComponent> for QualifyingPartyArrayOfBusinessIdentityEvidenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfBusinessIdentityEvidenceIDComponent> for QualifyingPartyArrayOfBusinessIdentityEvidenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfBusinessIdentityEvidenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfBusinessIdentityEvidenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfBusinessIdentityEvidenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BusinessIdentityEvidenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BusinessIdentityEvidenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BusinessIdentityEvidenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BusinessIdentityEvidenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualifyingPartyArrayOfCompletedTaskComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CompletedTask>,
}

impl AsMut<QualifyingPartyArrayOfCompletedTaskComponent> for QualifyingPartyArrayOfCompletedTaskComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfCompletedTaskComponent> for QualifyingPartyArrayOfCompletedTaskComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfCompletedTaskComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfCompletedTaskComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CompletedTask) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CompletedTask) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CompletedTask> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CompletedTask> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualifyingPartyArrayOfDeclarationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Declaration>,
}

impl AsMut<QualifyingPartyArrayOfDeclarationComponent> for QualifyingPartyArrayOfDeclarationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfDeclarationComponent> for QualifyingPartyArrayOfDeclarationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfDeclarationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfDeclarationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Declaration) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Declaration) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Declaration> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Declaration> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualifyingPartyArrayOfEconomicOperatorRoleComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EconomicOperatorRole>,
}

impl AsMut<QualifyingPartyArrayOfEconomicOperatorRoleComponent> for QualifyingPartyArrayOfEconomicOperatorRoleComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfEconomicOperatorRoleComponent> for QualifyingPartyArrayOfEconomicOperatorRoleComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfEconomicOperatorRoleComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfEconomicOperatorRoleComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfEconomicOperatorRoleComponent {
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
pub struct QualifyingPartyArrayOfEmployeeQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EmployeeQuantity>,
}

impl AsMut<QualifyingPartyArrayOfEmployeeQuantityComponent> for QualifyingPartyArrayOfEmployeeQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfEmployeeQuantityComponent> for QualifyingPartyArrayOfEmployeeQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfEmployeeQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfEmployeeQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfEmployeeQuantityComponent {
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
pub struct QualifyingPartyArrayOfFinancialCapabilityComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FinancialCapability>,
}

impl AsMut<QualifyingPartyArrayOfFinancialCapabilityComponent> for QualifyingPartyArrayOfFinancialCapabilityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfFinancialCapabilityComponent> for QualifyingPartyArrayOfFinancialCapabilityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfFinancialCapabilityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfFinancialCapabilityComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FinancialCapability) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FinancialCapability) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FinancialCapability> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FinancialCapability> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualifyingPartyArrayOfOperatingYearsQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OperatingYearsQuantity>,
}

impl AsMut<QualifyingPartyArrayOfOperatingYearsQuantityComponent> for QualifyingPartyArrayOfOperatingYearsQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfOperatingYearsQuantityComponent> for QualifyingPartyArrayOfOperatingYearsQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfOperatingYearsQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfOperatingYearsQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfOperatingYearsQuantityComponent {
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
pub struct QualifyingPartyArrayOfParticipationPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ParticipationPercent>,
}

impl AsMut<QualifyingPartyArrayOfParticipationPercentComponent> for QualifyingPartyArrayOfParticipationPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfParticipationPercentComponent> for QualifyingPartyArrayOfParticipationPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfParticipationPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfParticipationPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfParticipationPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ParticipationPercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ParticipationPercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ParticipationPercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ParticipationPercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualifyingPartyArrayOfPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Party>,
}

impl AsMut<QualifyingPartyArrayOfPartyComponent> for QualifyingPartyArrayOfPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfPartyComponent> for QualifyingPartyArrayOfPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Party) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Party) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Party> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Party> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualifyingPartyArrayOfPersonalSituationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PersonalSituation>,
}

impl AsMut<QualifyingPartyArrayOfPersonalSituationComponent> for QualifyingPartyArrayOfPersonalSituationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfPersonalSituationComponent> for QualifyingPartyArrayOfPersonalSituationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfPersonalSituationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfPersonalSituationComponent {
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
pub struct QualifyingPartyArrayOfTechnicalCapabilityComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TechnicalCapability>,
}

impl AsMut<QualifyingPartyArrayOfTechnicalCapabilityComponent> for QualifyingPartyArrayOfTechnicalCapabilityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfTechnicalCapabilityComponent> for QualifyingPartyArrayOfTechnicalCapabilityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfTechnicalCapabilityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfTechnicalCapabilityComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TechnicalCapability) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TechnicalCapability) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TechnicalCapability> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TechnicalCapability> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct QualifyingPartyArrayOfTendererRoleCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TendererRoleCode>,
}

impl AsMut<QualifyingPartyArrayOfTendererRoleCodeComponent> for QualifyingPartyArrayOfTendererRoleCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<QualifyingPartyArrayOfTendererRoleCodeComponent> for QualifyingPartyArrayOfTendererRoleCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfTendererRoleCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("QualifyingPartyArrayOfTendererRoleCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl QualifyingPartyArrayOfTendererRoleCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TendererRoleCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TendererRoleCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TendererRoleCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TendererRoleCode> {
        self.items.iter()
    }
}


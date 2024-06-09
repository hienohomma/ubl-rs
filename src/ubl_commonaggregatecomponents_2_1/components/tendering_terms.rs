use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenderingTerms {
    #[serde(rename = "AcceptedVariantsDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_variants_description: Option<TenderingTermsArrayOfAcceptedVariantsDescriptionComponent>,
    #[serde(rename = "AdditionalConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_conditions: Option<TenderingTermsArrayOfAdditionalConditionsComponent>,
    #[serde(rename = "AdditionalInformationParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information_party: Option<TenderingTermsArrayOfAdditionalInformationPartyComponent>,
    #[serde(rename = "AllowedSubcontractTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_subcontract_terms: Option<TenderingTermsArrayOfAllowedSubcontractTermsComponent>,
    #[serde(rename = "AppealTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appeal_terms: Option<TenderingTermsArrayOfAppealTermsComponent>,
    #[serde(rename = "AwardingMethodTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarding_method_type_code: Option<TenderingTermsArrayOfAwardingMethodTypeCodeComponent>,
    #[serde(rename = "AwardingTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarding_terms: Option<TenderingTermsArrayOfAwardingTermsComponent>,
    #[serde(rename = "BudgetAccountLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_account_line: Option<TenderingTermsArrayOfBudgetAccountLineComponent>,
    #[serde(rename = "CallForTendersDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_for_tenders_document_reference: Option<TenderingTermsArrayOfCallForTendersDocumentReferenceComponent>,
    #[serde(rename = "ContractAcceptancePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_acceptance_period: Option<TenderingTermsArrayOfContractAcceptancePeriodComponent>,
    #[serde(rename = "ContractExecutionRequirement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_execution_requirement: Option<TenderingTermsArrayOfContractExecutionRequirementComponent>,
    #[serde(rename = "ContractResponsibleParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_responsible_party: Option<TenderingTermsArrayOfContractResponsiblePartyComponent>,
    #[serde(rename = "ContractualDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contractual_document_reference: Option<TenderingTermsArrayOfContractualDocumentReferenceComponent>,
    #[serde(rename = "DocumentProviderParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_provider_party: Option<TenderingTermsArrayOfDocumentProviderPartyComponent>,
    #[serde(rename = "DocumentationFeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_fee_amount: Option<TenderingTermsArrayOfDocumentationFeeAmountComponent>,
    #[serde(rename = "EconomicOperatorRegistryURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_operator_registry_uri: Option<TenderingTermsArrayOfEconomicOperatorRegistryURIComponent>,
    #[serde(rename = "EmploymentLegislationDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_legislation_document_reference: Option<TenderingTermsArrayOfEmploymentLegislationDocumentReferenceComponent>,
    #[serde(rename = "EnvironmentalLegislationDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environmental_legislation_document_reference: Option<TenderingTermsArrayOfEnvironmentalLegislationDocumentReferenceComponent>,
    #[serde(rename = "FiscalLegislationDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiscal_legislation_document_reference: Option<TenderingTermsArrayOfFiscalLegislationDocumentReferenceComponent>,
    #[serde(rename = "FundingProgram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_program: Option<TenderingTermsArrayOfFundingProgramComponent>,
    #[serde(rename = "FundingProgramCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_program_code: Option<TenderingTermsArrayOfFundingProgramCodeComponent>,
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<TenderingTermsArrayOfLanguageComponent>,
    #[serde(rename = "LatestSecurityClearanceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_security_clearance_date: Option<TenderingTermsArrayOfLatestSecurityClearanceDateComponent>,
    #[serde(rename = "MaximumAdvertisementAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_advertisement_amount: Option<TenderingTermsArrayOfMaximumAdvertisementAmountComponent>,
    #[serde(rename = "MaximumVariantQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_variant_quantity: Option<TenderingTermsArrayOfMaximumVariantQuantityComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<TenderingTermsArrayOfNoteComponent>,
    #[serde(rename = "OtherConditionsIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_conditions_indicator: Option<TenderingTermsArrayOfOtherConditionsIndicatorComponent>,
    #[serde(rename = "PaymentFrequencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_frequency_code: Option<TenderingTermsArrayOfPaymentFrequencyCodeComponent>,
    #[serde(rename = "PaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<TenderingTermsArrayOfPaymentTermsComponent>,
    #[serde(rename = "PenaltyClause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_clause: Option<TenderingTermsArrayOfPenaltyClauseComponent>,
    #[serde(rename = "PriceEvaluationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_evaluation_code: Option<TenderingTermsArrayOfPriceEvaluationCodeComponent>,
    #[serde(rename = "PriceRevisionFormulaDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_revision_formula_description: Option<TenderingTermsArrayOfPriceRevisionFormulaDescriptionComponent>,
    #[serde(rename = "ProcurementLegislationDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procurement_legislation_document_reference: Option<TenderingTermsArrayOfProcurementLegislationDocumentReferenceComponent>,
    #[serde(rename = "ReplacedNoticeDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_notice_document_reference: Option<TenderingTermsArrayOfReplacedNoticeDocumentReferenceComponent>,
    #[serde(rename = "RequiredCurriculaIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_curricula_indicator: Option<TenderingTermsArrayOfRequiredCurriculaIndicatorComponent>,
    #[serde(rename = "RequiredFinancialGuarantee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_financial_guarantee: Option<TenderingTermsArrayOfRequiredFinancialGuaranteeComponent>,
    #[serde(rename = "TenderEvaluationParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_evaluation_party: Option<TenderingTermsArrayOfTenderEvaluationPartyComponent>,
    #[serde(rename = "TenderPreparation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_preparation: Option<TenderingTermsArrayOfTenderPreparationComponent>,
    #[serde(rename = "TenderRecipientParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_recipient_party: Option<TenderingTermsArrayOfTenderRecipientPartyComponent>,
    #[serde(rename = "TenderValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_validity_period: Option<TenderingTermsArrayOfTenderValidityPeriodComponent>,
    #[serde(rename = "TendererQualificationRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenderer_qualification_request: Option<TenderingTermsArrayOfTendererQualificationRequestComponent>,
    #[serde(rename = "VariantConstraintIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_constraint_indicator: Option<TenderingTermsArrayOfVariantConstraintIndicatorComponent>,
    #[serde(rename = "WarrantyValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_validity_period: Option<TenderingTermsArrayOfWarrantyValidityPeriodComponent>,
}

impl AsMut<TenderingTerms> for TenderingTerms {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTerms> for TenderingTerms {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.tender_recipient_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.tender_recipient_party", e));
            }
        }
        if let Some(v) = &self.warranty_validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.warranty_validity_period", e));
            }
        }
        if let Some(v) = &self.funding_program {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.funding_program", e));
            }
        }
        if let Some(v) = &self.documentation_fee_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.documentation_fee_amount", e));
            }
        }
        if let Some(v) = &self.funding_program_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.funding_program_code", e));
            }
        }
        if let Some(v) = &self.other_conditions_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.other_conditions_indicator", e));
            }
        }
        if let Some(v) = &self.price_revision_formula_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.price_revision_formula_description", e));
            }
        }
        if let Some(v) = &self.tender_evaluation_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.tender_evaluation_party", e));
            }
        }
        if let Some(v) = &self.contract_responsible_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.contract_responsible_party", e));
            }
        }
        if let Some(v) = &self.language {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.language", e));
            }
        }
        if let Some(v) = &self.allowed_subcontract_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.allowed_subcontract_terms", e));
            }
        }
        if let Some(v) = &self.call_for_tenders_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.call_for_tenders_document_reference", e));
            }
        }
        if let Some(v) = &self.employment_legislation_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.employment_legislation_document_reference", e));
            }
        }
        if let Some(v) = &self.replaced_notice_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.replaced_notice_document_reference", e));
            }
        }
        if let Some(v) = &self.maximum_variant_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.maximum_variant_quantity", e));
            }
        }
        if let Some(v) = &self.contractual_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.contractual_document_reference", e));
            }
        }
        if let Some(v) = &self.tenderer_qualification_request {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.tenderer_qualification_request", e));
            }
        }
        if let Some(v) = &self.awarding_method_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.awarding_method_type_code", e));
            }
        }
        if let Some(v) = &self.payment_frequency_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.payment_frequency_code", e));
            }
        }
        if let Some(v) = &self.price_evaluation_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.price_evaluation_code", e));
            }
        }
        if let Some(v) = &self.required_financial_guarantee {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.required_financial_guarantee", e));
            }
        }
        if let Some(v) = &self.budget_account_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.budget_account_line", e));
            }
        }
        if let Some(v) = &self.tender_preparation {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.tender_preparation", e));
            }
        }
        if let Some(v) = &self.maximum_advertisement_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.maximum_advertisement_amount", e));
            }
        }
        if let Some(v) = &self.accepted_variants_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.accepted_variants_description", e));
            }
        }
        if let Some(v) = &self.additional_conditions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.additional_conditions", e));
            }
        }
        if let Some(v) = &self.awarding_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.awarding_terms", e));
            }
        }
        if let Some(v) = &self.economic_operator_registry_uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.economic_operator_registry_uri", e));
            }
        }
        if let Some(v) = &self.environmental_legislation_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.environmental_legislation_document_reference", e));
            }
        }
        if let Some(v) = &self.payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.payment_terms", e));
            }
        }
        if let Some(v) = &self.fiscal_legislation_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.fiscal_legislation_document_reference", e));
            }
        }
        if let Some(v) = &self.latest_security_clearance_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.latest_security_clearance_date", e));
            }
        }
        if let Some(v) = &self.penalty_clause {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.penalty_clause", e));
            }
        }
        if let Some(v) = &self.procurement_legislation_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.procurement_legislation_document_reference", e));
            }
        }
        if let Some(v) = &self.required_curricula_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.required_curricula_indicator", e));
            }
        }
        if let Some(v) = &self.variant_constraint_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.variant_constraint_indicator", e));
            }
        }
        if let Some(v) = &self.document_provider_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.document_provider_party", e));
            }
        }
        if let Some(v) = &self.additional_information_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.additional_information_party", e));
            }
        }
        if let Some(v) = &self.appeal_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.appeal_terms", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.note", e));
            }
        }
        if let Some(v) = &self.contract_execution_requirement {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.contract_execution_requirement", e));
            }
        }
        if let Some(v) = &self.tender_validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.tender_validity_period", e));
            }
        }
        if let Some(v) = &self.contract_acceptance_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingTerms.contract_acceptance_period", e));
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

impl TenderingTerms {
    pub fn title() -> &'static str {
        "Tendering Terms. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe tendering terms for a tendering process."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            contract_responsible_party: None,
            maximum_variant_quantity: None,
            penalty_clause: None,
            procurement_legislation_document_reference: None,
            environmental_legislation_document_reference: None,
            budget_account_line: None,
            tender_preparation: None,
            contract_execution_requirement: None,
            awarding_terms: None,
            latest_security_clearance_date: None,
            tender_evaluation_party: None,
            documentation_fee_amount: None,
            employment_legislation_document_reference: None,
            contract_acceptance_period: None,
            note: None,
            tender_recipient_party: None,
            warranty_validity_period: None,
            additional_information_party: None,
            maximum_advertisement_amount: None,
            allowed_subcontract_terms: None,
            tender_validity_period: None,
            replaced_notice_document_reference: None,
            funding_program_code: None,
            additional_conditions: None,
            variant_constraint_indicator: None,
            tenderer_qualification_request: None,
            required_curricula_indicator: None,
            contractual_document_reference: None,
            accepted_variants_description: None,
            payment_terms: None,
            price_revision_formula_description: None,
            appeal_terms: None,
            price_evaluation_code: None,
            language: None,
            fiscal_legislation_document_reference: None,
            required_financial_guarantee: None,
            payment_frequency_code: None,
            other_conditions_indicator: None,
            awarding_method_type_code: None,
            economic_operator_registry_uri: None,
            funding_program: None,
            document_provider_party: None,
            call_for_tenders_document_reference: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfAcceptedVariantsDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AcceptedVariantsDescription>,
}

impl AsMut<TenderingTermsArrayOfAcceptedVariantsDescriptionComponent> for TenderingTermsArrayOfAcceptedVariantsDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfAcceptedVariantsDescriptionComponent> for TenderingTermsArrayOfAcceptedVariantsDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfAcceptedVariantsDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfAcceptedVariantsDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AcceptedVariantsDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AcceptedVariantsDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AcceptedVariantsDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AcceptedVariantsDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfAdditionalConditionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AdditionalConditions>,
}

impl AsMut<TenderingTermsArrayOfAdditionalConditionsComponent> for TenderingTermsArrayOfAdditionalConditionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfAdditionalConditionsComponent> for TenderingTermsArrayOfAdditionalConditionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfAdditionalConditionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfAdditionalConditionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AdditionalConditions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AdditionalConditions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AdditionalConditions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AdditionalConditions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfAdditionalInformationPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AdditionalInformationParty>,
}

impl AsMut<TenderingTermsArrayOfAdditionalInformationPartyComponent> for TenderingTermsArrayOfAdditionalInformationPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfAdditionalInformationPartyComponent> for TenderingTermsArrayOfAdditionalInformationPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfAdditionalInformationPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfAdditionalInformationPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfAdditionalInformationPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AdditionalInformationParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AdditionalInformationParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AdditionalInformationParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AdditionalInformationParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfAllowedSubcontractTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowedSubcontractTerms>,
}

impl AsMut<TenderingTermsArrayOfAllowedSubcontractTermsComponent> for TenderingTermsArrayOfAllowedSubcontractTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfAllowedSubcontractTermsComponent> for TenderingTermsArrayOfAllowedSubcontractTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfAllowedSubcontractTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfAllowedSubcontractTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AllowedSubcontractTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AllowedSubcontractTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AllowedSubcontractTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AllowedSubcontractTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfAppealTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AppealTerms>,
}

impl AsMut<TenderingTermsArrayOfAppealTermsComponent> for TenderingTermsArrayOfAppealTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfAppealTermsComponent> for TenderingTermsArrayOfAppealTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfAppealTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfAppealTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfAppealTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AppealTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AppealTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AppealTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AppealTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfAwardingMethodTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AwardingMethodTypeCode>,
}

impl AsMut<TenderingTermsArrayOfAwardingMethodTypeCodeComponent> for TenderingTermsArrayOfAwardingMethodTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfAwardingMethodTypeCodeComponent> for TenderingTermsArrayOfAwardingMethodTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfAwardingMethodTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfAwardingMethodTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfAwardingMethodTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AwardingMethodTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AwardingMethodTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AwardingMethodTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AwardingMethodTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfAwardingTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AwardingTerms>,
}

impl AsMut<TenderingTermsArrayOfAwardingTermsComponent> for TenderingTermsArrayOfAwardingTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfAwardingTermsComponent> for TenderingTermsArrayOfAwardingTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfAwardingTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfAwardingTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfAwardingTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AwardingTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AwardingTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AwardingTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AwardingTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfBudgetAccountLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BudgetAccountLine>,
}

impl AsMut<TenderingTermsArrayOfBudgetAccountLineComponent> for TenderingTermsArrayOfBudgetAccountLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfBudgetAccountLineComponent> for TenderingTermsArrayOfBudgetAccountLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfBudgetAccountLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfBudgetAccountLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BudgetAccountLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BudgetAccountLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BudgetAccountLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BudgetAccountLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfCallForTendersDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference>,
}

impl AsMut<TenderingTermsArrayOfCallForTendersDocumentReferenceComponent> for TenderingTermsArrayOfCallForTendersDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfCallForTendersDocumentReferenceComponent> for TenderingTermsArrayOfCallForTendersDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfCallForTendersDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfCallForTendersDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfCallForTendersDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfContractAcceptancePeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractAcceptancePeriod>,
}

impl AsMut<TenderingTermsArrayOfContractAcceptancePeriodComponent> for TenderingTermsArrayOfContractAcceptancePeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfContractAcceptancePeriodComponent> for TenderingTermsArrayOfContractAcceptancePeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfContractAcceptancePeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfContractAcceptancePeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfContractAcceptancePeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContractAcceptancePeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContractAcceptancePeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContractAcceptancePeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContractAcceptancePeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfContractExecutionRequirementComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractExecutionRequirement>,
}

impl AsMut<TenderingTermsArrayOfContractExecutionRequirementComponent> for TenderingTermsArrayOfContractExecutionRequirementComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfContractExecutionRequirementComponent> for TenderingTermsArrayOfContractExecutionRequirementComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfContractExecutionRequirementComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfContractExecutionRequirementComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContractExecutionRequirement) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContractExecutionRequirement) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContractExecutionRequirement> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContractExecutionRequirement> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfContractResponsiblePartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractResponsibleParty>,
}

impl AsMut<TenderingTermsArrayOfContractResponsiblePartyComponent> for TenderingTermsArrayOfContractResponsiblePartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfContractResponsiblePartyComponent> for TenderingTermsArrayOfContractResponsiblePartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfContractResponsiblePartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfContractResponsiblePartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfContractResponsiblePartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContractResponsibleParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContractResponsibleParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContractResponsibleParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContractResponsibleParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfContractualDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractualDocumentReference>,
}

impl AsMut<TenderingTermsArrayOfContractualDocumentReferenceComponent> for TenderingTermsArrayOfContractualDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfContractualDocumentReferenceComponent> for TenderingTermsArrayOfContractualDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfContractualDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfContractualDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContractualDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContractualDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContractualDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContractualDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfDocumentProviderPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentProviderParty>,
}

impl AsMut<TenderingTermsArrayOfDocumentProviderPartyComponent> for TenderingTermsArrayOfDocumentProviderPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfDocumentProviderPartyComponent> for TenderingTermsArrayOfDocumentProviderPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfDocumentProviderPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfDocumentProviderPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfDocumentProviderPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DocumentProviderParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DocumentProviderParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DocumentProviderParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DocumentProviderParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfDocumentationFeeAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DocumentationFeeAmount>,
}

impl AsMut<TenderingTermsArrayOfDocumentationFeeAmountComponent> for TenderingTermsArrayOfDocumentationFeeAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfDocumentationFeeAmountComponent> for TenderingTermsArrayOfDocumentationFeeAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfDocumentationFeeAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfDocumentationFeeAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfDocumentationFeeAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DocumentationFeeAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DocumentationFeeAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DocumentationFeeAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DocumentationFeeAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfEconomicOperatorRegistryURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EconomicOperatorRegistryURI>,
}

impl AsMut<TenderingTermsArrayOfEconomicOperatorRegistryURIComponent> for TenderingTermsArrayOfEconomicOperatorRegistryURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfEconomicOperatorRegistryURIComponent> for TenderingTermsArrayOfEconomicOperatorRegistryURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfEconomicOperatorRegistryURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfEconomicOperatorRegistryURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfEconomicOperatorRegistryURIComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EconomicOperatorRegistryURI) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EconomicOperatorRegistryURI) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EconomicOperatorRegistryURI> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EconomicOperatorRegistryURI> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfEmploymentLegislationDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EmploymentLegislationDocumentReference>,
}

impl AsMut<TenderingTermsArrayOfEmploymentLegislationDocumentReferenceComponent> for TenderingTermsArrayOfEmploymentLegislationDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfEmploymentLegislationDocumentReferenceComponent> for TenderingTermsArrayOfEmploymentLegislationDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfEmploymentLegislationDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfEmploymentLegislationDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfEmploymentLegislationDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EmploymentLegislationDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EmploymentLegislationDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EmploymentLegislationDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EmploymentLegislationDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfEnvironmentalLegislationDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EnvironmentalLegislationDocumentReference>,
}

impl AsMut<TenderingTermsArrayOfEnvironmentalLegislationDocumentReferenceComponent> for TenderingTermsArrayOfEnvironmentalLegislationDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfEnvironmentalLegislationDocumentReferenceComponent> for TenderingTermsArrayOfEnvironmentalLegislationDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfEnvironmentalLegislationDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfEnvironmentalLegislationDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfEnvironmentalLegislationDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EnvironmentalLegislationDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EnvironmentalLegislationDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EnvironmentalLegislationDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EnvironmentalLegislationDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfFiscalLegislationDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FiscalLegislationDocumentReference>,
}

impl AsMut<TenderingTermsArrayOfFiscalLegislationDocumentReferenceComponent> for TenderingTermsArrayOfFiscalLegislationDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfFiscalLegislationDocumentReferenceComponent> for TenderingTermsArrayOfFiscalLegislationDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfFiscalLegislationDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfFiscalLegislationDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfFiscalLegislationDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FiscalLegislationDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FiscalLegislationDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FiscalLegislationDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FiscalLegislationDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfFundingProgramComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FundingProgram>,
}

impl AsMut<TenderingTermsArrayOfFundingProgramComponent> for TenderingTermsArrayOfFundingProgramComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfFundingProgramComponent> for TenderingTermsArrayOfFundingProgramComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfFundingProgramComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfFundingProgramComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FundingProgram) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FundingProgram) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FundingProgram> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FundingProgram> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfFundingProgramCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FundingProgramCode>,
}

impl AsMut<TenderingTermsArrayOfFundingProgramCodeComponent> for TenderingTermsArrayOfFundingProgramCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfFundingProgramCodeComponent> for TenderingTermsArrayOfFundingProgramCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfFundingProgramCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfFundingProgramCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfFundingProgramCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FundingProgramCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FundingProgramCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FundingProgramCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FundingProgramCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfLanguageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Language>,
}

impl AsMut<TenderingTermsArrayOfLanguageComponent> for TenderingTermsArrayOfLanguageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfLanguageComponent> for TenderingTermsArrayOfLanguageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfLanguageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfLanguageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Language) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Language) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Language> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Language> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfLatestSecurityClearanceDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatestSecurityClearanceDate>,
}

impl AsMut<TenderingTermsArrayOfLatestSecurityClearanceDateComponent> for TenderingTermsArrayOfLatestSecurityClearanceDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfLatestSecurityClearanceDateComponent> for TenderingTermsArrayOfLatestSecurityClearanceDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfLatestSecurityClearanceDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfLatestSecurityClearanceDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfLatestSecurityClearanceDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatestSecurityClearanceDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatestSecurityClearanceDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatestSecurityClearanceDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatestSecurityClearanceDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfMaximumAdvertisementAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumAdvertisementAmount>,
}

impl AsMut<TenderingTermsArrayOfMaximumAdvertisementAmountComponent> for TenderingTermsArrayOfMaximumAdvertisementAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfMaximumAdvertisementAmountComponent> for TenderingTermsArrayOfMaximumAdvertisementAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfMaximumAdvertisementAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfMaximumAdvertisementAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfMaximumAdvertisementAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumAdvertisementAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumAdvertisementAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumAdvertisementAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumAdvertisementAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfMaximumVariantQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumVariantQuantity>,
}

impl AsMut<TenderingTermsArrayOfMaximumVariantQuantityComponent> for TenderingTermsArrayOfMaximumVariantQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfMaximumVariantQuantityComponent> for TenderingTermsArrayOfMaximumVariantQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfMaximumVariantQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfMaximumVariantQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfMaximumVariantQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumVariantQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumVariantQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumVariantQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumVariantQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<TenderingTermsArrayOfNoteComponent> for TenderingTermsArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfNoteComponent> for TenderingTermsArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfNoteComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Note) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Note) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfOtherConditionsIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OtherConditionsIndicator>,
}

impl AsMut<TenderingTermsArrayOfOtherConditionsIndicatorComponent> for TenderingTermsArrayOfOtherConditionsIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfOtherConditionsIndicatorComponent> for TenderingTermsArrayOfOtherConditionsIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfOtherConditionsIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfOtherConditionsIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfOtherConditionsIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OtherConditionsIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OtherConditionsIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OtherConditionsIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OtherConditionsIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfPaymentFrequencyCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentFrequencyCode>,
}

impl AsMut<TenderingTermsArrayOfPaymentFrequencyCodeComponent> for TenderingTermsArrayOfPaymentFrequencyCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfPaymentFrequencyCodeComponent> for TenderingTermsArrayOfPaymentFrequencyCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfPaymentFrequencyCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfPaymentFrequencyCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfPaymentFrequencyCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaymentFrequencyCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaymentFrequencyCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaymentFrequencyCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaymentFrequencyCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms>,
}

impl AsMut<TenderingTermsArrayOfPaymentTermsComponent> for TenderingTermsArrayOfPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfPaymentTermsComponent> for TenderingTermsArrayOfPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfPaymentTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PaymentTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PaymentTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfPenaltyClauseComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PenaltyClause>,
}

impl AsMut<TenderingTermsArrayOfPenaltyClauseComponent> for TenderingTermsArrayOfPenaltyClauseComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfPenaltyClauseComponent> for TenderingTermsArrayOfPenaltyClauseComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfPenaltyClauseComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfPenaltyClauseComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PenaltyClause) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PenaltyClause) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PenaltyClause> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PenaltyClause> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfPriceEvaluationCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PriceEvaluationCode>,
}

impl AsMut<TenderingTermsArrayOfPriceEvaluationCodeComponent> for TenderingTermsArrayOfPriceEvaluationCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfPriceEvaluationCodeComponent> for TenderingTermsArrayOfPriceEvaluationCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfPriceEvaluationCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfPriceEvaluationCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfPriceEvaluationCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PriceEvaluationCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PriceEvaluationCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PriceEvaluationCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PriceEvaluationCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfPriceRevisionFormulaDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PriceRevisionFormulaDescription>,
}

impl AsMut<TenderingTermsArrayOfPriceRevisionFormulaDescriptionComponent> for TenderingTermsArrayOfPriceRevisionFormulaDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfPriceRevisionFormulaDescriptionComponent> for TenderingTermsArrayOfPriceRevisionFormulaDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfPriceRevisionFormulaDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfPriceRevisionFormulaDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PriceRevisionFormulaDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PriceRevisionFormulaDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PriceRevisionFormulaDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PriceRevisionFormulaDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfProcurementLegislationDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ProcurementLegislationDocumentReference>,
}

impl AsMut<TenderingTermsArrayOfProcurementLegislationDocumentReferenceComponent> for TenderingTermsArrayOfProcurementLegislationDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfProcurementLegislationDocumentReferenceComponent> for TenderingTermsArrayOfProcurementLegislationDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfProcurementLegislationDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfProcurementLegislationDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfProcurementLegislationDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ProcurementLegislationDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ProcurementLegislationDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ProcurementLegislationDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ProcurementLegislationDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfReplacedNoticeDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReplacedNoticeDocumentReference>,
}

impl AsMut<TenderingTermsArrayOfReplacedNoticeDocumentReferenceComponent> for TenderingTermsArrayOfReplacedNoticeDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfReplacedNoticeDocumentReferenceComponent> for TenderingTermsArrayOfReplacedNoticeDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfReplacedNoticeDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfReplacedNoticeDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfReplacedNoticeDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReplacedNoticeDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReplacedNoticeDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReplacedNoticeDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReplacedNoticeDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfRequiredCurriculaIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RequiredCurriculaIndicator>,
}

impl AsMut<TenderingTermsArrayOfRequiredCurriculaIndicatorComponent> for TenderingTermsArrayOfRequiredCurriculaIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfRequiredCurriculaIndicatorComponent> for TenderingTermsArrayOfRequiredCurriculaIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfRequiredCurriculaIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfRequiredCurriculaIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfRequiredCurriculaIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RequiredCurriculaIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RequiredCurriculaIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RequiredCurriculaIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RequiredCurriculaIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfRequiredFinancialGuaranteeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequiredFinancialGuarantee>,
}

impl AsMut<TenderingTermsArrayOfRequiredFinancialGuaranteeComponent> for TenderingTermsArrayOfRequiredFinancialGuaranteeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfRequiredFinancialGuaranteeComponent> for TenderingTermsArrayOfRequiredFinancialGuaranteeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfRequiredFinancialGuaranteeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfRequiredFinancialGuaranteeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequiredFinancialGuarantee) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequiredFinancialGuarantee) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequiredFinancialGuarantee> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequiredFinancialGuarantee> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfTenderEvaluationPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TenderEvaluationParty>,
}

impl AsMut<TenderingTermsArrayOfTenderEvaluationPartyComponent> for TenderingTermsArrayOfTenderEvaluationPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfTenderEvaluationPartyComponent> for TenderingTermsArrayOfTenderEvaluationPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfTenderEvaluationPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfTenderEvaluationPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TenderEvaluationParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TenderEvaluationParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TenderEvaluationParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TenderEvaluationParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfTenderPreparationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TenderPreparation>,
}

impl AsMut<TenderingTermsArrayOfTenderPreparationComponent> for TenderingTermsArrayOfTenderPreparationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfTenderPreparationComponent> for TenderingTermsArrayOfTenderPreparationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfTenderPreparationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfTenderPreparationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TenderPreparation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TenderPreparation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TenderPreparation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TenderPreparation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfTenderRecipientPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TenderRecipientParty>,
}

impl AsMut<TenderingTermsArrayOfTenderRecipientPartyComponent> for TenderingTermsArrayOfTenderRecipientPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfTenderRecipientPartyComponent> for TenderingTermsArrayOfTenderRecipientPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfTenderRecipientPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfTenderRecipientPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfTenderRecipientPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TenderRecipientParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TenderRecipientParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TenderRecipientParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TenderRecipientParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfTenderValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TenderValidityPeriod>,
}

impl AsMut<TenderingTermsArrayOfTenderValidityPeriodComponent> for TenderingTermsArrayOfTenderValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfTenderValidityPeriodComponent> for TenderingTermsArrayOfTenderValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfTenderValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfTenderValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfTenderValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TenderValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TenderValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TenderValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TenderValidityPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfTendererQualificationRequestComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TendererQualificationRequest>,
}

impl AsMut<TenderingTermsArrayOfTendererQualificationRequestComponent> for TenderingTermsArrayOfTendererQualificationRequestComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfTendererQualificationRequestComponent> for TenderingTermsArrayOfTendererQualificationRequestComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfTendererQualificationRequestComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfTendererQualificationRequestComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TendererQualificationRequest) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TendererQualificationRequest) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TendererQualificationRequest> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TendererQualificationRequest> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfVariantConstraintIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::VariantConstraintIndicator>,
}

impl AsMut<TenderingTermsArrayOfVariantConstraintIndicatorComponent> for TenderingTermsArrayOfVariantConstraintIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfVariantConstraintIndicatorComponent> for TenderingTermsArrayOfVariantConstraintIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfVariantConstraintIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfVariantConstraintIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfVariantConstraintIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::VariantConstraintIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::VariantConstraintIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::VariantConstraintIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::VariantConstraintIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingTermsArrayOfWarrantyValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod>,
}

impl AsMut<TenderingTermsArrayOfWarrantyValidityPeriodComponent> for TenderingTermsArrayOfWarrantyValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingTermsArrayOfWarrantyValidityPeriodComponent> for TenderingTermsArrayOfWarrantyValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfWarrantyValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingTermsArrayOfWarrantyValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingTermsArrayOfWarrantyValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod> {
        self.items.iter()
    }
}


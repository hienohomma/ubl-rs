use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenderingProcess {
    #[serde(rename = "AdditionalDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document_reference: Option<TenderingProcessArrayOfAdditionalDocumentReferenceComponent>,
    #[serde(rename = "AuctionTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auction_terms: Option<TenderingProcessArrayOfAuctionTermsComponent>,
    #[serde(rename = "CandidateReductionConstraintIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_reduction_constraint_indicator: Option<TenderingProcessArrayOfCandidateReductionConstraintIndicatorComponent>,
    #[serde(rename = "ContractingSystemCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracting_system_code: Option<TenderingProcessArrayOfContractingSystemCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TenderingProcessArrayOfDescriptionComponent>,
    #[serde(rename = "DocumentAvailabilityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_availability_period: Option<TenderingProcessArrayOfDocumentAvailabilityPeriodComponent>,
    #[serde(rename = "EconomicOperatorShortList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_operator_short_list: Option<TenderingProcessArrayOfEconomicOperatorShortListComponent>,
    #[serde(rename = "ExpenseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_code: Option<TenderingProcessArrayOfExpenseCodeComponent>,
    #[serde(rename = "FrameworkAgreement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_agreement: Option<TenderingProcessArrayOfFrameworkAgreementComponent>,
    #[serde(rename = "GovernmentAgreementConstraintIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub government_agreement_constraint_indicator: Option<TenderingProcessArrayOfGovernmentAgreementConstraintIndicatorComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TenderingProcessArrayOfIDComponent>,
    #[serde(rename = "InvitationSubmissionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_submission_period: Option<TenderingProcessArrayOfInvitationSubmissionPeriodComponent>,
    #[serde(rename = "NegotiationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negotiation_description: Option<TenderingProcessArrayOfNegotiationDescriptionComponent>,
    #[serde(rename = "NoticeDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_document_reference: Option<TenderingProcessArrayOfNoticeDocumentReferenceComponent>,
    #[serde(rename = "OpenTenderEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_tender_event: Option<TenderingProcessArrayOfOpenTenderEventComponent>,
    #[serde(rename = "OriginalContractingSystemID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_contracting_system_id: Option<TenderingProcessArrayOfOriginalContractingSystemIDComponent>,
    #[serde(rename = "PartPresentationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_presentation_code: Option<TenderingProcessArrayOfPartPresentationCodeComponent>,
    #[serde(rename = "ParticipationRequestReceptionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participation_request_reception_period: Option<TenderingProcessArrayOfParticipationRequestReceptionPeriodComponent>,
    #[serde(rename = "ProcedureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_code: Option<TenderingProcessArrayOfProcedureCodeComponent>,
    #[serde(rename = "ProcessJustification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_justification: Option<TenderingProcessArrayOfProcessJustificationComponent>,
    #[serde(rename = "SubmissionMethodCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_method_code: Option<TenderingProcessArrayOfSubmissionMethodCodeComponent>,
    #[serde(rename = "TenderSubmissionDeadlinePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_submission_deadline_period: Option<TenderingProcessArrayOfTenderSubmissionDeadlinePeriodComponent>,
    #[serde(rename = "UrgencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency_code: Option<TenderingProcessArrayOfUrgencyCodeComponent>,
}

impl AsMut<TenderingProcess> for TenderingProcess {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcess> for TenderingProcess {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.open_tender_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.open_tender_event", e));
            }
        }
        if let Some(v) = &self.additional_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.additional_document_reference", e));
            }
        }
        if let Some(v) = &self.government_agreement_constraint_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.government_agreement_constraint_indicator", e));
            }
        }
        if let Some(v) = &self.part_presentation_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.part_presentation_code", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.id", e));
            }
        }
        if let Some(v) = &self.candidate_reduction_constraint_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.candidate_reduction_constraint_indicator", e));
            }
        }
        if let Some(v) = &self.negotiation_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.negotiation_description", e));
            }
        }
        if let Some(v) = &self.expense_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.expense_code", e));
            }
        }
        if let Some(v) = &self.document_availability_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.document_availability_period", e));
            }
        }
        if let Some(v) = &self.notice_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.notice_document_reference", e));
            }
        }
        if let Some(v) = &self.original_contracting_system_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.original_contracting_system_id", e));
            }
        }
        if let Some(v) = &self.participation_request_reception_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.participation_request_reception_period", e));
            }
        }
        if let Some(v) = &self.process_justification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.process_justification", e));
            }
        }
        if let Some(v) = &self.framework_agreement {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.framework_agreement", e));
            }
        }
        if let Some(v) = &self.contracting_system_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.contracting_system_code", e));
            }
        }
        if let Some(v) = &self.procedure_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.procedure_code", e));
            }
        }
        if let Some(v) = &self.economic_operator_short_list {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.economic_operator_short_list", e));
            }
        }
        if let Some(v) = &self.invitation_submission_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.invitation_submission_period", e));
            }
        }
        if let Some(v) = &self.tender_submission_deadline_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.tender_submission_deadline_period", e));
            }
        }
        if let Some(v) = &self.submission_method_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.submission_method_code", e));
            }
        }
        if let Some(v) = &self.urgency_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.urgency_code", e));
            }
        }
        if let Some(v) = &self.auction_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.auction_terms", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderingProcess.description", e));
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

impl TenderingProcess {
    pub fn title() -> &'static str {
        "Tendering Process. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the process of a formal offer and response to execute work or supply goods at a stated price."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            invitation_submission_period: None,
            part_presentation_code: None,
            negotiation_description: None,
            participation_request_reception_period: None,
            tender_submission_deadline_period: None,
            urgency_code: None,
            economic_operator_short_list: None,
            expense_code: None,
            submission_method_code: None,
            framework_agreement: None,
            candidate_reduction_constraint_indicator: None,
            id: None,
            open_tender_event: None,
            original_contracting_system_id: None,
            auction_terms: None,
            document_availability_period: None,
            additional_document_reference: None,
            description: None,
            government_agreement_constraint_indicator: None,
            notice_document_reference: None,
            procedure_code: None,
            process_justification: None,
            contracting_system_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfAdditionalDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AdditionalDocumentReference>,
}

impl AsMut<TenderingProcessArrayOfAdditionalDocumentReferenceComponent> for TenderingProcessArrayOfAdditionalDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfAdditionalDocumentReferenceComponent> for TenderingProcessArrayOfAdditionalDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfAdditionalDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfAdditionalDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AdditionalDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AdditionalDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AdditionalDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AdditionalDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfAuctionTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AuctionTerms>,
}

impl AsMut<TenderingProcessArrayOfAuctionTermsComponent> for TenderingProcessArrayOfAuctionTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfAuctionTermsComponent> for TenderingProcessArrayOfAuctionTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfAuctionTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfAuctionTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfAuctionTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AuctionTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AuctionTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AuctionTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AuctionTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfCandidateReductionConstraintIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CandidateReductionConstraintIndicator>,
}

impl AsMut<TenderingProcessArrayOfCandidateReductionConstraintIndicatorComponent> for TenderingProcessArrayOfCandidateReductionConstraintIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfCandidateReductionConstraintIndicatorComponent> for TenderingProcessArrayOfCandidateReductionConstraintIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfCandidateReductionConstraintIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfCandidateReductionConstraintIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfCandidateReductionConstraintIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CandidateReductionConstraintIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CandidateReductionConstraintIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CandidateReductionConstraintIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CandidateReductionConstraintIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfContractingSystemCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ContractingSystemCode>,
}

impl AsMut<TenderingProcessArrayOfContractingSystemCodeComponent> for TenderingProcessArrayOfContractingSystemCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfContractingSystemCodeComponent> for TenderingProcessArrayOfContractingSystemCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfContractingSystemCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfContractingSystemCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfContractingSystemCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ContractingSystemCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ContractingSystemCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ContractingSystemCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ContractingSystemCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<TenderingProcessArrayOfDescriptionComponent> for TenderingProcessArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfDescriptionComponent> for TenderingProcessArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfDescriptionComponent {
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
pub struct TenderingProcessArrayOfDocumentAvailabilityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentAvailabilityPeriod>,
}

impl AsMut<TenderingProcessArrayOfDocumentAvailabilityPeriodComponent> for TenderingProcessArrayOfDocumentAvailabilityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfDocumentAvailabilityPeriodComponent> for TenderingProcessArrayOfDocumentAvailabilityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfDocumentAvailabilityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfDocumentAvailabilityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfDocumentAvailabilityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DocumentAvailabilityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DocumentAvailabilityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DocumentAvailabilityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DocumentAvailabilityPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfEconomicOperatorShortListComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EconomicOperatorShortList>,
}

impl AsMut<TenderingProcessArrayOfEconomicOperatorShortListComponent> for TenderingProcessArrayOfEconomicOperatorShortListComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfEconomicOperatorShortListComponent> for TenderingProcessArrayOfEconomicOperatorShortListComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfEconomicOperatorShortListComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfEconomicOperatorShortListComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfEconomicOperatorShortListComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EconomicOperatorShortList) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EconomicOperatorShortList) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EconomicOperatorShortList> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EconomicOperatorShortList> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfExpenseCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExpenseCode>,
}

impl AsMut<TenderingProcessArrayOfExpenseCodeComponent> for TenderingProcessArrayOfExpenseCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfExpenseCodeComponent> for TenderingProcessArrayOfExpenseCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfExpenseCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfExpenseCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfExpenseCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExpenseCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExpenseCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExpenseCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExpenseCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfFrameworkAgreementComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FrameworkAgreement>,
}

impl AsMut<TenderingProcessArrayOfFrameworkAgreementComponent> for TenderingProcessArrayOfFrameworkAgreementComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfFrameworkAgreementComponent> for TenderingProcessArrayOfFrameworkAgreementComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfFrameworkAgreementComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfFrameworkAgreementComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfFrameworkAgreementComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FrameworkAgreement) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FrameworkAgreement) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FrameworkAgreement> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FrameworkAgreement> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfGovernmentAgreementConstraintIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GovernmentAgreementConstraintIndicator>,
}

impl AsMut<TenderingProcessArrayOfGovernmentAgreementConstraintIndicatorComponent> for TenderingProcessArrayOfGovernmentAgreementConstraintIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfGovernmentAgreementConstraintIndicatorComponent> for TenderingProcessArrayOfGovernmentAgreementConstraintIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfGovernmentAgreementConstraintIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfGovernmentAgreementConstraintIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfGovernmentAgreementConstraintIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GovernmentAgreementConstraintIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GovernmentAgreementConstraintIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GovernmentAgreementConstraintIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GovernmentAgreementConstraintIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<TenderingProcessArrayOfIDComponent> for TenderingProcessArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfIDComponent> for TenderingProcessArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfIDComponent {
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
pub struct TenderingProcessArrayOfInvitationSubmissionPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::InvitationSubmissionPeriod>,
}

impl AsMut<TenderingProcessArrayOfInvitationSubmissionPeriodComponent> for TenderingProcessArrayOfInvitationSubmissionPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfInvitationSubmissionPeriodComponent> for TenderingProcessArrayOfInvitationSubmissionPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfInvitationSubmissionPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfInvitationSubmissionPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfInvitationSubmissionPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::InvitationSubmissionPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::InvitationSubmissionPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::InvitationSubmissionPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::InvitationSubmissionPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfNegotiationDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NegotiationDescription>,
}

impl AsMut<TenderingProcessArrayOfNegotiationDescriptionComponent> for TenderingProcessArrayOfNegotiationDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfNegotiationDescriptionComponent> for TenderingProcessArrayOfNegotiationDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfNegotiationDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfNegotiationDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NegotiationDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NegotiationDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NegotiationDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NegotiationDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfNoticeDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::NoticeDocumentReference>,
}

impl AsMut<TenderingProcessArrayOfNoticeDocumentReferenceComponent> for TenderingProcessArrayOfNoticeDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfNoticeDocumentReferenceComponent> for TenderingProcessArrayOfNoticeDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfNoticeDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfNoticeDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::NoticeDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::NoticeDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::NoticeDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::NoticeDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfOpenTenderEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OpenTenderEvent>,
}

impl AsMut<TenderingProcessArrayOfOpenTenderEventComponent> for TenderingProcessArrayOfOpenTenderEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfOpenTenderEventComponent> for TenderingProcessArrayOfOpenTenderEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfOpenTenderEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfOpenTenderEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OpenTenderEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OpenTenderEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OpenTenderEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OpenTenderEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfOriginalContractingSystemIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OriginalContractingSystemID>,
}

impl AsMut<TenderingProcessArrayOfOriginalContractingSystemIDComponent> for TenderingProcessArrayOfOriginalContractingSystemIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfOriginalContractingSystemIDComponent> for TenderingProcessArrayOfOriginalContractingSystemIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfOriginalContractingSystemIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfOriginalContractingSystemIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfOriginalContractingSystemIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OriginalContractingSystemID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OriginalContractingSystemID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OriginalContractingSystemID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OriginalContractingSystemID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfPartPresentationCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PartPresentationCode>,
}

impl AsMut<TenderingProcessArrayOfPartPresentationCodeComponent> for TenderingProcessArrayOfPartPresentationCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfPartPresentationCodeComponent> for TenderingProcessArrayOfPartPresentationCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfPartPresentationCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfPartPresentationCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfPartPresentationCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PartPresentationCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PartPresentationCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PartPresentationCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PartPresentationCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfParticipationRequestReceptionPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ParticipationRequestReceptionPeriod>,
}

impl AsMut<TenderingProcessArrayOfParticipationRequestReceptionPeriodComponent> for TenderingProcessArrayOfParticipationRequestReceptionPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfParticipationRequestReceptionPeriodComponent> for TenderingProcessArrayOfParticipationRequestReceptionPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfParticipationRequestReceptionPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfParticipationRequestReceptionPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfParticipationRequestReceptionPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ParticipationRequestReceptionPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ParticipationRequestReceptionPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ParticipationRequestReceptionPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ParticipationRequestReceptionPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfProcedureCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ProcedureCode>,
}

impl AsMut<TenderingProcessArrayOfProcedureCodeComponent> for TenderingProcessArrayOfProcedureCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfProcedureCodeComponent> for TenderingProcessArrayOfProcedureCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfProcedureCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfProcedureCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfProcedureCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ProcedureCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ProcedureCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ProcedureCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ProcedureCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfProcessJustificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ProcessJustification>,
}

impl AsMut<TenderingProcessArrayOfProcessJustificationComponent> for TenderingProcessArrayOfProcessJustificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfProcessJustificationComponent> for TenderingProcessArrayOfProcessJustificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfProcessJustificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfProcessJustificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ProcessJustification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ProcessJustification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ProcessJustification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ProcessJustification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfSubmissionMethodCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SubmissionMethodCode>,
}

impl AsMut<TenderingProcessArrayOfSubmissionMethodCodeComponent> for TenderingProcessArrayOfSubmissionMethodCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfSubmissionMethodCodeComponent> for TenderingProcessArrayOfSubmissionMethodCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfSubmissionMethodCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfSubmissionMethodCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfSubmissionMethodCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SubmissionMethodCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SubmissionMethodCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SubmissionMethodCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SubmissionMethodCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfTenderSubmissionDeadlinePeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TenderSubmissionDeadlinePeriod>,
}

impl AsMut<TenderingProcessArrayOfTenderSubmissionDeadlinePeriodComponent> for TenderingProcessArrayOfTenderSubmissionDeadlinePeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfTenderSubmissionDeadlinePeriodComponent> for TenderingProcessArrayOfTenderSubmissionDeadlinePeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfTenderSubmissionDeadlinePeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfTenderSubmissionDeadlinePeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfTenderSubmissionDeadlinePeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TenderSubmissionDeadlinePeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TenderSubmissionDeadlinePeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TenderSubmissionDeadlinePeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TenderSubmissionDeadlinePeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderingProcessArrayOfUrgencyCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UrgencyCode>,
}

impl AsMut<TenderingProcessArrayOfUrgencyCodeComponent> for TenderingProcessArrayOfUrgencyCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderingProcessArrayOfUrgencyCodeComponent> for TenderingProcessArrayOfUrgencyCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfUrgencyCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderingProcessArrayOfUrgencyCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderingProcessArrayOfUrgencyCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::UrgencyCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::UrgencyCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::UrgencyCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::UrgencyCode> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenderedProject {
    #[serde(rename = "AwardingCriterionResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarding_criterion_response: Option<TenderedProjectArrayOfAwardingCriterionResponseComponent>,
    #[serde(rename = "EvidenceDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_document_reference: Option<TenderedProjectArrayOfEvidenceDocumentReferenceComponent>,
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<TenderedProjectArrayOfFeeAmountComponent>,
    #[serde(rename = "FeeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_description: Option<TenderedProjectArrayOfFeeDescriptionComponent>,
    #[serde(rename = "LegalMonetaryTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_monetary_total: Option<TenderedProjectArrayOfLegalMonetaryTotalComponent>,
    #[serde(rename = "ProcurementProjectLot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procurement_project_lot: Option<TenderedProjectArrayOfProcurementProjectLotComponent>,
    #[serde(rename = "TaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<TenderedProjectArrayOfTaxTotalComponent>,
    #[serde(rename = "TenderEnvelopeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_envelope_id: Option<TenderedProjectArrayOfTenderEnvelopeIDComponent>,
    #[serde(rename = "TenderEnvelopeTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_envelope_type_code: Option<TenderedProjectArrayOfTenderEnvelopeTypeCodeComponent>,
    #[serde(rename = "TenderLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_line: Option<TenderedProjectArrayOfTenderLineComponent>,
    #[serde(rename = "VariantID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<TenderedProjectArrayOfVariantIDComponent>,
}

impl AsMut<TenderedProject> for TenderedProject {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProject> for TenderedProject {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.tender_envelope_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderedProject.tender_envelope_type_code", e));
            }
        }
        if let Some(v) = &self.tender_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderedProject.tender_line", e));
            }
        }
        if let Some(v) = &self.variant_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderedProject.variant_id", e));
            }
        }
        if let Some(v) = &self.tender_envelope_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderedProject.tender_envelope_id", e));
            }
        }
        if let Some(v) = &self.fee_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderedProject.fee_description", e));
            }
        }
        if let Some(v) = &self.fee_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderedProject.fee_amount", e));
            }
        }
        if let Some(v) = &self.procurement_project_lot {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderedProject.procurement_project_lot", e));
            }
        }
        if let Some(v) = &self.evidence_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderedProject.evidence_document_reference", e));
            }
        }
        if let Some(v) = &self.tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderedProject.tax_total", e));
            }
        }
        if let Some(v) = &self.legal_monetary_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderedProject.legal_monetary_total", e));
            }
        }
        if let Some(v) = &self.awarding_criterion_response {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderedProject.awarding_criterion_response", e));
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

impl TenderedProject {
    pub fn title() -> &'static str {
        "Tendered Project. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a tendered project or project lot."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            tender_line: None,
            legal_monetary_total: None,
            tender_envelope_id: None,
            tender_envelope_type_code: None,
            awarding_criterion_response: None,
            evidence_document_reference: None,
            fee_description: None,
            variant_id: None,
            tax_total: None,
            procurement_project_lot: None,
            fee_amount: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderedProjectArrayOfAwardingCriterionResponseComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AwardingCriterionResponse>,
}

impl AsMut<TenderedProjectArrayOfAwardingCriterionResponseComponent> for TenderedProjectArrayOfAwardingCriterionResponseComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProjectArrayOfAwardingCriterionResponseComponent> for TenderedProjectArrayOfAwardingCriterionResponseComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfAwardingCriterionResponseComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderedProjectArrayOfAwardingCriterionResponseComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AwardingCriterionResponse) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AwardingCriterionResponse) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AwardingCriterionResponse> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AwardingCriterionResponse> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderedProjectArrayOfEvidenceDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EvidenceDocumentReference>,
}

impl AsMut<TenderedProjectArrayOfEvidenceDocumentReferenceComponent> for TenderedProjectArrayOfEvidenceDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProjectArrayOfEvidenceDocumentReferenceComponent> for TenderedProjectArrayOfEvidenceDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfEvidenceDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderedProjectArrayOfEvidenceDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EvidenceDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EvidenceDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EvidenceDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EvidenceDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderedProjectArrayOfFeeAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FeeAmount>,
}

impl AsMut<TenderedProjectArrayOfFeeAmountComponent> for TenderedProjectArrayOfFeeAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProjectArrayOfFeeAmountComponent> for TenderedProjectArrayOfFeeAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfFeeAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfFeeAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderedProjectArrayOfFeeAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FeeAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FeeAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FeeAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FeeAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderedProjectArrayOfFeeDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FeeDescription>,
}

impl AsMut<TenderedProjectArrayOfFeeDescriptionComponent> for TenderedProjectArrayOfFeeDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProjectArrayOfFeeDescriptionComponent> for TenderedProjectArrayOfFeeDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfFeeDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderedProjectArrayOfFeeDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FeeDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FeeDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FeeDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FeeDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderedProjectArrayOfLegalMonetaryTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LegalMonetaryTotal>,
}

impl AsMut<TenderedProjectArrayOfLegalMonetaryTotalComponent> for TenderedProjectArrayOfLegalMonetaryTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProjectArrayOfLegalMonetaryTotalComponent> for TenderedProjectArrayOfLegalMonetaryTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfLegalMonetaryTotalComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfLegalMonetaryTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderedProjectArrayOfLegalMonetaryTotalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LegalMonetaryTotal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LegalMonetaryTotal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LegalMonetaryTotal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LegalMonetaryTotal> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderedProjectArrayOfProcurementProjectLotComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ProcurementProjectLot>,
}

impl AsMut<TenderedProjectArrayOfProcurementProjectLotComponent> for TenderedProjectArrayOfProcurementProjectLotComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProjectArrayOfProcurementProjectLotComponent> for TenderedProjectArrayOfProcurementProjectLotComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfProcurementProjectLotComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfProcurementProjectLotComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderedProjectArrayOfProcurementProjectLotComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ProcurementProjectLot) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ProcurementProjectLot) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ProcurementProjectLot> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ProcurementProjectLot> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderedProjectArrayOfTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxTotal>,
}

impl AsMut<TenderedProjectArrayOfTaxTotalComponent> for TenderedProjectArrayOfTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProjectArrayOfTaxTotalComponent> for TenderedProjectArrayOfTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderedProjectArrayOfTaxTotalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderedProjectArrayOfTenderEnvelopeIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeID>,
}

impl AsMut<TenderedProjectArrayOfTenderEnvelopeIDComponent> for TenderedProjectArrayOfTenderEnvelopeIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProjectArrayOfTenderEnvelopeIDComponent> for TenderedProjectArrayOfTenderEnvelopeIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfTenderEnvelopeIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfTenderEnvelopeIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderedProjectArrayOfTenderEnvelopeIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderedProjectArrayOfTenderEnvelopeTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeTypeCode>,
}

impl AsMut<TenderedProjectArrayOfTenderEnvelopeTypeCodeComponent> for TenderedProjectArrayOfTenderEnvelopeTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProjectArrayOfTenderEnvelopeTypeCodeComponent> for TenderedProjectArrayOfTenderEnvelopeTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfTenderEnvelopeTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfTenderEnvelopeTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderedProjectArrayOfTenderEnvelopeTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TenderEnvelopeTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderedProjectArrayOfTenderLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TenderLine>,
}

impl AsMut<TenderedProjectArrayOfTenderLineComponent> for TenderedProjectArrayOfTenderLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProjectArrayOfTenderLineComponent> for TenderedProjectArrayOfTenderLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfTenderLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderedProjectArrayOfTenderLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TenderLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TenderLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TenderLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TenderLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderedProjectArrayOfVariantIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::VariantID>,
}

impl AsMut<TenderedProjectArrayOfVariantIDComponent> for TenderedProjectArrayOfVariantIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderedProjectArrayOfVariantIDComponent> for TenderedProjectArrayOfVariantIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfVariantIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderedProjectArrayOfVariantIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderedProjectArrayOfVariantIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::VariantID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::VariantID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::VariantID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::VariantID> {
        self.items.iter()
    }
}


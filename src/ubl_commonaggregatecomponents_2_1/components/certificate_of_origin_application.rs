use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CertificateOfOriginApplication {
    #[serde(rename = "ApplicationStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_status_code: Option<CertificateOfOriginApplicationArrayOfApplicationStatusCodeComponent>,
    #[serde(rename = "CertificateType")]
    pub certificate_type: CertificateOfOriginApplicationArrayOfCertificateTypeComponent,
    #[serde(rename = "DocumentDistribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_distribution: Option<CertificateOfOriginApplicationArrayOfDocumentDistributionComponent>,
    #[serde(rename = "EndorserParty")]
    pub endorser_party: CertificateOfOriginApplicationArrayOfEndorserPartyComponent,
    #[serde(rename = "ExporterParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exporter_party: Option<CertificateOfOriginApplicationArrayOfExporterPartyComponent>,
    #[serde(rename = "ImporterParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importer_party: Option<CertificateOfOriginApplicationArrayOfImporterPartyComponent>,
    #[serde(rename = "IssuerParty")]
    pub issuer_party: CertificateOfOriginApplicationArrayOfIssuerPartyComponent,
    #[serde(rename = "IssuingCountry")]
    pub issuing_country: CertificateOfOriginApplicationArrayOfIssuingCountryComponent,
    #[serde(rename = "OriginalJobID")]
    pub original_job_id: CertificateOfOriginApplicationArrayOfOriginalJobIDComponent,
    #[serde(rename = "PreparationParty")]
    pub preparation_party: CertificateOfOriginApplicationArrayOfPreparationPartyComponent,
    #[serde(rename = "PreviousJobID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_job_id: Option<CertificateOfOriginApplicationArrayOfPreviousJobIDComponent>,
    #[serde(rename = "ReferenceID")]
    pub reference_id: CertificateOfOriginApplicationArrayOfReferenceIDComponent,
    #[serde(rename = "Remarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<CertificateOfOriginApplicationArrayOfRemarksComponent>,
    #[serde(rename = "Shipment")]
    pub shipment: CertificateOfOriginApplicationArrayOfShipmentComponent,
    #[serde(rename = "Signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<CertificateOfOriginApplicationArrayOfSignatureComponent>,
    #[serde(rename = "SupportingDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_document_reference: Option<CertificateOfOriginApplicationArrayOfSupportingDocumentReferenceComponent>,
}

impl AsMut<CertificateOfOriginApplication> for CertificateOfOriginApplication {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplication> for CertificateOfOriginApplication {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.importer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CertificateOfOriginApplication.importer_party", e));
            }
        }
        if let Err(e) = self.issuer_party.validate() {
            return Err(UblError::component("CertificateOfOriginApplication.issuer_party", e));
        }
        if let Some(v) = &self.application_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CertificateOfOriginApplication.application_status_code", e));
            }
        }
        if let Err(e) = self.shipment.validate() {
            return Err(UblError::component("CertificateOfOriginApplication.shipment", e));
        }
        if let Err(e) = self.endorser_party.validate() {
            return Err(UblError::component("CertificateOfOriginApplication.endorser_party", e));
        }
        if let Err(e) = self.issuing_country.validate() {
            return Err(UblError::component("CertificateOfOriginApplication.issuing_country", e));
        }
        if let Err(e) = self.original_job_id.validate() {
            return Err(UblError::component("CertificateOfOriginApplication.original_job_id", e));
        }
        if let Some(v) = &self.previous_job_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CertificateOfOriginApplication.previous_job_id", e));
            }
        }
        if let Some(v) = &self.exporter_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CertificateOfOriginApplication.exporter_party", e));
            }
        }
        if let Some(v) = &self.remarks {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CertificateOfOriginApplication.remarks", e));
            }
        }
        if let Err(e) = self.preparation_party.validate() {
            return Err(UblError::component("CertificateOfOriginApplication.preparation_party", e));
        }
        if let Some(v) = &self.signature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CertificateOfOriginApplication.signature", e));
            }
        }
        if let Err(e) = self.reference_id.validate() {
            return Err(UblError::component("CertificateOfOriginApplication.reference_id", e));
        }
        if let Some(v) = &self.document_distribution {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CertificateOfOriginApplication.document_distribution", e));
            }
        }
        if let Some(v) = &self.supporting_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CertificateOfOriginApplication.supporting_document_reference", e));
            }
        }
        if let Err(e) = self.certificate_type.validate() {
            return Err(UblError::component("CertificateOfOriginApplication.certificate_type", e));
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

impl CertificateOfOriginApplication {
    pub fn title() -> &'static str {
        "Certificate Of Origin Application. Details"
    }
    pub fn description() -> &'static str {
        "A class to define an application for a Certificate of Origin (CoO)."
    }
    pub fn new(certificate_type: CertificateOfOriginApplicationArrayOfCertificateTypeComponent, endorser_party: CertificateOfOriginApplicationArrayOfEndorserPartyComponent, issuer_party: CertificateOfOriginApplicationArrayOfIssuerPartyComponent, issuing_country: CertificateOfOriginApplicationArrayOfIssuingCountryComponent, original_job_id: CertificateOfOriginApplicationArrayOfOriginalJobIDComponent, preparation_party: CertificateOfOriginApplicationArrayOfPreparationPartyComponent, reference_id: CertificateOfOriginApplicationArrayOfReferenceIDComponent, shipment: CertificateOfOriginApplicationArrayOfShipmentComponent) -> Component<Self> {
        Component(Self {
            preparation_party,
            exporter_party: None,
            issuer_party,
            original_job_id,
            previous_job_id: None,
            shipment,
            application_status_code: None,
            reference_id,
            remarks: None,
            endorser_party,
            signature: None,
            certificate_type,
            document_distribution: None,
            supporting_document_reference: None,
            issuing_country,
            importer_party: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfApplicationStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ApplicationStatusCode>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfApplicationStatusCodeComponent> for CertificateOfOriginApplicationArrayOfApplicationStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfApplicationStatusCodeComponent> for CertificateOfOriginApplicationArrayOfApplicationStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfApplicationStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfApplicationStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfApplicationStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ApplicationStatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ApplicationStatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ApplicationStatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ApplicationStatusCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfCertificateTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CertificateType>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfCertificateTypeComponent> for CertificateOfOriginApplicationArrayOfCertificateTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfCertificateTypeComponent> for CertificateOfOriginApplicationArrayOfCertificateTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfCertificateTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfCertificateTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfCertificateTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CertificateType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CertificateType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CertificateType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CertificateType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfDocumentDistributionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentDistribution>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfDocumentDistributionComponent> for CertificateOfOriginApplicationArrayOfDocumentDistributionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfDocumentDistributionComponent> for CertificateOfOriginApplicationArrayOfDocumentDistributionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfDocumentDistributionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfDocumentDistributionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DocumentDistribution) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DocumentDistribution) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DocumentDistribution> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DocumentDistribution> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfEndorserPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EndorserParty>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfEndorserPartyComponent> for CertificateOfOriginApplicationArrayOfEndorserPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfEndorserPartyComponent> for CertificateOfOriginApplicationArrayOfEndorserPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfEndorserPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfEndorserPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EndorserParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EndorserParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EndorserParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EndorserParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfExporterPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExporterParty>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfExporterPartyComponent> for CertificateOfOriginApplicationArrayOfExporterPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfExporterPartyComponent> for CertificateOfOriginApplicationArrayOfExporterPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfExporterPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfExporterPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfExporterPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ExporterParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ExporterParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ExporterParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ExporterParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfImporterPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ImporterParty>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfImporterPartyComponent> for CertificateOfOriginApplicationArrayOfImporterPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfImporterPartyComponent> for CertificateOfOriginApplicationArrayOfImporterPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfImporterPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfImporterPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfImporterPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ImporterParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ImporterParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ImporterParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ImporterParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfIssuerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::IssuerParty>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfIssuerPartyComponent> for CertificateOfOriginApplicationArrayOfIssuerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfIssuerPartyComponent> for CertificateOfOriginApplicationArrayOfIssuerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfIssuerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfIssuerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfIssuerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::IssuerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::IssuerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::IssuerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::IssuerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfIssuingCountryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::IssuingCountry>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfIssuingCountryComponent> for CertificateOfOriginApplicationArrayOfIssuingCountryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfIssuingCountryComponent> for CertificateOfOriginApplicationArrayOfIssuingCountryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfIssuingCountryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfIssuingCountryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfIssuingCountryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::IssuingCountry) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::IssuingCountry) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::IssuingCountry> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::IssuingCountry> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfOriginalJobIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OriginalJobID>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfOriginalJobIDComponent> for CertificateOfOriginApplicationArrayOfOriginalJobIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfOriginalJobIDComponent> for CertificateOfOriginApplicationArrayOfOriginalJobIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfOriginalJobIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfOriginalJobIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfOriginalJobIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OriginalJobID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OriginalJobID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OriginalJobID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OriginalJobID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfPreparationPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PreparationParty>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfPreparationPartyComponent> for CertificateOfOriginApplicationArrayOfPreparationPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfPreparationPartyComponent> for CertificateOfOriginApplicationArrayOfPreparationPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfPreparationPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfPreparationPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfPreparationPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PreparationParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PreparationParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PreparationParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PreparationParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfPreviousJobIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PreviousJobID>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfPreviousJobIDComponent> for CertificateOfOriginApplicationArrayOfPreviousJobIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfPreviousJobIDComponent> for CertificateOfOriginApplicationArrayOfPreviousJobIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfPreviousJobIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfPreviousJobIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfPreviousJobIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PreviousJobID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PreviousJobID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PreviousJobID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PreviousJobID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfReferenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReferenceID>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfReferenceIDComponent> for CertificateOfOriginApplicationArrayOfReferenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfReferenceIDComponent> for CertificateOfOriginApplicationArrayOfReferenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfReferenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfReferenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfReferenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReferenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReferenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReferenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReferenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfRemarksComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Remarks>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfRemarksComponent> for CertificateOfOriginApplicationArrayOfRemarksComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfRemarksComponent> for CertificateOfOriginApplicationArrayOfRemarksComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfRemarksComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfRemarksComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Remarks) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Remarks) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Remarks> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Remarks> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfShipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Shipment>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfShipmentComponent> for CertificateOfOriginApplicationArrayOfShipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfShipmentComponent> for CertificateOfOriginApplicationArrayOfShipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfShipmentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfShipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfShipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Shipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Shipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Shipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Shipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfSignatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Signature>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfSignatureComponent> for CertificateOfOriginApplicationArrayOfSignatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfSignatureComponent> for CertificateOfOriginApplicationArrayOfSignatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfSignatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfSignatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Signature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Signature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Signature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Signature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateOfOriginApplicationArrayOfSupportingDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SupportingDocumentReference>,
}

impl AsMut<CertificateOfOriginApplicationArrayOfSupportingDocumentReferenceComponent> for CertificateOfOriginApplicationArrayOfSupportingDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateOfOriginApplicationArrayOfSupportingDocumentReferenceComponent> for CertificateOfOriginApplicationArrayOfSupportingDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CertificateOfOriginApplicationArrayOfSupportingDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateOfOriginApplicationArrayOfSupportingDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SupportingDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SupportingDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SupportingDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SupportingDocumentReference> {
        self.items.iter()
    }
}


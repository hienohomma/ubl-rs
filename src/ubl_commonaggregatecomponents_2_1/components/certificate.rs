use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(rename = "CertificateType")]
    pub certificate_type: CertificateArrayOfCertificateTypeComponent,
    #[serde(rename = "CertificateTypeCode")]
    pub certificate_type_code: CertificateArrayOfCertificateTypeCodeComponent,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<CertificateArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ID")]
    pub id: CertificateArrayOfIDComponent,
    #[serde(rename = "IssuerParty")]
    pub issuer_party: CertificateArrayOfIssuerPartyComponent,
    #[serde(rename = "Remarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<CertificateArrayOfRemarksComponent>,
    #[serde(rename = "Signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<CertificateArrayOfSignatureComponent>,
}

impl AsMut<Certificate> for Certificate {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Certificate> for Certificate {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("Certificate.id", e));
        }
        if let Some(v) = &self.remarks {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Certificate.remarks", e));
            }
        }
        if let Err(e) = self.certificate_type.validate() {
            return Err(UblError::component("Certificate.certificate_type", e));
        }
        if let Err(e) = self.certificate_type_code.validate() {
            return Err(UblError::component("Certificate.certificate_type_code", e));
        }
        if let Some(v) = &self.signature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Certificate.signature", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Certificate.document_reference", e));
            }
        }
        if let Err(e) = self.issuer_party.validate() {
            return Err(UblError::component("Certificate.issuer_party", e));
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

impl Certificate {
    pub fn title() -> &'static str {
        "Certificate. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a certificate applied to the item. Certificated can be a requirement to sell goods or services in a jurisdiction."
    }
    pub fn new(certificate_type: CertificateArrayOfCertificateTypeComponent, certificate_type_code: CertificateArrayOfCertificateTypeCodeComponent, id: CertificateArrayOfIDComponent, issuer_party: CertificateArrayOfIssuerPartyComponent) -> Component<Self> {
        Component(Self {
            signature: None,
            document_reference: None,
            certificate_type,
            id,
            certificate_type_code,
            issuer_party,
            remarks: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateArrayOfCertificateTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CertificateType>,
}

impl AsMut<CertificateArrayOfCertificateTypeComponent> for CertificateArrayOfCertificateTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateArrayOfCertificateTypeComponent> for CertificateArrayOfCertificateTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateArrayOfCertificateTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateArrayOfCertificateTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateArrayOfCertificateTypeComponent {
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
pub struct CertificateArrayOfCertificateTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CertificateTypeCode>,
}

impl AsMut<CertificateArrayOfCertificateTypeCodeComponent> for CertificateArrayOfCertificateTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateArrayOfCertificateTypeCodeComponent> for CertificateArrayOfCertificateTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateArrayOfCertificateTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateArrayOfCertificateTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateArrayOfCertificateTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CertificateTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CertificateTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CertificateTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CertificateTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<CertificateArrayOfDocumentReferenceComponent> for CertificateArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateArrayOfDocumentReferenceComponent> for CertificateArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CertificateArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateArrayOfDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CertificateArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<CertificateArrayOfIDComponent> for CertificateArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateArrayOfIDComponent> for CertificateArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateArrayOfIDComponent {
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
pub struct CertificateArrayOfIssuerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::IssuerParty>,
}

impl AsMut<CertificateArrayOfIssuerPartyComponent> for CertificateArrayOfIssuerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateArrayOfIssuerPartyComponent> for CertificateArrayOfIssuerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CertificateArrayOfIssuerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CertificateArrayOfIssuerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateArrayOfIssuerPartyComponent {
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
pub struct CertificateArrayOfRemarksComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Remarks>,
}

impl AsMut<CertificateArrayOfRemarksComponent> for CertificateArrayOfRemarksComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateArrayOfRemarksComponent> for CertificateArrayOfRemarksComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CertificateArrayOfRemarksComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateArrayOfRemarksComponent {
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
pub struct CertificateArrayOfSignatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Signature>,
}

impl AsMut<CertificateArrayOfSignatureComponent> for CertificateArrayOfSignatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CertificateArrayOfSignatureComponent> for CertificateArrayOfSignatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CertificateArrayOfSignatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CertificateArrayOfSignatureComponent {
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


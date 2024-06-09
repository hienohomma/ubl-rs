use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmobilizedSecurity {
    #[serde(rename = "FaceValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_value_amount: Option<ImmobilizedSecurityArrayOfFaceValueAmountComponent>,
    #[serde(rename = "ImmobilizationCertificateID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immobilization_certificate_id: Option<ImmobilizedSecurityArrayOfImmobilizationCertificateIDComponent>,
    #[serde(rename = "IssueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<ImmobilizedSecurityArrayOfIssueDateComponent>,
    #[serde(rename = "IssuerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_party: Option<ImmobilizedSecurityArrayOfIssuerPartyComponent>,
    #[serde(rename = "MarketValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_value_amount: Option<ImmobilizedSecurityArrayOfMarketValueAmountComponent>,
    #[serde(rename = "SecurityID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<ImmobilizedSecurityArrayOfSecurityIDComponent>,
    #[serde(rename = "SharesNumberQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_number_quantity: Option<ImmobilizedSecurityArrayOfSharesNumberQuantityComponent>,
}

impl AsMut<ImmobilizedSecurity> for ImmobilizedSecurity {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ImmobilizedSecurity> for ImmobilizedSecurity {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.security_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ImmobilizedSecurity.security_id", e));
            }
        }
        if let Some(v) = &self.issue_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ImmobilizedSecurity.issue_date", e));
            }
        }
        if let Some(v) = &self.market_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ImmobilizedSecurity.market_value_amount", e));
            }
        }
        if let Some(v) = &self.shares_number_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ImmobilizedSecurity.shares_number_quantity", e));
            }
        }
        if let Some(v) = &self.face_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ImmobilizedSecurity.face_value_amount", e));
            }
        }
        if let Some(v) = &self.immobilization_certificate_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ImmobilizedSecurity.immobilization_certificate_id", e));
            }
        }
        if let Some(v) = &self.issuer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ImmobilizedSecurity.issuer_party", e));
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

impl ImmobilizedSecurity {
    pub fn title() -> &'static str {
        "Immobilized Security. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an immobilized security to be used as a guarantee."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            immobilization_certificate_id: None,
            issuer_party: None,
            security_id: None,
            shares_number_quantity: None,
            face_value_amount: None,
            issue_date: None,
            market_value_amount: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ImmobilizedSecurityArrayOfFaceValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FaceValueAmount>,
}

impl AsMut<ImmobilizedSecurityArrayOfFaceValueAmountComponent> for ImmobilizedSecurityArrayOfFaceValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ImmobilizedSecurityArrayOfFaceValueAmountComponent> for ImmobilizedSecurityArrayOfFaceValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfFaceValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfFaceValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ImmobilizedSecurityArrayOfFaceValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FaceValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FaceValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FaceValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FaceValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ImmobilizedSecurityArrayOfImmobilizationCertificateIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ImmobilizationCertificateID>,
}

impl AsMut<ImmobilizedSecurityArrayOfImmobilizationCertificateIDComponent> for ImmobilizedSecurityArrayOfImmobilizationCertificateIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ImmobilizedSecurityArrayOfImmobilizationCertificateIDComponent> for ImmobilizedSecurityArrayOfImmobilizationCertificateIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfImmobilizationCertificateIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfImmobilizationCertificateIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ImmobilizedSecurityArrayOfImmobilizationCertificateIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ImmobilizationCertificateID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ImmobilizationCertificateID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ImmobilizationCertificateID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ImmobilizationCertificateID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ImmobilizedSecurityArrayOfIssueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueDate>,
}

impl AsMut<ImmobilizedSecurityArrayOfIssueDateComponent> for ImmobilizedSecurityArrayOfIssueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ImmobilizedSecurityArrayOfIssueDateComponent> for ImmobilizedSecurityArrayOfIssueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfIssueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfIssueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ImmobilizedSecurityArrayOfIssueDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IssueDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IssueDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IssueDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IssueDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ImmobilizedSecurityArrayOfIssuerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::IssuerParty>,
}

impl AsMut<ImmobilizedSecurityArrayOfIssuerPartyComponent> for ImmobilizedSecurityArrayOfIssuerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ImmobilizedSecurityArrayOfIssuerPartyComponent> for ImmobilizedSecurityArrayOfIssuerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfIssuerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfIssuerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ImmobilizedSecurityArrayOfIssuerPartyComponent {
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
pub struct ImmobilizedSecurityArrayOfMarketValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MarketValueAmount>,
}

impl AsMut<ImmobilizedSecurityArrayOfMarketValueAmountComponent> for ImmobilizedSecurityArrayOfMarketValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ImmobilizedSecurityArrayOfMarketValueAmountComponent> for ImmobilizedSecurityArrayOfMarketValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfMarketValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfMarketValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ImmobilizedSecurityArrayOfMarketValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MarketValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MarketValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MarketValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MarketValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ImmobilizedSecurityArrayOfSecurityIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SecurityID>,
}

impl AsMut<ImmobilizedSecurityArrayOfSecurityIDComponent> for ImmobilizedSecurityArrayOfSecurityIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ImmobilizedSecurityArrayOfSecurityIDComponent> for ImmobilizedSecurityArrayOfSecurityIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfSecurityIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfSecurityIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ImmobilizedSecurityArrayOfSecurityIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SecurityID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SecurityID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SecurityID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SecurityID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ImmobilizedSecurityArrayOfSharesNumberQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SharesNumberQuantity>,
}

impl AsMut<ImmobilizedSecurityArrayOfSharesNumberQuantityComponent> for ImmobilizedSecurityArrayOfSharesNumberQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ImmobilizedSecurityArrayOfSharesNumberQuantityComponent> for ImmobilizedSecurityArrayOfSharesNumberQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfSharesNumberQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ImmobilizedSecurityArrayOfSharesNumberQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ImmobilizedSecurityArrayOfSharesNumberQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SharesNumberQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SharesNumberQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SharesNumberQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SharesNumberQuantity> {
        self.items.iter()
    }
}


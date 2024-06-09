use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartyTaxScheme {
    #[serde(rename = "CompanyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<PartyTaxSchemeArrayOfCompanyIDComponent>,
    #[serde(rename = "ExemptionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_reason: Option<PartyTaxSchemeArrayOfExemptionReasonComponent>,
    #[serde(rename = "ExemptionReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_reason_code: Option<PartyTaxSchemeArrayOfExemptionReasonCodeComponent>,
    #[serde(rename = "RegistrationAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_address: Option<PartyTaxSchemeArrayOfRegistrationAddressComponent>,
    #[serde(rename = "RegistrationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_name: Option<PartyTaxSchemeArrayOfRegistrationNameComponent>,
    #[serde(rename = "TaxLevelCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_level_code: Option<PartyTaxSchemeArrayOfTaxLevelCodeComponent>,
    #[serde(rename = "TaxScheme")]
    pub tax_scheme: PartyTaxSchemeArrayOfTaxSchemeComponent,
}

impl AsMut<PartyTaxScheme> for PartyTaxScheme {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyTaxScheme> for PartyTaxScheme {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.tax_level_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyTaxScheme.tax_level_code", e));
            }
        }
        if let Some(v) = &self.company_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyTaxScheme.company_id", e));
            }
        }
        if let Some(v) = &self.exemption_reason_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyTaxScheme.exemption_reason_code", e));
            }
        }
        if let Some(v) = &self.exemption_reason {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyTaxScheme.exemption_reason", e));
            }
        }
        if let Some(v) = &self.registration_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyTaxScheme.registration_address", e));
            }
        }
        if let Some(v) = &self.registration_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyTaxScheme.registration_name", e));
            }
        }
        if let Err(e) = self.tax_scheme.validate() {
            return Err(UblError::component("PartyTaxScheme.tax_scheme", e));
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

impl PartyTaxScheme {
    pub fn title() -> &'static str {
        "Party Tax Scheme. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a taxation scheme applying to a party."
    }
    pub fn new(tax_scheme: PartyTaxSchemeArrayOfTaxSchemeComponent) -> Component<Self> {
        Component(Self {
            registration_address: None,
            registration_name: None,
            tax_scheme,
            exemption_reason_code: None,
            company_id: None,
            tax_level_code: None,
            exemption_reason: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyTaxSchemeArrayOfCompanyIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CompanyID>,
}

impl AsMut<PartyTaxSchemeArrayOfCompanyIDComponent> for PartyTaxSchemeArrayOfCompanyIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyTaxSchemeArrayOfCompanyIDComponent> for PartyTaxSchemeArrayOfCompanyIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfCompanyIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfCompanyIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyTaxSchemeArrayOfCompanyIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CompanyID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CompanyID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CompanyID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CompanyID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyTaxSchemeArrayOfExemptionReasonComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExemptionReason>,
}

impl AsMut<PartyTaxSchemeArrayOfExemptionReasonComponent> for PartyTaxSchemeArrayOfExemptionReasonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyTaxSchemeArrayOfExemptionReasonComponent> for PartyTaxSchemeArrayOfExemptionReasonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfExemptionReasonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyTaxSchemeArrayOfExemptionReasonComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExemptionReason) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExemptionReason) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExemptionReason> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExemptionReason> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyTaxSchemeArrayOfExemptionReasonCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExemptionReasonCode>,
}

impl AsMut<PartyTaxSchemeArrayOfExemptionReasonCodeComponent> for PartyTaxSchemeArrayOfExemptionReasonCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyTaxSchemeArrayOfExemptionReasonCodeComponent> for PartyTaxSchemeArrayOfExemptionReasonCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfExemptionReasonCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfExemptionReasonCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyTaxSchemeArrayOfExemptionReasonCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExemptionReasonCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExemptionReasonCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExemptionReasonCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExemptionReasonCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyTaxSchemeArrayOfRegistrationAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RegistrationAddress>,
}

impl AsMut<PartyTaxSchemeArrayOfRegistrationAddressComponent> for PartyTaxSchemeArrayOfRegistrationAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyTaxSchemeArrayOfRegistrationAddressComponent> for PartyTaxSchemeArrayOfRegistrationAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfRegistrationAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfRegistrationAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyTaxSchemeArrayOfRegistrationAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RegistrationAddress) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RegistrationAddress) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RegistrationAddress> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RegistrationAddress> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyTaxSchemeArrayOfRegistrationNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RegistrationName>,
}

impl AsMut<PartyTaxSchemeArrayOfRegistrationNameComponent> for PartyTaxSchemeArrayOfRegistrationNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyTaxSchemeArrayOfRegistrationNameComponent> for PartyTaxSchemeArrayOfRegistrationNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfRegistrationNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfRegistrationNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyTaxSchemeArrayOfRegistrationNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RegistrationName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RegistrationName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RegistrationName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RegistrationName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyTaxSchemeArrayOfTaxLevelCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxLevelCode>,
}

impl AsMut<PartyTaxSchemeArrayOfTaxLevelCodeComponent> for PartyTaxSchemeArrayOfTaxLevelCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyTaxSchemeArrayOfTaxLevelCodeComponent> for PartyTaxSchemeArrayOfTaxLevelCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfTaxLevelCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfTaxLevelCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyTaxSchemeArrayOfTaxLevelCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxLevelCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxLevelCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxLevelCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxLevelCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyTaxSchemeArrayOfTaxSchemeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxScheme>,
}

impl AsMut<PartyTaxSchemeArrayOfTaxSchemeComponent> for PartyTaxSchemeArrayOfTaxSchemeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyTaxSchemeArrayOfTaxSchemeComponent> for PartyTaxSchemeArrayOfTaxSchemeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfTaxSchemeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyTaxSchemeArrayOfTaxSchemeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyTaxSchemeArrayOfTaxSchemeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxScheme) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxScheme) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxScheme> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxScheme> {
        self.items.iter()
    }
}


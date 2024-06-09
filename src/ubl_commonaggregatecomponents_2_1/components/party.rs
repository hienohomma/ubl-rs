use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Party {
    #[serde(rename = "AgentParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_party: Option<PartyArrayOfAgentPartyComponent>,
    #[serde(rename = "Contact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<PartyArrayOfContactComponent>,
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<PartyArrayOfEndpointIDComponent>,
    #[serde(rename = "FinancialAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<PartyArrayOfFinancialAccountComponent>,
    #[serde(rename = "IndustryClassificationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_classification_code: Option<PartyArrayOfIndustryClassificationCodeComponent>,
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<PartyArrayOfLanguageComponent>,
    #[serde(rename = "LogoReferenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_reference_id: Option<PartyArrayOfLogoReferenceIDComponent>,
    #[serde(rename = "MarkAttentionIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_attention_indicator: Option<PartyArrayOfMarkAttentionIndicatorComponent>,
    #[serde(rename = "MarkCareIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_care_indicator: Option<PartyArrayOfMarkCareIndicatorComponent>,
    #[serde(rename = "PartyIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_identification: Option<PartyArrayOfPartyIdentificationComponent>,
    #[serde(rename = "PartyLegalEntity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_legal_entity: Option<PartyArrayOfPartyLegalEntityComponent>,
    #[serde(rename = "PartyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_name: Option<PartyArrayOfPartyNameComponent>,
    #[serde(rename = "PartyTaxScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_tax_scheme: Option<PartyArrayOfPartyTaxSchemeComponent>,
    #[serde(rename = "Person")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PartyArrayOfPersonComponent>,
    #[serde(rename = "PhysicalLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_location: Option<PartyArrayOfPhysicalLocationComponent>,
    #[serde(rename = "PostalAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<PartyArrayOfPostalAddressComponent>,
    #[serde(rename = "PowerOfAttorney")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_of_attorney: Option<PartyArrayOfPowerOfAttorneyComponent>,
    #[serde(rename = "ServiceProviderParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_provider_party: Option<PartyArrayOfServiceProviderPartyComponent>,
    #[serde(rename = "WebsiteURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_uri: Option<PartyArrayOfWebsiteURIComponent>,
}

impl AsMut<Party> for Party {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Party> for Party {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.service_provider_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.service_provider_party", e));
            }
        }
        if let Some(v) = &self.endpoint_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.endpoint_id", e));
            }
        }
        if let Some(v) = &self.logo_reference_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.logo_reference_id", e));
            }
        }
        if let Some(v) = &self.power_of_attorney {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.power_of_attorney", e));
            }
        }
        if let Some(v) = &self.website_uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.website_uri", e));
            }
        }
        if let Some(v) = &self.mark_attention_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.mark_attention_indicator", e));
            }
        }
        if let Some(v) = &self.financial_account {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.financial_account", e));
            }
        }
        if let Some(v) = &self.person {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.person", e));
            }
        }
        if let Some(v) = &self.agent_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.agent_party", e));
            }
        }
        if let Some(v) = &self.language {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.language", e));
            }
        }
        if let Some(v) = &self.industry_classification_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.industry_classification_code", e));
            }
        }
        if let Some(v) = &self.contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.contact", e));
            }
        }
        if let Some(v) = &self.party_legal_entity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.party_legal_entity", e));
            }
        }
        if let Some(v) = &self.party_identification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.party_identification", e));
            }
        }
        if let Some(v) = &self.physical_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.physical_location", e));
            }
        }
        if let Some(v) = &self.party_tax_scheme {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.party_tax_scheme", e));
            }
        }
        if let Some(v) = &self.party_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.party_name", e));
            }
        }
        if let Some(v) = &self.mark_care_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.mark_care_indicator", e));
            }
        }
        if let Some(v) = &self.postal_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Party.postal_address", e));
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

impl Party {
    pub fn title() -> &'static str {
        "Party. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an organization, sub-organization, or individual fulfilling a role in a business process."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            mark_attention_indicator: None,
            party_tax_scheme: None,
            logo_reference_id: None,
            endpoint_id: None,
            language: None,
            party_legal_entity: None,
            postal_address: None,
            service_provider_party: None,
            financial_account: None,
            contact: None,
            mark_care_indicator: None,
            party_name: None,
            power_of_attorney: None,
            agent_party: None,
            person: None,
            industry_classification_code: None,
            physical_location: None,
            website_uri: None,
            party_identification: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfAgentPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AgentParty>,
}

impl AsMut<PartyArrayOfAgentPartyComponent> for PartyArrayOfAgentPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfAgentPartyComponent> for PartyArrayOfAgentPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfAgentPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfAgentPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfAgentPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AgentParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AgentParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AgentParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AgentParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Contact>,
}

impl AsMut<PartyArrayOfContactComponent> for PartyArrayOfContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfContactComponent> for PartyArrayOfContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfContactComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Contact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Contact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Contact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Contact> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfEndpointIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EndpointID>,
}

impl AsMut<PartyArrayOfEndpointIDComponent> for PartyArrayOfEndpointIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfEndpointIDComponent> for PartyArrayOfEndpointIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfEndpointIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfEndpointIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfEndpointIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EndpointID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EndpointID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EndpointID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EndpointID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfFinancialAccountComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FinancialAccount>,
}

impl AsMut<PartyArrayOfFinancialAccountComponent> for PartyArrayOfFinancialAccountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfFinancialAccountComponent> for PartyArrayOfFinancialAccountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfFinancialAccountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfFinancialAccountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfFinancialAccountComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FinancialAccount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FinancialAccount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FinancialAccount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FinancialAccount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfIndustryClassificationCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IndustryClassificationCode>,
}

impl AsMut<PartyArrayOfIndustryClassificationCodeComponent> for PartyArrayOfIndustryClassificationCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfIndustryClassificationCodeComponent> for PartyArrayOfIndustryClassificationCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfIndustryClassificationCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfIndustryClassificationCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfIndustryClassificationCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IndustryClassificationCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IndustryClassificationCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IndustryClassificationCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IndustryClassificationCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfLanguageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Language>,
}

impl AsMut<PartyArrayOfLanguageComponent> for PartyArrayOfLanguageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfLanguageComponent> for PartyArrayOfLanguageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfLanguageComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfLanguageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfLanguageComponent {
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
pub struct PartyArrayOfLogoReferenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LogoReferenceID>,
}

impl AsMut<PartyArrayOfLogoReferenceIDComponent> for PartyArrayOfLogoReferenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfLogoReferenceIDComponent> for PartyArrayOfLogoReferenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfLogoReferenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfLogoReferenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfLogoReferenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LogoReferenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LogoReferenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LogoReferenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LogoReferenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfMarkAttentionIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MarkAttentionIndicator>,
}

impl AsMut<PartyArrayOfMarkAttentionIndicatorComponent> for PartyArrayOfMarkAttentionIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfMarkAttentionIndicatorComponent> for PartyArrayOfMarkAttentionIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfMarkAttentionIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfMarkAttentionIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfMarkAttentionIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MarkAttentionIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MarkAttentionIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MarkAttentionIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MarkAttentionIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfMarkCareIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MarkCareIndicator>,
}

impl AsMut<PartyArrayOfMarkCareIndicatorComponent> for PartyArrayOfMarkCareIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfMarkCareIndicatorComponent> for PartyArrayOfMarkCareIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfMarkCareIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfMarkCareIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfMarkCareIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MarkCareIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MarkCareIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MarkCareIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MarkCareIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfPartyIdentificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PartyIdentification>,
}

impl AsMut<PartyArrayOfPartyIdentificationComponent> for PartyArrayOfPartyIdentificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfPartyIdentificationComponent> for PartyArrayOfPartyIdentificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfPartyIdentificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfPartyIdentificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PartyIdentification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PartyIdentification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PartyIdentification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PartyIdentification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfPartyLegalEntityComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PartyLegalEntity>,
}

impl AsMut<PartyArrayOfPartyLegalEntityComponent> for PartyArrayOfPartyLegalEntityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfPartyLegalEntityComponent> for PartyArrayOfPartyLegalEntityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfPartyLegalEntityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfPartyLegalEntityComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PartyLegalEntity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PartyLegalEntity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PartyLegalEntity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PartyLegalEntity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfPartyNameComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PartyName>,
}

impl AsMut<PartyArrayOfPartyNameComponent> for PartyArrayOfPartyNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfPartyNameComponent> for PartyArrayOfPartyNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfPartyNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfPartyNameComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PartyName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PartyName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PartyName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PartyName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfPartyTaxSchemeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PartyTaxScheme>,
}

impl AsMut<PartyArrayOfPartyTaxSchemeComponent> for PartyArrayOfPartyTaxSchemeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfPartyTaxSchemeComponent> for PartyArrayOfPartyTaxSchemeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfPartyTaxSchemeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfPartyTaxSchemeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PartyTaxScheme) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PartyTaxScheme) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PartyTaxScheme> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PartyTaxScheme> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfPersonComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Person>,
}

impl AsMut<PartyArrayOfPersonComponent> for PartyArrayOfPersonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfPersonComponent> for PartyArrayOfPersonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfPersonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfPersonComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Person) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Person) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Person> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Person> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfPhysicalLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PhysicalLocation>,
}

impl AsMut<PartyArrayOfPhysicalLocationComponent> for PartyArrayOfPhysicalLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfPhysicalLocationComponent> for PartyArrayOfPhysicalLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfPhysicalLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfPhysicalLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfPhysicalLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PhysicalLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PhysicalLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PhysicalLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PhysicalLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfPostalAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PostalAddress>,
}

impl AsMut<PartyArrayOfPostalAddressComponent> for PartyArrayOfPostalAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfPostalAddressComponent> for PartyArrayOfPostalAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfPostalAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfPostalAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfPostalAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PostalAddress) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PostalAddress) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PostalAddress> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PostalAddress> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfPowerOfAttorneyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PowerOfAttorney>,
}

impl AsMut<PartyArrayOfPowerOfAttorneyComponent> for PartyArrayOfPowerOfAttorneyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfPowerOfAttorneyComponent> for PartyArrayOfPowerOfAttorneyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfPowerOfAttorneyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfPowerOfAttorneyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PowerOfAttorney) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PowerOfAttorney) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PowerOfAttorney> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PowerOfAttorney> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfServiceProviderPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ServiceProviderParty>,
}

impl AsMut<PartyArrayOfServiceProviderPartyComponent> for PartyArrayOfServiceProviderPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfServiceProviderPartyComponent> for PartyArrayOfServiceProviderPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfServiceProviderPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfServiceProviderPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ServiceProviderParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ServiceProviderParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ServiceProviderParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ServiceProviderParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyArrayOfWebsiteURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::WebsiteURI>,
}

impl AsMut<PartyArrayOfWebsiteURIComponent> for PartyArrayOfWebsiteURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyArrayOfWebsiteURIComponent> for PartyArrayOfWebsiteURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyArrayOfWebsiteURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyArrayOfWebsiteURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyArrayOfWebsiteURIComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::WebsiteURI) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::WebsiteURI) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::WebsiteURI> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::WebsiteURI> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppealTerms {
    #[serde(rename = "AppealInformationParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appeal_information_party: Option<AppealTermsArrayOfAppealInformationPartyComponent>,
    #[serde(rename = "AppealReceiverParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appeal_receiver_party: Option<AppealTermsArrayOfAppealReceiverPartyComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<AppealTermsArrayOfDescriptionComponent>,
    #[serde(rename = "MediationParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediation_party: Option<AppealTermsArrayOfMediationPartyComponent>,
    #[serde(rename = "PresentationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_period: Option<AppealTermsArrayOfPresentationPeriodComponent>,
}

impl AsMut<AppealTerms> for AppealTerms {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AppealTerms> for AppealTerms {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.mediation_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AppealTerms.mediation_party", e));
            }
        }
        if let Some(v) = &self.appeal_information_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AppealTerms.appeal_information_party", e));
            }
        }
        if let Some(v) = &self.appeal_receiver_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AppealTerms.appeal_receiver_party", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AppealTerms.description", e));
            }
        }
        if let Some(v) = &self.presentation_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AppealTerms.presentation_period", e));
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

impl AppealTerms {
    pub fn title() -> &'static str {
        "Appeal Terms. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the terms and conditions, set by the contracting authority, under which an appeal can be lodged for a tender award."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            appeal_information_party: None,
            mediation_party: None,
            description: None,
            appeal_receiver_party: None,
            presentation_period: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AppealTermsArrayOfAppealInformationPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AppealInformationParty>,
}

impl AsMut<AppealTermsArrayOfAppealInformationPartyComponent> for AppealTermsArrayOfAppealInformationPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AppealTermsArrayOfAppealInformationPartyComponent> for AppealTermsArrayOfAppealInformationPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AppealTermsArrayOfAppealInformationPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AppealTermsArrayOfAppealInformationPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AppealTermsArrayOfAppealInformationPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AppealInformationParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AppealInformationParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AppealInformationParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AppealInformationParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AppealTermsArrayOfAppealReceiverPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AppealReceiverParty>,
}

impl AsMut<AppealTermsArrayOfAppealReceiverPartyComponent> for AppealTermsArrayOfAppealReceiverPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AppealTermsArrayOfAppealReceiverPartyComponent> for AppealTermsArrayOfAppealReceiverPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AppealTermsArrayOfAppealReceiverPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AppealTermsArrayOfAppealReceiverPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AppealTermsArrayOfAppealReceiverPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AppealReceiverParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AppealReceiverParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AppealReceiverParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AppealReceiverParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AppealTermsArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<AppealTermsArrayOfDescriptionComponent> for AppealTermsArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AppealTermsArrayOfDescriptionComponent> for AppealTermsArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AppealTermsArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AppealTermsArrayOfDescriptionComponent {
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
pub struct AppealTermsArrayOfMediationPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MediationParty>,
}

impl AsMut<AppealTermsArrayOfMediationPartyComponent> for AppealTermsArrayOfMediationPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AppealTermsArrayOfMediationPartyComponent> for AppealTermsArrayOfMediationPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AppealTermsArrayOfMediationPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AppealTermsArrayOfMediationPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AppealTermsArrayOfMediationPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MediationParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MediationParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MediationParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MediationParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AppealTermsArrayOfPresentationPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PresentationPeriod>,
}

impl AsMut<AppealTermsArrayOfPresentationPeriodComponent> for AppealTermsArrayOfPresentationPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AppealTermsArrayOfPresentationPeriodComponent> for AppealTermsArrayOfPresentationPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AppealTermsArrayOfPresentationPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AppealTermsArrayOfPresentationPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AppealTermsArrayOfPresentationPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PresentationPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PresentationPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PresentationPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PresentationPeriod> {
        self.items.iter()
    }
}


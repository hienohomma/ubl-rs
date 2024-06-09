use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradingTerms {
    #[serde(rename = "ApplicableAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_address: Option<TradingTermsArrayOfApplicableAddressComponent>,
    #[serde(rename = "Information")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<TradingTermsArrayOfInformationComponent>,
    #[serde(rename = "Reference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<TradingTermsArrayOfReferenceComponent>,
}

impl AsMut<TradingTerms> for TradingTerms {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradingTerms> for TradingTerms {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.information {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TradingTerms.information", e));
            }
        }
        if let Some(v) = &self.applicable_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TradingTerms.applicable_address", e));
            }
        }
        if let Some(v) = &self.reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TradingTerms.reference", e));
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

impl TradingTerms {
    pub fn title() -> &'static str {
        "Trading Terms. Details"
    }
    pub fn description() -> &'static str {
        "A class for describing the terms of a trade agreement."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            applicable_address: None,
            reference: None,
            information: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TradingTermsArrayOfApplicableAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ApplicableAddress>,
}

impl AsMut<TradingTermsArrayOfApplicableAddressComponent> for TradingTermsArrayOfApplicableAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradingTermsArrayOfApplicableAddressComponent> for TradingTermsArrayOfApplicableAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TradingTermsArrayOfApplicableAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TradingTermsArrayOfApplicableAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TradingTermsArrayOfApplicableAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ApplicableAddress) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ApplicableAddress) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ApplicableAddress> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ApplicableAddress> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TradingTermsArrayOfInformationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Information>,
}

impl AsMut<TradingTermsArrayOfInformationComponent> for TradingTermsArrayOfInformationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradingTermsArrayOfInformationComponent> for TradingTermsArrayOfInformationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TradingTermsArrayOfInformationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TradingTermsArrayOfInformationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Information) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Information) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Information> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Information> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TradingTermsArrayOfReferenceComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Reference>,
}

impl AsMut<TradingTermsArrayOfReferenceComponent> for TradingTermsArrayOfReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradingTermsArrayOfReferenceComponent> for TradingTermsArrayOfReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TradingTermsArrayOfReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TradingTermsArrayOfReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TradingTermsArrayOfReferenceComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Reference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Reference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Reference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Reference> {
        self.items.iter()
    }
}


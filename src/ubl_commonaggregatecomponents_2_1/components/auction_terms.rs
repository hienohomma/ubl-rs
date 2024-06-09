use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuctionTerms {
    #[serde(rename = "AuctionConstraintIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auction_constraint_indicator: Option<AuctionTermsArrayOfAuctionConstraintIndicatorComponent>,
    #[serde(rename = "AuctionURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auction_uri: Option<AuctionTermsArrayOfAuctionURIComponent>,
    #[serde(rename = "ConditionsDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions_description: Option<AuctionTermsArrayOfConditionsDescriptionComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<AuctionTermsArrayOfDescriptionComponent>,
    #[serde(rename = "ElectronicDeviceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_device_description: Option<AuctionTermsArrayOfElectronicDeviceDescriptionComponent>,
    #[serde(rename = "JustificationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification_description: Option<AuctionTermsArrayOfJustificationDescriptionComponent>,
    #[serde(rename = "ProcessDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_description: Option<AuctionTermsArrayOfProcessDescriptionComponent>,
}

impl AsMut<AuctionTerms> for AuctionTerms {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AuctionTerms> for AuctionTerms {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.conditions_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AuctionTerms.conditions_description", e));
            }
        }
        if let Some(v) = &self.process_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AuctionTerms.process_description", e));
            }
        }
        if let Some(v) = &self.auction_uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AuctionTerms.auction_uri", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AuctionTerms.description", e));
            }
        }
        if let Some(v) = &self.auction_constraint_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AuctionTerms.auction_constraint_indicator", e));
            }
        }
        if let Some(v) = &self.electronic_device_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AuctionTerms.electronic_device_description", e));
            }
        }
        if let Some(v) = &self.justification_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("AuctionTerms.justification_description", e));
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

impl AuctionTerms {
    pub fn title() -> &'static str {
        "Auction Terms. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the terms to be fulfilled by tenderers if an auction is to be executed before the awarding of a tender."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            auction_constraint_indicator: None,
            auction_uri: None,
            conditions_description: None,
            electronic_device_description: None,
            justification_description: None,
            process_description: None,
            description: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AuctionTermsArrayOfAuctionConstraintIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AuctionConstraintIndicator>,
}

impl AsMut<AuctionTermsArrayOfAuctionConstraintIndicatorComponent> for AuctionTermsArrayOfAuctionConstraintIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AuctionTermsArrayOfAuctionConstraintIndicatorComponent> for AuctionTermsArrayOfAuctionConstraintIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AuctionTermsArrayOfAuctionConstraintIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AuctionTermsArrayOfAuctionConstraintIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AuctionTermsArrayOfAuctionConstraintIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AuctionConstraintIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AuctionConstraintIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AuctionConstraintIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AuctionConstraintIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AuctionTermsArrayOfAuctionURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AuctionURI>,
}

impl AsMut<AuctionTermsArrayOfAuctionURIComponent> for AuctionTermsArrayOfAuctionURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AuctionTermsArrayOfAuctionURIComponent> for AuctionTermsArrayOfAuctionURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AuctionTermsArrayOfAuctionURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AuctionTermsArrayOfAuctionURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AuctionTermsArrayOfAuctionURIComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AuctionURI) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AuctionURI) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AuctionURI> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AuctionURI> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AuctionTermsArrayOfConditionsDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConditionsDescription>,
}

impl AsMut<AuctionTermsArrayOfConditionsDescriptionComponent> for AuctionTermsArrayOfConditionsDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AuctionTermsArrayOfConditionsDescriptionComponent> for AuctionTermsArrayOfConditionsDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AuctionTermsArrayOfConditionsDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AuctionTermsArrayOfConditionsDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConditionsDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConditionsDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConditionsDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConditionsDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AuctionTermsArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<AuctionTermsArrayOfDescriptionComponent> for AuctionTermsArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AuctionTermsArrayOfDescriptionComponent> for AuctionTermsArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AuctionTermsArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AuctionTermsArrayOfDescriptionComponent {
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
pub struct AuctionTermsArrayOfElectronicDeviceDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ElectronicDeviceDescription>,
}

impl AsMut<AuctionTermsArrayOfElectronicDeviceDescriptionComponent> for AuctionTermsArrayOfElectronicDeviceDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AuctionTermsArrayOfElectronicDeviceDescriptionComponent> for AuctionTermsArrayOfElectronicDeviceDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AuctionTermsArrayOfElectronicDeviceDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AuctionTermsArrayOfElectronicDeviceDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ElectronicDeviceDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ElectronicDeviceDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ElectronicDeviceDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ElectronicDeviceDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AuctionTermsArrayOfJustificationDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::JustificationDescription>,
}

impl AsMut<AuctionTermsArrayOfJustificationDescriptionComponent> for AuctionTermsArrayOfJustificationDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AuctionTermsArrayOfJustificationDescriptionComponent> for AuctionTermsArrayOfJustificationDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AuctionTermsArrayOfJustificationDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AuctionTermsArrayOfJustificationDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::JustificationDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::JustificationDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::JustificationDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::JustificationDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AuctionTermsArrayOfProcessDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ProcessDescription>,
}

impl AsMut<AuctionTermsArrayOfProcessDescriptionComponent> for AuctionTermsArrayOfProcessDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AuctionTermsArrayOfProcessDescriptionComponent> for AuctionTermsArrayOfProcessDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AuctionTermsArrayOfProcessDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AuctionTermsArrayOfProcessDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ProcessDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ProcessDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ProcessDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ProcessDescription> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardAccount {
    #[serde(rename = "CV2ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cv2id: Option<CardAccountArrayOfCV2IDComponent>,
    #[serde(rename = "CardChipCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_chip_code: Option<CardAccountArrayOfCardChipCodeComponent>,
    #[serde(rename = "CardTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type_code: Option<CardAccountArrayOfCardTypeCodeComponent>,
    #[serde(rename = "ChipApplicationID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chip_application_id: Option<CardAccountArrayOfChipApplicationIDComponent>,
    #[serde(rename = "ExpiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<CardAccountArrayOfExpiryDateComponent>,
    #[serde(rename = "HolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<CardAccountArrayOfHolderNameComponent>,
    #[serde(rename = "IssueNumberID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_number_id: Option<CardAccountArrayOfIssueNumberIDComponent>,
    #[serde(rename = "IssuerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_id: Option<CardAccountArrayOfIssuerIDComponent>,
    #[serde(rename = "NetworkID")]
    pub network_id: CardAccountArrayOfNetworkIDComponent,
    #[serde(rename = "PrimaryAccountNumberID")]
    pub primary_account_number_id: CardAccountArrayOfPrimaryAccountNumberIDComponent,
    #[serde(rename = "ValidityStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_start_date: Option<CardAccountArrayOfValidityStartDateComponent>,
}

impl AsMut<CardAccount> for CardAccount {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccount> for CardAccount {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.network_id.validate() {
            return Err(UblError::component("CardAccount.network_id", e));
        }
        if let Some(v) = &self.card_chip_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CardAccount.card_chip_code", e));
            }
        }
        if let Some(v) = &self.holder_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CardAccount.holder_name", e));
            }
        }
        if let Some(v) = &self.chip_application_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CardAccount.chip_application_id", e));
            }
        }
        if let Some(v) = &self.card_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CardAccount.card_type_code", e));
            }
        }
        if let Some(v) = &self.issue_number_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CardAccount.issue_number_id", e));
            }
        }
        if let Some(v) = &self.cv2id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CardAccount.cv2id", e));
            }
        }
        if let Some(v) = &self.issuer_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CardAccount.issuer_id", e));
            }
        }
        if let Err(e) = self.primary_account_number_id.validate() {
            return Err(UblError::component("CardAccount.primary_account_number_id", e));
        }
        if let Some(v) = &self.expiry_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CardAccount.expiry_date", e));
            }
        }
        if let Some(v) = &self.validity_start_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CardAccount.validity_start_date", e));
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

impl CardAccount {
    pub fn title() -> &'static str {
        "Card Account. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a credit card, debit card, or charge card account."
    }
    pub fn new(network_id: CardAccountArrayOfNetworkIDComponent, primary_account_number_id: CardAccountArrayOfPrimaryAccountNumberIDComponent) -> Component<Self> {
        Component(Self {
            card_chip_code: None,
            issuer_id: None,
            chip_application_id: None,
            issue_number_id: None,
            holder_name: None,
            network_id,
            primary_account_number_id,
            validity_start_date: None,
            card_type_code: None,
            expiry_date: None,
            cv2id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardAccountArrayOfCV2IDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CV2ID>,
}

impl AsMut<CardAccountArrayOfCV2IDComponent> for CardAccountArrayOfCV2IDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccountArrayOfCV2IDComponent> for CardAccountArrayOfCV2IDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CardAccountArrayOfCV2IDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CardAccountArrayOfCV2IDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CardAccountArrayOfCV2IDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CV2ID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CV2ID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CV2ID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CV2ID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardAccountArrayOfCardChipCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CardChipCode>,
}

impl AsMut<CardAccountArrayOfCardChipCodeComponent> for CardAccountArrayOfCardChipCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccountArrayOfCardChipCodeComponent> for CardAccountArrayOfCardChipCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CardAccountArrayOfCardChipCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CardAccountArrayOfCardChipCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CardAccountArrayOfCardChipCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CardChipCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CardChipCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CardChipCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CardChipCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardAccountArrayOfCardTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CardTypeCode>,
}

impl AsMut<CardAccountArrayOfCardTypeCodeComponent> for CardAccountArrayOfCardTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccountArrayOfCardTypeCodeComponent> for CardAccountArrayOfCardTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CardAccountArrayOfCardTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CardAccountArrayOfCardTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CardAccountArrayOfCardTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CardTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CardTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CardTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CardTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardAccountArrayOfChipApplicationIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ChipApplicationID>,
}

impl AsMut<CardAccountArrayOfChipApplicationIDComponent> for CardAccountArrayOfChipApplicationIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccountArrayOfChipApplicationIDComponent> for CardAccountArrayOfChipApplicationIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CardAccountArrayOfChipApplicationIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CardAccountArrayOfChipApplicationIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CardAccountArrayOfChipApplicationIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ChipApplicationID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ChipApplicationID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ChipApplicationID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ChipApplicationID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardAccountArrayOfExpiryDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExpiryDate>,
}

impl AsMut<CardAccountArrayOfExpiryDateComponent> for CardAccountArrayOfExpiryDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccountArrayOfExpiryDateComponent> for CardAccountArrayOfExpiryDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CardAccountArrayOfExpiryDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CardAccountArrayOfExpiryDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CardAccountArrayOfExpiryDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExpiryDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExpiryDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExpiryDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExpiryDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardAccountArrayOfHolderNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HolderName>,
}

impl AsMut<CardAccountArrayOfHolderNameComponent> for CardAccountArrayOfHolderNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccountArrayOfHolderNameComponent> for CardAccountArrayOfHolderNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CardAccountArrayOfHolderNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CardAccountArrayOfHolderNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CardAccountArrayOfHolderNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HolderName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HolderName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HolderName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HolderName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardAccountArrayOfIssueNumberIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueNumberID>,
}

impl AsMut<CardAccountArrayOfIssueNumberIDComponent> for CardAccountArrayOfIssueNumberIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccountArrayOfIssueNumberIDComponent> for CardAccountArrayOfIssueNumberIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CardAccountArrayOfIssueNumberIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CardAccountArrayOfIssueNumberIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CardAccountArrayOfIssueNumberIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IssueNumberID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IssueNumberID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IssueNumberID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IssueNumberID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardAccountArrayOfIssuerIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssuerID>,
}

impl AsMut<CardAccountArrayOfIssuerIDComponent> for CardAccountArrayOfIssuerIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccountArrayOfIssuerIDComponent> for CardAccountArrayOfIssuerIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CardAccountArrayOfIssuerIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CardAccountArrayOfIssuerIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CardAccountArrayOfIssuerIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IssuerID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IssuerID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IssuerID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IssuerID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardAccountArrayOfNetworkIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetworkID>,
}

impl AsMut<CardAccountArrayOfNetworkIDComponent> for CardAccountArrayOfNetworkIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccountArrayOfNetworkIDComponent> for CardAccountArrayOfNetworkIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CardAccountArrayOfNetworkIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CardAccountArrayOfNetworkIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CardAccountArrayOfNetworkIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NetworkID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NetworkID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NetworkID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NetworkID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardAccountArrayOfPrimaryAccountNumberIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PrimaryAccountNumberID>,
}

impl AsMut<CardAccountArrayOfPrimaryAccountNumberIDComponent> for CardAccountArrayOfPrimaryAccountNumberIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccountArrayOfPrimaryAccountNumberIDComponent> for CardAccountArrayOfPrimaryAccountNumberIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CardAccountArrayOfPrimaryAccountNumberIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CardAccountArrayOfPrimaryAccountNumberIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CardAccountArrayOfPrimaryAccountNumberIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PrimaryAccountNumberID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PrimaryAccountNumberID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PrimaryAccountNumberID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PrimaryAccountNumberID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardAccountArrayOfValidityStartDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValidityStartDate>,
}

impl AsMut<CardAccountArrayOfValidityStartDateComponent> for CardAccountArrayOfValidityStartDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CardAccountArrayOfValidityStartDateComponent> for CardAccountArrayOfValidityStartDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CardAccountArrayOfValidityStartDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CardAccountArrayOfValidityStartDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CardAccountArrayOfValidityStartDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValidityStartDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValidityStartDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValidityStartDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValidityStartDate> {
        self.items.iter()
    }
}


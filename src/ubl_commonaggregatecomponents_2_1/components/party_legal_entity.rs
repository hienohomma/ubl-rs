use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartyLegalEntity {
    #[serde(rename = "CompanyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<PartyLegalEntityArrayOfCompanyIDComponent>,
    #[serde(rename = "CompanyLegalForm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_legal_form: Option<PartyLegalEntityArrayOfCompanyLegalFormComponent>,
    #[serde(rename = "CompanyLegalFormCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_legal_form_code: Option<PartyLegalEntityArrayOfCompanyLegalFormCodeComponent>,
    #[serde(rename = "CompanyLiquidationStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_liquidation_status_code: Option<PartyLegalEntityArrayOfCompanyLiquidationStatusCodeComponent>,
    #[serde(rename = "CorporateRegistrationScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corporate_registration_scheme: Option<PartyLegalEntityArrayOfCorporateRegistrationSchemeComponent>,
    #[serde(rename = "CorporateStockAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corporate_stock_amount: Option<PartyLegalEntityArrayOfCorporateStockAmountComponent>,
    #[serde(rename = "FullyPaidSharesIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_paid_shares_indicator: Option<PartyLegalEntityArrayOfFullyPaidSharesIndicatorComponent>,
    #[serde(rename = "HeadOfficeParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_office_party: Option<PartyLegalEntityArrayOfHeadOfficePartyComponent>,
    #[serde(rename = "RegistrationAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_address: Option<PartyLegalEntityArrayOfRegistrationAddressComponent>,
    #[serde(rename = "RegistrationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<PartyLegalEntityArrayOfRegistrationDateComponent>,
    #[serde(rename = "RegistrationExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_expiration_date: Option<PartyLegalEntityArrayOfRegistrationExpirationDateComponent>,
    #[serde(rename = "RegistrationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_name: Option<PartyLegalEntityArrayOfRegistrationNameComponent>,
    #[serde(rename = "ShareholderParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shareholder_party: Option<PartyLegalEntityArrayOfShareholderPartyComponent>,
    #[serde(rename = "SoleProprietorshipIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sole_proprietorship_indicator: Option<PartyLegalEntityArrayOfSoleProprietorshipIndicatorComponent>,
}

impl AsMut<PartyLegalEntity> for PartyLegalEntity {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntity> for PartyLegalEntity {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.corporate_registration_scheme {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.corporate_registration_scheme", e));
            }
        }
        if let Some(v) = &self.shareholder_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.shareholder_party", e));
            }
        }
        if let Some(v) = &self.company_legal_form_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.company_legal_form_code", e));
            }
        }
        if let Some(v) = &self.corporate_stock_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.corporate_stock_amount", e));
            }
        }
        if let Some(v) = &self.company_liquidation_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.company_liquidation_status_code", e));
            }
        }
        if let Some(v) = &self.company_legal_form {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.company_legal_form", e));
            }
        }
        if let Some(v) = &self.head_office_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.head_office_party", e));
            }
        }
        if let Some(v) = &self.registration_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.registration_address", e));
            }
        }
        if let Some(v) = &self.registration_expiration_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.registration_expiration_date", e));
            }
        }
        if let Some(v) = &self.sole_proprietorship_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.sole_proprietorship_indicator", e));
            }
        }
        if let Some(v) = &self.company_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.company_id", e));
            }
        }
        if let Some(v) = &self.registration_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.registration_date", e));
            }
        }
        if let Some(v) = &self.fully_paid_shares_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.fully_paid_shares_indicator", e));
            }
        }
        if let Some(v) = &self.registration_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PartyLegalEntity.registration_name", e));
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

impl PartyLegalEntity {
    pub fn title() -> &'static str {
        "Party Legal Entity. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a party as a legal entity."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            company_legal_form_code: None,
            company_legal_form: None,
            fully_paid_shares_indicator: None,
            registration_address: None,
            corporate_stock_amount: None,
            company_liquidation_status_code: None,
            registration_date: None,
            head_office_party: None,
            registration_name: None,
            company_id: None,
            shareholder_party: None,
            sole_proprietorship_indicator: None,
            registration_expiration_date: None,
            corporate_registration_scheme: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyLegalEntityArrayOfCompanyIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CompanyID>,
}

impl AsMut<PartyLegalEntityArrayOfCompanyIDComponent> for PartyLegalEntityArrayOfCompanyIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfCompanyIDComponent> for PartyLegalEntityArrayOfCompanyIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCompanyIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCompanyIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfCompanyIDComponent {
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
pub struct PartyLegalEntityArrayOfCompanyLegalFormComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CompanyLegalForm>,
}

impl AsMut<PartyLegalEntityArrayOfCompanyLegalFormComponent> for PartyLegalEntityArrayOfCompanyLegalFormComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfCompanyLegalFormComponent> for PartyLegalEntityArrayOfCompanyLegalFormComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCompanyLegalFormComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCompanyLegalFormComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfCompanyLegalFormComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CompanyLegalForm) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CompanyLegalForm) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CompanyLegalForm> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CompanyLegalForm> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyLegalEntityArrayOfCompanyLegalFormCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CompanyLegalFormCode>,
}

impl AsMut<PartyLegalEntityArrayOfCompanyLegalFormCodeComponent> for PartyLegalEntityArrayOfCompanyLegalFormCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfCompanyLegalFormCodeComponent> for PartyLegalEntityArrayOfCompanyLegalFormCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCompanyLegalFormCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCompanyLegalFormCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfCompanyLegalFormCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CompanyLegalFormCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CompanyLegalFormCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CompanyLegalFormCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CompanyLegalFormCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyLegalEntityArrayOfCompanyLiquidationStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CompanyLiquidationStatusCode>,
}

impl AsMut<PartyLegalEntityArrayOfCompanyLiquidationStatusCodeComponent> for PartyLegalEntityArrayOfCompanyLiquidationStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfCompanyLiquidationStatusCodeComponent> for PartyLegalEntityArrayOfCompanyLiquidationStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCompanyLiquidationStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCompanyLiquidationStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfCompanyLiquidationStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CompanyLiquidationStatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CompanyLiquidationStatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CompanyLiquidationStatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CompanyLiquidationStatusCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyLegalEntityArrayOfCorporateRegistrationSchemeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CorporateRegistrationScheme>,
}

impl AsMut<PartyLegalEntityArrayOfCorporateRegistrationSchemeComponent> for PartyLegalEntityArrayOfCorporateRegistrationSchemeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfCorporateRegistrationSchemeComponent> for PartyLegalEntityArrayOfCorporateRegistrationSchemeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCorporateRegistrationSchemeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCorporateRegistrationSchemeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfCorporateRegistrationSchemeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CorporateRegistrationScheme) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CorporateRegistrationScheme) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CorporateRegistrationScheme> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CorporateRegistrationScheme> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyLegalEntityArrayOfCorporateStockAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CorporateStockAmount>,
}

impl AsMut<PartyLegalEntityArrayOfCorporateStockAmountComponent> for PartyLegalEntityArrayOfCorporateStockAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfCorporateStockAmountComponent> for PartyLegalEntityArrayOfCorporateStockAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCorporateStockAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfCorporateStockAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfCorporateStockAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CorporateStockAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CorporateStockAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CorporateStockAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CorporateStockAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyLegalEntityArrayOfFullyPaidSharesIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FullyPaidSharesIndicator>,
}

impl AsMut<PartyLegalEntityArrayOfFullyPaidSharesIndicatorComponent> for PartyLegalEntityArrayOfFullyPaidSharesIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfFullyPaidSharesIndicatorComponent> for PartyLegalEntityArrayOfFullyPaidSharesIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfFullyPaidSharesIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfFullyPaidSharesIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfFullyPaidSharesIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FullyPaidSharesIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FullyPaidSharesIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FullyPaidSharesIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FullyPaidSharesIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyLegalEntityArrayOfHeadOfficePartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::HeadOfficeParty>,
}

impl AsMut<PartyLegalEntityArrayOfHeadOfficePartyComponent> for PartyLegalEntityArrayOfHeadOfficePartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfHeadOfficePartyComponent> for PartyLegalEntityArrayOfHeadOfficePartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfHeadOfficePartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfHeadOfficePartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfHeadOfficePartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::HeadOfficeParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::HeadOfficeParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::HeadOfficeParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::HeadOfficeParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyLegalEntityArrayOfRegistrationAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RegistrationAddress>,
}

impl AsMut<PartyLegalEntityArrayOfRegistrationAddressComponent> for PartyLegalEntityArrayOfRegistrationAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfRegistrationAddressComponent> for PartyLegalEntityArrayOfRegistrationAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfRegistrationAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfRegistrationAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfRegistrationAddressComponent {
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
pub struct PartyLegalEntityArrayOfRegistrationDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RegistrationDate>,
}

impl AsMut<PartyLegalEntityArrayOfRegistrationDateComponent> for PartyLegalEntityArrayOfRegistrationDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfRegistrationDateComponent> for PartyLegalEntityArrayOfRegistrationDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfRegistrationDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfRegistrationDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfRegistrationDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RegistrationDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RegistrationDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RegistrationDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RegistrationDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyLegalEntityArrayOfRegistrationExpirationDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RegistrationExpirationDate>,
}

impl AsMut<PartyLegalEntityArrayOfRegistrationExpirationDateComponent> for PartyLegalEntityArrayOfRegistrationExpirationDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfRegistrationExpirationDateComponent> for PartyLegalEntityArrayOfRegistrationExpirationDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfRegistrationExpirationDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfRegistrationExpirationDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfRegistrationExpirationDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RegistrationExpirationDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RegistrationExpirationDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RegistrationExpirationDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RegistrationExpirationDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyLegalEntityArrayOfRegistrationNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RegistrationName>,
}

impl AsMut<PartyLegalEntityArrayOfRegistrationNameComponent> for PartyLegalEntityArrayOfRegistrationNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfRegistrationNameComponent> for PartyLegalEntityArrayOfRegistrationNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfRegistrationNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfRegistrationNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfRegistrationNameComponent {
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
pub struct PartyLegalEntityArrayOfShareholderPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ShareholderParty>,
}

impl AsMut<PartyLegalEntityArrayOfShareholderPartyComponent> for PartyLegalEntityArrayOfShareholderPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfShareholderPartyComponent> for PartyLegalEntityArrayOfShareholderPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfShareholderPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfShareholderPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ShareholderParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ShareholderParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ShareholderParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ShareholderParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyLegalEntityArrayOfSoleProprietorshipIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SoleProprietorshipIndicator>,
}

impl AsMut<PartyLegalEntityArrayOfSoleProprietorshipIndicatorComponent> for PartyLegalEntityArrayOfSoleProprietorshipIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PartyLegalEntityArrayOfSoleProprietorshipIndicatorComponent> for PartyLegalEntityArrayOfSoleProprietorshipIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfSoleProprietorshipIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PartyLegalEntityArrayOfSoleProprietorshipIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PartyLegalEntityArrayOfSoleProprietorshipIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SoleProprietorshipIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SoleProprietorshipIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SoleProprietorshipIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SoleProprietorshipIndicator> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FinancialAccount {
    #[serde(rename = "AccountFormatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_format_code: Option<FinancialAccountArrayOfAccountFormatCodeComponent>,
    #[serde(rename = "AccountTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type_code: Option<FinancialAccountArrayOfAccountTypeCodeComponent>,
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<FinancialAccountArrayOfAliasNameComponent>,
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<FinancialAccountArrayOfCountryComponent>,
    #[serde(rename = "CurrencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<FinancialAccountArrayOfCurrencyCodeComponent>,
    #[serde(rename = "FinancialInstitutionBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_institution_branch: Option<FinancialAccountArrayOfFinancialInstitutionBranchComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<FinancialAccountArrayOfIDComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<FinancialAccountArrayOfNameComponent>,
    #[serde(rename = "PaymentNote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_note: Option<FinancialAccountArrayOfPaymentNoteComponent>,
}

impl AsMut<FinancialAccount> for FinancialAccount {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialAccount> for FinancialAccount {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.account_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialAccount.account_type_code", e));
            }
        }
        if let Some(v) = &self.alias_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialAccount.alias_name", e));
            }
        }
        if let Some(v) = &self.account_format_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialAccount.account_format_code", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialAccount.id", e));
            }
        }
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialAccount.name", e));
            }
        }
        if let Some(v) = &self.currency_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialAccount.currency_code", e));
            }
        }
        if let Some(v) = &self.financial_institution_branch {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialAccount.financial_institution_branch", e));
            }
        }
        if let Some(v) = &self.payment_note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialAccount.payment_note", e));
            }
        }
        if let Some(v) = &self.country {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialAccount.country", e));
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

impl FinancialAccount {
    pub fn title() -> &'static str {
        "Financial Account. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a financial account."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            country: None,
            currency_code: None,
            id: None,
            name: None,
            account_type_code: None,
            alias_name: None,
            account_format_code: None,
            payment_note: None,
            financial_institution_branch: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialAccountArrayOfAccountFormatCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountFormatCode>,
}

impl AsMut<FinancialAccountArrayOfAccountFormatCodeComponent> for FinancialAccountArrayOfAccountFormatCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialAccountArrayOfAccountFormatCodeComponent> for FinancialAccountArrayOfAccountFormatCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfAccountFormatCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfAccountFormatCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialAccountArrayOfAccountFormatCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AccountFormatCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AccountFormatCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AccountFormatCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AccountFormatCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialAccountArrayOfAccountTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountTypeCode>,
}

impl AsMut<FinancialAccountArrayOfAccountTypeCodeComponent> for FinancialAccountArrayOfAccountTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialAccountArrayOfAccountTypeCodeComponent> for FinancialAccountArrayOfAccountTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfAccountTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfAccountTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialAccountArrayOfAccountTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AccountTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AccountTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AccountTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AccountTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialAccountArrayOfAliasNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AliasName>,
}

impl AsMut<FinancialAccountArrayOfAliasNameComponent> for FinancialAccountArrayOfAliasNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialAccountArrayOfAliasNameComponent> for FinancialAccountArrayOfAliasNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfAliasNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfAliasNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialAccountArrayOfAliasNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AliasName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AliasName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AliasName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AliasName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialAccountArrayOfCountryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Country>,
}

impl AsMut<FinancialAccountArrayOfCountryComponent> for FinancialAccountArrayOfCountryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialAccountArrayOfCountryComponent> for FinancialAccountArrayOfCountryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfCountryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfCountryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialAccountArrayOfCountryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Country) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Country) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Country> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Country> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialAccountArrayOfCurrencyCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CurrencyCode>,
}

impl AsMut<FinancialAccountArrayOfCurrencyCodeComponent> for FinancialAccountArrayOfCurrencyCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialAccountArrayOfCurrencyCodeComponent> for FinancialAccountArrayOfCurrencyCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfCurrencyCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfCurrencyCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialAccountArrayOfCurrencyCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CurrencyCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CurrencyCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CurrencyCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CurrencyCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialAccountArrayOfFinancialInstitutionBranchComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FinancialInstitutionBranch>,
}

impl AsMut<FinancialAccountArrayOfFinancialInstitutionBranchComponent> for FinancialAccountArrayOfFinancialInstitutionBranchComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialAccountArrayOfFinancialInstitutionBranchComponent> for FinancialAccountArrayOfFinancialInstitutionBranchComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfFinancialInstitutionBranchComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfFinancialInstitutionBranchComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialAccountArrayOfFinancialInstitutionBranchComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FinancialInstitutionBranch) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FinancialInstitutionBranch) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FinancialInstitutionBranch> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FinancialInstitutionBranch> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialAccountArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<FinancialAccountArrayOfIDComponent> for FinancialAccountArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialAccountArrayOfIDComponent> for FinancialAccountArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialAccountArrayOfIDComponent {
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
pub struct FinancialAccountArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<FinancialAccountArrayOfNameComponent> for FinancialAccountArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialAccountArrayOfNameComponent> for FinancialAccountArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialAccountArrayOfNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Name) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Name) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialAccountArrayOfPaymentNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PaymentNote>,
}

impl AsMut<FinancialAccountArrayOfPaymentNoteComponent> for FinancialAccountArrayOfPaymentNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialAccountArrayOfPaymentNoteComponent> for FinancialAccountArrayOfPaymentNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("FinancialAccountArrayOfPaymentNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialAccountArrayOfPaymentNoteComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PaymentNote) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PaymentNote) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PaymentNote> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PaymentNote> {
        self.items.iter()
    }
}


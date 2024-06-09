use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreditAccount {
    #[serde(rename = "AccountID")]
    pub account_id: CreditAccountArrayOfAccountIDComponent,
}

impl AsMut<CreditAccount> for CreditAccount {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditAccount> for CreditAccount {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.account_id.validate() {
            return Err(UblError::component("CreditAccount.account_id", e));
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

impl CreditAccount {
    pub fn title() -> &'static str {
        "Credit Account. Details"
    }
    pub fn description() -> &'static str {
        "A class to identify a credit account for sales on account."
    }
    pub fn new(account_id: CreditAccountArrayOfAccountIDComponent) -> Component<Self> {
        Component(Self {
            account_id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreditAccountArrayOfAccountIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AccountID>,
}

impl AsMut<CreditAccountArrayOfAccountIDComponent> for CreditAccountArrayOfAccountIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CreditAccountArrayOfAccountIDComponent> for CreditAccountArrayOfAccountIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CreditAccountArrayOfAccountIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CreditAccountArrayOfAccountIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CreditAccountArrayOfAccountIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AccountID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AccountID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AccountID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AccountID> {
        self.items.iter()
    }
}


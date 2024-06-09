use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BudgetAccountLine {
    #[serde(rename = "BudgetAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_account: Option<BudgetAccountLineArrayOfBudgetAccountComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<BudgetAccountLineArrayOfIDComponent>,
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<BudgetAccountLineArrayOfTotalAmountComponent>,
}

impl AsMut<BudgetAccountLine> for BudgetAccountLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BudgetAccountLine> for BudgetAccountLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.total_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BudgetAccountLine.total_amount", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BudgetAccountLine.id", e));
            }
        }
        if let Some(v) = &self.budget_account {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BudgetAccountLine.budget_account", e));
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

impl BudgetAccountLine {
    pub fn title() -> &'static str {
        "Budget Account Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a budget account line."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            total_amount: None,
            budget_account: None,
            id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BudgetAccountLineArrayOfBudgetAccountComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BudgetAccount>,
}

impl AsMut<BudgetAccountLineArrayOfBudgetAccountComponent> for BudgetAccountLineArrayOfBudgetAccountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BudgetAccountLineArrayOfBudgetAccountComponent> for BudgetAccountLineArrayOfBudgetAccountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("BudgetAccountLineArrayOfBudgetAccountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BudgetAccountLineArrayOfBudgetAccountComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BudgetAccount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BudgetAccount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BudgetAccount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BudgetAccount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BudgetAccountLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<BudgetAccountLineArrayOfIDComponent> for BudgetAccountLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BudgetAccountLineArrayOfIDComponent> for BudgetAccountLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BudgetAccountLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BudgetAccountLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BudgetAccountLineArrayOfIDComponent {
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
pub struct BudgetAccountLineArrayOfTotalAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalAmount>,
}

impl AsMut<BudgetAccountLineArrayOfTotalAmountComponent> for BudgetAccountLineArrayOfTotalAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BudgetAccountLineArrayOfTotalAmountComponent> for BudgetAccountLineArrayOfTotalAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BudgetAccountLineArrayOfTotalAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BudgetAccountLineArrayOfTotalAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BudgetAccountLineArrayOfTotalAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalAmount> {
        self.items.iter()
    }
}


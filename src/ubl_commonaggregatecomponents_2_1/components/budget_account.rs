use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BudgetAccount {
    #[serde(rename = "BudgetYearNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_year_numeric: Option<BudgetAccountArrayOfBudgetYearNumericComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<BudgetAccountArrayOfIDComponent>,
    #[serde(rename = "RequiredClassificationScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_classification_scheme: Option<BudgetAccountArrayOfRequiredClassificationSchemeComponent>,
}

impl AsMut<BudgetAccount> for BudgetAccount {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BudgetAccount> for BudgetAccount {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.budget_year_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BudgetAccount.budget_year_numeric", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BudgetAccount.id", e));
            }
        }
        if let Some(v) = &self.required_classification_scheme {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("BudgetAccount.required_classification_scheme", e));
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

impl BudgetAccount {
    pub fn title() -> &'static str {
        "Budget Account. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a budget account."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            required_classification_scheme: None,
            budget_year_numeric: None,
            id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BudgetAccountArrayOfBudgetYearNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BudgetYearNumeric>,
}

impl AsMut<BudgetAccountArrayOfBudgetYearNumericComponent> for BudgetAccountArrayOfBudgetYearNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BudgetAccountArrayOfBudgetYearNumericComponent> for BudgetAccountArrayOfBudgetYearNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BudgetAccountArrayOfBudgetYearNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BudgetAccountArrayOfBudgetYearNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BudgetAccountArrayOfBudgetYearNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BudgetYearNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BudgetYearNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BudgetYearNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BudgetYearNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BudgetAccountArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<BudgetAccountArrayOfIDComponent> for BudgetAccountArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BudgetAccountArrayOfIDComponent> for BudgetAccountArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BudgetAccountArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BudgetAccountArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BudgetAccountArrayOfIDComponent {
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
pub struct BudgetAccountArrayOfRequiredClassificationSchemeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequiredClassificationScheme>,
}

impl AsMut<BudgetAccountArrayOfRequiredClassificationSchemeComponent> for BudgetAccountArrayOfRequiredClassificationSchemeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<BudgetAccountArrayOfRequiredClassificationSchemeComponent> for BudgetAccountArrayOfRequiredClassificationSchemeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("BudgetAccountArrayOfRequiredClassificationSchemeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("BudgetAccountArrayOfRequiredClassificationSchemeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl BudgetAccountArrayOfRequiredClassificationSchemeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequiredClassificationScheme) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequiredClassificationScheme) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequiredClassificationScheme> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequiredClassificationScheme> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Duty {
    #[serde(rename = "Amount")]
    pub amount: DutyArrayOfAmountComponent,
    #[serde(rename = "Duty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duty: Option<DutyArrayOfDutyComponent>,
    #[serde(rename = "DutyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duty_code: Option<DutyArrayOfDutyCodeComponent>,
    #[serde(rename = "TaxCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_category: Option<DutyArrayOfTaxCategoryComponent>,
}

impl AsMut<Duty> for Duty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Duty> for Duty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.duty_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Duty.duty_code", e));
            }
        }
        if let Some(v) = &self.duty {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Duty.duty", e));
            }
        }
        if let Some(v) = &self.tax_category {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Duty.tax_category", e));
            }
        }
        if let Err(e) = self.amount.validate() {
            return Err(UblError::component("Duty.amount", e));
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

impl Duty {
    pub fn title() -> &'static str {
        "Duty. Details"
    }
    pub fn description() -> &'static str {
        "The charging rate used for both call charging and time dependent charging"
    }
    pub fn new(amount: DutyArrayOfAmountComponent) -> Component<Self> {
        Component(Self {
            tax_category: None,
            duty_code: None,
            duty: None,
            amount,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DutyArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<DutyArrayOfAmountComponent> for DutyArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DutyArrayOfAmountComponent> for DutyArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DutyArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DutyArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DutyArrayOfAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Amount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Amount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Amount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Amount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DutyArrayOfDutyComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Duty>,
}

impl AsMut<DutyArrayOfDutyComponent> for DutyArrayOfDutyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DutyArrayOfDutyComponent> for DutyArrayOfDutyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DutyArrayOfDutyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DutyArrayOfDutyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DutyArrayOfDutyComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Duty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Duty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Duty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Duty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DutyArrayOfDutyCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DutyCode>,
}

impl AsMut<DutyArrayOfDutyCodeComponent> for DutyArrayOfDutyCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DutyArrayOfDutyCodeComponent> for DutyArrayOfDutyCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DutyArrayOfDutyCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DutyArrayOfDutyCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DutyArrayOfDutyCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DutyCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DutyCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DutyCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DutyCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DutyArrayOfTaxCategoryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxCategory>,
}

impl AsMut<DutyArrayOfTaxCategoryComponent> for DutyArrayOfTaxCategoryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DutyArrayOfTaxCategoryComponent> for DutyArrayOfTaxCategoryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DutyArrayOfTaxCategoryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DutyArrayOfTaxCategoryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DutyArrayOfTaxCategoryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxCategory) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxCategory) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxCategory> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxCategory> {
        self.items.iter()
    }
}


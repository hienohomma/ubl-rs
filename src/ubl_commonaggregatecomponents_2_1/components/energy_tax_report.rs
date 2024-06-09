use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnergyTaxReport {
    #[serde(rename = "TaxEnergyAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_energy_amount: Option<EnergyTaxReportArrayOfTaxEnergyAmountComponent>,
    #[serde(rename = "TaxEnergyBalanceAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_energy_balance_amount: Option<EnergyTaxReportArrayOfTaxEnergyBalanceAmountComponent>,
    #[serde(rename = "TaxEnergyOnAccountAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_energy_on_account_amount: Option<EnergyTaxReportArrayOfTaxEnergyOnAccountAmountComponent>,
    #[serde(rename = "TaxScheme")]
    pub tax_scheme: EnergyTaxReportArrayOfTaxSchemeComponent,
}

impl AsMut<EnergyTaxReport> for EnergyTaxReport {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnergyTaxReport> for EnergyTaxReport {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.tax_scheme.validate() {
            return Err(UblError::component("EnergyTaxReport.tax_scheme", e));
        }
        if let Some(v) = &self.tax_energy_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EnergyTaxReport.tax_energy_amount", e));
            }
        }
        if let Some(v) = &self.tax_energy_balance_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EnergyTaxReport.tax_energy_balance_amount", e));
            }
        }
        if let Some(v) = &self.tax_energy_on_account_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EnergyTaxReport.tax_energy_on_account_amount", e));
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

impl EnergyTaxReport {
    pub fn title() -> &'static str {
        "Energy Tax Report. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe energy taxes."
    }
    pub fn new(tax_scheme: EnergyTaxReportArrayOfTaxSchemeComponent) -> Component<Self> {
        Component(Self {
            tax_scheme,
            tax_energy_on_account_amount: None,
            tax_energy_balance_amount: None,
            tax_energy_amount: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EnergyTaxReportArrayOfTaxEnergyAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxEnergyAmount>,
}

impl AsMut<EnergyTaxReportArrayOfTaxEnergyAmountComponent> for EnergyTaxReportArrayOfTaxEnergyAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnergyTaxReportArrayOfTaxEnergyAmountComponent> for EnergyTaxReportArrayOfTaxEnergyAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EnergyTaxReportArrayOfTaxEnergyAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EnergyTaxReportArrayOfTaxEnergyAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnergyTaxReportArrayOfTaxEnergyAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxEnergyAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxEnergyAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxEnergyAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxEnergyAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EnergyTaxReportArrayOfTaxEnergyBalanceAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxEnergyBalanceAmount>,
}

impl AsMut<EnergyTaxReportArrayOfTaxEnergyBalanceAmountComponent> for EnergyTaxReportArrayOfTaxEnergyBalanceAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnergyTaxReportArrayOfTaxEnergyBalanceAmountComponent> for EnergyTaxReportArrayOfTaxEnergyBalanceAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EnergyTaxReportArrayOfTaxEnergyBalanceAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EnergyTaxReportArrayOfTaxEnergyBalanceAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnergyTaxReportArrayOfTaxEnergyBalanceAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxEnergyBalanceAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxEnergyBalanceAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxEnergyBalanceAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxEnergyBalanceAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EnergyTaxReportArrayOfTaxEnergyOnAccountAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TaxEnergyOnAccountAmount>,
}

impl AsMut<EnergyTaxReportArrayOfTaxEnergyOnAccountAmountComponent> for EnergyTaxReportArrayOfTaxEnergyOnAccountAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnergyTaxReportArrayOfTaxEnergyOnAccountAmountComponent> for EnergyTaxReportArrayOfTaxEnergyOnAccountAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EnergyTaxReportArrayOfTaxEnergyOnAccountAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EnergyTaxReportArrayOfTaxEnergyOnAccountAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnergyTaxReportArrayOfTaxEnergyOnAccountAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TaxEnergyOnAccountAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TaxEnergyOnAccountAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TaxEnergyOnAccountAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TaxEnergyOnAccountAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EnergyTaxReportArrayOfTaxSchemeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxScheme>,
}

impl AsMut<EnergyTaxReportArrayOfTaxSchemeComponent> for EnergyTaxReportArrayOfTaxSchemeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EnergyTaxReportArrayOfTaxSchemeComponent> for EnergyTaxReportArrayOfTaxSchemeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EnergyTaxReportArrayOfTaxSchemeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EnergyTaxReportArrayOfTaxSchemeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EnergyTaxReportArrayOfTaxSchemeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxScheme) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxScheme) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxScheme> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxScheme> {
        self.items.iter()
    }
}


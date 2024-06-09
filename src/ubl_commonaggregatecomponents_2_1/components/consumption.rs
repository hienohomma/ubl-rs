use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Consumption {
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<ConsumptionArrayOfAllowanceChargeComponent>,
    #[serde(rename = "EnergyWaterSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_water_supply: Option<ConsumptionArrayOfEnergyWaterSupplyComponent>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: ConsumptionArrayOfLegalMonetaryTotalComponent,
    #[serde(rename = "MainPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_period: Option<ConsumptionArrayOfMainPeriodComponent>,
    #[serde(rename = "TaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<ConsumptionArrayOfTaxTotalComponent>,
    #[serde(rename = "TelecommunicationsSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecommunications_supply: Option<ConsumptionArrayOfTelecommunicationsSupplyComponent>,
    #[serde(rename = "UtilityStatementTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_statement_type_code: Option<ConsumptionArrayOfUtilityStatementTypeCodeComponent>,
}

impl AsMut<Consumption> for Consumption {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Consumption> for Consumption {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consumption.allowance_charge", e));
            }
        }
        if let Err(e) = self.legal_monetary_total.validate() {
            return Err(UblError::component("Consumption.legal_monetary_total", e));
        }
        if let Some(v) = &self.main_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consumption.main_period", e));
            }
        }
        if let Some(v) = &self.energy_water_supply {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consumption.energy_water_supply", e));
            }
        }
        if let Some(v) = &self.tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consumption.tax_total", e));
            }
        }
        if let Some(v) = &self.telecommunications_supply {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consumption.telecommunications_supply", e));
            }
        }
        if let Some(v) = &self.utility_statement_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Consumption.utility_statement_type_code", e));
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

impl Consumption {
    pub fn title() -> &'static str {
        "Consumption. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the consumption of a utility."
    }
    pub fn new(legal_monetary_total: ConsumptionArrayOfLegalMonetaryTotalComponent) -> Component<Self> {
        Component(Self {
            utility_statement_type_code: None,
            tax_total: None,
            main_period: None,
            legal_monetary_total,
            allowance_charge: None,
            energy_water_supply: None,
            telecommunications_supply: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<ConsumptionArrayOfAllowanceChargeComponent> for ConsumptionArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionArrayOfAllowanceChargeComponent> for ConsumptionArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionArrayOfAllowanceChargeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionArrayOfEnergyWaterSupplyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EnergyWaterSupply>,
}

impl AsMut<ConsumptionArrayOfEnergyWaterSupplyComponent> for ConsumptionArrayOfEnergyWaterSupplyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionArrayOfEnergyWaterSupplyComponent> for ConsumptionArrayOfEnergyWaterSupplyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfEnergyWaterSupplyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfEnergyWaterSupplyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionArrayOfEnergyWaterSupplyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EnergyWaterSupply) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EnergyWaterSupply) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EnergyWaterSupply> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EnergyWaterSupply> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionArrayOfLegalMonetaryTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LegalMonetaryTotal>,
}

impl AsMut<ConsumptionArrayOfLegalMonetaryTotalComponent> for ConsumptionArrayOfLegalMonetaryTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionArrayOfLegalMonetaryTotalComponent> for ConsumptionArrayOfLegalMonetaryTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfLegalMonetaryTotalComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfLegalMonetaryTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionArrayOfLegalMonetaryTotalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LegalMonetaryTotal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LegalMonetaryTotal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LegalMonetaryTotal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LegalMonetaryTotal> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionArrayOfMainPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MainPeriod>,
}

impl AsMut<ConsumptionArrayOfMainPeriodComponent> for ConsumptionArrayOfMainPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionArrayOfMainPeriodComponent> for ConsumptionArrayOfMainPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfMainPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfMainPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionArrayOfMainPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MainPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MainPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MainPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MainPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionArrayOfTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxTotal>,
}

impl AsMut<ConsumptionArrayOfTaxTotalComponent> for ConsumptionArrayOfTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionArrayOfTaxTotalComponent> for ConsumptionArrayOfTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionArrayOfTaxTotalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionArrayOfTelecommunicationsSupplyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsSupply>,
}

impl AsMut<ConsumptionArrayOfTelecommunicationsSupplyComponent> for ConsumptionArrayOfTelecommunicationsSupplyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionArrayOfTelecommunicationsSupplyComponent> for ConsumptionArrayOfTelecommunicationsSupplyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfTelecommunicationsSupplyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfTelecommunicationsSupplyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionArrayOfTelecommunicationsSupplyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsSupply) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsSupply) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsSupply> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsSupply> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionArrayOfUtilityStatementTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UtilityStatementTypeCode>,
}

impl AsMut<ConsumptionArrayOfUtilityStatementTypeCodeComponent> for ConsumptionArrayOfUtilityStatementTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionArrayOfUtilityStatementTypeCodeComponent> for ConsumptionArrayOfUtilityStatementTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfUtilityStatementTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionArrayOfUtilityStatementTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionArrayOfUtilityStatementTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::UtilityStatementTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::UtilityStatementTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::UtilityStatementTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::UtilityStatementTypeCode> {
        self.items.iter()
    }
}


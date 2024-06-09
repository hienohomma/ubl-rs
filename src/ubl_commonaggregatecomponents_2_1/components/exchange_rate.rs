use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExchangeRate {
    #[serde(rename = "CalculationRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_rate: Option<ExchangeRateArrayOfCalculationRateComponent>,
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<ExchangeRateArrayOfDateComponent>,
    #[serde(rename = "ExchangeMarketID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_market_id: Option<ExchangeRateArrayOfExchangeMarketIDComponent>,
    #[serde(rename = "ForeignExchangeContract")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_exchange_contract: Option<ExchangeRateArrayOfForeignExchangeContractComponent>,
    #[serde(rename = "MathematicOperatorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mathematic_operator_code: Option<ExchangeRateArrayOfMathematicOperatorCodeComponent>,
    #[serde(rename = "SourceCurrencyBaseRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_currency_base_rate: Option<ExchangeRateArrayOfSourceCurrencyBaseRateComponent>,
    #[serde(rename = "SourceCurrencyCode")]
    pub source_currency_code: ExchangeRateArrayOfSourceCurrencyCodeComponent,
    #[serde(rename = "TargetCurrencyBaseRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_currency_base_rate: Option<ExchangeRateArrayOfTargetCurrencyBaseRateComponent>,
    #[serde(rename = "TargetCurrencyCode")]
    pub target_currency_code: ExchangeRateArrayOfTargetCurrencyCodeComponent,
}

impl AsMut<ExchangeRate> for ExchangeRate {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExchangeRate> for ExchangeRate {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExchangeRate.date", e));
            }
        }
        if let Some(v) = &self.calculation_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExchangeRate.calculation_rate", e));
            }
        }
        if let Some(v) = &self.mathematic_operator_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExchangeRate.mathematic_operator_code", e));
            }
        }
        if let Err(e) = self.source_currency_code.validate() {
            return Err(UblError::component("ExchangeRate.source_currency_code", e));
        }
        if let Some(v) = &self.source_currency_base_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExchangeRate.source_currency_base_rate", e));
            }
        }
        if let Some(v) = &self.target_currency_base_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExchangeRate.target_currency_base_rate", e));
            }
        }
        if let Err(e) = self.target_currency_code.validate() {
            return Err(UblError::component("ExchangeRate.target_currency_code", e));
        }
        if let Some(v) = &self.exchange_market_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExchangeRate.exchange_market_id", e));
            }
        }
        if let Some(v) = &self.foreign_exchange_contract {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExchangeRate.foreign_exchange_contract", e));
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

impl ExchangeRate {
    pub fn title() -> &'static str {
        "Exchange Rate. Details"
    }
    pub fn description() -> &'static str {
        "A class to define an exchange rate."
    }
    pub fn new(source_currency_code: ExchangeRateArrayOfSourceCurrencyCodeComponent, target_currency_code: ExchangeRateArrayOfTargetCurrencyCodeComponent) -> Component<Self> {
        Component(Self {
            source_currency_code,
            date: None,
            foreign_exchange_contract: None,
            source_currency_base_rate: None,
            target_currency_code,
            calculation_rate: None,
            target_currency_base_rate: None,
            mathematic_operator_code: None,
            exchange_market_id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExchangeRateArrayOfCalculationRateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CalculationRate>,
}

impl AsMut<ExchangeRateArrayOfCalculationRateComponent> for ExchangeRateArrayOfCalculationRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExchangeRateArrayOfCalculationRateComponent> for ExchangeRateArrayOfCalculationRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfCalculationRateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfCalculationRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExchangeRateArrayOfCalculationRateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CalculationRate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CalculationRate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CalculationRate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CalculationRate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExchangeRateArrayOfDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Date>,
}

impl AsMut<ExchangeRateArrayOfDateComponent> for ExchangeRateArrayOfDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExchangeRateArrayOfDateComponent> for ExchangeRateArrayOfDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExchangeRateArrayOfDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Date) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Date) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Date> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Date> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExchangeRateArrayOfExchangeMarketIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExchangeMarketID>,
}

impl AsMut<ExchangeRateArrayOfExchangeMarketIDComponent> for ExchangeRateArrayOfExchangeMarketIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExchangeRateArrayOfExchangeMarketIDComponent> for ExchangeRateArrayOfExchangeMarketIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfExchangeMarketIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfExchangeMarketIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExchangeRateArrayOfExchangeMarketIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExchangeMarketID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExchangeMarketID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExchangeMarketID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExchangeMarketID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExchangeRateArrayOfForeignExchangeContractComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ForeignExchangeContract>,
}

impl AsMut<ExchangeRateArrayOfForeignExchangeContractComponent> for ExchangeRateArrayOfForeignExchangeContractComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExchangeRateArrayOfForeignExchangeContractComponent> for ExchangeRateArrayOfForeignExchangeContractComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfForeignExchangeContractComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfForeignExchangeContractComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExchangeRateArrayOfForeignExchangeContractComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ForeignExchangeContract) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ForeignExchangeContract) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ForeignExchangeContract> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ForeignExchangeContract> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExchangeRateArrayOfMathematicOperatorCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MathematicOperatorCode>,
}

impl AsMut<ExchangeRateArrayOfMathematicOperatorCodeComponent> for ExchangeRateArrayOfMathematicOperatorCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExchangeRateArrayOfMathematicOperatorCodeComponent> for ExchangeRateArrayOfMathematicOperatorCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfMathematicOperatorCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfMathematicOperatorCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExchangeRateArrayOfMathematicOperatorCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MathematicOperatorCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MathematicOperatorCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MathematicOperatorCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MathematicOperatorCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExchangeRateArrayOfSourceCurrencyBaseRateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SourceCurrencyBaseRate>,
}

impl AsMut<ExchangeRateArrayOfSourceCurrencyBaseRateComponent> for ExchangeRateArrayOfSourceCurrencyBaseRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExchangeRateArrayOfSourceCurrencyBaseRateComponent> for ExchangeRateArrayOfSourceCurrencyBaseRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfSourceCurrencyBaseRateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfSourceCurrencyBaseRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExchangeRateArrayOfSourceCurrencyBaseRateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SourceCurrencyBaseRate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SourceCurrencyBaseRate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SourceCurrencyBaseRate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SourceCurrencyBaseRate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExchangeRateArrayOfSourceCurrencyCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SourceCurrencyCode>,
}

impl AsMut<ExchangeRateArrayOfSourceCurrencyCodeComponent> for ExchangeRateArrayOfSourceCurrencyCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExchangeRateArrayOfSourceCurrencyCodeComponent> for ExchangeRateArrayOfSourceCurrencyCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfSourceCurrencyCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfSourceCurrencyCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExchangeRateArrayOfSourceCurrencyCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SourceCurrencyCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SourceCurrencyCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SourceCurrencyCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SourceCurrencyCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExchangeRateArrayOfTargetCurrencyBaseRateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TargetCurrencyBaseRate>,
}

impl AsMut<ExchangeRateArrayOfTargetCurrencyBaseRateComponent> for ExchangeRateArrayOfTargetCurrencyBaseRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExchangeRateArrayOfTargetCurrencyBaseRateComponent> for ExchangeRateArrayOfTargetCurrencyBaseRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfTargetCurrencyBaseRateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfTargetCurrencyBaseRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExchangeRateArrayOfTargetCurrencyBaseRateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TargetCurrencyBaseRate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TargetCurrencyBaseRate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TargetCurrencyBaseRate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TargetCurrencyBaseRate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExchangeRateArrayOfTargetCurrencyCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TargetCurrencyCode>,
}

impl AsMut<ExchangeRateArrayOfTargetCurrencyCodeComponent> for ExchangeRateArrayOfTargetCurrencyCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExchangeRateArrayOfTargetCurrencyCodeComponent> for ExchangeRateArrayOfTargetCurrencyCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfTargetCurrencyCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExchangeRateArrayOfTargetCurrencyCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExchangeRateArrayOfTargetCurrencyCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TargetCurrencyCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TargetCurrencyCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TargetCurrencyCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TargetCurrencyCode> {
        self.items.iter()
    }
}


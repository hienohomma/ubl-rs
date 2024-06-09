use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemInformationRequestLine {
    #[serde(rename = "ForecastTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_type_code: Option<ItemInformationRequestLineArrayOfForecastTypeCodeComponent>,
    #[serde(rename = "PerformanceMetricTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_metric_type_code: Option<ItemInformationRequestLineArrayOfPerformanceMetricTypeCodeComponent>,
    #[serde(rename = "Period")]
    pub period: ItemInformationRequestLineArrayOfPeriodComponent,
    #[serde(rename = "SalesItem")]
    pub sales_item: ItemInformationRequestLineArrayOfSalesItemComponent,
    #[serde(rename = "SupplyChainActivityTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_chain_activity_type_code: Option<ItemInformationRequestLineArrayOfSupplyChainActivityTypeCodeComponent>,
    #[serde(rename = "TimeFrequencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_frequency_code: Option<ItemInformationRequestLineArrayOfTimeFrequencyCodeComponent>,
}

impl AsMut<ItemInformationRequestLine> for ItemInformationRequestLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInformationRequestLine> for ItemInformationRequestLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.supply_chain_activity_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInformationRequestLine.supply_chain_activity_type_code", e));
            }
        }
        if let Some(v) = &self.forecast_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInformationRequestLine.forecast_type_code", e));
            }
        }
        if let Some(v) = &self.time_frequency_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInformationRequestLine.time_frequency_code", e));
            }
        }
        if let Some(v) = &self.performance_metric_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemInformationRequestLine.performance_metric_type_code", e));
            }
        }
        if let Err(e) = self.period.validate() {
            return Err(UblError::component("ItemInformationRequestLine.period", e));
        }
        if let Err(e) = self.sales_item.validate() {
            return Err(UblError::component("ItemInformationRequestLine.sales_item", e));
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

impl ItemInformationRequestLine {
    pub fn title() -> &'static str {
        "Item Information Request Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in an Item Information Request asking a trading partner for item information."
    }
    pub fn new(period: ItemInformationRequestLineArrayOfPeriodComponent, sales_item: ItemInformationRequestLineArrayOfSalesItemComponent) -> Component<Self> {
        Component(Self {
            forecast_type_code: None,
            sales_item,
            period,
            supply_chain_activity_type_code: None,
            time_frequency_code: None,
            performance_metric_type_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInformationRequestLineArrayOfForecastTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode>,
}

impl AsMut<ItemInformationRequestLineArrayOfForecastTypeCodeComponent> for ItemInformationRequestLineArrayOfForecastTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInformationRequestLineArrayOfForecastTypeCodeComponent> for ItemInformationRequestLineArrayOfForecastTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemInformationRequestLineArrayOfForecastTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemInformationRequestLineArrayOfForecastTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInformationRequestLineArrayOfForecastTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInformationRequestLineArrayOfPerformanceMetricTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode>,
}

impl AsMut<ItemInformationRequestLineArrayOfPerformanceMetricTypeCodeComponent> for ItemInformationRequestLineArrayOfPerformanceMetricTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInformationRequestLineArrayOfPerformanceMetricTypeCodeComponent> for ItemInformationRequestLineArrayOfPerformanceMetricTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemInformationRequestLineArrayOfPerformanceMetricTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemInformationRequestLineArrayOfPerformanceMetricTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInformationRequestLineArrayOfPerformanceMetricTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInformationRequestLineArrayOfPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Period>,
}

impl AsMut<ItemInformationRequestLineArrayOfPeriodComponent> for ItemInformationRequestLineArrayOfPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInformationRequestLineArrayOfPeriodComponent> for ItemInformationRequestLineArrayOfPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemInformationRequestLineArrayOfPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInformationRequestLineArrayOfPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Period) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Period) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Period> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Period> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInformationRequestLineArrayOfSalesItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SalesItem>,
}

impl AsMut<ItemInformationRequestLineArrayOfSalesItemComponent> for ItemInformationRequestLineArrayOfSalesItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInformationRequestLineArrayOfSalesItemComponent> for ItemInformationRequestLineArrayOfSalesItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemInformationRequestLineArrayOfSalesItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInformationRequestLineArrayOfSalesItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SalesItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SalesItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SalesItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SalesItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInformationRequestLineArrayOfSupplyChainActivityTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode>,
}

impl AsMut<ItemInformationRequestLineArrayOfSupplyChainActivityTypeCodeComponent> for ItemInformationRequestLineArrayOfSupplyChainActivityTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInformationRequestLineArrayOfSupplyChainActivityTypeCodeComponent> for ItemInformationRequestLineArrayOfSupplyChainActivityTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemInformationRequestLineArrayOfSupplyChainActivityTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemInformationRequestLineArrayOfSupplyChainActivityTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInformationRequestLineArrayOfSupplyChainActivityTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemInformationRequestLineArrayOfTimeFrequencyCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TimeFrequencyCode>,
}

impl AsMut<ItemInformationRequestLineArrayOfTimeFrequencyCodeComponent> for ItemInformationRequestLineArrayOfTimeFrequencyCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemInformationRequestLineArrayOfTimeFrequencyCodeComponent> for ItemInformationRequestLineArrayOfTimeFrequencyCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemInformationRequestLineArrayOfTimeFrequencyCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemInformationRequestLineArrayOfTimeFrequencyCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemInformationRequestLineArrayOfTimeFrequencyCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TimeFrequencyCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TimeFrequencyCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TimeFrequencyCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TimeFrequencyCode> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForecastExceptionCriterionLine {
    #[serde(rename = "ComparisonDataSourceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_data_source_code: Option<ForecastExceptionCriterionLineArrayOfComparisonDataSourceCodeComponent>,
    #[serde(rename = "DataSourceCode")]
    pub data_source_code: ForecastExceptionCriterionLineArrayOfDataSourceCodeComponent,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: ForecastExceptionCriterionLineArrayOfForecastPurposeCodeComponent,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: ForecastExceptionCriterionLineArrayOfForecastTypeCodeComponent,
    #[serde(rename = "TimeDeltaDaysQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delta_days_quantity: Option<ForecastExceptionCriterionLineArrayOfTimeDeltaDaysQuantityComponent>,
}

impl AsMut<ForecastExceptionCriterionLine> for ForecastExceptionCriterionLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionCriterionLine> for ForecastExceptionCriterionLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.forecast_purpose_code.validate() {
            return Err(UblError::component("ForecastExceptionCriterionLine.forecast_purpose_code", e));
        }
        if let Some(v) = &self.time_delta_days_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastExceptionCriterionLine.time_delta_days_quantity", e));
            }
        }
        if let Some(v) = &self.comparison_data_source_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastExceptionCriterionLine.comparison_data_source_code", e));
            }
        }
        if let Err(e) = self.forecast_type_code.validate() {
            return Err(UblError::component("ForecastExceptionCriterionLine.forecast_type_code", e));
        }
        if let Err(e) = self.data_source_code.validate() {
            return Err(UblError::component("ForecastExceptionCriterionLine.data_source_code", e));
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

impl ForecastExceptionCriterionLine {
    pub fn title() -> &'static str {
        "Forecast Exception Criterion Line. Details"
    }
    pub fn description() -> &'static str {
        "Establishes the criterion for one of the three types of exceptions. This class provides criteria for the kind of forecast exception, the identification of the purpose of the forecast, the source of data and the time basis criterion for the exception."
    }
    pub fn new(data_source_code: ForecastExceptionCriterionLineArrayOfDataSourceCodeComponent, forecast_purpose_code: ForecastExceptionCriterionLineArrayOfForecastPurposeCodeComponent, forecast_type_code: ForecastExceptionCriterionLineArrayOfForecastTypeCodeComponent) -> Component<Self> {
        Component(Self {
            data_source_code,
            forecast_type_code,
            forecast_purpose_code,
            comparison_data_source_code: None,
            time_delta_days_quantity: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastExceptionCriterionLineArrayOfComparisonDataSourceCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ComparisonDataSourceCode>,
}

impl AsMut<ForecastExceptionCriterionLineArrayOfComparisonDataSourceCodeComponent> for ForecastExceptionCriterionLineArrayOfComparisonDataSourceCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionCriterionLineArrayOfComparisonDataSourceCodeComponent> for ForecastExceptionCriterionLineArrayOfComparisonDataSourceCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionCriterionLineArrayOfComparisonDataSourceCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionCriterionLineArrayOfComparisonDataSourceCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionCriterionLineArrayOfComparisonDataSourceCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ComparisonDataSourceCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ComparisonDataSourceCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ComparisonDataSourceCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ComparisonDataSourceCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastExceptionCriterionLineArrayOfDataSourceCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DataSourceCode>,
}

impl AsMut<ForecastExceptionCriterionLineArrayOfDataSourceCodeComponent> for ForecastExceptionCriterionLineArrayOfDataSourceCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionCriterionLineArrayOfDataSourceCodeComponent> for ForecastExceptionCriterionLineArrayOfDataSourceCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionCriterionLineArrayOfDataSourceCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionCriterionLineArrayOfDataSourceCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionCriterionLineArrayOfDataSourceCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DataSourceCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DataSourceCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DataSourceCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DataSourceCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastExceptionCriterionLineArrayOfForecastPurposeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ForecastPurposeCode>,
}

impl AsMut<ForecastExceptionCriterionLineArrayOfForecastPurposeCodeComponent> for ForecastExceptionCriterionLineArrayOfForecastPurposeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionCriterionLineArrayOfForecastPurposeCodeComponent> for ForecastExceptionCriterionLineArrayOfForecastPurposeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionCriterionLineArrayOfForecastPurposeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionCriterionLineArrayOfForecastPurposeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionCriterionLineArrayOfForecastPurposeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ForecastPurposeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ForecastPurposeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ForecastPurposeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ForecastPurposeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastExceptionCriterionLineArrayOfForecastTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode>,
}

impl AsMut<ForecastExceptionCriterionLineArrayOfForecastTypeCodeComponent> for ForecastExceptionCriterionLineArrayOfForecastTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionCriterionLineArrayOfForecastTypeCodeComponent> for ForecastExceptionCriterionLineArrayOfForecastTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionCriterionLineArrayOfForecastTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionCriterionLineArrayOfForecastTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionCriterionLineArrayOfForecastTypeCodeComponent {
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
pub struct ForecastExceptionCriterionLineArrayOfTimeDeltaDaysQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TimeDeltaDaysQuantity>,
}

impl AsMut<ForecastExceptionCriterionLineArrayOfTimeDeltaDaysQuantityComponent> for ForecastExceptionCriterionLineArrayOfTimeDeltaDaysQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionCriterionLineArrayOfTimeDeltaDaysQuantityComponent> for ForecastExceptionCriterionLineArrayOfTimeDeltaDaysQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionCriterionLineArrayOfTimeDeltaDaysQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionCriterionLineArrayOfTimeDeltaDaysQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionCriterionLineArrayOfTimeDeltaDaysQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TimeDeltaDaysQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TimeDeltaDaysQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TimeDeltaDaysQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TimeDeltaDaysQuantity> {
        self.items.iter()
    }
}


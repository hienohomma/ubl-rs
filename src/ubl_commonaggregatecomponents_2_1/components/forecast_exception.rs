use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForecastException {
    #[serde(rename = "ComparisonDataCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_data_code: Option<ForecastExceptionArrayOfComparisonDataCodeComponent>,
    #[serde(rename = "ComparisonForecastIssueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_forecast_issue_date: Option<ForecastExceptionArrayOfComparisonForecastIssueDateComponent>,
    #[serde(rename = "ComparisonForecastIssueTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_forecast_issue_time: Option<ForecastExceptionArrayOfComparisonForecastIssueTimeComponent>,
    #[serde(rename = "DataSourceCode")]
    pub data_source_code: ForecastExceptionArrayOfDataSourceCodeComponent,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: ForecastExceptionArrayOfForecastPurposeCodeComponent,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: ForecastExceptionArrayOfForecastTypeCodeComponent,
    #[serde(rename = "IssueDate")]
    pub issue_date: ForecastExceptionArrayOfIssueDateComponent,
    #[serde(rename = "IssueTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ForecastExceptionArrayOfIssueTimeComponent>,
}

impl AsMut<ForecastException> for ForecastException {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastException> for ForecastException {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.forecast_purpose_code.validate() {
            return Err(UblError::component("ForecastException.forecast_purpose_code", e));
        }
        if let Err(e) = self.data_source_code.validate() {
            return Err(UblError::component("ForecastException.data_source_code", e));
        }
        if let Err(e) = self.issue_date.validate() {
            return Err(UblError::component("ForecastException.issue_date", e));
        }
        if let Err(e) = self.forecast_type_code.validate() {
            return Err(UblError::component("ForecastException.forecast_type_code", e));
        }
        if let Some(v) = &self.comparison_data_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastException.comparison_data_code", e));
            }
        }
        if let Some(v) = &self.comparison_forecast_issue_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastException.comparison_forecast_issue_date", e));
            }
        }
        if let Some(v) = &self.comparison_forecast_issue_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastException.comparison_forecast_issue_time", e));
            }
        }
        if let Some(v) = &self.issue_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastException.issue_time", e));
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

impl ForecastException {
    pub fn title() -> &'static str {
        "Forecast Exception. Details"
    }
    pub fn description() -> &'static str {
        "As explained in Exception Criteria Line: Three types of exception criteria can be defined, Operational, Metric or Forecast Exceptions. This class provides criteria for forecast exception type: the identification of the purpose of the forecast, the source of data and the time basis criteria for the exception."
    }
    pub fn new(data_source_code: ForecastExceptionArrayOfDataSourceCodeComponent, forecast_purpose_code: ForecastExceptionArrayOfForecastPurposeCodeComponent, forecast_type_code: ForecastExceptionArrayOfForecastTypeCodeComponent, issue_date: ForecastExceptionArrayOfIssueDateComponent) -> Component<Self> {
        Component(Self {
            issue_time: None,
            forecast_purpose_code,
            comparison_forecast_issue_time: None,
            data_source_code,
            comparison_data_code: None,
            comparison_forecast_issue_date: None,
            issue_date,
            forecast_type_code,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastExceptionArrayOfComparisonDataCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ComparisonDataCode>,
}

impl AsMut<ForecastExceptionArrayOfComparisonDataCodeComponent> for ForecastExceptionArrayOfComparisonDataCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionArrayOfComparisonDataCodeComponent> for ForecastExceptionArrayOfComparisonDataCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfComparisonDataCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfComparisonDataCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionArrayOfComparisonDataCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ComparisonDataCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ComparisonDataCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ComparisonDataCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ComparisonDataCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastExceptionArrayOfComparisonForecastIssueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ComparisonForecastIssueDate>,
}

impl AsMut<ForecastExceptionArrayOfComparisonForecastIssueDateComponent> for ForecastExceptionArrayOfComparisonForecastIssueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionArrayOfComparisonForecastIssueDateComponent> for ForecastExceptionArrayOfComparisonForecastIssueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfComparisonForecastIssueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfComparisonForecastIssueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionArrayOfComparisonForecastIssueDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ComparisonForecastIssueDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ComparisonForecastIssueDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ComparisonForecastIssueDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ComparisonForecastIssueDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastExceptionArrayOfComparisonForecastIssueTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ComparisonForecastIssueTime>,
}

impl AsMut<ForecastExceptionArrayOfComparisonForecastIssueTimeComponent> for ForecastExceptionArrayOfComparisonForecastIssueTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionArrayOfComparisonForecastIssueTimeComponent> for ForecastExceptionArrayOfComparisonForecastIssueTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfComparisonForecastIssueTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfComparisonForecastIssueTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionArrayOfComparisonForecastIssueTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ComparisonForecastIssueTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ComparisonForecastIssueTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ComparisonForecastIssueTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ComparisonForecastIssueTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastExceptionArrayOfDataSourceCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DataSourceCode>,
}

impl AsMut<ForecastExceptionArrayOfDataSourceCodeComponent> for ForecastExceptionArrayOfDataSourceCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionArrayOfDataSourceCodeComponent> for ForecastExceptionArrayOfDataSourceCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfDataSourceCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfDataSourceCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionArrayOfDataSourceCodeComponent {
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
pub struct ForecastExceptionArrayOfForecastPurposeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ForecastPurposeCode>,
}

impl AsMut<ForecastExceptionArrayOfForecastPurposeCodeComponent> for ForecastExceptionArrayOfForecastPurposeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionArrayOfForecastPurposeCodeComponent> for ForecastExceptionArrayOfForecastPurposeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfForecastPurposeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfForecastPurposeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionArrayOfForecastPurposeCodeComponent {
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
pub struct ForecastExceptionArrayOfForecastTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode>,
}

impl AsMut<ForecastExceptionArrayOfForecastTypeCodeComponent> for ForecastExceptionArrayOfForecastTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionArrayOfForecastTypeCodeComponent> for ForecastExceptionArrayOfForecastTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfForecastTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfForecastTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionArrayOfForecastTypeCodeComponent {
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
pub struct ForecastExceptionArrayOfIssueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueDate>,
}

impl AsMut<ForecastExceptionArrayOfIssueDateComponent> for ForecastExceptionArrayOfIssueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionArrayOfIssueDateComponent> for ForecastExceptionArrayOfIssueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfIssueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfIssueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionArrayOfIssueDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IssueDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IssueDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IssueDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IssueDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastExceptionArrayOfIssueTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueTime>,
}

impl AsMut<ForecastExceptionArrayOfIssueTimeComponent> for ForecastExceptionArrayOfIssueTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastExceptionArrayOfIssueTimeComponent> for ForecastExceptionArrayOfIssueTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfIssueTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastExceptionArrayOfIssueTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastExceptionArrayOfIssueTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IssueTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IssueTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IssueTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IssueTime> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForecastRevisionLine {
    #[serde(rename = "AdjustmentReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_reason_code: Option<ForecastRevisionLineArrayOfAdjustmentReasonCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ForecastRevisionLineArrayOfDescriptionComponent>,
    #[serde(rename = "ForecastPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_period: Option<ForecastRevisionLineArrayOfForecastPeriodComponent>,
    #[serde(rename = "ID")]
    pub id: ForecastRevisionLineArrayOfIDComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<ForecastRevisionLineArrayOfNoteComponent>,
    #[serde(rename = "RevisedForecastLineID")]
    pub revised_forecast_line_id: ForecastRevisionLineArrayOfRevisedForecastLineIDComponent,
    #[serde(rename = "SalesItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_item: Option<ForecastRevisionLineArrayOfSalesItemComponent>,
    #[serde(rename = "SourceForecastIssueDate")]
    pub source_forecast_issue_date: ForecastRevisionLineArrayOfSourceForecastIssueDateComponent,
    #[serde(rename = "SourceForecastIssueTime")]
    pub source_forecast_issue_time: ForecastRevisionLineArrayOfSourceForecastIssueTimeComponent,
}

impl AsMut<ForecastRevisionLine> for ForecastRevisionLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastRevisionLine> for ForecastRevisionLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.sales_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastRevisionLine.sales_item", e));
            }
        }
        if let Some(v) = &self.adjustment_reason_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastRevisionLine.adjustment_reason_code", e));
            }
        }
        if let Err(e) = self.source_forecast_issue_date.validate() {
            return Err(UblError::component("ForecastRevisionLine.source_forecast_issue_date", e));
        }
        if let Err(e) = self.revised_forecast_line_id.validate() {
            return Err(UblError::component("ForecastRevisionLine.revised_forecast_line_id", e));
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastRevisionLine.description", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastRevisionLine.note", e));
            }
        }
        if let Err(e) = self.source_forecast_issue_time.validate() {
            return Err(UblError::component("ForecastRevisionLine.source_forecast_issue_time", e));
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ForecastRevisionLine.id", e));
        }
        if let Some(v) = &self.forecast_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastRevisionLine.forecast_period", e));
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

impl ForecastRevisionLine {
    pub fn title() -> &'static str {
        "Forecast Revision Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Forecast Revision describing a revision to a line in a Forecast."
    }
    pub fn new(id: ForecastRevisionLineArrayOfIDComponent, revised_forecast_line_id: ForecastRevisionLineArrayOfRevisedForecastLineIDComponent, source_forecast_issue_date: ForecastRevisionLineArrayOfSourceForecastIssueDateComponent, source_forecast_issue_time: ForecastRevisionLineArrayOfSourceForecastIssueTimeComponent) -> Component<Self> {
        Component(Self {
            description: None,
            note: None,
            adjustment_reason_code: None,
            source_forecast_issue_time,
            forecast_period: None,
            sales_item: None,
            source_forecast_issue_date,
            revised_forecast_line_id,
            id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastRevisionLineArrayOfAdjustmentReasonCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AdjustmentReasonCode>,
}

impl AsMut<ForecastRevisionLineArrayOfAdjustmentReasonCodeComponent> for ForecastRevisionLineArrayOfAdjustmentReasonCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastRevisionLineArrayOfAdjustmentReasonCodeComponent> for ForecastRevisionLineArrayOfAdjustmentReasonCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfAdjustmentReasonCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfAdjustmentReasonCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastRevisionLineArrayOfAdjustmentReasonCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AdjustmentReasonCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AdjustmentReasonCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AdjustmentReasonCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AdjustmentReasonCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastRevisionLineArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ForecastRevisionLineArrayOfDescriptionComponent> for ForecastRevisionLineArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastRevisionLineArrayOfDescriptionComponent> for ForecastRevisionLineArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastRevisionLineArrayOfDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Description) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Description) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastRevisionLineArrayOfForecastPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ForecastPeriod>,
}

impl AsMut<ForecastRevisionLineArrayOfForecastPeriodComponent> for ForecastRevisionLineArrayOfForecastPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastRevisionLineArrayOfForecastPeriodComponent> for ForecastRevisionLineArrayOfForecastPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfForecastPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfForecastPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastRevisionLineArrayOfForecastPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ForecastPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ForecastPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ForecastPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ForecastPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastRevisionLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ForecastRevisionLineArrayOfIDComponent> for ForecastRevisionLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastRevisionLineArrayOfIDComponent> for ForecastRevisionLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastRevisionLineArrayOfIDComponent {
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
pub struct ForecastRevisionLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<ForecastRevisionLineArrayOfNoteComponent> for ForecastRevisionLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastRevisionLineArrayOfNoteComponent> for ForecastRevisionLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastRevisionLineArrayOfNoteComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Note) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Note) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastRevisionLineArrayOfRevisedForecastLineIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RevisedForecastLineID>,
}

impl AsMut<ForecastRevisionLineArrayOfRevisedForecastLineIDComponent> for ForecastRevisionLineArrayOfRevisedForecastLineIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastRevisionLineArrayOfRevisedForecastLineIDComponent> for ForecastRevisionLineArrayOfRevisedForecastLineIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfRevisedForecastLineIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfRevisedForecastLineIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastRevisionLineArrayOfRevisedForecastLineIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RevisedForecastLineID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RevisedForecastLineID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RevisedForecastLineID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RevisedForecastLineID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastRevisionLineArrayOfSalesItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SalesItem>,
}

impl AsMut<ForecastRevisionLineArrayOfSalesItemComponent> for ForecastRevisionLineArrayOfSalesItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastRevisionLineArrayOfSalesItemComponent> for ForecastRevisionLineArrayOfSalesItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfSalesItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfSalesItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastRevisionLineArrayOfSalesItemComponent {
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
pub struct ForecastRevisionLineArrayOfSourceForecastIssueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SourceForecastIssueDate>,
}

impl AsMut<ForecastRevisionLineArrayOfSourceForecastIssueDateComponent> for ForecastRevisionLineArrayOfSourceForecastIssueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastRevisionLineArrayOfSourceForecastIssueDateComponent> for ForecastRevisionLineArrayOfSourceForecastIssueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfSourceForecastIssueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfSourceForecastIssueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastRevisionLineArrayOfSourceForecastIssueDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SourceForecastIssueDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SourceForecastIssueDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SourceForecastIssueDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SourceForecastIssueDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastRevisionLineArrayOfSourceForecastIssueTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SourceForecastIssueTime>,
}

impl AsMut<ForecastRevisionLineArrayOfSourceForecastIssueTimeComponent> for ForecastRevisionLineArrayOfSourceForecastIssueTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastRevisionLineArrayOfSourceForecastIssueTimeComponent> for ForecastRevisionLineArrayOfSourceForecastIssueTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfSourceForecastIssueTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastRevisionLineArrayOfSourceForecastIssueTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastRevisionLineArrayOfSourceForecastIssueTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SourceForecastIssueTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SourceForecastIssueTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SourceForecastIssueTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SourceForecastIssueTime> {
        self.items.iter()
    }
}


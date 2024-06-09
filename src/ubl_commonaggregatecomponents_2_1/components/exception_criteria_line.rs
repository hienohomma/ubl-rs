use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExceptionCriteriaLine {
    #[serde(rename = "CollaborationPriorityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaboration_priority_code: Option<ExceptionCriteriaLineArrayOfCollaborationPriorityCodeComponent>,
    #[serde(rename = "EffectivePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_period: Option<ExceptionCriteriaLineArrayOfEffectivePeriodComponent>,
    #[serde(rename = "ExceptionResolutionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_resolution_code: Option<ExceptionCriteriaLineArrayOfExceptionResolutionCodeComponent>,
    #[serde(rename = "ExceptionStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_status_code: Option<ExceptionCriteriaLineArrayOfExceptionStatusCodeComponent>,
    #[serde(rename = "ForecastExceptionCriterionLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_exception_criterion_line: Option<ExceptionCriteriaLineArrayOfForecastExceptionCriterionLineComponent>,
    #[serde(rename = "ID")]
    pub id: ExceptionCriteriaLineArrayOfIDComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<ExceptionCriteriaLineArrayOfNoteComponent>,
    #[serde(rename = "PerformanceMetricTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_metric_type_code: Option<ExceptionCriteriaLineArrayOfPerformanceMetricTypeCodeComponent>,
    #[serde(rename = "SupplyChainActivityTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_chain_activity_type_code: Option<ExceptionCriteriaLineArrayOfSupplyChainActivityTypeCodeComponent>,
    #[serde(rename = "SupplyItem")]
    pub supply_item: ExceptionCriteriaLineArrayOfSupplyItemComponent,
    #[serde(rename = "ThresholdQuantity")]
    pub threshold_quantity: ExceptionCriteriaLineArrayOfThresholdQuantityComponent,
    #[serde(rename = "ThresholdValueComparisonCode")]
    pub threshold_value_comparison_code: ExceptionCriteriaLineArrayOfThresholdValueComparisonCodeComponent,
}

impl AsMut<ExceptionCriteriaLine> for ExceptionCriteriaLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLine> for ExceptionCriteriaLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.supply_chain_activity_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionCriteriaLine.supply_chain_activity_type_code", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionCriteriaLine.note", e));
            }
        }
        if let Err(e) = self.supply_item.validate() {
            return Err(UblError::component("ExceptionCriteriaLine.supply_item", e));
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ExceptionCriteriaLine.id", e));
        }
        if let Err(e) = self.threshold_value_comparison_code.validate() {
            return Err(UblError::component("ExceptionCriteriaLine.threshold_value_comparison_code", e));
        }
        if let Some(v) = &self.forecast_exception_criterion_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionCriteriaLine.forecast_exception_criterion_line", e));
            }
        }
        if let Some(v) = &self.performance_metric_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionCriteriaLine.performance_metric_type_code", e));
            }
        }
        if let Err(e) = self.threshold_quantity.validate() {
            return Err(UblError::component("ExceptionCriteriaLine.threshold_quantity", e));
        }
        if let Some(v) = &self.effective_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionCriteriaLine.effective_period", e));
            }
        }
        if let Some(v) = &self.exception_resolution_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionCriteriaLine.exception_resolution_code", e));
            }
        }
        if let Some(v) = &self.collaboration_priority_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionCriteriaLine.collaboration_priority_code", e));
            }
        }
        if let Some(v) = &self.exception_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionCriteriaLine.exception_status_code", e));
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

impl ExceptionCriteriaLine {
    pub fn title() -> &'static str {
        "Exception Criteria Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in an ExceptionCriteria document that specifies a threshold for forecast variance, product activity, or performance history, the exceeding of which should trigger an exception message."
    }
    pub fn new(id: ExceptionCriteriaLineArrayOfIDComponent, supply_item: ExceptionCriteriaLineArrayOfSupplyItemComponent, threshold_quantity: ExceptionCriteriaLineArrayOfThresholdQuantityComponent, threshold_value_comparison_code: ExceptionCriteriaLineArrayOfThresholdValueComparisonCodeComponent) -> Component<Self> {
        Component(Self {
            id,
            performance_metric_type_code: None,
            collaboration_priority_code: None,
            supply_chain_activity_type_code: None,
            supply_item,
            effective_period: None,
            note: None,
            threshold_quantity,
            exception_status_code: None,
            threshold_value_comparison_code,
            exception_resolution_code: None,
            forecast_exception_criterion_line: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionCriteriaLineArrayOfCollaborationPriorityCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CollaborationPriorityCode>,
}

impl AsMut<ExceptionCriteriaLineArrayOfCollaborationPriorityCodeComponent> for ExceptionCriteriaLineArrayOfCollaborationPriorityCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfCollaborationPriorityCodeComponent> for ExceptionCriteriaLineArrayOfCollaborationPriorityCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfCollaborationPriorityCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfCollaborationPriorityCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfCollaborationPriorityCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CollaborationPriorityCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CollaborationPriorityCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CollaborationPriorityCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CollaborationPriorityCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionCriteriaLineArrayOfEffectivePeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EffectivePeriod>,
}

impl AsMut<ExceptionCriteriaLineArrayOfEffectivePeriodComponent> for ExceptionCriteriaLineArrayOfEffectivePeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfEffectivePeriodComponent> for ExceptionCriteriaLineArrayOfEffectivePeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfEffectivePeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfEffectivePeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfEffectivePeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EffectivePeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EffectivePeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EffectivePeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EffectivePeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionCriteriaLineArrayOfExceptionResolutionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExceptionResolutionCode>,
}

impl AsMut<ExceptionCriteriaLineArrayOfExceptionResolutionCodeComponent> for ExceptionCriteriaLineArrayOfExceptionResolutionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfExceptionResolutionCodeComponent> for ExceptionCriteriaLineArrayOfExceptionResolutionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfExceptionResolutionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfExceptionResolutionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfExceptionResolutionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExceptionResolutionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExceptionResolutionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExceptionResolutionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExceptionResolutionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionCriteriaLineArrayOfExceptionStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExceptionStatusCode>,
}

impl AsMut<ExceptionCriteriaLineArrayOfExceptionStatusCodeComponent> for ExceptionCriteriaLineArrayOfExceptionStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfExceptionStatusCodeComponent> for ExceptionCriteriaLineArrayOfExceptionStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfExceptionStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfExceptionStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfExceptionStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExceptionStatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExceptionStatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExceptionStatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExceptionStatusCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionCriteriaLineArrayOfForecastExceptionCriterionLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ForecastExceptionCriterionLine>,
}

impl AsMut<ExceptionCriteriaLineArrayOfForecastExceptionCriterionLineComponent> for ExceptionCriteriaLineArrayOfForecastExceptionCriterionLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfForecastExceptionCriterionLineComponent> for ExceptionCriteriaLineArrayOfForecastExceptionCriterionLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfForecastExceptionCriterionLineComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfForecastExceptionCriterionLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfForecastExceptionCriterionLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ForecastExceptionCriterionLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ForecastExceptionCriterionLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ForecastExceptionCriterionLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ForecastExceptionCriterionLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionCriteriaLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ExceptionCriteriaLineArrayOfIDComponent> for ExceptionCriteriaLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfIDComponent> for ExceptionCriteriaLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfIDComponent {
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
pub struct ExceptionCriteriaLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<ExceptionCriteriaLineArrayOfNoteComponent> for ExceptionCriteriaLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfNoteComponent> for ExceptionCriteriaLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfNoteComponent {
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
pub struct ExceptionCriteriaLineArrayOfPerformanceMetricTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode>,
}

impl AsMut<ExceptionCriteriaLineArrayOfPerformanceMetricTypeCodeComponent> for ExceptionCriteriaLineArrayOfPerformanceMetricTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfPerformanceMetricTypeCodeComponent> for ExceptionCriteriaLineArrayOfPerformanceMetricTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfPerformanceMetricTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfPerformanceMetricTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfPerformanceMetricTypeCodeComponent {
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
pub struct ExceptionCriteriaLineArrayOfSupplyChainActivityTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode>,
}

impl AsMut<ExceptionCriteriaLineArrayOfSupplyChainActivityTypeCodeComponent> for ExceptionCriteriaLineArrayOfSupplyChainActivityTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfSupplyChainActivityTypeCodeComponent> for ExceptionCriteriaLineArrayOfSupplyChainActivityTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfSupplyChainActivityTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfSupplyChainActivityTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfSupplyChainActivityTypeCodeComponent {
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
pub struct ExceptionCriteriaLineArrayOfSupplyItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SupplyItem>,
}

impl AsMut<ExceptionCriteriaLineArrayOfSupplyItemComponent> for ExceptionCriteriaLineArrayOfSupplyItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfSupplyItemComponent> for ExceptionCriteriaLineArrayOfSupplyItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfSupplyItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfSupplyItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SupplyItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SupplyItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SupplyItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SupplyItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionCriteriaLineArrayOfThresholdQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ThresholdQuantity>,
}

impl AsMut<ExceptionCriteriaLineArrayOfThresholdQuantityComponent> for ExceptionCriteriaLineArrayOfThresholdQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfThresholdQuantityComponent> for ExceptionCriteriaLineArrayOfThresholdQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfThresholdQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfThresholdQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfThresholdQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ThresholdQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ThresholdQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ThresholdQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ThresholdQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionCriteriaLineArrayOfThresholdValueComparisonCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ThresholdValueComparisonCode>,
}

impl AsMut<ExceptionCriteriaLineArrayOfThresholdValueComparisonCodeComponent> for ExceptionCriteriaLineArrayOfThresholdValueComparisonCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionCriteriaLineArrayOfThresholdValueComparisonCodeComponent> for ExceptionCriteriaLineArrayOfThresholdValueComparisonCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfThresholdValueComparisonCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionCriteriaLineArrayOfThresholdValueComparisonCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionCriteriaLineArrayOfThresholdValueComparisonCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ThresholdValueComparisonCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ThresholdValueComparisonCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ThresholdValueComparisonCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ThresholdValueComparisonCode> {
        self.items.iter()
    }
}


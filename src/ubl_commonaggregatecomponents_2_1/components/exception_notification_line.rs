use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExceptionNotificationLine {
    #[serde(rename = "CollaborationPriorityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaboration_priority_code: Option<ExceptionNotificationLineArrayOfCollaborationPriorityCodeComponent>,
    #[serde(rename = "ComparedValueMeasure")]
    pub compared_value_measure: ExceptionNotificationLineArrayOfComparedValueMeasureComponent,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ExceptionNotificationLineArrayOfDescriptionComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ExceptionNotificationLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ExceptionObservationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_observation_period: Option<ExceptionNotificationLineArrayOfExceptionObservationPeriodComponent>,
    #[serde(rename = "ExceptionStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_status_code: Option<ExceptionNotificationLineArrayOfExceptionStatusCodeComponent>,
    #[serde(rename = "ForecastException")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_exception: Option<ExceptionNotificationLineArrayOfForecastExceptionComponent>,
    #[serde(rename = "ID")]
    pub id: ExceptionNotificationLineArrayOfIDComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<ExceptionNotificationLineArrayOfNoteComponent>,
    #[serde(rename = "PerformanceMetricTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_metric_type_code: Option<ExceptionNotificationLineArrayOfPerformanceMetricTypeCodeComponent>,
    #[serde(rename = "ResolutionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_code: Option<ExceptionNotificationLineArrayOfResolutionCodeComponent>,
    #[serde(rename = "SourceValueMeasure")]
    pub source_value_measure: ExceptionNotificationLineArrayOfSourceValueMeasureComponent,
    #[serde(rename = "SupplyChainActivityTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_chain_activity_type_code: Option<ExceptionNotificationLineArrayOfSupplyChainActivityTypeCodeComponent>,
    #[serde(rename = "SupplyItem")]
    pub supply_item: ExceptionNotificationLineArrayOfSupplyItemComponent,
    #[serde(rename = "VarianceQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variance_quantity: Option<ExceptionNotificationLineArrayOfVarianceQuantityComponent>,
}

impl AsMut<ExceptionNotificationLine> for ExceptionNotificationLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLine> for ExceptionNotificationLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.compared_value_measure.validate() {
            return Err(UblError::component("ExceptionNotificationLine.compared_value_measure", e));
        }
        if let Some(v) = &self.forecast_exception {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionNotificationLine.forecast_exception", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionNotificationLine.note", e));
            }
        }
        if let Some(v) = &self.exception_observation_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionNotificationLine.exception_observation_period", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionNotificationLine.description", e));
            }
        }
        if let Some(v) = &self.performance_metric_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionNotificationLine.performance_metric_type_code", e));
            }
        }
        if let Some(v) = &self.exception_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionNotificationLine.exception_status_code", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ExceptionNotificationLine.id", e));
        }
        if let Some(v) = &self.supply_chain_activity_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionNotificationLine.supply_chain_activity_type_code", e));
            }
        }
        if let Some(v) = &self.variance_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionNotificationLine.variance_quantity", e));
            }
        }
        if let Err(e) = self.source_value_measure.validate() {
            return Err(UblError::component("ExceptionNotificationLine.source_value_measure", e));
        }
        if let Err(e) = self.supply_item.validate() {
            return Err(UblError::component("ExceptionNotificationLine.supply_item", e));
        }
        if let Some(v) = &self.resolution_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionNotificationLine.resolution_code", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionNotificationLine.document_reference", e));
            }
        }
        if let Some(v) = &self.collaboration_priority_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ExceptionNotificationLine.collaboration_priority_code", e));
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

impl ExceptionNotificationLine {
    pub fn title() -> &'static str {
        "Exception Notification Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in an Exception Notification."
    }
    pub fn new(compared_value_measure: ExceptionNotificationLineArrayOfComparedValueMeasureComponent, id: ExceptionNotificationLineArrayOfIDComponent, source_value_measure: ExceptionNotificationLineArrayOfSourceValueMeasureComponent, supply_item: ExceptionNotificationLineArrayOfSupplyItemComponent) -> Component<Self> {
        Component(Self {
            exception_observation_period: None,
            note: None,
            exception_status_code: None,
            resolution_code: None,
            supply_item,
            source_value_measure,
            document_reference: None,
            id,
            compared_value_measure,
            collaboration_priority_code: None,
            variance_quantity: None,
            supply_chain_activity_type_code: None,
            description: None,
            forecast_exception: None,
            performance_metric_type_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionNotificationLineArrayOfCollaborationPriorityCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CollaborationPriorityCode>,
}

impl AsMut<ExceptionNotificationLineArrayOfCollaborationPriorityCodeComponent> for ExceptionNotificationLineArrayOfCollaborationPriorityCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfCollaborationPriorityCodeComponent> for ExceptionNotificationLineArrayOfCollaborationPriorityCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfCollaborationPriorityCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfCollaborationPriorityCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfCollaborationPriorityCodeComponent {
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
pub struct ExceptionNotificationLineArrayOfComparedValueMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ComparedValueMeasure>,
}

impl AsMut<ExceptionNotificationLineArrayOfComparedValueMeasureComponent> for ExceptionNotificationLineArrayOfComparedValueMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfComparedValueMeasureComponent> for ExceptionNotificationLineArrayOfComparedValueMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfComparedValueMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfComparedValueMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfComparedValueMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ComparedValueMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ComparedValueMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ComparedValueMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ComparedValueMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionNotificationLineArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ExceptionNotificationLineArrayOfDescriptionComponent> for ExceptionNotificationLineArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfDescriptionComponent> for ExceptionNotificationLineArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfDescriptionComponent {
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
pub struct ExceptionNotificationLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<ExceptionNotificationLineArrayOfDocumentReferenceComponent> for ExceptionNotificationLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfDocumentReferenceComponent> for ExceptionNotificationLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionNotificationLineArrayOfExceptionObservationPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExceptionObservationPeriod>,
}

impl AsMut<ExceptionNotificationLineArrayOfExceptionObservationPeriodComponent> for ExceptionNotificationLineArrayOfExceptionObservationPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfExceptionObservationPeriodComponent> for ExceptionNotificationLineArrayOfExceptionObservationPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfExceptionObservationPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfExceptionObservationPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfExceptionObservationPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ExceptionObservationPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ExceptionObservationPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ExceptionObservationPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ExceptionObservationPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionNotificationLineArrayOfExceptionStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExceptionStatusCode>,
}

impl AsMut<ExceptionNotificationLineArrayOfExceptionStatusCodeComponent> for ExceptionNotificationLineArrayOfExceptionStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfExceptionStatusCodeComponent> for ExceptionNotificationLineArrayOfExceptionStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfExceptionStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfExceptionStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfExceptionStatusCodeComponent {
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
pub struct ExceptionNotificationLineArrayOfForecastExceptionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ForecastException>,
}

impl AsMut<ExceptionNotificationLineArrayOfForecastExceptionComponent> for ExceptionNotificationLineArrayOfForecastExceptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfForecastExceptionComponent> for ExceptionNotificationLineArrayOfForecastExceptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfForecastExceptionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfForecastExceptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfForecastExceptionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ForecastException) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ForecastException) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ForecastException> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ForecastException> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionNotificationLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ExceptionNotificationLineArrayOfIDComponent> for ExceptionNotificationLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfIDComponent> for ExceptionNotificationLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfIDComponent {
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
pub struct ExceptionNotificationLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<ExceptionNotificationLineArrayOfNoteComponent> for ExceptionNotificationLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfNoteComponent> for ExceptionNotificationLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfNoteComponent {
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
pub struct ExceptionNotificationLineArrayOfPerformanceMetricTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode>,
}

impl AsMut<ExceptionNotificationLineArrayOfPerformanceMetricTypeCodeComponent> for ExceptionNotificationLineArrayOfPerformanceMetricTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfPerformanceMetricTypeCodeComponent> for ExceptionNotificationLineArrayOfPerformanceMetricTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfPerformanceMetricTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfPerformanceMetricTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfPerformanceMetricTypeCodeComponent {
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
pub struct ExceptionNotificationLineArrayOfResolutionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ResolutionCode>,
}

impl AsMut<ExceptionNotificationLineArrayOfResolutionCodeComponent> for ExceptionNotificationLineArrayOfResolutionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfResolutionCodeComponent> for ExceptionNotificationLineArrayOfResolutionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfResolutionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfResolutionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfResolutionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ResolutionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ResolutionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ResolutionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ResolutionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionNotificationLineArrayOfSourceValueMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SourceValueMeasure>,
}

impl AsMut<ExceptionNotificationLineArrayOfSourceValueMeasureComponent> for ExceptionNotificationLineArrayOfSourceValueMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfSourceValueMeasureComponent> for ExceptionNotificationLineArrayOfSourceValueMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfSourceValueMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfSourceValueMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfSourceValueMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SourceValueMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SourceValueMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SourceValueMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SourceValueMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExceptionNotificationLineArrayOfSupplyChainActivityTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode>,
}

impl AsMut<ExceptionNotificationLineArrayOfSupplyChainActivityTypeCodeComponent> for ExceptionNotificationLineArrayOfSupplyChainActivityTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfSupplyChainActivityTypeCodeComponent> for ExceptionNotificationLineArrayOfSupplyChainActivityTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfSupplyChainActivityTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfSupplyChainActivityTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfSupplyChainActivityTypeCodeComponent {
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
pub struct ExceptionNotificationLineArrayOfSupplyItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SupplyItem>,
}

impl AsMut<ExceptionNotificationLineArrayOfSupplyItemComponent> for ExceptionNotificationLineArrayOfSupplyItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfSupplyItemComponent> for ExceptionNotificationLineArrayOfSupplyItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfSupplyItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfSupplyItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfSupplyItemComponent {
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
pub struct ExceptionNotificationLineArrayOfVarianceQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::VarianceQuantity>,
}

impl AsMut<ExceptionNotificationLineArrayOfVarianceQuantityComponent> for ExceptionNotificationLineArrayOfVarianceQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ExceptionNotificationLineArrayOfVarianceQuantityComponent> for ExceptionNotificationLineArrayOfVarianceQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfVarianceQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ExceptionNotificationLineArrayOfVarianceQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ExceptionNotificationLineArrayOfVarianceQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::VarianceQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::VarianceQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::VarianceQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::VarianceQuantity> {
        self.items.iter()
    }
}


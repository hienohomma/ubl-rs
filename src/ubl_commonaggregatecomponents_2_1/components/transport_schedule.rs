use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransportSchedule {
    #[serde(rename = "ActualArrivalTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_arrival_transport_event: Option<TransportScheduleArrayOfActualArrivalTransportEventComponent>,
    #[serde(rename = "ActualDepartureTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_departure_transport_event: Option<TransportScheduleArrayOfActualDepartureTransportEventComponent>,
    #[serde(rename = "EstimatedArrivalTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_arrival_transport_event: Option<TransportScheduleArrayOfEstimatedArrivalTransportEventComponent>,
    #[serde(rename = "EstimatedDepartureTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_departure_transport_event: Option<TransportScheduleArrayOfEstimatedDepartureTransportEventComponent>,
    #[serde(rename = "PlannedArrivalTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_arrival_transport_event: Option<TransportScheduleArrayOfPlannedArrivalTransportEventComponent>,
    #[serde(rename = "PlannedDepartureTransportEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_departure_transport_event: Option<TransportScheduleArrayOfPlannedDepartureTransportEventComponent>,
    #[serde(rename = "ReferenceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_date: Option<TransportScheduleArrayOfReferenceDateComponent>,
    #[serde(rename = "ReferenceTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_time: Option<TransportScheduleArrayOfReferenceTimeComponent>,
    #[serde(rename = "ReliabilityPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reliability_percent: Option<TransportScheduleArrayOfReliabilityPercentComponent>,
    #[serde(rename = "Remarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<TransportScheduleArrayOfRemarksComponent>,
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: TransportScheduleArrayOfSequenceNumericComponent,
    #[serde(rename = "StatusLocation")]
    pub status_location: TransportScheduleArrayOfStatusLocationComponent,
}

impl AsMut<TransportSchedule> for TransportSchedule {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportSchedule> for TransportSchedule {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.remarks {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportSchedule.remarks", e));
            }
        }
        if let Err(e) = self.sequence_numeric.validate() {
            return Err(UblError::component("TransportSchedule.sequence_numeric", e));
        }
        if let Some(v) = &self.reference_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportSchedule.reference_time", e));
            }
        }
        if let Some(v) = &self.planned_departure_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportSchedule.planned_departure_transport_event", e));
            }
        }
        if let Err(e) = self.status_location.validate() {
            return Err(UblError::component("TransportSchedule.status_location", e));
        }
        if let Some(v) = &self.actual_departure_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportSchedule.actual_departure_transport_event", e));
            }
        }
        if let Some(v) = &self.estimated_arrival_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportSchedule.estimated_arrival_transport_event", e));
            }
        }
        if let Some(v) = &self.reference_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportSchedule.reference_date", e));
            }
        }
        if let Some(v) = &self.estimated_departure_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportSchedule.estimated_departure_transport_event", e));
            }
        }
        if let Some(v) = &self.actual_arrival_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportSchedule.actual_arrival_transport_event", e));
            }
        }
        if let Some(v) = &self.planned_arrival_transport_event {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportSchedule.planned_arrival_transport_event", e));
            }
        }
        if let Some(v) = &self.reliability_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportSchedule.reliability_percent", e));
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

impl TransportSchedule {
    pub fn title() -> &'static str {
        "Transport Schedule. Details"
    }
    pub fn description() -> &'static str {
        "Describes the location and schedule relating to a transport means."
    }
    pub fn new(sequence_numeric: TransportScheduleArrayOfSequenceNumericComponent, status_location: TransportScheduleArrayOfStatusLocationComponent) -> Component<Self> {
        Component(Self {
            reliability_percent: None,
            reference_time: None,
            sequence_numeric,
            planned_departure_transport_event: None,
            remarks: None,
            estimated_arrival_transport_event: None,
            status_location,
            planned_arrival_transport_event: None,
            reference_date: None,
            actual_arrival_transport_event: None,
            actual_departure_transport_event: None,
            estimated_departure_transport_event: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfActualArrivalTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ActualArrivalTransportEvent>,
}

impl AsMut<TransportScheduleArrayOfActualArrivalTransportEventComponent> for TransportScheduleArrayOfActualArrivalTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfActualArrivalTransportEventComponent> for TransportScheduleArrayOfActualArrivalTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfActualArrivalTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfActualArrivalTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfActualArrivalTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ActualArrivalTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ActualArrivalTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ActualArrivalTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ActualArrivalTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfActualDepartureTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ActualDepartureTransportEvent>,
}

impl AsMut<TransportScheduleArrayOfActualDepartureTransportEventComponent> for TransportScheduleArrayOfActualDepartureTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfActualDepartureTransportEventComponent> for TransportScheduleArrayOfActualDepartureTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfActualDepartureTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfActualDepartureTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfActualDepartureTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ActualDepartureTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ActualDepartureTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ActualDepartureTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ActualDepartureTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfEstimatedArrivalTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EstimatedArrivalTransportEvent>,
}

impl AsMut<TransportScheduleArrayOfEstimatedArrivalTransportEventComponent> for TransportScheduleArrayOfEstimatedArrivalTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfEstimatedArrivalTransportEventComponent> for TransportScheduleArrayOfEstimatedArrivalTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfEstimatedArrivalTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfEstimatedArrivalTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfEstimatedArrivalTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EstimatedArrivalTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EstimatedArrivalTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EstimatedArrivalTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EstimatedArrivalTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfEstimatedDepartureTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EstimatedDepartureTransportEvent>,
}

impl AsMut<TransportScheduleArrayOfEstimatedDepartureTransportEventComponent> for TransportScheduleArrayOfEstimatedDepartureTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfEstimatedDepartureTransportEventComponent> for TransportScheduleArrayOfEstimatedDepartureTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfEstimatedDepartureTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfEstimatedDepartureTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfEstimatedDepartureTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EstimatedDepartureTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EstimatedDepartureTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EstimatedDepartureTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EstimatedDepartureTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfPlannedArrivalTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PlannedArrivalTransportEvent>,
}

impl AsMut<TransportScheduleArrayOfPlannedArrivalTransportEventComponent> for TransportScheduleArrayOfPlannedArrivalTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfPlannedArrivalTransportEventComponent> for TransportScheduleArrayOfPlannedArrivalTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfPlannedArrivalTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfPlannedArrivalTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfPlannedArrivalTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PlannedArrivalTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PlannedArrivalTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PlannedArrivalTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PlannedArrivalTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfPlannedDepartureTransportEventComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PlannedDepartureTransportEvent>,
}

impl AsMut<TransportScheduleArrayOfPlannedDepartureTransportEventComponent> for TransportScheduleArrayOfPlannedDepartureTransportEventComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfPlannedDepartureTransportEventComponent> for TransportScheduleArrayOfPlannedDepartureTransportEventComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfPlannedDepartureTransportEventComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfPlannedDepartureTransportEventComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfPlannedDepartureTransportEventComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PlannedDepartureTransportEvent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PlannedDepartureTransportEvent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PlannedDepartureTransportEvent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PlannedDepartureTransportEvent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfReferenceDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReferenceDate>,
}

impl AsMut<TransportScheduleArrayOfReferenceDateComponent> for TransportScheduleArrayOfReferenceDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfReferenceDateComponent> for TransportScheduleArrayOfReferenceDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfReferenceDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfReferenceDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfReferenceDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReferenceDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReferenceDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReferenceDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReferenceDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfReferenceTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReferenceTime>,
}

impl AsMut<TransportScheduleArrayOfReferenceTimeComponent> for TransportScheduleArrayOfReferenceTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfReferenceTimeComponent> for TransportScheduleArrayOfReferenceTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfReferenceTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfReferenceTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfReferenceTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReferenceTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReferenceTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReferenceTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReferenceTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfReliabilityPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReliabilityPercent>,
}

impl AsMut<TransportScheduleArrayOfReliabilityPercentComponent> for TransportScheduleArrayOfReliabilityPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfReliabilityPercentComponent> for TransportScheduleArrayOfReliabilityPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfReliabilityPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfReliabilityPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfReliabilityPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReliabilityPercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReliabilityPercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReliabilityPercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReliabilityPercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfRemarksComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Remarks>,
}

impl AsMut<TransportScheduleArrayOfRemarksComponent> for TransportScheduleArrayOfRemarksComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfRemarksComponent> for TransportScheduleArrayOfRemarksComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfRemarksComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfRemarksComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Remarks) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Remarks) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Remarks> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Remarks> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfSequenceNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric>,
}

impl AsMut<TransportScheduleArrayOfSequenceNumericComponent> for TransportScheduleArrayOfSequenceNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfSequenceNumericComponent> for TransportScheduleArrayOfSequenceNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfSequenceNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfSequenceNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfSequenceNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SequenceNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SequenceNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SequenceNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportScheduleArrayOfStatusLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::StatusLocation>,
}

impl AsMut<TransportScheduleArrayOfStatusLocationComponent> for TransportScheduleArrayOfStatusLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportScheduleArrayOfStatusLocationComponent> for TransportScheduleArrayOfStatusLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfStatusLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportScheduleArrayOfStatusLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportScheduleArrayOfStatusLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::StatusLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::StatusLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::StatusLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::StatusLocation> {
        self.items.iter()
    }
}


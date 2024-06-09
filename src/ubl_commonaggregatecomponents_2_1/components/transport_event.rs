use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransportEvent {
    #[serde(rename = "CompletionIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_indicator: Option<TransportEventArrayOfCompletionIndicatorComponent>,
    #[serde(rename = "Contact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<TransportEventArrayOfContactComponent>,
    #[serde(rename = "CurrentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_status: Option<TransportEventArrayOfCurrentStatusComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TransportEventArrayOfDescriptionComponent>,
    #[serde(rename = "IdentificationID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_id: Option<TransportEventArrayOfIdentificationIDComponent>,
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TransportEventArrayOfLocationComponent>,
    #[serde(rename = "OccurrenceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrence_date: Option<TransportEventArrayOfOccurrenceDateComponent>,
    #[serde(rename = "OccurrenceTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrence_time: Option<TransportEventArrayOfOccurrenceTimeComponent>,
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<TransportEventArrayOfPeriodComponent>,
    #[serde(rename = "ReportedShipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_shipment: Option<TransportEventArrayOfReportedShipmentComponent>,
    #[serde(rename = "Signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<TransportEventArrayOfSignatureComponent>,
    #[serde(rename = "TransportEventTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_event_type_code: Option<TransportEventArrayOfTransportEventTypeCodeComponent>,
}

impl AsMut<TransportEvent> for TransportEvent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEvent> for TransportEvent {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.reported_shipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.reported_shipment", e));
            }
        }
        if let Some(v) = &self.current_status {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.current_status", e));
            }
        }
        if let Some(v) = &self.identification_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.identification_id", e));
            }
        }
        if let Some(v) = &self.occurrence_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.occurrence_date", e));
            }
        }
        if let Some(v) = &self.location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.location", e));
            }
        }
        if let Some(v) = &self.occurrence_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.occurrence_time", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.description", e));
            }
        }
        if let Some(v) = &self.period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.period", e));
            }
        }
        if let Some(v) = &self.signature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.signature", e));
            }
        }
        if let Some(v) = &self.contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.contact", e));
            }
        }
        if let Some(v) = &self.transport_event_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.transport_event_type_code", e));
            }
        }
        if let Some(v) = &self.completion_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportEvent.completion_indicator", e));
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

impl TransportEvent {
    pub fn title() -> &'static str {
        "Transport Event. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a significant occurrence or happening related to the transportation of goods."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            period: None,
            reported_shipment: None,
            signature: None,
            transport_event_type_code: None,
            contact: None,
            completion_indicator: None,
            current_status: None,
            description: None,
            identification_id: None,
            location: None,
            occurrence_date: None,
            occurrence_time: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEventArrayOfCompletionIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CompletionIndicator>,
}

impl AsMut<TransportEventArrayOfCompletionIndicatorComponent> for TransportEventArrayOfCompletionIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfCompletionIndicatorComponent> for TransportEventArrayOfCompletionIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEventArrayOfCompletionIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfCompletionIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfCompletionIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CompletionIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CompletionIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CompletionIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CompletionIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEventArrayOfContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Contact>,
}

impl AsMut<TransportEventArrayOfContactComponent> for TransportEventArrayOfContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfContactComponent> for TransportEventArrayOfContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Contact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Contact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Contact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Contact> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEventArrayOfCurrentStatusComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CurrentStatus>,
}

impl AsMut<TransportEventArrayOfCurrentStatusComponent> for TransportEventArrayOfCurrentStatusComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfCurrentStatusComponent> for TransportEventArrayOfCurrentStatusComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfCurrentStatusComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfCurrentStatusComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CurrentStatus) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CurrentStatus) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CurrentStatus> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CurrentStatus> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEventArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<TransportEventArrayOfDescriptionComponent> for TransportEventArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfDescriptionComponent> for TransportEventArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfDescriptionComponent {
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
pub struct TransportEventArrayOfIdentificationIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IdentificationID>,
}

impl AsMut<TransportEventArrayOfIdentificationIDComponent> for TransportEventArrayOfIdentificationIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfIdentificationIDComponent> for TransportEventArrayOfIdentificationIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEventArrayOfIdentificationIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfIdentificationIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfIdentificationIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IdentificationID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IdentificationID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IdentificationID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IdentificationID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEventArrayOfLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Location>,
}

impl AsMut<TransportEventArrayOfLocationComponent> for TransportEventArrayOfLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfLocationComponent> for TransportEventArrayOfLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEventArrayOfLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Location) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Location) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Location> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Location> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEventArrayOfOccurrenceDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OccurrenceDate>,
}

impl AsMut<TransportEventArrayOfOccurrenceDateComponent> for TransportEventArrayOfOccurrenceDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfOccurrenceDateComponent> for TransportEventArrayOfOccurrenceDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEventArrayOfOccurrenceDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfOccurrenceDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfOccurrenceDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OccurrenceDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OccurrenceDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OccurrenceDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OccurrenceDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEventArrayOfOccurrenceTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OccurrenceTime>,
}

impl AsMut<TransportEventArrayOfOccurrenceTimeComponent> for TransportEventArrayOfOccurrenceTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfOccurrenceTimeComponent> for TransportEventArrayOfOccurrenceTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEventArrayOfOccurrenceTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfOccurrenceTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfOccurrenceTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OccurrenceTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OccurrenceTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OccurrenceTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OccurrenceTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEventArrayOfPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Period>,
}

impl AsMut<TransportEventArrayOfPeriodComponent> for TransportEventArrayOfPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfPeriodComponent> for TransportEventArrayOfPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfPeriodComponent {
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
pub struct TransportEventArrayOfReportedShipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReportedShipment>,
}

impl AsMut<TransportEventArrayOfReportedShipmentComponent> for TransportEventArrayOfReportedShipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfReportedShipmentComponent> for TransportEventArrayOfReportedShipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEventArrayOfReportedShipmentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfReportedShipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfReportedShipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReportedShipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReportedShipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReportedShipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReportedShipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEventArrayOfSignatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Signature>,
}

impl AsMut<TransportEventArrayOfSignatureComponent> for TransportEventArrayOfSignatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfSignatureComponent> for TransportEventArrayOfSignatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEventArrayOfSignatureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfSignatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfSignatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Signature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Signature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Signature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Signature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportEventArrayOfTransportEventTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportEventTypeCode>,
}

impl AsMut<TransportEventArrayOfTransportEventTypeCodeComponent> for TransportEventArrayOfTransportEventTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportEventArrayOfTransportEventTypeCodeComponent> for TransportEventArrayOfTransportEventTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportEventArrayOfTransportEventTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportEventArrayOfTransportEventTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportEventArrayOfTransportEventTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportEventTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportEventTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportEventTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportEventTypeCode> {
        self.items.iter()
    }
}


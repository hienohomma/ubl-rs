use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<StatusArrayOfConditionComponent>,
    #[serde(rename = "ConditionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_code: Option<StatusArrayOfConditionCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<StatusArrayOfDescriptionComponent>,
    #[serde(rename = "IndicationIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indication_indicator: Option<StatusArrayOfIndicationIndicatorComponent>,
    #[serde(rename = "Percent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<StatusArrayOfPercentComponent>,
    #[serde(rename = "ReferenceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_date: Option<StatusArrayOfReferenceDateComponent>,
    #[serde(rename = "ReferenceTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_time: Option<StatusArrayOfReferenceTimeComponent>,
    #[serde(rename = "ReliabilityPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reliability_percent: Option<StatusArrayOfReliabilityPercentComponent>,
    #[serde(rename = "SequenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<StatusArrayOfSequenceIDComponent>,
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<StatusArrayOfStatusReasonComponent>,
    #[serde(rename = "StatusReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason_code: Option<StatusArrayOfStatusReasonCodeComponent>,
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<StatusArrayOfTextComponent>,
}

impl AsMut<Status> for Status {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Status> for Status {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.reference_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.reference_time", e));
            }
        }
        if let Some(v) = &self.condition {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.condition", e));
            }
        }
        if let Some(v) = &self.percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.percent", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.description", e));
            }
        }
        if let Some(v) = &self.sequence_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.sequence_id", e));
            }
        }
        if let Some(v) = &self.reference_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.reference_date", e));
            }
        }
        if let Some(v) = &self.indication_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.indication_indicator", e));
            }
        }
        if let Some(v) = &self.reliability_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.reliability_percent", e));
            }
        }
        if let Some(v) = &self.status_reason {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.status_reason", e));
            }
        }
        if let Some(v) = &self.status_reason_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.status_reason_code", e));
            }
        }
        if let Some(v) = &self.condition_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.condition_code", e));
            }
        }
        if let Some(v) = &self.text {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Status.text", e));
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

impl Status {
    pub fn title() -> &'static str {
        "Status. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the condition or position of an object."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            reference_time: None,
            description: None,
            condition: None,
            percent: None,
            indication_indicator: None,
            sequence_id: None,
            status_reason_code: None,
            condition_code: None,
            text: None,
            reference_date: None,
            status_reason: None,
            reliability_percent: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatusArrayOfConditionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Condition>,
}

impl AsMut<StatusArrayOfConditionComponent> for StatusArrayOfConditionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfConditionComponent> for StatusArrayOfConditionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfConditionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfConditionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Condition) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Condition) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Condition> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Condition> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatusArrayOfConditionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConditionCode>,
}

impl AsMut<StatusArrayOfConditionCodeComponent> for StatusArrayOfConditionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfConditionCodeComponent> for StatusArrayOfConditionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatusArrayOfConditionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfConditionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfConditionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConditionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConditionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConditionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConditionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatusArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<StatusArrayOfDescriptionComponent> for StatusArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfDescriptionComponent> for StatusArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfDescriptionComponent {
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
pub struct StatusArrayOfIndicationIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IndicationIndicator>,
}

impl AsMut<StatusArrayOfIndicationIndicatorComponent> for StatusArrayOfIndicationIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfIndicationIndicatorComponent> for StatusArrayOfIndicationIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatusArrayOfIndicationIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfIndicationIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfIndicationIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::IndicationIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::IndicationIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::IndicationIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::IndicationIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatusArrayOfPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Percent>,
}

impl AsMut<StatusArrayOfPercentComponent> for StatusArrayOfPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfPercentComponent> for StatusArrayOfPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatusArrayOfPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Percent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Percent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Percent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Percent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatusArrayOfReferenceDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReferenceDate>,
}

impl AsMut<StatusArrayOfReferenceDateComponent> for StatusArrayOfReferenceDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfReferenceDateComponent> for StatusArrayOfReferenceDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatusArrayOfReferenceDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfReferenceDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfReferenceDateComponent {
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
pub struct StatusArrayOfReferenceTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReferenceTime>,
}

impl AsMut<StatusArrayOfReferenceTimeComponent> for StatusArrayOfReferenceTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfReferenceTimeComponent> for StatusArrayOfReferenceTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatusArrayOfReferenceTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfReferenceTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfReferenceTimeComponent {
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
pub struct StatusArrayOfReliabilityPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReliabilityPercent>,
}

impl AsMut<StatusArrayOfReliabilityPercentComponent> for StatusArrayOfReliabilityPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfReliabilityPercentComponent> for StatusArrayOfReliabilityPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatusArrayOfReliabilityPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfReliabilityPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfReliabilityPercentComponent {
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
pub struct StatusArrayOfSequenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SequenceID>,
}

impl AsMut<StatusArrayOfSequenceIDComponent> for StatusArrayOfSequenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfSequenceIDComponent> for StatusArrayOfSequenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatusArrayOfSequenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfSequenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfSequenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SequenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SequenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SequenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SequenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatusArrayOfStatusReasonComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::StatusReason>,
}

impl AsMut<StatusArrayOfStatusReasonComponent> for StatusArrayOfStatusReasonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfStatusReasonComponent> for StatusArrayOfStatusReasonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfStatusReasonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfStatusReasonComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::StatusReason) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::StatusReason) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::StatusReason> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::StatusReason> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatusArrayOfStatusReasonCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::StatusReasonCode>,
}

impl AsMut<StatusArrayOfStatusReasonCodeComponent> for StatusArrayOfStatusReasonCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfStatusReasonCodeComponent> for StatusArrayOfStatusReasonCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StatusArrayOfStatusReasonCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfStatusReasonCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfStatusReasonCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::StatusReasonCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::StatusReasonCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::StatusReasonCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::StatusReasonCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StatusArrayOfTextComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Text>,
}

impl AsMut<StatusArrayOfTextComponent> for StatusArrayOfTextComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StatusArrayOfTextComponent> for StatusArrayOfTextComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StatusArrayOfTextComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StatusArrayOfTextComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Text) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Text) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Text> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Text> {
        self.items.iter()
    }
}


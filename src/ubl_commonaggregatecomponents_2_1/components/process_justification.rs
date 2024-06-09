use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessJustification {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ProcessJustificationArrayOfDescriptionComponent>,
    #[serde(rename = "PreviousCancellationReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_cancellation_reason_code: Option<ProcessJustificationArrayOfPreviousCancellationReasonCodeComponent>,
    #[serde(rename = "ProcessReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_reason: Option<ProcessJustificationArrayOfProcessReasonComponent>,
    #[serde(rename = "ProcessReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_reason_code: Option<ProcessJustificationArrayOfProcessReasonCodeComponent>,
}

impl AsMut<ProcessJustification> for ProcessJustification {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcessJustification> for ProcessJustification {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.process_reason_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcessJustification.process_reason_code", e));
            }
        }
        if let Some(v) = &self.process_reason {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcessJustification.process_reason", e));
            }
        }
        if let Some(v) = &self.previous_cancellation_reason_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcessJustification.previous_cancellation_reason_code", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcessJustification.description", e));
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

impl ProcessJustification {
    pub fn title() -> &'static str {
        "Process Justification. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a justification for the choice of tendering process."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            process_reason_code: None,
            description: None,
            previous_cancellation_reason_code: None,
            process_reason: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcessJustificationArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ProcessJustificationArrayOfDescriptionComponent> for ProcessJustificationArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcessJustificationArrayOfDescriptionComponent> for ProcessJustificationArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ProcessJustificationArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcessJustificationArrayOfDescriptionComponent {
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
pub struct ProcessJustificationArrayOfPreviousCancellationReasonCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PreviousCancellationReasonCode>,
}

impl AsMut<ProcessJustificationArrayOfPreviousCancellationReasonCodeComponent> for ProcessJustificationArrayOfPreviousCancellationReasonCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcessJustificationArrayOfPreviousCancellationReasonCodeComponent> for ProcessJustificationArrayOfPreviousCancellationReasonCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcessJustificationArrayOfPreviousCancellationReasonCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcessJustificationArrayOfPreviousCancellationReasonCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcessJustificationArrayOfPreviousCancellationReasonCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PreviousCancellationReasonCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PreviousCancellationReasonCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PreviousCancellationReasonCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PreviousCancellationReasonCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcessJustificationArrayOfProcessReasonComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ProcessReason>,
}

impl AsMut<ProcessJustificationArrayOfProcessReasonComponent> for ProcessJustificationArrayOfProcessReasonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcessJustificationArrayOfProcessReasonComponent> for ProcessJustificationArrayOfProcessReasonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ProcessJustificationArrayOfProcessReasonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcessJustificationArrayOfProcessReasonComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ProcessReason) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ProcessReason) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ProcessReason> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ProcessReason> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcessJustificationArrayOfProcessReasonCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ProcessReasonCode>,
}

impl AsMut<ProcessJustificationArrayOfProcessReasonCodeComponent> for ProcessJustificationArrayOfProcessReasonCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcessJustificationArrayOfProcessReasonCodeComponent> for ProcessJustificationArrayOfProcessReasonCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcessJustificationArrayOfProcessReasonCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcessJustificationArrayOfProcessReasonCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcessJustificationArrayOfProcessReasonCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ProcessReasonCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ProcessReasonCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ProcessReasonCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ProcessReasonCode> {
        self.items.iter()
    }
}


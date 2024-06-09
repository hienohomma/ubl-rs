use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotificationRequirement {
    #[serde(rename = "NotificationLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_location: Option<NotificationRequirementArrayOfNotificationLocationComponent>,
    #[serde(rename = "NotificationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_period: Option<NotificationRequirementArrayOfNotificationPeriodComponent>,
    #[serde(rename = "NotificationTypeCode")]
    pub notification_type_code: NotificationRequirementArrayOfNotificationTypeCodeComponent,
    #[serde(rename = "NotifyParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_party: Option<NotificationRequirementArrayOfNotifyPartyComponent>,
    #[serde(rename = "PostEventNotificationDurationMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_event_notification_duration_measure: Option<NotificationRequirementArrayOfPostEventNotificationDurationMeasureComponent>,
    #[serde(rename = "PreEventNotificationDurationMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_event_notification_duration_measure: Option<NotificationRequirementArrayOfPreEventNotificationDurationMeasureComponent>,
}

impl AsMut<NotificationRequirement> for NotificationRequirement {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<NotificationRequirement> for NotificationRequirement {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.notify_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("NotificationRequirement.notify_party", e));
            }
        }
        if let Some(v) = &self.notification_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("NotificationRequirement.notification_location", e));
            }
        }
        if let Some(v) = &self.notification_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("NotificationRequirement.notification_period", e));
            }
        }
        if let Some(v) = &self.pre_event_notification_duration_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("NotificationRequirement.pre_event_notification_duration_measure", e));
            }
        }
        if let Some(v) = &self.post_event_notification_duration_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("NotificationRequirement.post_event_notification_duration_measure", e));
            }
        }
        if let Err(e) = self.notification_type_code.validate() {
            return Err(UblError::component("NotificationRequirement.notification_type_code", e));
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

impl NotificationRequirement {
    pub fn title() -> &'static str {
        "Notification Requirement. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a notification requirement."
    }
    pub fn new(notification_type_code: NotificationRequirementArrayOfNotificationTypeCodeComponent) -> Component<Self> {
        Component(Self {
            pre_event_notification_duration_measure: None,
            notification_period: None,
            notification_type_code,
            post_event_notification_duration_measure: None,
            notification_location: None,
            notify_party: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NotificationRequirementArrayOfNotificationLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::NotificationLocation>,
}

impl AsMut<NotificationRequirementArrayOfNotificationLocationComponent> for NotificationRequirementArrayOfNotificationLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<NotificationRequirementArrayOfNotificationLocationComponent> for NotificationRequirementArrayOfNotificationLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("NotificationRequirementArrayOfNotificationLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl NotificationRequirementArrayOfNotificationLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::NotificationLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::NotificationLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::NotificationLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::NotificationLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NotificationRequirementArrayOfNotificationPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::NotificationPeriod>,
}

impl AsMut<NotificationRequirementArrayOfNotificationPeriodComponent> for NotificationRequirementArrayOfNotificationPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<NotificationRequirementArrayOfNotificationPeriodComponent> for NotificationRequirementArrayOfNotificationPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("NotificationRequirementArrayOfNotificationPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl NotificationRequirementArrayOfNotificationPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::NotificationPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::NotificationPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::NotificationPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::NotificationPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NotificationRequirementArrayOfNotificationTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NotificationTypeCode>,
}

impl AsMut<NotificationRequirementArrayOfNotificationTypeCodeComponent> for NotificationRequirementArrayOfNotificationTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<NotificationRequirementArrayOfNotificationTypeCodeComponent> for NotificationRequirementArrayOfNotificationTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("NotificationRequirementArrayOfNotificationTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("NotificationRequirementArrayOfNotificationTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl NotificationRequirementArrayOfNotificationTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NotificationTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NotificationTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NotificationTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NotificationTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NotificationRequirementArrayOfNotifyPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::NotifyParty>,
}

impl AsMut<NotificationRequirementArrayOfNotifyPartyComponent> for NotificationRequirementArrayOfNotifyPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<NotificationRequirementArrayOfNotifyPartyComponent> for NotificationRequirementArrayOfNotifyPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("NotificationRequirementArrayOfNotifyPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl NotificationRequirementArrayOfNotifyPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::NotifyParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::NotifyParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::NotifyParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::NotifyParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NotificationRequirementArrayOfPostEventNotificationDurationMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PostEventNotificationDurationMeasure>,
}

impl AsMut<NotificationRequirementArrayOfPostEventNotificationDurationMeasureComponent> for NotificationRequirementArrayOfPostEventNotificationDurationMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<NotificationRequirementArrayOfPostEventNotificationDurationMeasureComponent> for NotificationRequirementArrayOfPostEventNotificationDurationMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("NotificationRequirementArrayOfPostEventNotificationDurationMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("NotificationRequirementArrayOfPostEventNotificationDurationMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl NotificationRequirementArrayOfPostEventNotificationDurationMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PostEventNotificationDurationMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PostEventNotificationDurationMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PostEventNotificationDurationMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PostEventNotificationDurationMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NotificationRequirementArrayOfPreEventNotificationDurationMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PreEventNotificationDurationMeasure>,
}

impl AsMut<NotificationRequirementArrayOfPreEventNotificationDurationMeasureComponent> for NotificationRequirementArrayOfPreEventNotificationDurationMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<NotificationRequirementArrayOfPreEventNotificationDurationMeasureComponent> for NotificationRequirementArrayOfPreEventNotificationDurationMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("NotificationRequirementArrayOfPreEventNotificationDurationMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("NotificationRequirementArrayOfPreEventNotificationDurationMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl NotificationRequirementArrayOfPreEventNotificationDurationMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PreEventNotificationDurationMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PreEventNotificationDurationMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PreEventNotificationDurationMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PreEventNotificationDurationMeasure> {
        self.items.iter()
    }
}


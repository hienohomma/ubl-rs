use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventComment {
    #[serde(rename = "Comment")]
    pub comment: EventCommentArrayOfCommentComponent,
    #[serde(rename = "IssueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<EventCommentArrayOfIssueDateComponent>,
    #[serde(rename = "IssueTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<EventCommentArrayOfIssueTimeComponent>,
}

impl AsMut<EventComment> for EventComment {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventComment> for EventComment {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.issue_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventComment.issue_time", e));
            }
        }
        if let Some(v) = &self.issue_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventComment.issue_date", e));
            }
        }
        if let Err(e) = self.comment.validate() {
            return Err(UblError::component("EventComment.comment", e));
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

impl EventComment {
    pub fn title() -> &'static str {
        "Event Comment. Details"
    }
    pub fn description() -> &'static str {
        "A class to define comments about a retail event."
    }
    pub fn new(comment: EventCommentArrayOfCommentComponent) -> Component<Self> {
        Component(Self {
            issue_time: None,
            comment,
            issue_date: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventCommentArrayOfCommentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Comment>,
}

impl AsMut<EventCommentArrayOfCommentComponent> for EventCommentArrayOfCommentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventCommentArrayOfCommentComponent> for EventCommentArrayOfCommentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventCommentArrayOfCommentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventCommentArrayOfCommentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventCommentArrayOfCommentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Comment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Comment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Comment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Comment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventCommentArrayOfIssueDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueDate>,
}

impl AsMut<EventCommentArrayOfIssueDateComponent> for EventCommentArrayOfIssueDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventCommentArrayOfIssueDateComponent> for EventCommentArrayOfIssueDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventCommentArrayOfIssueDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventCommentArrayOfIssueDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventCommentArrayOfIssueDateComponent {
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
pub struct EventCommentArrayOfIssueTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::IssueTime>,
}

impl AsMut<EventCommentArrayOfIssueTimeComponent> for EventCommentArrayOfIssueTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventCommentArrayOfIssueTimeComponent> for EventCommentArrayOfIssueTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventCommentArrayOfIssueTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventCommentArrayOfIssueTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventCommentArrayOfIssueTimeComponent {
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


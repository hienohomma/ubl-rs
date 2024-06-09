use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventTactic {
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<EventTacticArrayOfCommentComponent>,
    #[serde(rename = "EventTacticEnumeration")]
    pub event_tactic_enumeration: EventTacticArrayOfEventTacticEnumerationComponent,
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<EventTacticArrayOfPeriodComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<EventTacticArrayOfQuantityComponent>,
}

impl AsMut<EventTactic> for EventTactic {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventTactic> for EventTactic {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.event_tactic_enumeration.validate() {
            return Err(UblError::component("EventTactic.event_tactic_enumeration", e));
        }
        if let Some(v) = &self.period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventTactic.period", e));
            }
        }
        if let Some(v) = &self.comment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventTactic.comment", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventTactic.quantity", e));
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

impl EventTactic {
    pub fn title() -> &'static str {
        "Event Tactic. Details"
    }
    pub fn description() -> &'static str {
        "A class defining a specific type of action or situation arranged by the Buyer or the Seller to promote the product or products."
    }
    pub fn new(event_tactic_enumeration: EventTacticArrayOfEventTacticEnumerationComponent) -> Component<Self> {
        Component(Self {
            period: None,
            event_tactic_enumeration,
            quantity: None,
            comment: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventTacticArrayOfCommentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Comment>,
}

impl AsMut<EventTacticArrayOfCommentComponent> for EventTacticArrayOfCommentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventTacticArrayOfCommentComponent> for EventTacticArrayOfCommentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventTacticArrayOfCommentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventTacticArrayOfCommentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventTacticArrayOfCommentComponent {
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
pub struct EventTacticArrayOfEventTacticEnumerationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EventTacticEnumeration>,
}

impl AsMut<EventTacticArrayOfEventTacticEnumerationComponent> for EventTacticArrayOfEventTacticEnumerationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventTacticArrayOfEventTacticEnumerationComponent> for EventTacticArrayOfEventTacticEnumerationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventTacticArrayOfEventTacticEnumerationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventTacticArrayOfEventTacticEnumerationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventTacticArrayOfEventTacticEnumerationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EventTacticEnumeration) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EventTacticEnumeration) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EventTacticEnumeration> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EventTacticEnumeration> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventTacticArrayOfPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Period>,
}

impl AsMut<EventTacticArrayOfPeriodComponent> for EventTacticArrayOfPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventTacticArrayOfPeriodComponent> for EventTacticArrayOfPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventTacticArrayOfPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventTacticArrayOfPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventTacticArrayOfPeriodComponent {
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
pub struct EventTacticArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<EventTacticArrayOfQuantityComponent> for EventTacticArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventTacticArrayOfQuantityComponent> for EventTacticArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventTacticArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventTacticArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventTacticArrayOfQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Quantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Quantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter()
    }
}


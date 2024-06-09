use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ResponseArrayOfDescriptionComponent>,
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<ResponseArrayOfEffectiveDateComponent>,
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<ResponseArrayOfEffectiveTimeComponent>,
    #[serde(rename = "ReferenceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<ResponseArrayOfReferenceIDComponent>,
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<ResponseArrayOfResponseCodeComponent>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponseArrayOfStatusComponent>,
}

impl AsMut<Response> for Response {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Response> for Response {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.effective_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Response.effective_date", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Response.description", e));
            }
        }
        if let Some(v) = &self.effective_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Response.effective_time", e));
            }
        }
        if let Some(v) = &self.reference_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Response.reference_id", e));
            }
        }
        if let Some(v) = &self.response_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Response.response_code", e));
            }
        }
        if let Some(v) = &self.status {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Response.status", e));
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

impl Response {
    pub fn title() -> &'static str {
        "Response. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an application-level response to a document."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            status: None,
            effective_time: None,
            description: None,
            effective_date: None,
            response_code: None,
            reference_id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResponseArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ResponseArrayOfDescriptionComponent> for ResponseArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResponseArrayOfDescriptionComponent> for ResponseArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ResponseArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResponseArrayOfDescriptionComponent {
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
pub struct ResponseArrayOfEffectiveDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EffectiveDate>,
}

impl AsMut<ResponseArrayOfEffectiveDateComponent> for ResponseArrayOfEffectiveDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResponseArrayOfEffectiveDateComponent> for ResponseArrayOfEffectiveDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResponseArrayOfEffectiveDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResponseArrayOfEffectiveDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResponseArrayOfEffectiveDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EffectiveDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EffectiveDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EffectiveDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EffectiveDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResponseArrayOfEffectiveTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EffectiveTime>,
}

impl AsMut<ResponseArrayOfEffectiveTimeComponent> for ResponseArrayOfEffectiveTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResponseArrayOfEffectiveTimeComponent> for ResponseArrayOfEffectiveTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResponseArrayOfEffectiveTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResponseArrayOfEffectiveTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResponseArrayOfEffectiveTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EffectiveTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EffectiveTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EffectiveTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EffectiveTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResponseArrayOfReferenceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReferenceID>,
}

impl AsMut<ResponseArrayOfReferenceIDComponent> for ResponseArrayOfReferenceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResponseArrayOfReferenceIDComponent> for ResponseArrayOfReferenceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResponseArrayOfReferenceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResponseArrayOfReferenceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResponseArrayOfReferenceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReferenceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReferenceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReferenceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReferenceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResponseArrayOfResponseCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ResponseCode>,
}

impl AsMut<ResponseArrayOfResponseCodeComponent> for ResponseArrayOfResponseCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResponseArrayOfResponseCodeComponent> for ResponseArrayOfResponseCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResponseArrayOfResponseCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResponseArrayOfResponseCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResponseArrayOfResponseCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ResponseCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ResponseCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ResponseCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ResponseCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResponseArrayOfStatusComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Status>,
}

impl AsMut<ResponseArrayOfStatusComponent> for ResponseArrayOfStatusComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResponseArrayOfStatusComponent> for ResponseArrayOfStatusComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ResponseArrayOfStatusComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResponseArrayOfStatusComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Status) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Status) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Status> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Status> {
        self.items.iter()
    }
}


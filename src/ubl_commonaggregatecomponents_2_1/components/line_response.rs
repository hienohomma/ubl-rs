use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineResponse {
    #[serde(rename = "LineReference")]
    pub line_reference: LineResponseArrayOfLineReferenceComponent,
    #[serde(rename = "Response")]
    pub response: LineResponseArrayOfResponseComponent,
}

impl AsMut<LineResponse> for LineResponse {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineResponse> for LineResponse {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.response.validate() {
            return Err(UblError::component("LineResponse.response", e));
        }
        if let Err(e) = self.line_reference.validate() {
            return Err(UblError::component("LineResponse.line_reference", e));
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

impl LineResponse {
    pub fn title() -> &'static str {
        "Line Response. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe responses to a line in a document."
    }
    pub fn new(line_reference: LineResponseArrayOfLineReferenceComponent, response: LineResponseArrayOfResponseComponent) -> Component<Self> {
        Component(Self {
            response,
            line_reference,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineResponseArrayOfLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LineReference>,
}

impl AsMut<LineResponseArrayOfLineReferenceComponent> for LineResponseArrayOfLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineResponseArrayOfLineReferenceComponent> for LineResponseArrayOfLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LineResponseArrayOfLineReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LineResponseArrayOfLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineResponseArrayOfLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LineResponseArrayOfResponseComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Response>,
}

impl AsMut<LineResponseArrayOfResponseComponent> for LineResponseArrayOfResponseComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LineResponseArrayOfResponseComponent> for LineResponseArrayOfResponseComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LineResponseArrayOfResponseComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LineResponseArrayOfResponseComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Response) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Response) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Response> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Response> {
        self.items.iter()
    }
}


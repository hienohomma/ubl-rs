use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Communication {
    #[serde(rename = "Channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<CommunicationArrayOfChannelComponent>,
    #[serde(rename = "ChannelCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_code: Option<CommunicationArrayOfChannelCodeComponent>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<CommunicationArrayOfValueComponent>,
}

impl AsMut<Communication> for Communication {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Communication> for Communication {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.channel {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Communication.channel", e));
            }
        }
        if let Some(v) = &self.value {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Communication.value", e));
            }
        }
        if let Some(v) = &self.channel_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Communication.channel_code", e));
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

impl Communication {
    pub fn title() -> &'static str {
        "Communication. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a means of communication."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            value: None,
            channel: None,
            channel_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CommunicationArrayOfChannelComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Channel>,
}

impl AsMut<CommunicationArrayOfChannelComponent> for CommunicationArrayOfChannelComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CommunicationArrayOfChannelComponent> for CommunicationArrayOfChannelComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CommunicationArrayOfChannelComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CommunicationArrayOfChannelComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CommunicationArrayOfChannelComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Channel) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Channel) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Channel> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Channel> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CommunicationArrayOfChannelCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ChannelCode>,
}

impl AsMut<CommunicationArrayOfChannelCodeComponent> for CommunicationArrayOfChannelCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CommunicationArrayOfChannelCodeComponent> for CommunicationArrayOfChannelCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CommunicationArrayOfChannelCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CommunicationArrayOfChannelCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CommunicationArrayOfChannelCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ChannelCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ChannelCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ChannelCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ChannelCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CommunicationArrayOfValueComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Value>,
}

impl AsMut<CommunicationArrayOfValueComponent> for CommunicationArrayOfValueComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CommunicationArrayOfValueComponent> for CommunicationArrayOfValueComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CommunicationArrayOfValueComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CommunicationArrayOfValueComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CommunicationArrayOfValueComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Value) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Value) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Value> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Value> {
        self.items.iter()
    }
}


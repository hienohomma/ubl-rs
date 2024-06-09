use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebSiteAccess {
    #[serde(rename = "Login")]
    pub login: WebSiteAccessArrayOfLoginComponent,
    #[serde(rename = "Password")]
    pub password: WebSiteAccessArrayOfPasswordComponent,
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<WebSiteAccessArrayOfURIComponent>,
}

impl AsMut<WebSiteAccess> for WebSiteAccess {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WebSiteAccess> for WebSiteAccess {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.login.validate() {
            return Err(UblError::component("WebSiteAccess.login", e));
        }
        if let Err(e) = self.password.validate() {
            return Err(UblError::component("WebSiteAccess.password", e));
        }
        if let Some(v) = &self.uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("WebSiteAccess.uri", e));
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

impl WebSiteAccess {
    pub fn title() -> &'static str {
        "Web Site Access. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe access to a web site."
    }
    pub fn new(login: WebSiteAccessArrayOfLoginComponent, password: WebSiteAccessArrayOfPasswordComponent) -> Component<Self> {
        Component(Self {
            password,
            login,
            uri: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WebSiteAccessArrayOfLoginComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Login>,
}

impl AsMut<WebSiteAccessArrayOfLoginComponent> for WebSiteAccessArrayOfLoginComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WebSiteAccessArrayOfLoginComponent> for WebSiteAccessArrayOfLoginComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("WebSiteAccessArrayOfLoginComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("WebSiteAccessArrayOfLoginComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WebSiteAccessArrayOfLoginComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Login) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Login) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Login> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Login> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WebSiteAccessArrayOfPasswordComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Password>,
}

impl AsMut<WebSiteAccessArrayOfPasswordComponent> for WebSiteAccessArrayOfPasswordComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WebSiteAccessArrayOfPasswordComponent> for WebSiteAccessArrayOfPasswordComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("WebSiteAccessArrayOfPasswordComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("WebSiteAccessArrayOfPasswordComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WebSiteAccessArrayOfPasswordComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Password) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Password) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Password> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Password> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WebSiteAccessArrayOfURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::URI>,
}

impl AsMut<WebSiteAccessArrayOfURIComponent> for WebSiteAccessArrayOfURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WebSiteAccessArrayOfURIComponent> for WebSiteAccessArrayOfURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("WebSiteAccessArrayOfURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("WebSiteAccessArrayOfURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WebSiteAccessArrayOfURIComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::URI) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::URI) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::URI> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::URI> {
        self.items.iter()
    }
}


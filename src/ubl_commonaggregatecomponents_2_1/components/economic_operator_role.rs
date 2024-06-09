use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EconomicOperatorRole {
    #[serde(rename = "RoleCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_code: Option<EconomicOperatorRoleArrayOfRoleCodeComponent>,
    #[serde(rename = "RoleDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_description: Option<EconomicOperatorRoleArrayOfRoleDescriptionComponent>,
}

impl AsMut<EconomicOperatorRole> for EconomicOperatorRole {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EconomicOperatorRole> for EconomicOperatorRole {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.role_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EconomicOperatorRole.role_description", e));
            }
        }
        if let Some(v) = &self.role_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EconomicOperatorRole.role_code", e));
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

impl EconomicOperatorRole {
    pub fn title() -> &'static str {
        "Economic Operator Role. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the tenderer contracting role."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            role_code: None,
            role_description: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EconomicOperatorRoleArrayOfRoleCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RoleCode>,
}

impl AsMut<EconomicOperatorRoleArrayOfRoleCodeComponent> for EconomicOperatorRoleArrayOfRoleCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EconomicOperatorRoleArrayOfRoleCodeComponent> for EconomicOperatorRoleArrayOfRoleCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EconomicOperatorRoleArrayOfRoleCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EconomicOperatorRoleArrayOfRoleCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EconomicOperatorRoleArrayOfRoleCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RoleCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RoleCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RoleCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RoleCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EconomicOperatorRoleArrayOfRoleDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RoleDescription>,
}

impl AsMut<EconomicOperatorRoleArrayOfRoleDescriptionComponent> for EconomicOperatorRoleArrayOfRoleDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EconomicOperatorRoleArrayOfRoleDescriptionComponent> for EconomicOperatorRoleArrayOfRoleDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("EconomicOperatorRoleArrayOfRoleDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EconomicOperatorRoleArrayOfRoleDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RoleDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RoleDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RoleDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RoleDescription> {
        self.items.iter()
    }
}


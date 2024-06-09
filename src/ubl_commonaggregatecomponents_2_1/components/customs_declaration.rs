use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomsDeclaration {
    #[serde(rename = "ID")]
    pub id: CustomsDeclarationArrayOfIDComponent,
    #[serde(rename = "IssuerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_party: Option<CustomsDeclarationArrayOfIssuerPartyComponent>,
}

impl AsMut<CustomsDeclaration> for CustomsDeclaration {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CustomsDeclaration> for CustomsDeclaration {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("CustomsDeclaration.id", e));
        }
        if let Some(v) = &self.issuer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CustomsDeclaration.issuer_party", e));
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

impl CustomsDeclaration {
    pub fn title() -> &'static str {
        "Customs Declaration. Details"
    }
    pub fn description() -> &'static str {
        "A class describing identifiers or references relating to customs procedures."
    }
    pub fn new(id: CustomsDeclarationArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            issuer_party: None,
            id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CustomsDeclarationArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<CustomsDeclarationArrayOfIDComponent> for CustomsDeclarationArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CustomsDeclarationArrayOfIDComponent> for CustomsDeclarationArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CustomsDeclarationArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CustomsDeclarationArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CustomsDeclarationArrayOfIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CustomsDeclarationArrayOfIssuerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::IssuerParty>,
}

impl AsMut<CustomsDeclarationArrayOfIssuerPartyComponent> for CustomsDeclarationArrayOfIssuerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CustomsDeclarationArrayOfIssuerPartyComponent> for CustomsDeclarationArrayOfIssuerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CustomsDeclarationArrayOfIssuerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CustomsDeclarationArrayOfIssuerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CustomsDeclarationArrayOfIssuerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::IssuerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::IssuerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::IssuerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::IssuerParty> {
        self.items.iter()
    }
}


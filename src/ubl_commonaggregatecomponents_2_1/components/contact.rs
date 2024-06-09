use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contact {
    #[serde(rename = "ElectronicMail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_mail: Option<ContactArrayOfElectronicMailComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ContactArrayOfIDComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ContactArrayOfNameComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<ContactArrayOfNoteComponent>,
    #[serde(rename = "OtherCommunication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_communication: Option<ContactArrayOfOtherCommunicationComponent>,
    #[serde(rename = "Telefax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telefax: Option<ContactArrayOfTelefaxComponent>,
    #[serde(rename = "Telephone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<ContactArrayOfTelephoneComponent>,
}

impl AsMut<Contact> for Contact {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Contact> for Contact {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Contact.id", e));
            }
        }
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Contact.name", e));
            }
        }
        if let Some(v) = &self.electronic_mail {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Contact.electronic_mail", e));
            }
        }
        if let Some(v) = &self.other_communication {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Contact.other_communication", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Contact.note", e));
            }
        }
        if let Some(v) = &self.telephone {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Contact.telephone", e));
            }
        }
        if let Some(v) = &self.telefax {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Contact.telefax", e));
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

impl Contact {
    pub fn title() -> &'static str {
        "Contact. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a contactable person or department in an organization."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            telefax: None,
            note: None,
            telephone: None,
            electronic_mail: None,
            id: None,
            name: None,
            other_communication: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContactArrayOfElectronicMailComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ElectronicMail>,
}

impl AsMut<ContactArrayOfElectronicMailComponent> for ContactArrayOfElectronicMailComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContactArrayOfElectronicMailComponent> for ContactArrayOfElectronicMailComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContactArrayOfElectronicMailComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContactArrayOfElectronicMailComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContactArrayOfElectronicMailComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ElectronicMail) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ElectronicMail) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ElectronicMail> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ElectronicMail> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContactArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ContactArrayOfIDComponent> for ContactArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContactArrayOfIDComponent> for ContactArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContactArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContactArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContactArrayOfIDComponent {
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
pub struct ContactArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<ContactArrayOfNameComponent> for ContactArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContactArrayOfNameComponent> for ContactArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContactArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContactArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContactArrayOfNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Name) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Name) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContactArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<ContactArrayOfNoteComponent> for ContactArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContactArrayOfNoteComponent> for ContactArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ContactArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContactArrayOfNoteComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Note) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Note) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContactArrayOfOtherCommunicationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OtherCommunication>,
}

impl AsMut<ContactArrayOfOtherCommunicationComponent> for ContactArrayOfOtherCommunicationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContactArrayOfOtherCommunicationComponent> for ContactArrayOfOtherCommunicationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ContactArrayOfOtherCommunicationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContactArrayOfOtherCommunicationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OtherCommunication) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OtherCommunication) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OtherCommunication> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OtherCommunication> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContactArrayOfTelefaxComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Telefax>,
}

impl AsMut<ContactArrayOfTelefaxComponent> for ContactArrayOfTelefaxComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContactArrayOfTelefaxComponent> for ContactArrayOfTelefaxComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContactArrayOfTelefaxComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContactArrayOfTelefaxComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContactArrayOfTelefaxComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Telefax) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Telefax) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Telefax> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Telefax> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContactArrayOfTelephoneComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Telephone>,
}

impl AsMut<ContactArrayOfTelephoneComponent> for ContactArrayOfTelephoneComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContactArrayOfTelephoneComponent> for ContactArrayOfTelephoneComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContactArrayOfTelephoneComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContactArrayOfTelephoneComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContactArrayOfTelephoneComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Telephone) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Telephone) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Telephone> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Telephone> {
        self.items.iter()
    }
}


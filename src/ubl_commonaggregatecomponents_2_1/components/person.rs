use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Person {
    #[serde(rename = "BirthDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<PersonArrayOfBirthDateComponent>,
    #[serde(rename = "BirthplaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthplace_name: Option<PersonArrayOfBirthplaceNameComponent>,
    #[serde(rename = "Contact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<PersonArrayOfContactComponent>,
    #[serde(rename = "FamilyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<PersonArrayOfFamilyNameComponent>,
    #[serde(rename = "FinancialAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<PersonArrayOfFinancialAccountComponent>,
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<PersonArrayOfFirstNameComponent>,
    #[serde(rename = "GenderCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender_code: Option<PersonArrayOfGenderCodeComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PersonArrayOfIDComponent>,
    #[serde(rename = "IdentityDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_document_reference: Option<PersonArrayOfIdentityDocumentReferenceComponent>,
    #[serde(rename = "JobTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<PersonArrayOfJobTitleComponent>,
    #[serde(rename = "MiddleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<PersonArrayOfMiddleNameComponent>,
    #[serde(rename = "NameSuffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_suffix: Option<PersonArrayOfNameSuffixComponent>,
    #[serde(rename = "NationalityID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<PersonArrayOfNationalityIDComponent>,
    #[serde(rename = "OrganizationDepartment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_department: Option<PersonArrayOfOrganizationDepartmentComponent>,
    #[serde(rename = "OtherName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_name: Option<PersonArrayOfOtherNameComponent>,
    #[serde(rename = "ResidenceAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_address: Option<PersonArrayOfResidenceAddressComponent>,
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<PersonArrayOfTitleComponent>,
}

impl AsMut<Person> for Person {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Person> for Person {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.job_title {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.job_title", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.id", e));
            }
        }
        if let Some(v) = &self.other_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.other_name", e));
            }
        }
        if let Some(v) = &self.title {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.title", e));
            }
        }
        if let Some(v) = &self.name_suffix {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.name_suffix", e));
            }
        }
        if let Some(v) = &self.financial_account {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.financial_account", e));
            }
        }
        if let Some(v) = &self.family_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.family_name", e));
            }
        }
        if let Some(v) = &self.residence_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.residence_address", e));
            }
        }
        if let Some(v) = &self.identity_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.identity_document_reference", e));
            }
        }
        if let Some(v) = &self.birth_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.birth_date", e));
            }
        }
        if let Some(v) = &self.gender_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.gender_code", e));
            }
        }
        if let Some(v) = &self.organization_department {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.organization_department", e));
            }
        }
        if let Some(v) = &self.nationality_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.nationality_id", e));
            }
        }
        if let Some(v) = &self.contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.contact", e));
            }
        }
        if let Some(v) = &self.middle_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.middle_name", e));
            }
        }
        if let Some(v) = &self.birthplace_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.birthplace_name", e));
            }
        }
        if let Some(v) = &self.first_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Person.first_name", e));
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

impl Person {
    pub fn title() -> &'static str {
        "Person. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a person."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            id: None,
            middle_name: None,
            gender_code: None,
            identity_document_reference: None,
            residence_address: None,
            organization_department: None,
            birth_date: None,
            nationality_id: None,
            name_suffix: None,
            contact: None,
            birthplace_name: None,
            title: None,
            other_name: None,
            job_title: None,
            family_name: None,
            financial_account: None,
            first_name: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfBirthDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BirthDate>,
}

impl AsMut<PersonArrayOfBirthDateComponent> for PersonArrayOfBirthDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfBirthDateComponent> for PersonArrayOfBirthDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfBirthDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfBirthDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfBirthDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BirthDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BirthDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BirthDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BirthDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfBirthplaceNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BirthplaceName>,
}

impl AsMut<PersonArrayOfBirthplaceNameComponent> for PersonArrayOfBirthplaceNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfBirthplaceNameComponent> for PersonArrayOfBirthplaceNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfBirthplaceNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfBirthplaceNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfBirthplaceNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BirthplaceName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BirthplaceName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BirthplaceName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BirthplaceName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Contact>,
}

impl AsMut<PersonArrayOfContactComponent> for PersonArrayOfContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfContactComponent> for PersonArrayOfContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfContactComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Contact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Contact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Contact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Contact> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfFamilyNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FamilyName>,
}

impl AsMut<PersonArrayOfFamilyNameComponent> for PersonArrayOfFamilyNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfFamilyNameComponent> for PersonArrayOfFamilyNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfFamilyNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfFamilyNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfFamilyNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FamilyName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FamilyName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FamilyName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FamilyName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfFinancialAccountComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FinancialAccount>,
}

impl AsMut<PersonArrayOfFinancialAccountComponent> for PersonArrayOfFinancialAccountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfFinancialAccountComponent> for PersonArrayOfFinancialAccountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfFinancialAccountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfFinancialAccountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfFinancialAccountComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FinancialAccount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FinancialAccount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FinancialAccount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FinancialAccount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfFirstNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FirstName>,
}

impl AsMut<PersonArrayOfFirstNameComponent> for PersonArrayOfFirstNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfFirstNameComponent> for PersonArrayOfFirstNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfFirstNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfFirstNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfFirstNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FirstName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FirstName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FirstName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FirstName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfGenderCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GenderCode>,
}

impl AsMut<PersonArrayOfGenderCodeComponent> for PersonArrayOfGenderCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfGenderCodeComponent> for PersonArrayOfGenderCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfGenderCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfGenderCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfGenderCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GenderCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GenderCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GenderCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GenderCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<PersonArrayOfIDComponent> for PersonArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfIDComponent> for PersonArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfIDComponent {
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
pub struct PersonArrayOfIdentityDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::IdentityDocumentReference>,
}

impl AsMut<PersonArrayOfIdentityDocumentReferenceComponent> for PersonArrayOfIdentityDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfIdentityDocumentReferenceComponent> for PersonArrayOfIdentityDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfIdentityDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfIdentityDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::IdentityDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::IdentityDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::IdentityDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::IdentityDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfJobTitleComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::JobTitle>,
}

impl AsMut<PersonArrayOfJobTitleComponent> for PersonArrayOfJobTitleComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfJobTitleComponent> for PersonArrayOfJobTitleComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfJobTitleComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfJobTitleComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfJobTitleComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::JobTitle) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::JobTitle) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::JobTitle> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::JobTitle> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfMiddleNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MiddleName>,
}

impl AsMut<PersonArrayOfMiddleNameComponent> for PersonArrayOfMiddleNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfMiddleNameComponent> for PersonArrayOfMiddleNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfMiddleNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfMiddleNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfMiddleNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MiddleName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MiddleName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MiddleName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MiddleName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfNameSuffixComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NameSuffix>,
}

impl AsMut<PersonArrayOfNameSuffixComponent> for PersonArrayOfNameSuffixComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfNameSuffixComponent> for PersonArrayOfNameSuffixComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfNameSuffixComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfNameSuffixComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfNameSuffixComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NameSuffix) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NameSuffix) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NameSuffix> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NameSuffix> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfNationalityIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NationalityID>,
}

impl AsMut<PersonArrayOfNationalityIDComponent> for PersonArrayOfNationalityIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfNationalityIDComponent> for PersonArrayOfNationalityIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfNationalityIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfNationalityIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfNationalityIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NationalityID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NationalityID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NationalityID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NationalityID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfOrganizationDepartmentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OrganizationDepartment>,
}

impl AsMut<PersonArrayOfOrganizationDepartmentComponent> for PersonArrayOfOrganizationDepartmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfOrganizationDepartmentComponent> for PersonArrayOfOrganizationDepartmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfOrganizationDepartmentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfOrganizationDepartmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfOrganizationDepartmentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OrganizationDepartment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OrganizationDepartment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OrganizationDepartment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OrganizationDepartment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfOtherNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OtherName>,
}

impl AsMut<PersonArrayOfOtherNameComponent> for PersonArrayOfOtherNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfOtherNameComponent> for PersonArrayOfOtherNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfOtherNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfOtherNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfOtherNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OtherName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OtherName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OtherName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OtherName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfResidenceAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ResidenceAddress>,
}

impl AsMut<PersonArrayOfResidenceAddressComponent> for PersonArrayOfResidenceAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfResidenceAddressComponent> for PersonArrayOfResidenceAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfResidenceAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfResidenceAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfResidenceAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ResidenceAddress) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ResidenceAddress) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ResidenceAddress> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ResidenceAddress> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PersonArrayOfTitleComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Title>,
}

impl AsMut<PersonArrayOfTitleComponent> for PersonArrayOfTitleComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PersonArrayOfTitleComponent> for PersonArrayOfTitleComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PersonArrayOfTitleComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PersonArrayOfTitleComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PersonArrayOfTitleComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Title) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Title) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Title> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Title> {
        self.items.iter()
    }
}


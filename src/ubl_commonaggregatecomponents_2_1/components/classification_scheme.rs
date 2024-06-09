use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassificationScheme {
    #[serde(rename = "AgencyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency_id: Option<ClassificationSchemeArrayOfAgencyIDComponent>,
    #[serde(rename = "AgencyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency_name: Option<ClassificationSchemeArrayOfAgencyNameComponent>,
    #[serde(rename = "ClassificationCategory")]
    pub classification_category: ClassificationSchemeArrayOfClassificationCategoryComponent,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ClassificationSchemeArrayOfDescriptionComponent>,
    #[serde(rename = "ID")]
    pub id: ClassificationSchemeArrayOfIDComponent,
    #[serde(rename = "LanguageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<ClassificationSchemeArrayOfLanguageIDComponent>,
    #[serde(rename = "LastRevisionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_revision_date: Option<ClassificationSchemeArrayOfLastRevisionDateComponent>,
    #[serde(rename = "LastRevisionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_revision_time: Option<ClassificationSchemeArrayOfLastRevisionTimeComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ClassificationSchemeArrayOfNameComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<ClassificationSchemeArrayOfNoteComponent>,
    #[serde(rename = "SchemeURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_uri: Option<ClassificationSchemeArrayOfSchemeURIComponent>,
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<ClassificationSchemeArrayOfURIComponent>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ClassificationSchemeArrayOfUUIDComponent>,
    #[serde(rename = "VersionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<ClassificationSchemeArrayOfVersionIDComponent>,
}

impl AsMut<ClassificationScheme> for ClassificationScheme {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationScheme> for ClassificationScheme {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.version_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.version_id", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ClassificationScheme.id", e));
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.description", e));
            }
        }
        if let Some(v) = &self.language_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.language_id", e));
            }
        }
        if let Some(v) = &self.uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.uri", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.note", e));
            }
        }
        if let Some(v) = &self.agency_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.agency_id", e));
            }
        }
        if let Some(v) = &self.agency_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.agency_name", e));
            }
        }
        if let Some(v) = &self.last_revision_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.last_revision_date", e));
            }
        }
        if let Err(e) = self.classification_category.validate() {
            return Err(UblError::component("ClassificationScheme.classification_category", e));
        }
        if let Some(v) = &self.last_revision_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.last_revision_time", e));
            }
        }
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.name", e));
            }
        }
        if let Some(v) = &self.scheme_uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.scheme_uri", e));
            }
        }
        if let Some(v) = &self.uuid {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ClassificationScheme.uuid", e));
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

impl ClassificationScheme {
    pub fn title() -> &'static str {
        "Classification Scheme. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a classification scheme, such as a taxonomy for classifying goods or services."
    }
    pub fn new(classification_category: ClassificationSchemeArrayOfClassificationCategoryComponent, id: ClassificationSchemeArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            agency_name: None,
            uri: None,
            agency_id: None,
            version_id: None,
            classification_category,
            language_id: None,
            note: None,
            last_revision_date: None,
            scheme_uri: None,
            uuid: None,
            description: None,
            id,
            last_revision_time: None,
            name: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationSchemeArrayOfAgencyIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AgencyID>,
}

impl AsMut<ClassificationSchemeArrayOfAgencyIDComponent> for ClassificationSchemeArrayOfAgencyIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfAgencyIDComponent> for ClassificationSchemeArrayOfAgencyIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfAgencyIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfAgencyIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfAgencyIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AgencyID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AgencyID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AgencyID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AgencyID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationSchemeArrayOfAgencyNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AgencyName>,
}

impl AsMut<ClassificationSchemeArrayOfAgencyNameComponent> for ClassificationSchemeArrayOfAgencyNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfAgencyNameComponent> for ClassificationSchemeArrayOfAgencyNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfAgencyNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfAgencyNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfAgencyNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AgencyName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AgencyName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AgencyName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AgencyName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationSchemeArrayOfClassificationCategoryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ClassificationCategory>,
}

impl AsMut<ClassificationSchemeArrayOfClassificationCategoryComponent> for ClassificationSchemeArrayOfClassificationCategoryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfClassificationCategoryComponent> for ClassificationSchemeArrayOfClassificationCategoryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfClassificationCategoryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfClassificationCategoryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ClassificationCategory) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ClassificationCategory) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ClassificationCategory> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ClassificationCategory> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationSchemeArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ClassificationSchemeArrayOfDescriptionComponent> for ClassificationSchemeArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfDescriptionComponent> for ClassificationSchemeArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfDescriptionComponent {
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
pub struct ClassificationSchemeArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ClassificationSchemeArrayOfIDComponent> for ClassificationSchemeArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfIDComponent> for ClassificationSchemeArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfIDComponent {
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
pub struct ClassificationSchemeArrayOfLanguageIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LanguageID>,
}

impl AsMut<ClassificationSchemeArrayOfLanguageIDComponent> for ClassificationSchemeArrayOfLanguageIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfLanguageIDComponent> for ClassificationSchemeArrayOfLanguageIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfLanguageIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfLanguageIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfLanguageIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LanguageID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LanguageID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LanguageID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LanguageID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationSchemeArrayOfLastRevisionDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LastRevisionDate>,
}

impl AsMut<ClassificationSchemeArrayOfLastRevisionDateComponent> for ClassificationSchemeArrayOfLastRevisionDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfLastRevisionDateComponent> for ClassificationSchemeArrayOfLastRevisionDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfLastRevisionDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfLastRevisionDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfLastRevisionDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LastRevisionDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LastRevisionDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LastRevisionDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LastRevisionDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationSchemeArrayOfLastRevisionTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LastRevisionTime>,
}

impl AsMut<ClassificationSchemeArrayOfLastRevisionTimeComponent> for ClassificationSchemeArrayOfLastRevisionTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfLastRevisionTimeComponent> for ClassificationSchemeArrayOfLastRevisionTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfLastRevisionTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfLastRevisionTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfLastRevisionTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LastRevisionTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LastRevisionTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LastRevisionTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LastRevisionTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationSchemeArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<ClassificationSchemeArrayOfNameComponent> for ClassificationSchemeArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfNameComponent> for ClassificationSchemeArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfNameComponent {
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
pub struct ClassificationSchemeArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<ClassificationSchemeArrayOfNoteComponent> for ClassificationSchemeArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfNoteComponent> for ClassificationSchemeArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfNoteComponent {
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
pub struct ClassificationSchemeArrayOfSchemeURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SchemeURI>,
}

impl AsMut<ClassificationSchemeArrayOfSchemeURIComponent> for ClassificationSchemeArrayOfSchemeURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfSchemeURIComponent> for ClassificationSchemeArrayOfSchemeURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfSchemeURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfSchemeURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfSchemeURIComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SchemeURI) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SchemeURI) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SchemeURI> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SchemeURI> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationSchemeArrayOfURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::URI>,
}

impl AsMut<ClassificationSchemeArrayOfURIComponent> for ClassificationSchemeArrayOfURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfURIComponent> for ClassificationSchemeArrayOfURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfURIComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationSchemeArrayOfUUIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UUID>,
}

impl AsMut<ClassificationSchemeArrayOfUUIDComponent> for ClassificationSchemeArrayOfUUIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfUUIDComponent> for ClassificationSchemeArrayOfUUIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfUUIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfUUIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfUUIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::UUID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::UUID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::UUID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::UUID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClassificationSchemeArrayOfVersionIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::VersionID>,
}

impl AsMut<ClassificationSchemeArrayOfVersionIDComponent> for ClassificationSchemeArrayOfVersionIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ClassificationSchemeArrayOfVersionIDComponent> for ClassificationSchemeArrayOfVersionIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfVersionIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ClassificationSchemeArrayOfVersionIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ClassificationSchemeArrayOfVersionIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::VersionID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::VersionID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::VersionID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::VersionID> {
        self.items.iter()
    }
}


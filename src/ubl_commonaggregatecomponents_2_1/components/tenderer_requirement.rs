use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TendererRequirement {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TendererRequirementArrayOfDescriptionComponent>,
    #[serde(rename = "LegalReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_reference: Option<TendererRequirementArrayOfLegalReferenceComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TendererRequirementArrayOfNameComponent>,
    #[serde(rename = "SuggestedEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_evidence: Option<TendererRequirementArrayOfSuggestedEvidenceComponent>,
    #[serde(rename = "TendererRequirementTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenderer_requirement_type_code: Option<TendererRequirementArrayOfTendererRequirementTypeCodeComponent>,
}

impl AsMut<TendererRequirement> for TendererRequirement {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererRequirement> for TendererRequirement {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererRequirement.name", e));
            }
        }
        if let Some(v) = &self.legal_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererRequirement.legal_reference", e));
            }
        }
        if let Some(v) = &self.tenderer_requirement_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererRequirement.tenderer_requirement_type_code", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererRequirement.description", e));
            }
        }
        if let Some(v) = &self.suggested_evidence {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererRequirement.suggested_evidence", e));
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

impl TendererRequirement {
    pub fn title() -> &'static str {
        "Tenderer Requirement. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an action or statement required of an economic operator participating in a tendering process."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            description: None,
            name: None,
            legal_reference: None,
            tenderer_requirement_type_code: None,
            suggested_evidence: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererRequirementArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<TendererRequirementArrayOfDescriptionComponent> for TendererRequirementArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererRequirementArrayOfDescriptionComponent> for TendererRequirementArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererRequirementArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererRequirementArrayOfDescriptionComponent {
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
pub struct TendererRequirementArrayOfLegalReferenceComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LegalReference>,
}

impl AsMut<TendererRequirementArrayOfLegalReferenceComponent> for TendererRequirementArrayOfLegalReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererRequirementArrayOfLegalReferenceComponent> for TendererRequirementArrayOfLegalReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TendererRequirementArrayOfLegalReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TendererRequirementArrayOfLegalReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererRequirementArrayOfLegalReferenceComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LegalReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LegalReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LegalReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LegalReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererRequirementArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<TendererRequirementArrayOfNameComponent> for TendererRequirementArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererRequirementArrayOfNameComponent> for TendererRequirementArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererRequirementArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererRequirementArrayOfNameComponent {
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
pub struct TendererRequirementArrayOfSuggestedEvidenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SuggestedEvidence>,
}

impl AsMut<TendererRequirementArrayOfSuggestedEvidenceComponent> for TendererRequirementArrayOfSuggestedEvidenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererRequirementArrayOfSuggestedEvidenceComponent> for TendererRequirementArrayOfSuggestedEvidenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererRequirementArrayOfSuggestedEvidenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererRequirementArrayOfSuggestedEvidenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SuggestedEvidence) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SuggestedEvidence) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SuggestedEvidence> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SuggestedEvidence> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererRequirementArrayOfTendererRequirementTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TendererRequirementTypeCode>,
}

impl AsMut<TendererRequirementArrayOfTendererRequirementTypeCodeComponent> for TendererRequirementArrayOfTendererRequirementTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererRequirementArrayOfTendererRequirementTypeCodeComponent> for TendererRequirementArrayOfTendererRequirementTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TendererRequirementArrayOfTendererRequirementTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TendererRequirementArrayOfTendererRequirementTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererRequirementArrayOfTendererRequirementTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TendererRequirementTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TendererRequirementTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TendererRequirementTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TendererRequirementTypeCode> {
        self.items.iter()
    }
}


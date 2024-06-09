use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractExecutionRequirement {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ContractExecutionRequirementArrayOfDescriptionComponent>,
    #[serde(rename = "ExecutionRequirementCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_requirement_code: Option<ContractExecutionRequirementArrayOfExecutionRequirementCodeComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ContractExecutionRequirementArrayOfNameComponent>,
}

impl AsMut<ContractExecutionRequirement> for ContractExecutionRequirement {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractExecutionRequirement> for ContractExecutionRequirement {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.execution_requirement_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractExecutionRequirement.execution_requirement_code", e));
            }
        }
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractExecutionRequirement.name", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractExecutionRequirement.description", e));
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

impl ContractExecutionRequirement {
    pub fn title() -> &'static str {
        "Contract Execution Requirement. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a requirement for execution of a contract."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            execution_requirement_code: None,
            description: None,
            name: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractExecutionRequirementArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ContractExecutionRequirementArrayOfDescriptionComponent> for ContractExecutionRequirementArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractExecutionRequirementArrayOfDescriptionComponent> for ContractExecutionRequirementArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ContractExecutionRequirementArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractExecutionRequirementArrayOfDescriptionComponent {
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
pub struct ContractExecutionRequirementArrayOfExecutionRequirementCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExecutionRequirementCode>,
}

impl AsMut<ContractExecutionRequirementArrayOfExecutionRequirementCodeComponent> for ContractExecutionRequirementArrayOfExecutionRequirementCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractExecutionRequirementArrayOfExecutionRequirementCodeComponent> for ContractExecutionRequirementArrayOfExecutionRequirementCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContractExecutionRequirementArrayOfExecutionRequirementCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContractExecutionRequirementArrayOfExecutionRequirementCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractExecutionRequirementArrayOfExecutionRequirementCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExecutionRequirementCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExecutionRequirementCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExecutionRequirementCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExecutionRequirementCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractExecutionRequirementArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<ContractExecutionRequirementArrayOfNameComponent> for ContractExecutionRequirementArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractExecutionRequirementArrayOfNameComponent> for ContractExecutionRequirementArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ContractExecutionRequirementArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractExecutionRequirementArrayOfNameComponent {
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


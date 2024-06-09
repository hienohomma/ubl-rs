use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractingActivity {
    #[serde(rename = "ActivityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ContractingActivityArrayOfActivityTypeComponent>,
    #[serde(rename = "ActivityTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type_code: Option<ContractingActivityArrayOfActivityTypeCodeComponent>,
}

impl AsMut<ContractingActivity> for ContractingActivity {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractingActivity> for ContractingActivity {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.activity_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractingActivity.activity_type_code", e));
            }
        }
        if let Some(v) = &self.activity_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractingActivity.activity_type", e));
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

impl ContractingActivity {
    pub fn title() -> &'static str {
        "Contracting Activity. Details"
    }
    pub fn description() -> &'static str {
        "The nature of the type of business of the organization."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            activity_type: None,
            activity_type_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractingActivityArrayOfActivityTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ActivityType>,
}

impl AsMut<ContractingActivityArrayOfActivityTypeComponent> for ContractingActivityArrayOfActivityTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractingActivityArrayOfActivityTypeComponent> for ContractingActivityArrayOfActivityTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContractingActivityArrayOfActivityTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContractingActivityArrayOfActivityTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractingActivityArrayOfActivityTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ActivityType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ActivityType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ActivityType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ActivityType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractingActivityArrayOfActivityTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ActivityTypeCode>,
}

impl AsMut<ContractingActivityArrayOfActivityTypeCodeComponent> for ContractingActivityArrayOfActivityTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractingActivityArrayOfActivityTypeCodeComponent> for ContractingActivityArrayOfActivityTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContractingActivityArrayOfActivityTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContractingActivityArrayOfActivityTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractingActivityArrayOfActivityTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ActivityTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ActivityTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ActivityTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ActivityTypeCode> {
        self.items.iter()
    }
}


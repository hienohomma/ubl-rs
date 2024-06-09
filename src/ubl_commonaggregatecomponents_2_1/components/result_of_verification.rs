use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResultOfVerification {
    #[serde(rename = "SignatoryParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatory_party: Option<ResultOfVerificationArrayOfSignatoryPartyComponent>,
    #[serde(rename = "ValidateProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_process: Option<ResultOfVerificationArrayOfValidateProcessComponent>,
    #[serde(rename = "ValidateTool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_tool: Option<ResultOfVerificationArrayOfValidateToolComponent>,
    #[serde(rename = "ValidateToolVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_tool_version: Option<ResultOfVerificationArrayOfValidateToolVersionComponent>,
    #[serde(rename = "ValidationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_date: Option<ResultOfVerificationArrayOfValidationDateComponent>,
    #[serde(rename = "ValidationResultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_result_code: Option<ResultOfVerificationArrayOfValidationResultCodeComponent>,
    #[serde(rename = "ValidationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_time: Option<ResultOfVerificationArrayOfValidationTimeComponent>,
    #[serde(rename = "ValidatorID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator_id: Option<ResultOfVerificationArrayOfValidatorIDComponent>,
}

impl AsMut<ResultOfVerification> for ResultOfVerification {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResultOfVerification> for ResultOfVerification {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.validator_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ResultOfVerification.validator_id", e));
            }
        }
        if let Some(v) = &self.validate_tool {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ResultOfVerification.validate_tool", e));
            }
        }
        if let Some(v) = &self.validation_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ResultOfVerification.validation_date", e));
            }
        }
        if let Some(v) = &self.validation_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ResultOfVerification.validation_time", e));
            }
        }
        if let Some(v) = &self.validation_result_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ResultOfVerification.validation_result_code", e));
            }
        }
        if let Some(v) = &self.signatory_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ResultOfVerification.signatory_party", e));
            }
        }
        if let Some(v) = &self.validate_tool_version {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ResultOfVerification.validate_tool_version", e));
            }
        }
        if let Some(v) = &self.validate_process {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ResultOfVerification.validate_process", e));
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

impl ResultOfVerification {
    pub fn title() -> &'static str {
        "Result Of Verification. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the result of an attempt to verify a signature."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            validation_time: None,
            validate_tool: None,
            validator_id: None,
            signatory_party: None,
            validate_process: None,
            validate_tool_version: None,
            validation_date: None,
            validation_result_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResultOfVerificationArrayOfSignatoryPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SignatoryParty>,
}

impl AsMut<ResultOfVerificationArrayOfSignatoryPartyComponent> for ResultOfVerificationArrayOfSignatoryPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResultOfVerificationArrayOfSignatoryPartyComponent> for ResultOfVerificationArrayOfSignatoryPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfSignatoryPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfSignatoryPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResultOfVerificationArrayOfSignatoryPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SignatoryParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SignatoryParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SignatoryParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SignatoryParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResultOfVerificationArrayOfValidateProcessComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValidateProcess>,
}

impl AsMut<ResultOfVerificationArrayOfValidateProcessComponent> for ResultOfVerificationArrayOfValidateProcessComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResultOfVerificationArrayOfValidateProcessComponent> for ResultOfVerificationArrayOfValidateProcessComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidateProcessComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidateProcessComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResultOfVerificationArrayOfValidateProcessComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValidateProcess) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValidateProcess) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValidateProcess> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValidateProcess> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResultOfVerificationArrayOfValidateToolComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValidateTool>,
}

impl AsMut<ResultOfVerificationArrayOfValidateToolComponent> for ResultOfVerificationArrayOfValidateToolComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResultOfVerificationArrayOfValidateToolComponent> for ResultOfVerificationArrayOfValidateToolComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidateToolComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidateToolComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResultOfVerificationArrayOfValidateToolComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValidateTool) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValidateTool) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValidateTool> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValidateTool> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResultOfVerificationArrayOfValidateToolVersionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValidateToolVersion>,
}

impl AsMut<ResultOfVerificationArrayOfValidateToolVersionComponent> for ResultOfVerificationArrayOfValidateToolVersionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResultOfVerificationArrayOfValidateToolVersionComponent> for ResultOfVerificationArrayOfValidateToolVersionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidateToolVersionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidateToolVersionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResultOfVerificationArrayOfValidateToolVersionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValidateToolVersion) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValidateToolVersion) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValidateToolVersion> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValidateToolVersion> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResultOfVerificationArrayOfValidationDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValidationDate>,
}

impl AsMut<ResultOfVerificationArrayOfValidationDateComponent> for ResultOfVerificationArrayOfValidationDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResultOfVerificationArrayOfValidationDateComponent> for ResultOfVerificationArrayOfValidationDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidationDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidationDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResultOfVerificationArrayOfValidationDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValidationDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValidationDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValidationDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValidationDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResultOfVerificationArrayOfValidationResultCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValidationResultCode>,
}

impl AsMut<ResultOfVerificationArrayOfValidationResultCodeComponent> for ResultOfVerificationArrayOfValidationResultCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResultOfVerificationArrayOfValidationResultCodeComponent> for ResultOfVerificationArrayOfValidationResultCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidationResultCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidationResultCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResultOfVerificationArrayOfValidationResultCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValidationResultCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValidationResultCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValidationResultCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValidationResultCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResultOfVerificationArrayOfValidationTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValidationTime>,
}

impl AsMut<ResultOfVerificationArrayOfValidationTimeComponent> for ResultOfVerificationArrayOfValidationTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResultOfVerificationArrayOfValidationTimeComponent> for ResultOfVerificationArrayOfValidationTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidationTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidationTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResultOfVerificationArrayOfValidationTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValidationTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValidationTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValidationTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValidationTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ResultOfVerificationArrayOfValidatorIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValidatorID>,
}

impl AsMut<ResultOfVerificationArrayOfValidatorIDComponent> for ResultOfVerificationArrayOfValidatorIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ResultOfVerificationArrayOfValidatorIDComponent> for ResultOfVerificationArrayOfValidatorIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidatorIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ResultOfVerificationArrayOfValidatorIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ResultOfVerificationArrayOfValidatorIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValidatorID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValidatorID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValidatorID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValidatorID> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractExtension {
    #[serde(rename = "MaximumNumberNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_number_numeric: Option<ContractExtensionArrayOfMaximumNumberNumericComponent>,
    #[serde(rename = "MinimumNumberNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_number_numeric: Option<ContractExtensionArrayOfMinimumNumberNumericComponent>,
    #[serde(rename = "OptionValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_validity_period: Option<ContractExtensionArrayOfOptionValidityPeriodComponent>,
    #[serde(rename = "OptionsDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_description: Option<ContractExtensionArrayOfOptionsDescriptionComponent>,
    #[serde(rename = "Renewal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal: Option<ContractExtensionArrayOfRenewalComponent>,
}

impl AsMut<ContractExtension> for ContractExtension {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractExtension> for ContractExtension {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.renewal {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractExtension.renewal", e));
            }
        }
        if let Some(v) = &self.option_validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractExtension.option_validity_period", e));
            }
        }
        if let Some(v) = &self.minimum_number_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractExtension.minimum_number_numeric", e));
            }
        }
        if let Some(v) = &self.maximum_number_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractExtension.maximum_number_numeric", e));
            }
        }
        if let Some(v) = &self.options_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ContractExtension.options_description", e));
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

impl ContractExtension {
    pub fn title() -> &'static str {
        "Contract Extension. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe possible extensions to a contract."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            options_description: None,
            minimum_number_numeric: None,
            maximum_number_numeric: None,
            option_validity_period: None,
            renewal: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractExtensionArrayOfMaximumNumberNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumNumberNumeric>,
}

impl AsMut<ContractExtensionArrayOfMaximumNumberNumericComponent> for ContractExtensionArrayOfMaximumNumberNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractExtensionArrayOfMaximumNumberNumericComponent> for ContractExtensionArrayOfMaximumNumberNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContractExtensionArrayOfMaximumNumberNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContractExtensionArrayOfMaximumNumberNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractExtensionArrayOfMaximumNumberNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumNumberNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumNumberNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumNumberNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumNumberNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractExtensionArrayOfMinimumNumberNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumNumberNumeric>,
}

impl AsMut<ContractExtensionArrayOfMinimumNumberNumericComponent> for ContractExtensionArrayOfMinimumNumberNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractExtensionArrayOfMinimumNumberNumericComponent> for ContractExtensionArrayOfMinimumNumberNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContractExtensionArrayOfMinimumNumberNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContractExtensionArrayOfMinimumNumberNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractExtensionArrayOfMinimumNumberNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumNumberNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumNumberNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumNumberNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumNumberNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractExtensionArrayOfOptionValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OptionValidityPeriod>,
}

impl AsMut<ContractExtensionArrayOfOptionValidityPeriodComponent> for ContractExtensionArrayOfOptionValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractExtensionArrayOfOptionValidityPeriodComponent> for ContractExtensionArrayOfOptionValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ContractExtensionArrayOfOptionValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ContractExtensionArrayOfOptionValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractExtensionArrayOfOptionValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OptionValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OptionValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OptionValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OptionValidityPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractExtensionArrayOfOptionsDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OptionsDescription>,
}

impl AsMut<ContractExtensionArrayOfOptionsDescriptionComponent> for ContractExtensionArrayOfOptionsDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractExtensionArrayOfOptionsDescriptionComponent> for ContractExtensionArrayOfOptionsDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ContractExtensionArrayOfOptionsDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractExtensionArrayOfOptionsDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OptionsDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OptionsDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OptionsDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OptionsDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContractExtensionArrayOfRenewalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Renewal>,
}

impl AsMut<ContractExtensionArrayOfRenewalComponent> for ContractExtensionArrayOfRenewalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ContractExtensionArrayOfRenewalComponent> for ContractExtensionArrayOfRenewalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ContractExtensionArrayOfRenewalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ContractExtensionArrayOfRenewalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Renewal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Renewal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Renewal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Renewal> {
        self.items.iter()
    }
}


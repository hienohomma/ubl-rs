use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Capability {
    #[serde(rename = "CapabilityTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_type_code: Option<CapabilityArrayOfCapabilityTypeCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CapabilityArrayOfDescriptionComponent>,
    #[serde(rename = "EvidenceSupplied")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_supplied: Option<CapabilityArrayOfEvidenceSuppliedComponent>,
    #[serde(rename = "ValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<CapabilityArrayOfValidityPeriodComponent>,
    #[serde(rename = "ValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_amount: Option<CapabilityArrayOfValueAmountComponent>,
    #[serde(rename = "ValueQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_quantity: Option<CapabilityArrayOfValueQuantityComponent>,
}

impl AsMut<Capability> for Capability {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Capability> for Capability {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Capability.description", e));
            }
        }
        if let Some(v) = &self.evidence_supplied {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Capability.evidence_supplied", e));
            }
        }
        if let Some(v) = &self.capability_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Capability.capability_type_code", e));
            }
        }
        if let Some(v) = &self.validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Capability.validity_period", e));
            }
        }
        if let Some(v) = &self.value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Capability.value_amount", e));
            }
        }
        if let Some(v) = &self.value_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Capability.value_quantity", e));
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

impl Capability {
    pub fn title() -> &'static str {
        "Capability. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a specific capability of an organization."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            value_quantity: None,
            capability_type_code: None,
            validity_period: None,
            description: None,
            evidence_supplied: None,
            value_amount: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CapabilityArrayOfCapabilityTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CapabilityTypeCode>,
}

impl AsMut<CapabilityArrayOfCapabilityTypeCodeComponent> for CapabilityArrayOfCapabilityTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CapabilityArrayOfCapabilityTypeCodeComponent> for CapabilityArrayOfCapabilityTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CapabilityArrayOfCapabilityTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CapabilityArrayOfCapabilityTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CapabilityArrayOfCapabilityTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CapabilityTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CapabilityTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CapabilityTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CapabilityTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CapabilityArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<CapabilityArrayOfDescriptionComponent> for CapabilityArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CapabilityArrayOfDescriptionComponent> for CapabilityArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CapabilityArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CapabilityArrayOfDescriptionComponent {
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
pub struct CapabilityArrayOfEvidenceSuppliedComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied>,
}

impl AsMut<CapabilityArrayOfEvidenceSuppliedComponent> for CapabilityArrayOfEvidenceSuppliedComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CapabilityArrayOfEvidenceSuppliedComponent> for CapabilityArrayOfEvidenceSuppliedComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CapabilityArrayOfEvidenceSuppliedComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CapabilityArrayOfEvidenceSuppliedComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CapabilityArrayOfValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod>,
}

impl AsMut<CapabilityArrayOfValidityPeriodComponent> for CapabilityArrayOfValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CapabilityArrayOfValidityPeriodComponent> for CapabilityArrayOfValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CapabilityArrayOfValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CapabilityArrayOfValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CapabilityArrayOfValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CapabilityArrayOfValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValueAmount>,
}

impl AsMut<CapabilityArrayOfValueAmountComponent> for CapabilityArrayOfValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CapabilityArrayOfValueAmountComponent> for CapabilityArrayOfValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CapabilityArrayOfValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CapabilityArrayOfValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CapabilityArrayOfValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CapabilityArrayOfValueQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValueQuantity>,
}

impl AsMut<CapabilityArrayOfValueQuantityComponent> for CapabilityArrayOfValueQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CapabilityArrayOfValueQuantityComponent> for CapabilityArrayOfValueQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CapabilityArrayOfValueQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CapabilityArrayOfValueQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CapabilityArrayOfValueQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValueQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValueQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValueQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValueQuantity> {
        self.items.iter()
    }
}


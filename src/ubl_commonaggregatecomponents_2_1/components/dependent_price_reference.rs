use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DependentPriceReference {
    #[serde(rename = "DependentLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_line_reference: Option<DependentPriceReferenceArrayOfDependentLineReferenceComponent>,
    #[serde(rename = "LocationAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_address: Option<DependentPriceReferenceArrayOfLocationAddressComponent>,
    #[serde(rename = "Percent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<DependentPriceReferenceArrayOfPercentComponent>,
}

impl AsMut<DependentPriceReference> for DependentPriceReference {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DependentPriceReference> for DependentPriceReference {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.location_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DependentPriceReference.location_address", e));
            }
        }
        if let Some(v) = &self.percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DependentPriceReference.percent", e));
            }
        }
        if let Some(v) = &self.dependent_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DependentPriceReference.dependent_line_reference", e));
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

impl DependentPriceReference {
    pub fn title() -> &'static str {
        "Dependent Price Reference. Details"
    }
    pub fn description() -> &'static str {
        "A class to define the price of an item as a percentage of the price of a different item."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            percent: None,
            location_address: None,
            dependent_line_reference: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DependentPriceReferenceArrayOfDependentLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DependentLineReference>,
}

impl AsMut<DependentPriceReferenceArrayOfDependentLineReferenceComponent> for DependentPriceReferenceArrayOfDependentLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DependentPriceReferenceArrayOfDependentLineReferenceComponent> for DependentPriceReferenceArrayOfDependentLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DependentPriceReferenceArrayOfDependentLineReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DependentPriceReferenceArrayOfDependentLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DependentPriceReferenceArrayOfDependentLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DependentLineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DependentLineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DependentLineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DependentLineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DependentPriceReferenceArrayOfLocationAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LocationAddress>,
}

impl AsMut<DependentPriceReferenceArrayOfLocationAddressComponent> for DependentPriceReferenceArrayOfLocationAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DependentPriceReferenceArrayOfLocationAddressComponent> for DependentPriceReferenceArrayOfLocationAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DependentPriceReferenceArrayOfLocationAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DependentPriceReferenceArrayOfLocationAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DependentPriceReferenceArrayOfLocationAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LocationAddress) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LocationAddress) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LocationAddress> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LocationAddress> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DependentPriceReferenceArrayOfPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Percent>,
}

impl AsMut<DependentPriceReferenceArrayOfPercentComponent> for DependentPriceReferenceArrayOfPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DependentPriceReferenceArrayOfPercentComponent> for DependentPriceReferenceArrayOfPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DependentPriceReferenceArrayOfPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DependentPriceReferenceArrayOfPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DependentPriceReferenceArrayOfPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Percent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Percent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Percent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Percent> {
        self.items.iter()
    }
}


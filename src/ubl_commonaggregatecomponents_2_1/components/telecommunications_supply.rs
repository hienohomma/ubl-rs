use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TelecommunicationsSupply {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TelecommunicationsSupplyArrayOfDescriptionComponent>,
    #[serde(rename = "PrivacyCode")]
    pub privacy_code: TelecommunicationsSupplyArrayOfPrivacyCodeComponent,
    #[serde(rename = "TelecommunicationsSupplyLine")]
    pub telecommunications_supply_line: TelecommunicationsSupplyArrayOfTelecommunicationsSupplyLineComponent,
    #[serde(rename = "TelecommunicationsSupplyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecommunications_supply_type: Option<TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeComponent>,
    #[serde(rename = "TelecommunicationsSupplyTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecommunications_supply_type_code: Option<TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeCodeComponent>,
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<TelecommunicationsSupplyArrayOfTotalAmountComponent>,
}

impl AsMut<TelecommunicationsSupply> for TelecommunicationsSupply {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupply> for TelecommunicationsSupply {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.privacy_code.validate() {
            return Err(UblError::component("TelecommunicationsSupply.privacy_code", e));
        }
        if let Err(e) = self.telecommunications_supply_line.validate() {
            return Err(UblError::component("TelecommunicationsSupply.telecommunications_supply_line", e));
        }
        if let Some(v) = &self.telecommunications_supply_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsSupply.telecommunications_supply_type", e));
            }
        }
        if let Some(v) = &self.telecommunications_supply_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsSupply.telecommunications_supply_type_code", e));
            }
        }
        if let Some(v) = &self.total_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsSupply.total_amount", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsSupply.description", e));
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

impl TelecommunicationsSupply {
    pub fn title() -> &'static str {
        "Telecommunications Supply. Details"
    }
    pub fn description() -> &'static str {
        "A class describing the supply of a telecommunication service, e.g., providing telephone calls."
    }
    pub fn new(privacy_code: TelecommunicationsSupplyArrayOfPrivacyCodeComponent, telecommunications_supply_line: TelecommunicationsSupplyArrayOfTelecommunicationsSupplyLineComponent) -> Component<Self> {
        Component(Self {
            total_amount: None,
            telecommunications_supply_line,
            description: None,
            privacy_code,
            telecommunications_supply_type: None,
            telecommunications_supply_type_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsSupplyArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<TelecommunicationsSupplyArrayOfDescriptionComponent> for TelecommunicationsSupplyArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyArrayOfDescriptionComponent> for TelecommunicationsSupplyArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyArrayOfDescriptionComponent {
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
pub struct TelecommunicationsSupplyArrayOfPrivacyCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PrivacyCode>,
}

impl AsMut<TelecommunicationsSupplyArrayOfPrivacyCodeComponent> for TelecommunicationsSupplyArrayOfPrivacyCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyArrayOfPrivacyCodeComponent> for TelecommunicationsSupplyArrayOfPrivacyCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyArrayOfPrivacyCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyArrayOfPrivacyCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyArrayOfPrivacyCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PrivacyCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PrivacyCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PrivacyCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PrivacyCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsSupplyArrayOfTelecommunicationsSupplyLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsSupplyLine>,
}

impl AsMut<TelecommunicationsSupplyArrayOfTelecommunicationsSupplyLineComponent> for TelecommunicationsSupplyArrayOfTelecommunicationsSupplyLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyArrayOfTelecommunicationsSupplyLineComponent> for TelecommunicationsSupplyArrayOfTelecommunicationsSupplyLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyArrayOfTelecommunicationsSupplyLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyArrayOfTelecommunicationsSupplyLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsSupplyLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsSupplyLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsSupplyLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsSupplyLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsSupplyType>,
}

impl AsMut<TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeComponent> for TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeComponent> for TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsSupplyType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsSupplyType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsSupplyType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsSupplyType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsSupplyTypeCode>,
}

impl AsMut<TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeCodeComponent> for TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeCodeComponent> for TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyArrayOfTelecommunicationsSupplyTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsSupplyTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsSupplyTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsSupplyTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsSupplyTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsSupplyArrayOfTotalAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalAmount>,
}

impl AsMut<TelecommunicationsSupplyArrayOfTotalAmountComponent> for TelecommunicationsSupplyArrayOfTotalAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyArrayOfTotalAmountComponent> for TelecommunicationsSupplyArrayOfTotalAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyArrayOfTotalAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyArrayOfTotalAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyArrayOfTotalAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalAmount> {
        self.items.iter()
    }
}


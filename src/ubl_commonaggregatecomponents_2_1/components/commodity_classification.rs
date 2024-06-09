use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommodityClassification {
    #[serde(rename = "CargoTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_type_code: Option<CommodityClassificationArrayOfCargoTypeCodeComponent>,
    #[serde(rename = "CommodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<CommodityClassificationArrayOfCommodityCodeComponent>,
    #[serde(rename = "ItemClassificationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_classification_code: Option<CommodityClassificationArrayOfItemClassificationCodeComponent>,
    #[serde(rename = "NatureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nature_code: Option<CommodityClassificationArrayOfNatureCodeComponent>,
}

impl AsMut<CommodityClassification> for CommodityClassification {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CommodityClassification> for CommodityClassification {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.cargo_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CommodityClassification.cargo_type_code", e));
            }
        }
        if let Some(v) = &self.item_classification_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CommodityClassification.item_classification_code", e));
            }
        }
        if let Some(v) = &self.commodity_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CommodityClassification.commodity_code", e));
            }
        }
        if let Some(v) = &self.nature_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CommodityClassification.nature_code", e));
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

impl CommodityClassification {
    pub fn title() -> &'static str {
        "Commodity Classification. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the classification of a commodity."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            nature_code: None,
            item_classification_code: None,
            cargo_type_code: None,
            commodity_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CommodityClassificationArrayOfCargoTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CargoTypeCode>,
}

impl AsMut<CommodityClassificationArrayOfCargoTypeCodeComponent> for CommodityClassificationArrayOfCargoTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CommodityClassificationArrayOfCargoTypeCodeComponent> for CommodityClassificationArrayOfCargoTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CommodityClassificationArrayOfCargoTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CommodityClassificationArrayOfCargoTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CommodityClassificationArrayOfCargoTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CargoTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CargoTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CargoTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CargoTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CommodityClassificationArrayOfCommodityCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CommodityCode>,
}

impl AsMut<CommodityClassificationArrayOfCommodityCodeComponent> for CommodityClassificationArrayOfCommodityCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CommodityClassificationArrayOfCommodityCodeComponent> for CommodityClassificationArrayOfCommodityCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CommodityClassificationArrayOfCommodityCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CommodityClassificationArrayOfCommodityCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CommodityClassificationArrayOfCommodityCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CommodityCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CommodityCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CommodityCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CommodityCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CommodityClassificationArrayOfItemClassificationCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ItemClassificationCode>,
}

impl AsMut<CommodityClassificationArrayOfItemClassificationCodeComponent> for CommodityClassificationArrayOfItemClassificationCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CommodityClassificationArrayOfItemClassificationCodeComponent> for CommodityClassificationArrayOfItemClassificationCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CommodityClassificationArrayOfItemClassificationCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CommodityClassificationArrayOfItemClassificationCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CommodityClassificationArrayOfItemClassificationCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ItemClassificationCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ItemClassificationCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ItemClassificationCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ItemClassificationCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CommodityClassificationArrayOfNatureCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NatureCode>,
}

impl AsMut<CommodityClassificationArrayOfNatureCodeComponent> for CommodityClassificationArrayOfNatureCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CommodityClassificationArrayOfNatureCodeComponent> for CommodityClassificationArrayOfNatureCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CommodityClassificationArrayOfNatureCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CommodityClassificationArrayOfNatureCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CommodityClassificationArrayOfNatureCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NatureCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NatureCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NatureCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NatureCode> {
        self.items.iter()
    }
}


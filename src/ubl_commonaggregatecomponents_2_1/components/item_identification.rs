use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemIdentification {
    #[serde(rename = "BarcodeSymbologyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode_symbology_id: Option<ItemIdentificationArrayOfBarcodeSymbologyIDComponent>,
    #[serde(rename = "ExtendedID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_id: Option<ItemIdentificationArrayOfExtendedIDComponent>,
    #[serde(rename = "ID")]
    pub id: ItemIdentificationArrayOfIDComponent,
    #[serde(rename = "IssuerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_party: Option<ItemIdentificationArrayOfIssuerPartyComponent>,
    #[serde(rename = "MeasurementDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_dimension: Option<ItemIdentificationArrayOfMeasurementDimensionComponent>,
    #[serde(rename = "PhysicalAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_attribute: Option<ItemIdentificationArrayOfPhysicalAttributeComponent>,
}

impl AsMut<ItemIdentification> for ItemIdentification {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemIdentification> for ItemIdentification {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.extended_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemIdentification.extended_id", e));
            }
        }
        if let Some(v) = &self.physical_attribute {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemIdentification.physical_attribute", e));
            }
        }
        if let Some(v) = &self.barcode_symbology_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemIdentification.barcode_symbology_id", e));
            }
        }
        if let Some(v) = &self.issuer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemIdentification.issuer_party", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ItemIdentification.id", e));
        }
        if let Some(v) = &self.measurement_dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemIdentification.measurement_dimension", e));
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

impl ItemIdentification {
    pub fn title() -> &'static str {
        "Item Identification. Details"
    }
    pub fn description() -> &'static str {
        "A class for assigning identifying information to an item."
    }
    pub fn new(id: ItemIdentificationArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            issuer_party: None,
            extended_id: None,
            barcode_symbology_id: None,
            measurement_dimension: None,
            id,
            physical_attribute: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemIdentificationArrayOfBarcodeSymbologyIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BarcodeSymbologyID>,
}

impl AsMut<ItemIdentificationArrayOfBarcodeSymbologyIDComponent> for ItemIdentificationArrayOfBarcodeSymbologyIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemIdentificationArrayOfBarcodeSymbologyIDComponent> for ItemIdentificationArrayOfBarcodeSymbologyIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemIdentificationArrayOfBarcodeSymbologyIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemIdentificationArrayOfBarcodeSymbologyIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemIdentificationArrayOfBarcodeSymbologyIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BarcodeSymbologyID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BarcodeSymbologyID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BarcodeSymbologyID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BarcodeSymbologyID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemIdentificationArrayOfExtendedIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ExtendedID>,
}

impl AsMut<ItemIdentificationArrayOfExtendedIDComponent> for ItemIdentificationArrayOfExtendedIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemIdentificationArrayOfExtendedIDComponent> for ItemIdentificationArrayOfExtendedIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemIdentificationArrayOfExtendedIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemIdentificationArrayOfExtendedIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemIdentificationArrayOfExtendedIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ExtendedID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ExtendedID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ExtendedID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ExtendedID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemIdentificationArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ItemIdentificationArrayOfIDComponent> for ItemIdentificationArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemIdentificationArrayOfIDComponent> for ItemIdentificationArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemIdentificationArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemIdentificationArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemIdentificationArrayOfIDComponent {
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
pub struct ItemIdentificationArrayOfIssuerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::IssuerParty>,
}

impl AsMut<ItemIdentificationArrayOfIssuerPartyComponent> for ItemIdentificationArrayOfIssuerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemIdentificationArrayOfIssuerPartyComponent> for ItemIdentificationArrayOfIssuerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemIdentificationArrayOfIssuerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemIdentificationArrayOfIssuerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemIdentificationArrayOfIssuerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::IssuerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::IssuerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::IssuerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::IssuerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemIdentificationArrayOfMeasurementDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension>,
}

impl AsMut<ItemIdentificationArrayOfMeasurementDimensionComponent> for ItemIdentificationArrayOfMeasurementDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemIdentificationArrayOfMeasurementDimensionComponent> for ItemIdentificationArrayOfMeasurementDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemIdentificationArrayOfMeasurementDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemIdentificationArrayOfMeasurementDimensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemIdentificationArrayOfPhysicalAttributeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PhysicalAttribute>,
}

impl AsMut<ItemIdentificationArrayOfPhysicalAttributeComponent> for ItemIdentificationArrayOfPhysicalAttributeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemIdentificationArrayOfPhysicalAttributeComponent> for ItemIdentificationArrayOfPhysicalAttributeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemIdentificationArrayOfPhysicalAttributeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemIdentificationArrayOfPhysicalAttributeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PhysicalAttribute) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PhysicalAttribute) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PhysicalAttribute> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PhysicalAttribute> {
        self.items.iter()
    }
}


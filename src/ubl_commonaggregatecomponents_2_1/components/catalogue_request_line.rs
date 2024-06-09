use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatalogueRequestLine {
    #[serde(rename = "ContractSubdivision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_subdivision: Option<CatalogueRequestLineArrayOfContractSubdivisionComponent>,
    #[serde(rename = "ID")]
    pub id: CatalogueRequestLineArrayOfIDComponent,
    #[serde(rename = "Item")]
    pub item: CatalogueRequestLineArrayOfItemComponent,
    #[serde(rename = "LineValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_validity_period: Option<CatalogueRequestLineArrayOfLineValidityPeriodComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<CatalogueRequestLineArrayOfNoteComponent>,
    #[serde(rename = "RequiredItemLocationQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_item_location_quantity: Option<CatalogueRequestLineArrayOfRequiredItemLocationQuantityComponent>,
}

impl AsMut<CatalogueRequestLine> for CatalogueRequestLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueRequestLine> for CatalogueRequestLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.contract_subdivision {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueRequestLine.contract_subdivision", e));
            }
        }
        if let Some(v) = &self.line_validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueRequestLine.line_validity_period", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("CatalogueRequestLine.id", e));
        }
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("CatalogueRequestLine.item", e));
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueRequestLine.note", e));
            }
        }
        if let Some(v) = &self.required_item_location_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueRequestLine.required_item_location_quantity", e));
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

impl CatalogueRequestLine {
    pub fn title() -> &'static str {
        "Catalogue Request Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line describing a request for a catalogue line."
    }
    pub fn new(id: CatalogueRequestLineArrayOfIDComponent, item: CatalogueRequestLineArrayOfItemComponent) -> Component<Self> {
        Component(Self {
            required_item_location_quantity: None,
            id,
            contract_subdivision: None,
            item,
            line_validity_period: None,
            note: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueRequestLineArrayOfContractSubdivisionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ContractSubdivision>,
}

impl AsMut<CatalogueRequestLineArrayOfContractSubdivisionComponent> for CatalogueRequestLineArrayOfContractSubdivisionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueRequestLineArrayOfContractSubdivisionComponent> for CatalogueRequestLineArrayOfContractSubdivisionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueRequestLineArrayOfContractSubdivisionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueRequestLineArrayOfContractSubdivisionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueRequestLineArrayOfContractSubdivisionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ContractSubdivision) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ContractSubdivision) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ContractSubdivision> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ContractSubdivision> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueRequestLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<CatalogueRequestLineArrayOfIDComponent> for CatalogueRequestLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueRequestLineArrayOfIDComponent> for CatalogueRequestLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueRequestLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueRequestLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueRequestLineArrayOfIDComponent {
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
pub struct CatalogueRequestLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<CatalogueRequestLineArrayOfItemComponent> for CatalogueRequestLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueRequestLineArrayOfItemComponent> for CatalogueRequestLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueRequestLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueRequestLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueRequestLineArrayOfItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Item) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Item) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueRequestLineArrayOfLineValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LineValidityPeriod>,
}

impl AsMut<CatalogueRequestLineArrayOfLineValidityPeriodComponent> for CatalogueRequestLineArrayOfLineValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueRequestLineArrayOfLineValidityPeriodComponent> for CatalogueRequestLineArrayOfLineValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueRequestLineArrayOfLineValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueRequestLineArrayOfLineValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueRequestLineArrayOfLineValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LineValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LineValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LineValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LineValidityPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueRequestLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<CatalogueRequestLineArrayOfNoteComponent> for CatalogueRequestLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueRequestLineArrayOfNoteComponent> for CatalogueRequestLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueRequestLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueRequestLineArrayOfNoteComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Note) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Note) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueRequestLineArrayOfRequiredItemLocationQuantityComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity>,
}

impl AsMut<CatalogueRequestLineArrayOfRequiredItemLocationQuantityComponent> for CatalogueRequestLineArrayOfRequiredItemLocationQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueRequestLineArrayOfRequiredItemLocationQuantityComponent> for CatalogueRequestLineArrayOfRequiredItemLocationQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueRequestLineArrayOfRequiredItemLocationQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueRequestLineArrayOfRequiredItemLocationQuantityComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity> {
        self.items.iter()
    }
}


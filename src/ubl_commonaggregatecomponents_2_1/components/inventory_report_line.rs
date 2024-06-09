use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InventoryReportLine {
    #[serde(rename = "AvailabilityDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_date: Option<InventoryReportLineArrayOfAvailabilityDateComponent>,
    #[serde(rename = "AvailabilityStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status_code: Option<InventoryReportLineArrayOfAvailabilityStatusCodeComponent>,
    #[serde(rename = "ID")]
    pub id: InventoryReportLineArrayOfIDComponent,
    #[serde(rename = "InventoryLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_location: Option<InventoryReportLineArrayOfInventoryLocationComponent>,
    #[serde(rename = "InventoryValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_value_amount: Option<InventoryReportLineArrayOfInventoryValueAmountComponent>,
    #[serde(rename = "Item")]
    pub item: InventoryReportLineArrayOfItemComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<InventoryReportLineArrayOfNoteComponent>,
    #[serde(rename = "Quantity")]
    pub quantity: InventoryReportLineArrayOfQuantityComponent,
}

impl AsMut<InventoryReportLine> for InventoryReportLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InventoryReportLine> for InventoryReportLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.inventory_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InventoryReportLine.inventory_value_amount", e));
            }
        }
        if let Some(v) = &self.availability_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InventoryReportLine.availability_date", e));
            }
        }
        if let Err(e) = self.quantity.validate() {
            return Err(UblError::component("InventoryReportLine.quantity", e));
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("InventoryReportLine.id", e));
        }
        if let Some(v) = &self.availability_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InventoryReportLine.availability_status_code", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InventoryReportLine.note", e));
            }
        }
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("InventoryReportLine.item", e));
        }
        if let Some(v) = &self.inventory_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("InventoryReportLine.inventory_location", e));
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

impl InventoryReportLine {
    pub fn title() -> &'static str {
        "Inventory Report Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in an Inventory Report."
    }
    pub fn new(id: InventoryReportLineArrayOfIDComponent, item: InventoryReportLineArrayOfItemComponent, quantity: InventoryReportLineArrayOfQuantityComponent) -> Component<Self> {
        Component(Self {
            inventory_value_amount: None,
            note: None,
            id,
            availability_status_code: None,
            quantity,
            availability_date: None,
            inventory_location: None,
            item,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InventoryReportLineArrayOfAvailabilityDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AvailabilityDate>,
}

impl AsMut<InventoryReportLineArrayOfAvailabilityDateComponent> for InventoryReportLineArrayOfAvailabilityDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InventoryReportLineArrayOfAvailabilityDateComponent> for InventoryReportLineArrayOfAvailabilityDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfAvailabilityDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfAvailabilityDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InventoryReportLineArrayOfAvailabilityDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AvailabilityDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AvailabilityDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AvailabilityDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AvailabilityDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InventoryReportLineArrayOfAvailabilityStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AvailabilityStatusCode>,
}

impl AsMut<InventoryReportLineArrayOfAvailabilityStatusCodeComponent> for InventoryReportLineArrayOfAvailabilityStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InventoryReportLineArrayOfAvailabilityStatusCodeComponent> for InventoryReportLineArrayOfAvailabilityStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfAvailabilityStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfAvailabilityStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InventoryReportLineArrayOfAvailabilityStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AvailabilityStatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AvailabilityStatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AvailabilityStatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AvailabilityStatusCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InventoryReportLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<InventoryReportLineArrayOfIDComponent> for InventoryReportLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InventoryReportLineArrayOfIDComponent> for InventoryReportLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InventoryReportLineArrayOfIDComponent {
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
pub struct InventoryReportLineArrayOfInventoryLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::InventoryLocation>,
}

impl AsMut<InventoryReportLineArrayOfInventoryLocationComponent> for InventoryReportLineArrayOfInventoryLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InventoryReportLineArrayOfInventoryLocationComponent> for InventoryReportLineArrayOfInventoryLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfInventoryLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfInventoryLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InventoryReportLineArrayOfInventoryLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::InventoryLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::InventoryLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::InventoryLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::InventoryLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InventoryReportLineArrayOfInventoryValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InventoryValueAmount>,
}

impl AsMut<InventoryReportLineArrayOfInventoryValueAmountComponent> for InventoryReportLineArrayOfInventoryValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InventoryReportLineArrayOfInventoryValueAmountComponent> for InventoryReportLineArrayOfInventoryValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfInventoryValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfInventoryValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InventoryReportLineArrayOfInventoryValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InventoryValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InventoryValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InventoryValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InventoryValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InventoryReportLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<InventoryReportLineArrayOfItemComponent> for InventoryReportLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InventoryReportLineArrayOfItemComponent> for InventoryReportLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InventoryReportLineArrayOfItemComponent {
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
pub struct InventoryReportLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<InventoryReportLineArrayOfNoteComponent> for InventoryReportLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InventoryReportLineArrayOfNoteComponent> for InventoryReportLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InventoryReportLineArrayOfNoteComponent {
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
pub struct InventoryReportLineArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<InventoryReportLineArrayOfQuantityComponent> for InventoryReportLineArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<InventoryReportLineArrayOfQuantityComponent> for InventoryReportLineArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("InventoryReportLineArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl InventoryReportLineArrayOfQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Quantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Quantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter()
    }
}


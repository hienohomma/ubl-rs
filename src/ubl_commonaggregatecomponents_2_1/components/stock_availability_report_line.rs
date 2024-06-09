use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StockAvailabilityReportLine {
    #[serde(rename = "AvailabilityDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_date: Option<StockAvailabilityReportLineArrayOfAvailabilityDateComponent>,
    #[serde(rename = "AvailabilityStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status_code: Option<StockAvailabilityReportLineArrayOfAvailabilityStatusCodeComponent>,
    #[serde(rename = "ID")]
    pub id: StockAvailabilityReportLineArrayOfIDComponent,
    #[serde(rename = "Item")]
    pub item: StockAvailabilityReportLineArrayOfItemComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<StockAvailabilityReportLineArrayOfNoteComponent>,
    #[serde(rename = "Quantity")]
    pub quantity: StockAvailabilityReportLineArrayOfQuantityComponent,
    #[serde(rename = "ValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_amount: Option<StockAvailabilityReportLineArrayOfValueAmountComponent>,
}

impl AsMut<StockAvailabilityReportLine> for StockAvailabilityReportLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StockAvailabilityReportLine> for StockAvailabilityReportLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.quantity.validate() {
            return Err(UblError::component("StockAvailabilityReportLine.quantity", e));
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("StockAvailabilityReportLine.id", e));
        }
        if let Some(v) = &self.value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StockAvailabilityReportLine.value_amount", e));
            }
        }
        if let Some(v) = &self.availability_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StockAvailabilityReportLine.availability_date", e));
            }
        }
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("StockAvailabilityReportLine.item", e));
        }
        if let Some(v) = &self.availability_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StockAvailabilityReportLine.availability_status_code", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("StockAvailabilityReportLine.note", e));
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

impl StockAvailabilityReportLine {
    pub fn title() -> &'static str {
        "Stock Availability Report Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Stock Availability Report describing the availability of an item of sale."
    }
    pub fn new(id: StockAvailabilityReportLineArrayOfIDComponent, item: StockAvailabilityReportLineArrayOfItemComponent, quantity: StockAvailabilityReportLineArrayOfQuantityComponent) -> Component<Self> {
        Component(Self {
            availability_date: None,
            id,
            item,
            quantity,
            value_amount: None,
            availability_status_code: None,
            note: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StockAvailabilityReportLineArrayOfAvailabilityDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AvailabilityDate>,
}

impl AsMut<StockAvailabilityReportLineArrayOfAvailabilityDateComponent> for StockAvailabilityReportLineArrayOfAvailabilityDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StockAvailabilityReportLineArrayOfAvailabilityDateComponent> for StockAvailabilityReportLineArrayOfAvailabilityDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfAvailabilityDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfAvailabilityDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StockAvailabilityReportLineArrayOfAvailabilityDateComponent {
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
pub struct StockAvailabilityReportLineArrayOfAvailabilityStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AvailabilityStatusCode>,
}

impl AsMut<StockAvailabilityReportLineArrayOfAvailabilityStatusCodeComponent> for StockAvailabilityReportLineArrayOfAvailabilityStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StockAvailabilityReportLineArrayOfAvailabilityStatusCodeComponent> for StockAvailabilityReportLineArrayOfAvailabilityStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfAvailabilityStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfAvailabilityStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StockAvailabilityReportLineArrayOfAvailabilityStatusCodeComponent {
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
pub struct StockAvailabilityReportLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<StockAvailabilityReportLineArrayOfIDComponent> for StockAvailabilityReportLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StockAvailabilityReportLineArrayOfIDComponent> for StockAvailabilityReportLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StockAvailabilityReportLineArrayOfIDComponent {
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
pub struct StockAvailabilityReportLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<StockAvailabilityReportLineArrayOfItemComponent> for StockAvailabilityReportLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StockAvailabilityReportLineArrayOfItemComponent> for StockAvailabilityReportLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StockAvailabilityReportLineArrayOfItemComponent {
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
pub struct StockAvailabilityReportLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<StockAvailabilityReportLineArrayOfNoteComponent> for StockAvailabilityReportLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StockAvailabilityReportLineArrayOfNoteComponent> for StockAvailabilityReportLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StockAvailabilityReportLineArrayOfNoteComponent {
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
pub struct StockAvailabilityReportLineArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<StockAvailabilityReportLineArrayOfQuantityComponent> for StockAvailabilityReportLineArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StockAvailabilityReportLineArrayOfQuantityComponent> for StockAvailabilityReportLineArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StockAvailabilityReportLineArrayOfQuantityComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StockAvailabilityReportLineArrayOfValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValueAmount>,
}

impl AsMut<StockAvailabilityReportLineArrayOfValueAmountComponent> for StockAvailabilityReportLineArrayOfValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<StockAvailabilityReportLineArrayOfValueAmountComponent> for StockAvailabilityReportLineArrayOfValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("StockAvailabilityReportLineArrayOfValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl StockAvailabilityReportLineArrayOfValueAmountComponent {
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


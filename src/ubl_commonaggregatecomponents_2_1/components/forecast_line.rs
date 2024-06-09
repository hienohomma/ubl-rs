use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForecastLine {
    #[serde(rename = "ForecastPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_period: Option<ForecastLineArrayOfForecastPeriodComponent>,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: ForecastLineArrayOfForecastTypeCodeComponent,
    #[serde(rename = "FrozenDocumentIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_document_indicator: Option<ForecastLineArrayOfFrozenDocumentIndicatorComponent>,
    #[serde(rename = "ID")]
    pub id: ForecastLineArrayOfIDComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<ForecastLineArrayOfNoteComponent>,
    #[serde(rename = "SalesItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_item: Option<ForecastLineArrayOfSalesItemComponent>,
}

impl AsMut<ForecastLine> for ForecastLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastLine> for ForecastLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastLine.note", e));
            }
        }
        if let Some(v) = &self.forecast_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastLine.forecast_period", e));
            }
        }
        if let Some(v) = &self.frozen_document_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastLine.frozen_document_indicator", e));
            }
        }
        if let Err(e) = self.forecast_type_code.validate() {
            return Err(UblError::component("ForecastLine.forecast_type_code", e));
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ForecastLine.id", e));
        }
        if let Some(v) = &self.sales_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ForecastLine.sales_item", e));
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

impl ForecastLine {
    pub fn title() -> &'static str {
        "Forecast Line. Details"
    }
    pub fn description() -> &'static str {
        "Detailed information about a particular Forecast Line within a Forecast Document"
    }
    pub fn new(forecast_type_code: ForecastLineArrayOfForecastTypeCodeComponent, id: ForecastLineArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            frozen_document_indicator: None,
            id,
            note: None,
            forecast_period: None,
            forecast_type_code,
            sales_item: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastLineArrayOfForecastPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ForecastPeriod>,
}

impl AsMut<ForecastLineArrayOfForecastPeriodComponent> for ForecastLineArrayOfForecastPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastLineArrayOfForecastPeriodComponent> for ForecastLineArrayOfForecastPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastLineArrayOfForecastPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastLineArrayOfForecastPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastLineArrayOfForecastPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ForecastPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ForecastPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ForecastPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ForecastPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastLineArrayOfForecastTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode>,
}

impl AsMut<ForecastLineArrayOfForecastTypeCodeComponent> for ForecastLineArrayOfForecastTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastLineArrayOfForecastTypeCodeComponent> for ForecastLineArrayOfForecastTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastLineArrayOfForecastTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastLineArrayOfForecastTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastLineArrayOfForecastTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ForecastTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastLineArrayOfFrozenDocumentIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FrozenDocumentIndicator>,
}

impl AsMut<ForecastLineArrayOfFrozenDocumentIndicatorComponent> for ForecastLineArrayOfFrozenDocumentIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastLineArrayOfFrozenDocumentIndicatorComponent> for ForecastLineArrayOfFrozenDocumentIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastLineArrayOfFrozenDocumentIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastLineArrayOfFrozenDocumentIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastLineArrayOfFrozenDocumentIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FrozenDocumentIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FrozenDocumentIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FrozenDocumentIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FrozenDocumentIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ForecastLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ForecastLineArrayOfIDComponent> for ForecastLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastLineArrayOfIDComponent> for ForecastLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastLineArrayOfIDComponent {
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
pub struct ForecastLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<ForecastLineArrayOfNoteComponent> for ForecastLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastLineArrayOfNoteComponent> for ForecastLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ForecastLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastLineArrayOfNoteComponent {
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
pub struct ForecastLineArrayOfSalesItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SalesItem>,
}

impl AsMut<ForecastLineArrayOfSalesItemComponent> for ForecastLineArrayOfSalesItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ForecastLineArrayOfSalesItemComponent> for ForecastLineArrayOfSalesItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ForecastLineArrayOfSalesItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ForecastLineArrayOfSalesItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ForecastLineArrayOfSalesItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SalesItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SalesItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SalesItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SalesItem> {
        self.items.iter()
    }
}


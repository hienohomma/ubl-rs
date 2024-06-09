use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PerformanceDataLine {
    #[serde(rename = "ID")]
    pub id: PerformanceDataLineArrayOfIDComponent,
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<PerformanceDataLineArrayOfItemComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<PerformanceDataLineArrayOfNoteComponent>,
    #[serde(rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: PerformanceDataLineArrayOfPerformanceMetricTypeCodeComponent,
    #[serde(rename = "PerformanceValueQuantity")]
    pub performance_value_quantity: PerformanceDataLineArrayOfPerformanceValueQuantityComponent,
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<PerformanceDataLineArrayOfPeriodComponent>,
}

impl AsMut<PerformanceDataLine> for PerformanceDataLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PerformanceDataLine> for PerformanceDataLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("PerformanceDataLine.id", e));
        }
        if let Some(v) = &self.item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PerformanceDataLine.item", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PerformanceDataLine.note", e));
            }
        }
        if let Err(e) = self.performance_metric_type_code.validate() {
            return Err(UblError::component("PerformanceDataLine.performance_metric_type_code", e));
        }
        if let Err(e) = self.performance_value_quantity.validate() {
            return Err(UblError::component("PerformanceDataLine.performance_value_quantity", e));
        }
        if let Some(v) = &self.period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("PerformanceDataLine.period", e));
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

impl PerformanceDataLine {
    pub fn title() -> &'static str {
        "Performance Data Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Performance History."
    }
    pub fn new(id: PerformanceDataLineArrayOfIDComponent, performance_metric_type_code: PerformanceDataLineArrayOfPerformanceMetricTypeCodeComponent, performance_value_quantity: PerformanceDataLineArrayOfPerformanceValueQuantityComponent) -> Component<Self> {
        Component(Self {
            note: None,
            id,
            item: None,
            performance_metric_type_code,
            performance_value_quantity,
            period: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PerformanceDataLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<PerformanceDataLineArrayOfIDComponent> for PerformanceDataLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PerformanceDataLineArrayOfIDComponent> for PerformanceDataLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PerformanceDataLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PerformanceDataLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PerformanceDataLineArrayOfIDComponent {
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
pub struct PerformanceDataLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<PerformanceDataLineArrayOfItemComponent> for PerformanceDataLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PerformanceDataLineArrayOfItemComponent> for PerformanceDataLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PerformanceDataLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PerformanceDataLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PerformanceDataLineArrayOfItemComponent {
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
pub struct PerformanceDataLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<PerformanceDataLineArrayOfNoteComponent> for PerformanceDataLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PerformanceDataLineArrayOfNoteComponent> for PerformanceDataLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PerformanceDataLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PerformanceDataLineArrayOfNoteComponent {
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
pub struct PerformanceDataLineArrayOfPerformanceMetricTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode>,
}

impl AsMut<PerformanceDataLineArrayOfPerformanceMetricTypeCodeComponent> for PerformanceDataLineArrayOfPerformanceMetricTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PerformanceDataLineArrayOfPerformanceMetricTypeCodeComponent> for PerformanceDataLineArrayOfPerformanceMetricTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PerformanceDataLineArrayOfPerformanceMetricTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PerformanceDataLineArrayOfPerformanceMetricTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PerformanceDataLineArrayOfPerformanceMetricTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PerformanceMetricTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PerformanceDataLineArrayOfPerformanceValueQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PerformanceValueQuantity>,
}

impl AsMut<PerformanceDataLineArrayOfPerformanceValueQuantityComponent> for PerformanceDataLineArrayOfPerformanceValueQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PerformanceDataLineArrayOfPerformanceValueQuantityComponent> for PerformanceDataLineArrayOfPerformanceValueQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PerformanceDataLineArrayOfPerformanceValueQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PerformanceDataLineArrayOfPerformanceValueQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PerformanceDataLineArrayOfPerformanceValueQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PerformanceValueQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PerformanceValueQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PerformanceValueQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PerformanceValueQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PerformanceDataLineArrayOfPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Period>,
}

impl AsMut<PerformanceDataLineArrayOfPeriodComponent> for PerformanceDataLineArrayOfPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PerformanceDataLineArrayOfPeriodComponent> for PerformanceDataLineArrayOfPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PerformanceDataLineArrayOfPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PerformanceDataLineArrayOfPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PerformanceDataLineArrayOfPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Period) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Period) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Period> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Period> {
        self.items.iter()
    }
}


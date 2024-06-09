use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsumptionReportReference {
    #[serde(rename = "ConsumptionReportID")]
    pub consumption_report_id: ConsumptionReportReferenceArrayOfConsumptionReportIDComponent,
    #[serde(rename = "ConsumptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_type: Option<ConsumptionReportReferenceArrayOfConsumptionTypeComponent>,
    #[serde(rename = "ConsumptionTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_type_code: Option<ConsumptionReportReferenceArrayOfConsumptionTypeCodeComponent>,
    #[serde(rename = "Period")]
    pub period: ConsumptionReportReferenceArrayOfPeriodComponent,
    #[serde(rename = "TotalConsumedQuantity")]
    pub total_consumed_quantity: ConsumptionReportReferenceArrayOfTotalConsumedQuantityComponent,
}

impl AsMut<ConsumptionReportReference> for ConsumptionReportReference {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportReference> for ConsumptionReportReference {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.consumption_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReportReference.consumption_type_code", e));
            }
        }
        if let Err(e) = self.consumption_report_id.validate() {
            return Err(UblError::component("ConsumptionReportReference.consumption_report_id", e));
        }
        if let Err(e) = self.period.validate() {
            return Err(UblError::component("ConsumptionReportReference.period", e));
        }
        if let Err(e) = self.total_consumed_quantity.validate() {
            return Err(UblError::component("ConsumptionReportReference.total_consumed_quantity", e));
        }
        if let Some(v) = &self.consumption_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReportReference.consumption_type", e));
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

impl ConsumptionReportReference {
    pub fn title() -> &'static str {
        "Consumption Report Reference. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a reference to an earlier consumption report (e.g., last year's consumption)."
    }
    pub fn new(consumption_report_id: ConsumptionReportReferenceArrayOfConsumptionReportIDComponent, period: ConsumptionReportReferenceArrayOfPeriodComponent, total_consumed_quantity: ConsumptionReportReferenceArrayOfTotalConsumedQuantityComponent) -> Component<Self> {
        Component(Self {
            consumption_report_id,
            period,
            consumption_type_code: None,
            total_consumed_quantity,
            consumption_type: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportReferenceArrayOfConsumptionReportIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionReportID>,
}

impl AsMut<ConsumptionReportReferenceArrayOfConsumptionReportIDComponent> for ConsumptionReportReferenceArrayOfConsumptionReportIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportReferenceArrayOfConsumptionReportIDComponent> for ConsumptionReportReferenceArrayOfConsumptionReportIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportReferenceArrayOfConsumptionReportIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportReferenceArrayOfConsumptionReportIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportReferenceArrayOfConsumptionReportIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumptionReportID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumptionReportID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumptionReportID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumptionReportID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportReferenceArrayOfConsumptionTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionType>,
}

impl AsMut<ConsumptionReportReferenceArrayOfConsumptionTypeComponent> for ConsumptionReportReferenceArrayOfConsumptionTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportReferenceArrayOfConsumptionTypeComponent> for ConsumptionReportReferenceArrayOfConsumptionTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportReferenceArrayOfConsumptionTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportReferenceArrayOfConsumptionTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportReferenceArrayOfConsumptionTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumptionType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumptionType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumptionType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumptionType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportReferenceArrayOfConsumptionTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionTypeCode>,
}

impl AsMut<ConsumptionReportReferenceArrayOfConsumptionTypeCodeComponent> for ConsumptionReportReferenceArrayOfConsumptionTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportReferenceArrayOfConsumptionTypeCodeComponent> for ConsumptionReportReferenceArrayOfConsumptionTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportReferenceArrayOfConsumptionTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportReferenceArrayOfConsumptionTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportReferenceArrayOfConsumptionTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumptionTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumptionTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumptionTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumptionTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportReferenceArrayOfPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Period>,
}

impl AsMut<ConsumptionReportReferenceArrayOfPeriodComponent> for ConsumptionReportReferenceArrayOfPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportReferenceArrayOfPeriodComponent> for ConsumptionReportReferenceArrayOfPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportReferenceArrayOfPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportReferenceArrayOfPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportReferenceArrayOfPeriodComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportReferenceArrayOfTotalConsumedQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalConsumedQuantity>,
}

impl AsMut<ConsumptionReportReferenceArrayOfTotalConsumedQuantityComponent> for ConsumptionReportReferenceArrayOfTotalConsumedQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportReferenceArrayOfTotalConsumedQuantityComponent> for ConsumptionReportReferenceArrayOfTotalConsumedQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportReferenceArrayOfTotalConsumedQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportReferenceArrayOfTotalConsumedQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportReferenceArrayOfTotalConsumedQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalConsumedQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalConsumedQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalConsumedQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalConsumedQuantity> {
        self.items.iter()
    }
}


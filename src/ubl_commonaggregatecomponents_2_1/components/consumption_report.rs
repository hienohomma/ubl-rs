use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsumptionReport {
    #[serde(rename = "BasicConsumedQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_consumed_quantity: Option<ConsumptionReportArrayOfBasicConsumedQuantityComponent>,
    #[serde(rename = "ConsumersEnergyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumers_energy_level: Option<ConsumptionReportArrayOfConsumersEnergyLevelComponent>,
    #[serde(rename = "ConsumersEnergyLevelCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumers_energy_level_code: Option<ConsumptionReportArrayOfConsumersEnergyLevelCodeComponent>,
    #[serde(rename = "ConsumptionHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_history: Option<ConsumptionReportArrayOfConsumptionHistoryComponent>,
    #[serde(rename = "ConsumptionReportReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_report_reference: Option<ConsumptionReportArrayOfConsumptionReportReferenceComponent>,
    #[serde(rename = "ConsumptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_type: Option<ConsumptionReportArrayOfConsumptionTypeComponent>,
    #[serde(rename = "ConsumptionTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_type_code: Option<ConsumptionReportArrayOfConsumptionTypeCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ConsumptionReportArrayOfDescriptionComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ConsumptionReportArrayOfDocumentReferenceComponent>,
    #[serde(rename = "GuidanceDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_document_reference: Option<ConsumptionReportArrayOfGuidanceDocumentReferenceComponent>,
    #[serde(rename = "HeatingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heating_type: Option<ConsumptionReportArrayOfHeatingTypeComponent>,
    #[serde(rename = "HeatingTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heating_type_code: Option<ConsumptionReportArrayOfHeatingTypeCodeComponent>,
    #[serde(rename = "ID")]
    pub id: ConsumptionReportArrayOfIDComponent,
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<ConsumptionReportArrayOfPeriodComponent>,
    #[serde(rename = "ResidenceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_type: Option<ConsumptionReportArrayOfResidenceTypeComponent>,
    #[serde(rename = "ResidenceTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_type_code: Option<ConsumptionReportArrayOfResidenceTypeCodeComponent>,
    #[serde(rename = "ResidentOccupantsNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resident_occupants_numeric: Option<ConsumptionReportArrayOfResidentOccupantsNumericComponent>,
    #[serde(rename = "TotalConsumedQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_consumed_quantity: Option<ConsumptionReportArrayOfTotalConsumedQuantityComponent>,
}

impl AsMut<ConsumptionReport> for ConsumptionReport {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReport> for ConsumptionReport {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.period", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ConsumptionReport.id", e));
        }
        if let Some(v) = &self.heating_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.heating_type_code", e));
            }
        }
        if let Some(v) = &self.resident_occupants_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.resident_occupants_numeric", e));
            }
        }
        if let Some(v) = &self.guidance_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.guidance_document_reference", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.document_reference", e));
            }
        }
        if let Some(v) = &self.residence_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.residence_type_code", e));
            }
        }
        if let Some(v) = &self.total_consumed_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.total_consumed_quantity", e));
            }
        }
        if let Some(v) = &self.heating_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.heating_type", e));
            }
        }
        if let Some(v) = &self.consumers_energy_level_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.consumers_energy_level_code", e));
            }
        }
        if let Some(v) = &self.basic_consumed_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.basic_consumed_quantity", e));
            }
        }
        if let Some(v) = &self.consumption_history {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.consumption_history", e));
            }
        }
        if let Some(v) = &self.consumption_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.consumption_type", e));
            }
        }
        if let Some(v) = &self.consumption_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.consumption_type_code", e));
            }
        }
        if let Some(v) = &self.consumption_report_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.consumption_report_reference", e));
            }
        }
        if let Some(v) = &self.consumers_energy_level {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.consumers_energy_level", e));
            }
        }
        if let Some(v) = &self.residence_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.residence_type", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ConsumptionReport.description", e));
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

impl ConsumptionReport {
    pub fn title() -> &'static str {
        "Consumption Report. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe utility consumption, including details of the environment in which consumption takes place."
    }
    pub fn new(id: ConsumptionReportArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            consumption_history: None,
            heating_type: None,
            description: None,
            period: None,
            residence_type: None,
            resident_occupants_numeric: None,
            consumption_type_code: None,
            document_reference: None,
            basic_consumed_quantity: None,
            residence_type_code: None,
            total_consumed_quantity: None,
            id,
            consumption_type: None,
            guidance_document_reference: None,
            heating_type_code: None,
            consumers_energy_level: None,
            consumers_energy_level_code: None,
            consumption_report_reference: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfBasicConsumedQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BasicConsumedQuantity>,
}

impl AsMut<ConsumptionReportArrayOfBasicConsumedQuantityComponent> for ConsumptionReportArrayOfBasicConsumedQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfBasicConsumedQuantityComponent> for ConsumptionReportArrayOfBasicConsumedQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfBasicConsumedQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfBasicConsumedQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfBasicConsumedQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BasicConsumedQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BasicConsumedQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BasicConsumedQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BasicConsumedQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfConsumersEnergyLevelComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumersEnergyLevel>,
}

impl AsMut<ConsumptionReportArrayOfConsumersEnergyLevelComponent> for ConsumptionReportArrayOfConsumersEnergyLevelComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfConsumersEnergyLevelComponent> for ConsumptionReportArrayOfConsumersEnergyLevelComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfConsumersEnergyLevelComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfConsumersEnergyLevelComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfConsumersEnergyLevelComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumersEnergyLevel) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumersEnergyLevel) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumersEnergyLevel> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumersEnergyLevel> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfConsumersEnergyLevelCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumersEnergyLevelCode>,
}

impl AsMut<ConsumptionReportArrayOfConsumersEnergyLevelCodeComponent> for ConsumptionReportArrayOfConsumersEnergyLevelCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfConsumersEnergyLevelCodeComponent> for ConsumptionReportArrayOfConsumersEnergyLevelCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfConsumersEnergyLevelCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfConsumersEnergyLevelCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfConsumersEnergyLevelCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumersEnergyLevelCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumersEnergyLevelCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumersEnergyLevelCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumersEnergyLevelCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfConsumptionHistoryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ConsumptionHistory>,
}

impl AsMut<ConsumptionReportArrayOfConsumptionHistoryComponent> for ConsumptionReportArrayOfConsumptionHistoryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfConsumptionHistoryComponent> for ConsumptionReportArrayOfConsumptionHistoryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfConsumptionHistoryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfConsumptionHistoryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ConsumptionHistory) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ConsumptionHistory) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ConsumptionHistory> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ConsumptionHistory> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfConsumptionReportReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ConsumptionReportReference>,
}

impl AsMut<ConsumptionReportArrayOfConsumptionReportReferenceComponent> for ConsumptionReportArrayOfConsumptionReportReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfConsumptionReportReferenceComponent> for ConsumptionReportArrayOfConsumptionReportReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfConsumptionReportReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfConsumptionReportReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ConsumptionReportReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ConsumptionReportReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ConsumptionReportReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ConsumptionReportReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfConsumptionTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionType>,
}

impl AsMut<ConsumptionReportArrayOfConsumptionTypeComponent> for ConsumptionReportArrayOfConsumptionTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfConsumptionTypeComponent> for ConsumptionReportArrayOfConsumptionTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfConsumptionTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfConsumptionTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfConsumptionTypeComponent {
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
pub struct ConsumptionReportArrayOfConsumptionTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionTypeCode>,
}

impl AsMut<ConsumptionReportArrayOfConsumptionTypeCodeComponent> for ConsumptionReportArrayOfConsumptionTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfConsumptionTypeCodeComponent> for ConsumptionReportArrayOfConsumptionTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfConsumptionTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfConsumptionTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfConsumptionTypeCodeComponent {
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
pub struct ConsumptionReportArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ConsumptionReportArrayOfDescriptionComponent> for ConsumptionReportArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfDescriptionComponent> for ConsumptionReportArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfDescriptionComponent {
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
pub struct ConsumptionReportArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<ConsumptionReportArrayOfDocumentReferenceComponent> for ConsumptionReportArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfDocumentReferenceComponent> for ConsumptionReportArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfGuidanceDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::GuidanceDocumentReference>,
}

impl AsMut<ConsumptionReportArrayOfGuidanceDocumentReferenceComponent> for ConsumptionReportArrayOfGuidanceDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfGuidanceDocumentReferenceComponent> for ConsumptionReportArrayOfGuidanceDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfGuidanceDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfGuidanceDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfGuidanceDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::GuidanceDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::GuidanceDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::GuidanceDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::GuidanceDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfHeatingTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HeatingType>,
}

impl AsMut<ConsumptionReportArrayOfHeatingTypeComponent> for ConsumptionReportArrayOfHeatingTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfHeatingTypeComponent> for ConsumptionReportArrayOfHeatingTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfHeatingTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfHeatingTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfHeatingTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HeatingType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HeatingType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HeatingType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HeatingType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfHeatingTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HeatingTypeCode>,
}

impl AsMut<ConsumptionReportArrayOfHeatingTypeCodeComponent> for ConsumptionReportArrayOfHeatingTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfHeatingTypeCodeComponent> for ConsumptionReportArrayOfHeatingTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfHeatingTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfHeatingTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfHeatingTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HeatingTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HeatingTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HeatingTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HeatingTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ConsumptionReportArrayOfIDComponent> for ConsumptionReportArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfIDComponent> for ConsumptionReportArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfIDComponent {
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
pub struct ConsumptionReportArrayOfPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Period>,
}

impl AsMut<ConsumptionReportArrayOfPeriodComponent> for ConsumptionReportArrayOfPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfPeriodComponent> for ConsumptionReportArrayOfPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfPeriodComponent {
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
pub struct ConsumptionReportArrayOfResidenceTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ResidenceType>,
}

impl AsMut<ConsumptionReportArrayOfResidenceTypeComponent> for ConsumptionReportArrayOfResidenceTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfResidenceTypeComponent> for ConsumptionReportArrayOfResidenceTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfResidenceTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfResidenceTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfResidenceTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ResidenceType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ResidenceType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ResidenceType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ResidenceType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfResidenceTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ResidenceTypeCode>,
}

impl AsMut<ConsumptionReportArrayOfResidenceTypeCodeComponent> for ConsumptionReportArrayOfResidenceTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfResidenceTypeCodeComponent> for ConsumptionReportArrayOfResidenceTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfResidenceTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfResidenceTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfResidenceTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ResidenceTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ResidenceTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ResidenceTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ResidenceTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfResidentOccupantsNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ResidentOccupantsNumeric>,
}

impl AsMut<ConsumptionReportArrayOfResidentOccupantsNumericComponent> for ConsumptionReportArrayOfResidentOccupantsNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfResidentOccupantsNumericComponent> for ConsumptionReportArrayOfResidentOccupantsNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfResidentOccupantsNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfResidentOccupantsNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfResidentOccupantsNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ResidentOccupantsNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ResidentOccupantsNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ResidentOccupantsNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ResidentOccupantsNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConsumptionReportArrayOfTotalConsumedQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalConsumedQuantity>,
}

impl AsMut<ConsumptionReportArrayOfTotalConsumedQuantityComponent> for ConsumptionReportArrayOfTotalConsumedQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ConsumptionReportArrayOfTotalConsumedQuantityComponent> for ConsumptionReportArrayOfTotalConsumedQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfTotalConsumedQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ConsumptionReportArrayOfTotalConsumedQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ConsumptionReportArrayOfTotalConsumedQuantityComponent {
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


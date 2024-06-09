use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcurementProject {
    #[serde(rename = "AdditionalCommodityClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_commodity_classification: Option<ProcurementProjectArrayOfAdditionalCommodityClassificationComponent>,
    #[serde(rename = "ContractExtension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_extension: Option<ProcurementProjectArrayOfContractExtensionComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ProcurementProjectArrayOfDescriptionComponent>,
    #[serde(rename = "EstimatedOverallContractQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_overall_contract_quantity: Option<ProcurementProjectArrayOfEstimatedOverallContractQuantityComponent>,
    #[serde(rename = "FeeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_description: Option<ProcurementProjectArrayOfFeeDescriptionComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ProcurementProjectArrayOfIDComponent>,
    #[serde(rename = "MainCommodityClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_commodity_classification: Option<ProcurementProjectArrayOfMainCommodityClassificationComponent>,
    #[serde(rename = "Name")]
    pub name: ProcurementProjectArrayOfNameComponent,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<ProcurementProjectArrayOfNoteComponent>,
    #[serde(rename = "PlannedPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_period: Option<ProcurementProjectArrayOfPlannedPeriodComponent>,
    #[serde(rename = "ProcurementSubTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procurement_sub_type_code: Option<ProcurementProjectArrayOfProcurementSubTypeCodeComponent>,
    #[serde(rename = "ProcurementTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procurement_type_code: Option<ProcurementProjectArrayOfProcurementTypeCodeComponent>,
    #[serde(rename = "QualityControlCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_control_code: Option<ProcurementProjectArrayOfQualityControlCodeComponent>,
    #[serde(rename = "RealizedLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realized_location: Option<ProcurementProjectArrayOfRealizedLocationComponent>,
    #[serde(rename = "RequestForTenderLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_for_tender_line: Option<ProcurementProjectArrayOfRequestForTenderLineComponent>,
    #[serde(rename = "RequestedDeliveryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_delivery_date: Option<ProcurementProjectArrayOfRequestedDeliveryDateComponent>,
    #[serde(rename = "RequestedTenderTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_tender_total: Option<ProcurementProjectArrayOfRequestedTenderTotalComponent>,
    #[serde(rename = "RequiredFeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_fee_amount: Option<ProcurementProjectArrayOfRequiredFeeAmountComponent>,
}

impl AsMut<ProcurementProject> for ProcurementProject {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProject> for ProcurementProject {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.main_commodity_classification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.main_commodity_classification", e));
            }
        }
        if let Some(v) = &self.requested_delivery_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.requested_delivery_date", e));
            }
        }
        if let Some(v) = &self.required_fee_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.required_fee_amount", e));
            }
        }
        if let Some(v) = &self.estimated_overall_contract_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.estimated_overall_contract_quantity", e));
            }
        }
        if let Some(v) = &self.procurement_sub_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.procurement_sub_type_code", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.note", e));
            }
        }
        if let Some(v) = &self.planned_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.planned_period", e));
            }
        }
        if let Some(v) = &self.quality_control_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.quality_control_code", e));
            }
        }
        if let Some(v) = &self.request_for_tender_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.request_for_tender_line", e));
            }
        }
        if let Some(v) = &self.procurement_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.procurement_type_code", e));
            }
        }
        if let Some(v) = &self.realized_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.realized_location", e));
            }
        }
        if let Some(v) = &self.requested_tender_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.requested_tender_total", e));
            }
        }
        if let Some(v) = &self.fee_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.fee_description", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.id", e));
            }
        }
        if let Some(v) = &self.additional_commodity_classification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.additional_commodity_classification", e));
            }
        }
        if let Some(v) = &self.contract_extension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.contract_extension", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ProcurementProject.description", e));
            }
        }
        if let Err(e) = self.name.validate() {
            return Err(UblError::component("ProcurementProject.name", e));
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

impl ProcurementProject {
    pub fn title() -> &'static str {
        "Procurement Project. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a project to procure goods, works, or services."
    }
    pub fn new(name: ProcurementProjectArrayOfNameComponent) -> Component<Self> {
        Component(Self {
            procurement_type_code: None,
            required_fee_amount: None,
            realized_location: None,
            main_commodity_classification: None,
            estimated_overall_contract_quantity: None,
            description: None,
            fee_description: None,
            name,
            quality_control_code: None,
            requested_tender_total: None,
            additional_commodity_classification: None,
            contract_extension: None,
            procurement_sub_type_code: None,
            requested_delivery_date: None,
            request_for_tender_line: None,
            id: None,
            note: None,
            planned_period: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfAdditionalCommodityClassificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AdditionalCommodityClassification>,
}

impl AsMut<ProcurementProjectArrayOfAdditionalCommodityClassificationComponent> for ProcurementProjectArrayOfAdditionalCommodityClassificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfAdditionalCommodityClassificationComponent> for ProcurementProjectArrayOfAdditionalCommodityClassificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfAdditionalCommodityClassificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfAdditionalCommodityClassificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AdditionalCommodityClassification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AdditionalCommodityClassification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AdditionalCommodityClassification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AdditionalCommodityClassification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfContractExtensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractExtension>,
}

impl AsMut<ProcurementProjectArrayOfContractExtensionComponent> for ProcurementProjectArrayOfContractExtensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfContractExtensionComponent> for ProcurementProjectArrayOfContractExtensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfContractExtensionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfContractExtensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfContractExtensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContractExtension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContractExtension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContractExtension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContractExtension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ProcurementProjectArrayOfDescriptionComponent> for ProcurementProjectArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfDescriptionComponent> for ProcurementProjectArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfDescriptionComponent {
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
pub struct ProcurementProjectArrayOfEstimatedOverallContractQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EstimatedOverallContractQuantity>,
}

impl AsMut<ProcurementProjectArrayOfEstimatedOverallContractQuantityComponent> for ProcurementProjectArrayOfEstimatedOverallContractQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfEstimatedOverallContractQuantityComponent> for ProcurementProjectArrayOfEstimatedOverallContractQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfEstimatedOverallContractQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfEstimatedOverallContractQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfEstimatedOverallContractQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EstimatedOverallContractQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EstimatedOverallContractQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EstimatedOverallContractQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EstimatedOverallContractQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfFeeDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FeeDescription>,
}

impl AsMut<ProcurementProjectArrayOfFeeDescriptionComponent> for ProcurementProjectArrayOfFeeDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfFeeDescriptionComponent> for ProcurementProjectArrayOfFeeDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfFeeDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfFeeDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FeeDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FeeDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FeeDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FeeDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ProcurementProjectArrayOfIDComponent> for ProcurementProjectArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfIDComponent> for ProcurementProjectArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfIDComponent {
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
pub struct ProcurementProjectArrayOfMainCommodityClassificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MainCommodityClassification>,
}

impl AsMut<ProcurementProjectArrayOfMainCommodityClassificationComponent> for ProcurementProjectArrayOfMainCommodityClassificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfMainCommodityClassificationComponent> for ProcurementProjectArrayOfMainCommodityClassificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfMainCommodityClassificationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfMainCommodityClassificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfMainCommodityClassificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MainCommodityClassification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MainCommodityClassification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MainCommodityClassification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MainCommodityClassification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<ProcurementProjectArrayOfNameComponent> for ProcurementProjectArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfNameComponent> for ProcurementProjectArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Name) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Name) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<ProcurementProjectArrayOfNoteComponent> for ProcurementProjectArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfNoteComponent> for ProcurementProjectArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfNoteComponent {
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
pub struct ProcurementProjectArrayOfPlannedPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PlannedPeriod>,
}

impl AsMut<ProcurementProjectArrayOfPlannedPeriodComponent> for ProcurementProjectArrayOfPlannedPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfPlannedPeriodComponent> for ProcurementProjectArrayOfPlannedPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfPlannedPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfPlannedPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfPlannedPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PlannedPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PlannedPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PlannedPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PlannedPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfProcurementSubTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ProcurementSubTypeCode>,
}

impl AsMut<ProcurementProjectArrayOfProcurementSubTypeCodeComponent> for ProcurementProjectArrayOfProcurementSubTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfProcurementSubTypeCodeComponent> for ProcurementProjectArrayOfProcurementSubTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfProcurementSubTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfProcurementSubTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfProcurementSubTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ProcurementSubTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ProcurementSubTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ProcurementSubTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ProcurementSubTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfProcurementTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ProcurementTypeCode>,
}

impl AsMut<ProcurementProjectArrayOfProcurementTypeCodeComponent> for ProcurementProjectArrayOfProcurementTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfProcurementTypeCodeComponent> for ProcurementProjectArrayOfProcurementTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfProcurementTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfProcurementTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfProcurementTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ProcurementTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ProcurementTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ProcurementTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ProcurementTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfQualityControlCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::QualityControlCode>,
}

impl AsMut<ProcurementProjectArrayOfQualityControlCodeComponent> for ProcurementProjectArrayOfQualityControlCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfQualityControlCodeComponent> for ProcurementProjectArrayOfQualityControlCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfQualityControlCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfQualityControlCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfQualityControlCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::QualityControlCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::QualityControlCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::QualityControlCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::QualityControlCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfRealizedLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RealizedLocation>,
}

impl AsMut<ProcurementProjectArrayOfRealizedLocationComponent> for ProcurementProjectArrayOfRealizedLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfRealizedLocationComponent> for ProcurementProjectArrayOfRealizedLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfRealizedLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfRealizedLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RealizedLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RealizedLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RealizedLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RealizedLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfRequestForTenderLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequestForTenderLine>,
}

impl AsMut<ProcurementProjectArrayOfRequestForTenderLineComponent> for ProcurementProjectArrayOfRequestForTenderLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfRequestForTenderLineComponent> for ProcurementProjectArrayOfRequestForTenderLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfRequestForTenderLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfRequestForTenderLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequestForTenderLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequestForTenderLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequestForTenderLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequestForTenderLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfRequestedDeliveryDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RequestedDeliveryDate>,
}

impl AsMut<ProcurementProjectArrayOfRequestedDeliveryDateComponent> for ProcurementProjectArrayOfRequestedDeliveryDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfRequestedDeliveryDateComponent> for ProcurementProjectArrayOfRequestedDeliveryDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfRequestedDeliveryDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfRequestedDeliveryDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfRequestedDeliveryDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RequestedDeliveryDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RequestedDeliveryDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RequestedDeliveryDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RequestedDeliveryDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfRequestedTenderTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequestedTenderTotal>,
}

impl AsMut<ProcurementProjectArrayOfRequestedTenderTotalComponent> for ProcurementProjectArrayOfRequestedTenderTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfRequestedTenderTotalComponent> for ProcurementProjectArrayOfRequestedTenderTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfRequestedTenderTotalComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfRequestedTenderTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfRequestedTenderTotalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequestedTenderTotal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequestedTenderTotal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequestedTenderTotal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequestedTenderTotal> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProcurementProjectArrayOfRequiredFeeAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RequiredFeeAmount>,
}

impl AsMut<ProcurementProjectArrayOfRequiredFeeAmountComponent> for ProcurementProjectArrayOfRequiredFeeAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ProcurementProjectArrayOfRequiredFeeAmountComponent> for ProcurementProjectArrayOfRequiredFeeAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfRequiredFeeAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ProcurementProjectArrayOfRequiredFeeAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ProcurementProjectArrayOfRequiredFeeAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RequiredFeeAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RequiredFeeAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RequiredFeeAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RequiredFeeAmount> {
        self.items.iter()
    }
}


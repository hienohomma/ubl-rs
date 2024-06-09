use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TendererPartyQualification {
    #[serde(rename = "AdditionalQualifyingParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_qualifying_party: Option<TendererPartyQualificationArrayOfAdditionalQualifyingPartyComponent>,
    #[serde(rename = "InterestedProcurementProjectLot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interested_procurement_project_lot: Option<TendererPartyQualificationArrayOfInterestedProcurementProjectLotComponent>,
    #[serde(rename = "MainQualifyingParty")]
    pub main_qualifying_party: TendererPartyQualificationArrayOfMainQualifyingPartyComponent,
}

impl AsMut<TendererPartyQualification> for TendererPartyQualification {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererPartyQualification> for TendererPartyQualification {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.interested_procurement_project_lot {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererPartyQualification.interested_procurement_project_lot", e));
            }
        }
        if let Some(v) = &self.additional_qualifying_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TendererPartyQualification.additional_qualifying_party", e));
            }
        }
        if let Err(e) = self.main_qualifying_party.validate() {
            return Err(UblError::component("TendererPartyQualification.main_qualifying_party", e));
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

impl TendererPartyQualification {
    pub fn title() -> &'static str {
        "Tenderer Party Qualification. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the qualifications of a tenderer party."
    }
    pub fn new(main_qualifying_party: TendererPartyQualificationArrayOfMainQualifyingPartyComponent) -> Component<Self> {
        Component(Self {
            additional_qualifying_party: None,
            interested_procurement_project_lot: None,
            main_qualifying_party,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererPartyQualificationArrayOfAdditionalQualifyingPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AdditionalQualifyingParty>,
}

impl AsMut<TendererPartyQualificationArrayOfAdditionalQualifyingPartyComponent> for TendererPartyQualificationArrayOfAdditionalQualifyingPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererPartyQualificationArrayOfAdditionalQualifyingPartyComponent> for TendererPartyQualificationArrayOfAdditionalQualifyingPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererPartyQualificationArrayOfAdditionalQualifyingPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererPartyQualificationArrayOfAdditionalQualifyingPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AdditionalQualifyingParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AdditionalQualifyingParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AdditionalQualifyingParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AdditionalQualifyingParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererPartyQualificationArrayOfInterestedProcurementProjectLotComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::InterestedProcurementProjectLot>,
}

impl AsMut<TendererPartyQualificationArrayOfInterestedProcurementProjectLotComponent> for TendererPartyQualificationArrayOfInterestedProcurementProjectLotComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererPartyQualificationArrayOfInterestedProcurementProjectLotComponent> for TendererPartyQualificationArrayOfInterestedProcurementProjectLotComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TendererPartyQualificationArrayOfInterestedProcurementProjectLotComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererPartyQualificationArrayOfInterestedProcurementProjectLotComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::InterestedProcurementProjectLot) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::InterestedProcurementProjectLot) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::InterestedProcurementProjectLot> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::InterestedProcurementProjectLot> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TendererPartyQualificationArrayOfMainQualifyingPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MainQualifyingParty>,
}

impl AsMut<TendererPartyQualificationArrayOfMainQualifyingPartyComponent> for TendererPartyQualificationArrayOfMainQualifyingPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TendererPartyQualificationArrayOfMainQualifyingPartyComponent> for TendererPartyQualificationArrayOfMainQualifyingPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TendererPartyQualificationArrayOfMainQualifyingPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TendererPartyQualificationArrayOfMainQualifyingPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TendererPartyQualificationArrayOfMainQualifyingPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MainQualifyingParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MainQualifyingParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MainQualifyingParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MainQualifyingParty> {
        self.items.iter()
    }
}


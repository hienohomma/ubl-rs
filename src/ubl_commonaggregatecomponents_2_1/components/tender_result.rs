use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenderResult {
    #[serde(rename = "AdvertisementAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertisement_amount: Option<TenderResultArrayOfAdvertisementAmountComponent>,
    #[serde(rename = "AwardDate")]
    pub award_date: TenderResultArrayOfAwardDateComponent,
    #[serde(rename = "AwardTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_time: Option<TenderResultArrayOfAwardTimeComponent>,
    #[serde(rename = "AwardedTenderedProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarded_tendered_project: Option<TenderResultArrayOfAwardedTenderedProjectComponent>,
    #[serde(rename = "Contract")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<TenderResultArrayOfContractComponent>,
    #[serde(rename = "ContractFormalizationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_formalization_period: Option<TenderResultArrayOfContractFormalizationPeriodComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TenderResultArrayOfDescriptionComponent>,
    #[serde(rename = "HigherTenderAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub higher_tender_amount: Option<TenderResultArrayOfHigherTenderAmountComponent>,
    #[serde(rename = "LowerTenderAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_tender_amount: Option<TenderResultArrayOfLowerTenderAmountComponent>,
    #[serde(rename = "ReceivedElectronicTenderQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_electronic_tender_quantity: Option<TenderResultArrayOfReceivedElectronicTenderQuantityComponent>,
    #[serde(rename = "ReceivedForeignTenderQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_foreign_tender_quantity: Option<TenderResultArrayOfReceivedForeignTenderQuantityComponent>,
    #[serde(rename = "ReceivedTenderQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_tender_quantity: Option<TenderResultArrayOfReceivedTenderQuantityComponent>,
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<TenderResultArrayOfStartDateComponent>,
    #[serde(rename = "SubcontractTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcontract_terms: Option<TenderResultArrayOfSubcontractTermsComponent>,
    #[serde(rename = "TenderResultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_result_code: Option<TenderResultArrayOfTenderResultCodeComponent>,
    #[serde(rename = "WinningParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winning_party: Option<TenderResultArrayOfWinningPartyComponent>,
}

impl AsMut<TenderResult> for TenderResult {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResult> for TenderResult {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.start_date {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.start_date", e));
            }
        }
        if let Err(e) = self.award_date.validate() {
            return Err(UblError::component("TenderResult.award_date", e));
        }
        if let Some(v) = &self.contract {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.contract", e));
            }
        }
        if let Some(v) = &self.advertisement_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.advertisement_amount", e));
            }
        }
        if let Some(v) = &self.subcontract_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.subcontract_terms", e));
            }
        }
        if let Some(v) = &self.awarded_tendered_project {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.awarded_tendered_project", e));
            }
        }
        if let Some(v) = &self.received_foreign_tender_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.received_foreign_tender_quantity", e));
            }
        }
        if let Some(v) = &self.received_electronic_tender_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.received_electronic_tender_quantity", e));
            }
        }
        if let Some(v) = &self.award_time {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.award_time", e));
            }
        }
        if let Some(v) = &self.tender_result_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.tender_result_code", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.description", e));
            }
        }
        if let Some(v) = &self.winning_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.winning_party", e));
            }
        }
        if let Some(v) = &self.higher_tender_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.higher_tender_amount", e));
            }
        }
        if let Some(v) = &self.lower_tender_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.lower_tender_amount", e));
            }
        }
        if let Some(v) = &self.contract_formalization_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.contract_formalization_period", e));
            }
        }
        if let Some(v) = &self.received_tender_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TenderResult.received_tender_quantity", e));
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

impl TenderResult {
    pub fn title() -> &'static str {
        "Tender Result. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the awarding of a tender in a tendering process."
    }
    pub fn new(award_date: TenderResultArrayOfAwardDateComponent) -> Component<Self> {
        Component(Self {
            award_time: None,
            higher_tender_amount: None,
            received_tender_quantity: None,
            contract_formalization_period: None,
            awarded_tendered_project: None,
            start_date: None,
            advertisement_amount: None,
            contract: None,
            award_date,
            lower_tender_amount: None,
            received_electronic_tender_quantity: None,
            description: None,
            tender_result_code: None,
            winning_party: None,
            received_foreign_tender_quantity: None,
            subcontract_terms: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfAdvertisementAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AdvertisementAmount>,
}

impl AsMut<TenderResultArrayOfAdvertisementAmountComponent> for TenderResultArrayOfAdvertisementAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfAdvertisementAmountComponent> for TenderResultArrayOfAdvertisementAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfAdvertisementAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfAdvertisementAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfAdvertisementAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AdvertisementAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AdvertisementAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AdvertisementAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AdvertisementAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfAwardDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AwardDate>,
}

impl AsMut<TenderResultArrayOfAwardDateComponent> for TenderResultArrayOfAwardDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfAwardDateComponent> for TenderResultArrayOfAwardDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfAwardDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfAwardDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfAwardDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AwardDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AwardDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AwardDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AwardDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfAwardTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AwardTime>,
}

impl AsMut<TenderResultArrayOfAwardTimeComponent> for TenderResultArrayOfAwardTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfAwardTimeComponent> for TenderResultArrayOfAwardTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfAwardTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfAwardTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfAwardTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AwardTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AwardTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AwardTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AwardTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfAwardedTenderedProjectComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AwardedTenderedProject>,
}

impl AsMut<TenderResultArrayOfAwardedTenderedProjectComponent> for TenderResultArrayOfAwardedTenderedProjectComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfAwardedTenderedProjectComponent> for TenderResultArrayOfAwardedTenderedProjectComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfAwardedTenderedProjectComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfAwardedTenderedProjectComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfAwardedTenderedProjectComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AwardedTenderedProject) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AwardedTenderedProject) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AwardedTenderedProject> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AwardedTenderedProject> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfContractComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Contract>,
}

impl AsMut<TenderResultArrayOfContractComponent> for TenderResultArrayOfContractComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfContractComponent> for TenderResultArrayOfContractComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfContractComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfContractComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfContractComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Contract) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Contract) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Contract> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Contract> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfContractFormalizationPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractFormalizationPeriod>,
}

impl AsMut<TenderResultArrayOfContractFormalizationPeriodComponent> for TenderResultArrayOfContractFormalizationPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfContractFormalizationPeriodComponent> for TenderResultArrayOfContractFormalizationPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfContractFormalizationPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfContractFormalizationPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfContractFormalizationPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContractFormalizationPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContractFormalizationPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContractFormalizationPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContractFormalizationPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<TenderResultArrayOfDescriptionComponent> for TenderResultArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfDescriptionComponent> for TenderResultArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfDescriptionComponent {
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
pub struct TenderResultArrayOfHigherTenderAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HigherTenderAmount>,
}

impl AsMut<TenderResultArrayOfHigherTenderAmountComponent> for TenderResultArrayOfHigherTenderAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfHigherTenderAmountComponent> for TenderResultArrayOfHigherTenderAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfHigherTenderAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfHigherTenderAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfHigherTenderAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HigherTenderAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HigherTenderAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HigherTenderAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HigherTenderAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfLowerTenderAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LowerTenderAmount>,
}

impl AsMut<TenderResultArrayOfLowerTenderAmountComponent> for TenderResultArrayOfLowerTenderAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfLowerTenderAmountComponent> for TenderResultArrayOfLowerTenderAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfLowerTenderAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfLowerTenderAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfLowerTenderAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LowerTenderAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LowerTenderAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LowerTenderAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LowerTenderAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfReceivedElectronicTenderQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReceivedElectronicTenderQuantity>,
}

impl AsMut<TenderResultArrayOfReceivedElectronicTenderQuantityComponent> for TenderResultArrayOfReceivedElectronicTenderQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfReceivedElectronicTenderQuantityComponent> for TenderResultArrayOfReceivedElectronicTenderQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfReceivedElectronicTenderQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfReceivedElectronicTenderQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfReceivedElectronicTenderQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReceivedElectronicTenderQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReceivedElectronicTenderQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReceivedElectronicTenderQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReceivedElectronicTenderQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfReceivedForeignTenderQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReceivedForeignTenderQuantity>,
}

impl AsMut<TenderResultArrayOfReceivedForeignTenderQuantityComponent> for TenderResultArrayOfReceivedForeignTenderQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfReceivedForeignTenderQuantityComponent> for TenderResultArrayOfReceivedForeignTenderQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfReceivedForeignTenderQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfReceivedForeignTenderQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfReceivedForeignTenderQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReceivedForeignTenderQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReceivedForeignTenderQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReceivedForeignTenderQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReceivedForeignTenderQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfReceivedTenderQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReceivedTenderQuantity>,
}

impl AsMut<TenderResultArrayOfReceivedTenderQuantityComponent> for TenderResultArrayOfReceivedTenderQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfReceivedTenderQuantityComponent> for TenderResultArrayOfReceivedTenderQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfReceivedTenderQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfReceivedTenderQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfReceivedTenderQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReceivedTenderQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReceivedTenderQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReceivedTenderQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReceivedTenderQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfStartDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::StartDate>,
}

impl AsMut<TenderResultArrayOfStartDateComponent> for TenderResultArrayOfStartDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfStartDateComponent> for TenderResultArrayOfStartDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfStartDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfStartDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfStartDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::StartDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::StartDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::StartDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::StartDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfSubcontractTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubcontractTerms>,
}

impl AsMut<TenderResultArrayOfSubcontractTermsComponent> for TenderResultArrayOfSubcontractTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfSubcontractTermsComponent> for TenderResultArrayOfSubcontractTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfSubcontractTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfSubcontractTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubcontractTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubcontractTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubcontractTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubcontractTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfTenderResultCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TenderResultCode>,
}

impl AsMut<TenderResultArrayOfTenderResultCodeComponent> for TenderResultArrayOfTenderResultCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfTenderResultCodeComponent> for TenderResultArrayOfTenderResultCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TenderResultArrayOfTenderResultCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfTenderResultCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfTenderResultCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TenderResultCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TenderResultCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TenderResultCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TenderResultCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TenderResultArrayOfWinningPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WinningParty>,
}

impl AsMut<TenderResultArrayOfWinningPartyComponent> for TenderResultArrayOfWinningPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TenderResultArrayOfWinningPartyComponent> for TenderResultArrayOfWinningPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TenderResultArrayOfWinningPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TenderResultArrayOfWinningPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WinningParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WinningParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WinningParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WinningParty> {
        self.items.iter()
    }
}


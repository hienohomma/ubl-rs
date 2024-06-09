use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompletedTask {
    #[serde(rename = "AnnualAverageAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_average_amount: Option<CompletedTaskArrayOfAnnualAverageAmountComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CompletedTaskArrayOfDescriptionComponent>,
    #[serde(rename = "EvidenceSupplied")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_supplied: Option<CompletedTaskArrayOfEvidenceSuppliedComponent>,
    #[serde(rename = "PartyCapacityAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_capacity_amount: Option<CompletedTaskArrayOfPartyCapacityAmountComponent>,
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<CompletedTaskArrayOfPeriodComponent>,
    #[serde(rename = "RecipientCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_customer_party: Option<CompletedTaskArrayOfRecipientCustomerPartyComponent>,
    #[serde(rename = "TotalTaskAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_task_amount: Option<CompletedTaskArrayOfTotalTaskAmountComponent>,
}

impl AsMut<CompletedTask> for CompletedTask {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CompletedTask> for CompletedTask {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.party_capacity_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CompletedTask.party_capacity_amount", e));
            }
        }
        if let Some(v) = &self.total_task_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CompletedTask.total_task_amount", e));
            }
        }
        if let Some(v) = &self.recipient_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CompletedTask.recipient_customer_party", e));
            }
        }
        if let Some(v) = &self.period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CompletedTask.period", e));
            }
        }
        if let Some(v) = &self.evidence_supplied {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CompletedTask.evidence_supplied", e));
            }
        }
        if let Some(v) = &self.annual_average_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CompletedTask.annual_average_amount", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CompletedTask.description", e));
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

impl CompletedTask {
    pub fn title() -> &'static str {
        "Completed Task. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the completion of a specific task in the tendering process."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            total_task_amount: None,
            period: None,
            description: None,
            party_capacity_amount: None,
            evidence_supplied: None,
            recipient_customer_party: None,
            annual_average_amount: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CompletedTaskArrayOfAnnualAverageAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AnnualAverageAmount>,
}

impl AsMut<CompletedTaskArrayOfAnnualAverageAmountComponent> for CompletedTaskArrayOfAnnualAverageAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CompletedTaskArrayOfAnnualAverageAmountComponent> for CompletedTaskArrayOfAnnualAverageAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfAnnualAverageAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfAnnualAverageAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CompletedTaskArrayOfAnnualAverageAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AnnualAverageAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AnnualAverageAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AnnualAverageAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AnnualAverageAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CompletedTaskArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<CompletedTaskArrayOfDescriptionComponent> for CompletedTaskArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CompletedTaskArrayOfDescriptionComponent> for CompletedTaskArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CompletedTaskArrayOfDescriptionComponent {
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
pub struct CompletedTaskArrayOfEvidenceSuppliedComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied>,
}

impl AsMut<CompletedTaskArrayOfEvidenceSuppliedComponent> for CompletedTaskArrayOfEvidenceSuppliedComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CompletedTaskArrayOfEvidenceSuppliedComponent> for CompletedTaskArrayOfEvidenceSuppliedComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfEvidenceSuppliedComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CompletedTaskArrayOfEvidenceSuppliedComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EvidenceSupplied> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CompletedTaskArrayOfPartyCapacityAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PartyCapacityAmount>,
}

impl AsMut<CompletedTaskArrayOfPartyCapacityAmountComponent> for CompletedTaskArrayOfPartyCapacityAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CompletedTaskArrayOfPartyCapacityAmountComponent> for CompletedTaskArrayOfPartyCapacityAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfPartyCapacityAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfPartyCapacityAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CompletedTaskArrayOfPartyCapacityAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PartyCapacityAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PartyCapacityAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PartyCapacityAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PartyCapacityAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CompletedTaskArrayOfPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Period>,
}

impl AsMut<CompletedTaskArrayOfPeriodComponent> for CompletedTaskArrayOfPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CompletedTaskArrayOfPeriodComponent> for CompletedTaskArrayOfPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CompletedTaskArrayOfPeriodComponent {
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
pub struct CompletedTaskArrayOfRecipientCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RecipientCustomerParty>,
}

impl AsMut<CompletedTaskArrayOfRecipientCustomerPartyComponent> for CompletedTaskArrayOfRecipientCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CompletedTaskArrayOfRecipientCustomerPartyComponent> for CompletedTaskArrayOfRecipientCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfRecipientCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfRecipientCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CompletedTaskArrayOfRecipientCustomerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RecipientCustomerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RecipientCustomerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RecipientCustomerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RecipientCustomerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CompletedTaskArrayOfTotalTaskAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalTaskAmount>,
}

impl AsMut<CompletedTaskArrayOfTotalTaskAmountComponent> for CompletedTaskArrayOfTotalTaskAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CompletedTaskArrayOfTotalTaskAmountComponent> for CompletedTaskArrayOfTotalTaskAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfTotalTaskAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CompletedTaskArrayOfTotalTaskAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CompletedTaskArrayOfTotalTaskAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalTaskAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalTaskAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalTaskAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalTaskAmount> {
        self.items.iter()
    }
}


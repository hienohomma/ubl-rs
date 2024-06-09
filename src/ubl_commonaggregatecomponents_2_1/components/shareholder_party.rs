use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShareholderParty {
    #[serde(rename = "PartecipationPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partecipation_percent: Option<ShareholderPartyArrayOfPartecipationPercentComponent>,
    #[serde(rename = "Party")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<ShareholderPartyArrayOfPartyComponent>,
}

impl AsMut<ShareholderParty> for ShareholderParty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShareholderParty> for ShareholderParty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShareholderParty.party", e));
            }
        }
        if let Some(v) = &self.partecipation_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ShareholderParty.partecipation_percent", e));
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

impl ShareholderParty {
    pub fn title() -> &'static str {
        "Shareholder Party. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a shareholder party."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            party: None,
            partecipation_percent: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShareholderPartyArrayOfPartecipationPercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PartecipationPercent>,
}

impl AsMut<ShareholderPartyArrayOfPartecipationPercentComponent> for ShareholderPartyArrayOfPartecipationPercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShareholderPartyArrayOfPartecipationPercentComponent> for ShareholderPartyArrayOfPartecipationPercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShareholderPartyArrayOfPartecipationPercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShareholderPartyArrayOfPartecipationPercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShareholderPartyArrayOfPartecipationPercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PartecipationPercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PartecipationPercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PartecipationPercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PartecipationPercent> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShareholderPartyArrayOfPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Party>,
}

impl AsMut<ShareholderPartyArrayOfPartyComponent> for ShareholderPartyArrayOfPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShareholderPartyArrayOfPartyComponent> for ShareholderPartyArrayOfPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShareholderPartyArrayOfPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShareholderPartyArrayOfPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShareholderPartyArrayOfPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Party) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Party) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Party> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Party> {
        self.items.iter()
    }
}


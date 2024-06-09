use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WinningParty {
    #[serde(rename = "Party")]
    pub party: WinningPartyArrayOfPartyComponent,
    #[serde(rename = "Rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<WinningPartyArrayOfRankComponent>,
}

impl AsMut<WinningParty> for WinningParty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WinningParty> for WinningParty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.rank {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("WinningParty.rank", e));
            }
        }
        if let Err(e) = self.party.validate() {
            return Err(UblError::component("WinningParty.party", e));
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

impl WinningParty {
    pub fn title() -> &'static str {
        "Winning Party. Details"
    }
    pub fn description() -> &'static str {
        "A party that is identified as the awarded by a tender result."
    }
    pub fn new(party: WinningPartyArrayOfPartyComponent) -> Component<Self> {
        Component(Self {
            party,
            rank: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WinningPartyArrayOfPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Party>,
}

impl AsMut<WinningPartyArrayOfPartyComponent> for WinningPartyArrayOfPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WinningPartyArrayOfPartyComponent> for WinningPartyArrayOfPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("WinningPartyArrayOfPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("WinningPartyArrayOfPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WinningPartyArrayOfPartyComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WinningPartyArrayOfRankComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Rank>,
}

impl AsMut<WinningPartyArrayOfRankComponent> for WinningPartyArrayOfRankComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<WinningPartyArrayOfRankComponent> for WinningPartyArrayOfRankComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("WinningPartyArrayOfRankComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("WinningPartyArrayOfRankComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl WinningPartyArrayOfRankComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Rank) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Rank) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Rank> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Rank> {
        self.items.iter()
    }
}


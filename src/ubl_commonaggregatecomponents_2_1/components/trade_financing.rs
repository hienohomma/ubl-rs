use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeFinancing {
    #[serde(rename = "Clause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clause: Option<TradeFinancingArrayOfClauseComponent>,
    #[serde(rename = "ContractDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_document_reference: Option<TradeFinancingArrayOfContractDocumentReferenceComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<TradeFinancingArrayOfDocumentReferenceComponent>,
    #[serde(rename = "FinancingFinancialAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financing_financial_account: Option<TradeFinancingArrayOfFinancingFinancialAccountComponent>,
    #[serde(rename = "FinancingInstrumentCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financing_instrument_code: Option<TradeFinancingArrayOfFinancingInstrumentCodeComponent>,
    #[serde(rename = "FinancingParty")]
    pub financing_party: TradeFinancingArrayOfFinancingPartyComponent,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TradeFinancingArrayOfIDComponent>,
}

impl AsMut<TradeFinancing> for TradeFinancing {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradeFinancing> for TradeFinancing {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.clause {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TradeFinancing.clause", e));
            }
        }
        if let Some(v) = &self.financing_instrument_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TradeFinancing.financing_instrument_code", e));
            }
        }
        if let Some(v) = &self.contract_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TradeFinancing.contract_document_reference", e));
            }
        }
        if let Err(e) = self.financing_party.validate() {
            return Err(UblError::component("TradeFinancing.financing_party", e));
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TradeFinancing.id", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TradeFinancing.document_reference", e));
            }
        }
        if let Some(v) = &self.financing_financial_account {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TradeFinancing.financing_financial_account", e));
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

impl TradeFinancing {
    pub fn title() -> &'static str {
        "Trade Financing. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a trade financing instrument."
    }
    pub fn new(financing_party: TradeFinancingArrayOfFinancingPartyComponent) -> Component<Self> {
        Component(Self {
            clause: None,
            document_reference: None,
            financing_instrument_code: None,
            contract_document_reference: None,
            financing_financial_account: None,
            financing_party,
            id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TradeFinancingArrayOfClauseComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Clause>,
}

impl AsMut<TradeFinancingArrayOfClauseComponent> for TradeFinancingArrayOfClauseComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradeFinancingArrayOfClauseComponent> for TradeFinancingArrayOfClauseComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfClauseComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TradeFinancingArrayOfClauseComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Clause) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Clause) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Clause> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Clause> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TradeFinancingArrayOfContractDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractDocumentReference>,
}

impl AsMut<TradeFinancingArrayOfContractDocumentReferenceComponent> for TradeFinancingArrayOfContractDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradeFinancingArrayOfContractDocumentReferenceComponent> for TradeFinancingArrayOfContractDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfContractDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfContractDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TradeFinancingArrayOfContractDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContractDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContractDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContractDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContractDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TradeFinancingArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<TradeFinancingArrayOfDocumentReferenceComponent> for TradeFinancingArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradeFinancingArrayOfDocumentReferenceComponent> for TradeFinancingArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TradeFinancingArrayOfDocumentReferenceComponent {
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
pub struct TradeFinancingArrayOfFinancingFinancialAccountComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FinancingFinancialAccount>,
}

impl AsMut<TradeFinancingArrayOfFinancingFinancialAccountComponent> for TradeFinancingArrayOfFinancingFinancialAccountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradeFinancingArrayOfFinancingFinancialAccountComponent> for TradeFinancingArrayOfFinancingFinancialAccountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfFinancingFinancialAccountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfFinancingFinancialAccountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TradeFinancingArrayOfFinancingFinancialAccountComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FinancingFinancialAccount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FinancingFinancialAccount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FinancingFinancialAccount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FinancingFinancialAccount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TradeFinancingArrayOfFinancingInstrumentCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FinancingInstrumentCode>,
}

impl AsMut<TradeFinancingArrayOfFinancingInstrumentCodeComponent> for TradeFinancingArrayOfFinancingInstrumentCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradeFinancingArrayOfFinancingInstrumentCodeComponent> for TradeFinancingArrayOfFinancingInstrumentCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfFinancingInstrumentCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfFinancingInstrumentCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TradeFinancingArrayOfFinancingInstrumentCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FinancingInstrumentCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FinancingInstrumentCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FinancingInstrumentCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FinancingInstrumentCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TradeFinancingArrayOfFinancingPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FinancingParty>,
}

impl AsMut<TradeFinancingArrayOfFinancingPartyComponent> for TradeFinancingArrayOfFinancingPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradeFinancingArrayOfFinancingPartyComponent> for TradeFinancingArrayOfFinancingPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfFinancingPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfFinancingPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TradeFinancingArrayOfFinancingPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FinancingParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FinancingParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FinancingParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FinancingParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TradeFinancingArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<TradeFinancingArrayOfIDComponent> for TradeFinancingArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TradeFinancingArrayOfIDComponent> for TradeFinancingArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TradeFinancingArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TradeFinancingArrayOfIDComponent {
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


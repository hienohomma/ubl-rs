use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocumentDistribution {
    #[serde(rename = "MaximumCopiesNumeric")]
    pub maximum_copies_numeric: DocumentDistributionArrayOfMaximumCopiesNumericComponent,
    #[serde(rename = "Party")]
    pub party: DocumentDistributionArrayOfPartyComponent,
    #[serde(rename = "PrintQualifier")]
    pub print_qualifier: DocumentDistributionArrayOfPrintQualifierComponent,
}

impl AsMut<DocumentDistribution> for DocumentDistribution {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentDistribution> for DocumentDistribution {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.print_qualifier.validate() {
            return Err(UblError::component("DocumentDistribution.print_qualifier", e));
        }
        if let Err(e) = self.maximum_copies_numeric.validate() {
            return Err(UblError::component("DocumentDistribution.maximum_copies_numeric", e));
        }
        if let Err(e) = self.party.validate() {
            return Err(UblError::component("DocumentDistribution.party", e));
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

impl DocumentDistribution {
    pub fn title() -> &'static str {
        "Document Distribution. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the distribution of a document to an interested party."
    }
    pub fn new(maximum_copies_numeric: DocumentDistributionArrayOfMaximumCopiesNumericComponent, party: DocumentDistributionArrayOfPartyComponent, print_qualifier: DocumentDistributionArrayOfPrintQualifierComponent) -> Component<Self> {
        Component(Self {
            party,
            print_qualifier,
            maximum_copies_numeric,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentDistributionArrayOfMaximumCopiesNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumCopiesNumeric>,
}

impl AsMut<DocumentDistributionArrayOfMaximumCopiesNumericComponent> for DocumentDistributionArrayOfMaximumCopiesNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentDistributionArrayOfMaximumCopiesNumericComponent> for DocumentDistributionArrayOfMaximumCopiesNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentDistributionArrayOfMaximumCopiesNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentDistributionArrayOfMaximumCopiesNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentDistributionArrayOfMaximumCopiesNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumCopiesNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumCopiesNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumCopiesNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumCopiesNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DocumentDistributionArrayOfPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Party>,
}

impl AsMut<DocumentDistributionArrayOfPartyComponent> for DocumentDistributionArrayOfPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentDistributionArrayOfPartyComponent> for DocumentDistributionArrayOfPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentDistributionArrayOfPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentDistributionArrayOfPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentDistributionArrayOfPartyComponent {
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
pub struct DocumentDistributionArrayOfPrintQualifierComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PrintQualifier>,
}

impl AsMut<DocumentDistributionArrayOfPrintQualifierComponent> for DocumentDistributionArrayOfPrintQualifierComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DocumentDistributionArrayOfPrintQualifierComponent> for DocumentDistributionArrayOfPrintQualifierComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DocumentDistributionArrayOfPrintQualifierComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DocumentDistributionArrayOfPrintQualifierComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DocumentDistributionArrayOfPrintQualifierComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PrintQualifier) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PrintQualifier) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PrintQualifier> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PrintQualifier> {
        self.items.iter()
    }
}


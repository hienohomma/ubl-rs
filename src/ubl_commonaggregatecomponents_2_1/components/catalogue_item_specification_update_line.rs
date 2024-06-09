use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatalogueItemSpecificationUpdateLine {
    #[serde(rename = "ContractorCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contractor_customer_party: Option<CatalogueItemSpecificationUpdateLineArrayOfContractorCustomerPartyComponent>,
    #[serde(rename = "ID")]
    pub id: CatalogueItemSpecificationUpdateLineArrayOfIDComponent,
    #[serde(rename = "Item")]
    pub item: CatalogueItemSpecificationUpdateLineArrayOfItemComponent,
    #[serde(rename = "SellerSupplierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<CatalogueItemSpecificationUpdateLineArrayOfSellerSupplierPartyComponent>,
}

impl AsMut<CatalogueItemSpecificationUpdateLine> for CatalogueItemSpecificationUpdateLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueItemSpecificationUpdateLine> for CatalogueItemSpecificationUpdateLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("CatalogueItemSpecificationUpdateLine.item", e));
        }
        if let Some(v) = &self.seller_supplier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueItemSpecificationUpdateLine.seller_supplier_party", e));
            }
        }
        if let Some(v) = &self.contractor_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueItemSpecificationUpdateLine.contractor_customer_party", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("CatalogueItemSpecificationUpdateLine.id", e));
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

impl CatalogueItemSpecificationUpdateLine {
    pub fn title() -> &'static str {
        "Catalogue Item Specification Update Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line describing the transaction that updates the specification of an item in a catalogue."
    }
    pub fn new(id: CatalogueItemSpecificationUpdateLineArrayOfIDComponent, item: CatalogueItemSpecificationUpdateLineArrayOfItemComponent) -> Component<Self> {
        Component(Self {
            contractor_customer_party: None,
            id,
            item,
            seller_supplier_party: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueItemSpecificationUpdateLineArrayOfContractorCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractorCustomerParty>,
}

impl AsMut<CatalogueItemSpecificationUpdateLineArrayOfContractorCustomerPartyComponent> for CatalogueItemSpecificationUpdateLineArrayOfContractorCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueItemSpecificationUpdateLineArrayOfContractorCustomerPartyComponent> for CatalogueItemSpecificationUpdateLineArrayOfContractorCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueItemSpecificationUpdateLineArrayOfContractorCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueItemSpecificationUpdateLineArrayOfContractorCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueItemSpecificationUpdateLineArrayOfContractorCustomerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContractorCustomerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContractorCustomerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContractorCustomerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContractorCustomerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueItemSpecificationUpdateLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<CatalogueItemSpecificationUpdateLineArrayOfIDComponent> for CatalogueItemSpecificationUpdateLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueItemSpecificationUpdateLineArrayOfIDComponent> for CatalogueItemSpecificationUpdateLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueItemSpecificationUpdateLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueItemSpecificationUpdateLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueItemSpecificationUpdateLineArrayOfIDComponent {
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
pub struct CatalogueItemSpecificationUpdateLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<CatalogueItemSpecificationUpdateLineArrayOfItemComponent> for CatalogueItemSpecificationUpdateLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueItemSpecificationUpdateLineArrayOfItemComponent> for CatalogueItemSpecificationUpdateLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueItemSpecificationUpdateLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueItemSpecificationUpdateLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueItemSpecificationUpdateLineArrayOfItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Item) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Item) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueItemSpecificationUpdateLineArrayOfSellerSupplierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty>,
}

impl AsMut<CatalogueItemSpecificationUpdateLineArrayOfSellerSupplierPartyComponent> for CatalogueItemSpecificationUpdateLineArrayOfSellerSupplierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueItemSpecificationUpdateLineArrayOfSellerSupplierPartyComponent> for CatalogueItemSpecificationUpdateLineArrayOfSellerSupplierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueItemSpecificationUpdateLineArrayOfSellerSupplierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueItemSpecificationUpdateLineArrayOfSellerSupplierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueItemSpecificationUpdateLineArrayOfSellerSupplierPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty> {
        self.items.iter()
    }
}


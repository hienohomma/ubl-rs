use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CataloguePricingUpdateLine {
    #[serde(rename = "ContractorCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contractor_customer_party: Option<CataloguePricingUpdateLineArrayOfContractorCustomerPartyComponent>,
    #[serde(rename = "ID")]
    pub id: CataloguePricingUpdateLineArrayOfIDComponent,
    #[serde(rename = "RequiredItemLocationQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_item_location_quantity: Option<CataloguePricingUpdateLineArrayOfRequiredItemLocationQuantityComponent>,
    #[serde(rename = "SellerSupplierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<CataloguePricingUpdateLineArrayOfSellerSupplierPartyComponent>,
}

impl AsMut<CataloguePricingUpdateLine> for CataloguePricingUpdateLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CataloguePricingUpdateLine> for CataloguePricingUpdateLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("CataloguePricingUpdateLine.id", e));
        }
        if let Some(v) = &self.required_item_location_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CataloguePricingUpdateLine.required_item_location_quantity", e));
            }
        }
        if let Some(v) = &self.contractor_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CataloguePricingUpdateLine.contractor_customer_party", e));
            }
        }
        if let Some(v) = &self.seller_supplier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CataloguePricingUpdateLine.seller_supplier_party", e));
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

impl CataloguePricingUpdateLine {
    pub fn title() -> &'static str {
        "Catalogue Pricing Update Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line describing a pricing update to a catalogue line."
    }
    pub fn new(id: CataloguePricingUpdateLineArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            required_item_location_quantity: None,
            contractor_customer_party: None,
            seller_supplier_party: None,
            id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CataloguePricingUpdateLineArrayOfContractorCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractorCustomerParty>,
}

impl AsMut<CataloguePricingUpdateLineArrayOfContractorCustomerPartyComponent> for CataloguePricingUpdateLineArrayOfContractorCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CataloguePricingUpdateLineArrayOfContractorCustomerPartyComponent> for CataloguePricingUpdateLineArrayOfContractorCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CataloguePricingUpdateLineArrayOfContractorCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CataloguePricingUpdateLineArrayOfContractorCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CataloguePricingUpdateLineArrayOfContractorCustomerPartyComponent {
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
pub struct CataloguePricingUpdateLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<CataloguePricingUpdateLineArrayOfIDComponent> for CataloguePricingUpdateLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CataloguePricingUpdateLineArrayOfIDComponent> for CataloguePricingUpdateLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CataloguePricingUpdateLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CataloguePricingUpdateLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CataloguePricingUpdateLineArrayOfIDComponent {
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
pub struct CataloguePricingUpdateLineArrayOfRequiredItemLocationQuantityComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity>,
}

impl AsMut<CataloguePricingUpdateLineArrayOfRequiredItemLocationQuantityComponent> for CataloguePricingUpdateLineArrayOfRequiredItemLocationQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CataloguePricingUpdateLineArrayOfRequiredItemLocationQuantityComponent> for CataloguePricingUpdateLineArrayOfRequiredItemLocationQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CataloguePricingUpdateLineArrayOfRequiredItemLocationQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CataloguePricingUpdateLineArrayOfRequiredItemLocationQuantityComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CataloguePricingUpdateLineArrayOfSellerSupplierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty>,
}

impl AsMut<CataloguePricingUpdateLineArrayOfSellerSupplierPartyComponent> for CataloguePricingUpdateLineArrayOfSellerSupplierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CataloguePricingUpdateLineArrayOfSellerSupplierPartyComponent> for CataloguePricingUpdateLineArrayOfSellerSupplierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CataloguePricingUpdateLineArrayOfSellerSupplierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CataloguePricingUpdateLineArrayOfSellerSupplierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CataloguePricingUpdateLineArrayOfSellerSupplierPartyComponent {
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


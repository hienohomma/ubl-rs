use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomerParty {
    #[serde(rename = "AccountingContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_contact: Option<CustomerPartyArrayOfAccountingContactComponent>,
    #[serde(rename = "AdditionalAccountID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_account_id: Option<CustomerPartyArrayOfAdditionalAccountIDComponent>,
    #[serde(rename = "BuyerContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_contact: Option<CustomerPartyArrayOfBuyerContactComponent>,
    #[serde(rename = "CustomerAssignedAccountID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_assigned_account_id: Option<CustomerPartyArrayOfCustomerAssignedAccountIDComponent>,
    #[serde(rename = "DeliveryContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_contact: Option<CustomerPartyArrayOfDeliveryContactComponent>,
    #[serde(rename = "Party")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<CustomerPartyArrayOfPartyComponent>,
    #[serde(rename = "SupplierAssignedAccountID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_assigned_account_id: Option<CustomerPartyArrayOfSupplierAssignedAccountIDComponent>,
}

impl AsMut<CustomerParty> for CustomerParty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CustomerParty> for CustomerParty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.additional_account_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CustomerParty.additional_account_id", e));
            }
        }
        if let Some(v) = &self.customer_assigned_account_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CustomerParty.customer_assigned_account_id", e));
            }
        }
        if let Some(v) = &self.party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CustomerParty.party", e));
            }
        }
        if let Some(v) = &self.supplier_assigned_account_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CustomerParty.supplier_assigned_account_id", e));
            }
        }
        if let Some(v) = &self.buyer_contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CustomerParty.buyer_contact", e));
            }
        }
        if let Some(v) = &self.delivery_contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CustomerParty.delivery_contact", e));
            }
        }
        if let Some(v) = &self.accounting_contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CustomerParty.accounting_contact", e));
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

impl CustomerParty {
    pub fn title() -> &'static str {
        "Customer Party. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a customer party."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            party: None,
            delivery_contact: None,
            supplier_assigned_account_id: None,
            buyer_contact: None,
            accounting_contact: None,
            additional_account_id: None,
            customer_assigned_account_id: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CustomerPartyArrayOfAccountingContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AccountingContact>,
}

impl AsMut<CustomerPartyArrayOfAccountingContactComponent> for CustomerPartyArrayOfAccountingContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CustomerPartyArrayOfAccountingContactComponent> for CustomerPartyArrayOfAccountingContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfAccountingContactComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfAccountingContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CustomerPartyArrayOfAccountingContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AccountingContact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AccountingContact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AccountingContact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AccountingContact> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CustomerPartyArrayOfAdditionalAccountIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AdditionalAccountID>,
}

impl AsMut<CustomerPartyArrayOfAdditionalAccountIDComponent> for CustomerPartyArrayOfAdditionalAccountIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CustomerPartyArrayOfAdditionalAccountIDComponent> for CustomerPartyArrayOfAdditionalAccountIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfAdditionalAccountIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CustomerPartyArrayOfAdditionalAccountIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AdditionalAccountID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AdditionalAccountID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AdditionalAccountID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AdditionalAccountID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CustomerPartyArrayOfBuyerContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BuyerContact>,
}

impl AsMut<CustomerPartyArrayOfBuyerContactComponent> for CustomerPartyArrayOfBuyerContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CustomerPartyArrayOfBuyerContactComponent> for CustomerPartyArrayOfBuyerContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfBuyerContactComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfBuyerContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CustomerPartyArrayOfBuyerContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BuyerContact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BuyerContact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BuyerContact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BuyerContact> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CustomerPartyArrayOfCustomerAssignedAccountIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CustomerAssignedAccountID>,
}

impl AsMut<CustomerPartyArrayOfCustomerAssignedAccountIDComponent> for CustomerPartyArrayOfCustomerAssignedAccountIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CustomerPartyArrayOfCustomerAssignedAccountIDComponent> for CustomerPartyArrayOfCustomerAssignedAccountIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfCustomerAssignedAccountIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfCustomerAssignedAccountIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CustomerPartyArrayOfCustomerAssignedAccountIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CustomerAssignedAccountID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CustomerAssignedAccountID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CustomerAssignedAccountID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CustomerAssignedAccountID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CustomerPartyArrayOfDeliveryContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryContact>,
}

impl AsMut<CustomerPartyArrayOfDeliveryContactComponent> for CustomerPartyArrayOfDeliveryContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CustomerPartyArrayOfDeliveryContactComponent> for CustomerPartyArrayOfDeliveryContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfDeliveryContactComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfDeliveryContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CustomerPartyArrayOfDeliveryContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryContact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryContact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryContact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryContact> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CustomerPartyArrayOfPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Party>,
}

impl AsMut<CustomerPartyArrayOfPartyComponent> for CustomerPartyArrayOfPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CustomerPartyArrayOfPartyComponent> for CustomerPartyArrayOfPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CustomerPartyArrayOfPartyComponent {
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
pub struct CustomerPartyArrayOfSupplierAssignedAccountIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SupplierAssignedAccountID>,
}

impl AsMut<CustomerPartyArrayOfSupplierAssignedAccountIDComponent> for CustomerPartyArrayOfSupplierAssignedAccountIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CustomerPartyArrayOfSupplierAssignedAccountIDComponent> for CustomerPartyArrayOfSupplierAssignedAccountIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfSupplierAssignedAccountIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CustomerPartyArrayOfSupplierAssignedAccountIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CustomerPartyArrayOfSupplierAssignedAccountIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SupplierAssignedAccountID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SupplierAssignedAccountID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SupplierAssignedAccountID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SupplierAssignedAccountID> {
        self.items.iter()
    }
}


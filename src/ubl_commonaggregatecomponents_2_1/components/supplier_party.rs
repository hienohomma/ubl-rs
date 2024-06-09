use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SupplierParty {
    #[serde(rename = "AccountingContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_contact: Option<SupplierPartyArrayOfAccountingContactComponent>,
    #[serde(rename = "AdditionalAccountID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_account_id: Option<SupplierPartyArrayOfAdditionalAccountIDComponent>,
    #[serde(rename = "CustomerAssignedAccountID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_assigned_account_id: Option<SupplierPartyArrayOfCustomerAssignedAccountIDComponent>,
    #[serde(rename = "DataSendingCapability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sending_capability: Option<SupplierPartyArrayOfDataSendingCapabilityComponent>,
    #[serde(rename = "DespatchContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_contact: Option<SupplierPartyArrayOfDespatchContactComponent>,
    #[serde(rename = "Party")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<SupplierPartyArrayOfPartyComponent>,
    #[serde(rename = "SellerContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_contact: Option<SupplierPartyArrayOfSellerContactComponent>,
}

impl AsMut<SupplierParty> for SupplierParty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierParty> for SupplierParty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.seller_contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SupplierParty.seller_contact", e));
            }
        }
        if let Some(v) = &self.additional_account_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SupplierParty.additional_account_id", e));
            }
        }
        if let Some(v) = &self.accounting_contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SupplierParty.accounting_contact", e));
            }
        }
        if let Some(v) = &self.customer_assigned_account_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SupplierParty.customer_assigned_account_id", e));
            }
        }
        if let Some(v) = &self.despatch_contact {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SupplierParty.despatch_contact", e));
            }
        }
        if let Some(v) = &self.party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SupplierParty.party", e));
            }
        }
        if let Some(v) = &self.data_sending_capability {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SupplierParty.data_sending_capability", e));
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

impl SupplierParty {
    pub fn title() -> &'static str {
        "Supplier Party. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a supplier party."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            accounting_contact: None,
            data_sending_capability: None,
            additional_account_id: None,
            seller_contact: None,
            despatch_contact: None,
            customer_assigned_account_id: None,
            party: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SupplierPartyArrayOfAccountingContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AccountingContact>,
}

impl AsMut<SupplierPartyArrayOfAccountingContactComponent> for SupplierPartyArrayOfAccountingContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierPartyArrayOfAccountingContactComponent> for SupplierPartyArrayOfAccountingContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfAccountingContactComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfAccountingContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SupplierPartyArrayOfAccountingContactComponent {
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
pub struct SupplierPartyArrayOfAdditionalAccountIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AdditionalAccountID>,
}

impl AsMut<SupplierPartyArrayOfAdditionalAccountIDComponent> for SupplierPartyArrayOfAdditionalAccountIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierPartyArrayOfAdditionalAccountIDComponent> for SupplierPartyArrayOfAdditionalAccountIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfAdditionalAccountIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SupplierPartyArrayOfAdditionalAccountIDComponent {
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
pub struct SupplierPartyArrayOfCustomerAssignedAccountIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CustomerAssignedAccountID>,
}

impl AsMut<SupplierPartyArrayOfCustomerAssignedAccountIDComponent> for SupplierPartyArrayOfCustomerAssignedAccountIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierPartyArrayOfCustomerAssignedAccountIDComponent> for SupplierPartyArrayOfCustomerAssignedAccountIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfCustomerAssignedAccountIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfCustomerAssignedAccountIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SupplierPartyArrayOfCustomerAssignedAccountIDComponent {
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
pub struct SupplierPartyArrayOfDataSendingCapabilityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DataSendingCapability>,
}

impl AsMut<SupplierPartyArrayOfDataSendingCapabilityComponent> for SupplierPartyArrayOfDataSendingCapabilityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierPartyArrayOfDataSendingCapabilityComponent> for SupplierPartyArrayOfDataSendingCapabilityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfDataSendingCapabilityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfDataSendingCapabilityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SupplierPartyArrayOfDataSendingCapabilityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DataSendingCapability) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DataSendingCapability) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DataSendingCapability> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DataSendingCapability> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SupplierPartyArrayOfDespatchContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DespatchContact>,
}

impl AsMut<SupplierPartyArrayOfDespatchContactComponent> for SupplierPartyArrayOfDespatchContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierPartyArrayOfDespatchContactComponent> for SupplierPartyArrayOfDespatchContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfDespatchContactComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfDespatchContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SupplierPartyArrayOfDespatchContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DespatchContact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DespatchContact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DespatchContact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DespatchContact> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SupplierPartyArrayOfPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Party>,
}

impl AsMut<SupplierPartyArrayOfPartyComponent> for SupplierPartyArrayOfPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierPartyArrayOfPartyComponent> for SupplierPartyArrayOfPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SupplierPartyArrayOfPartyComponent {
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
pub struct SupplierPartyArrayOfSellerContactComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellerContact>,
}

impl AsMut<SupplierPartyArrayOfSellerContactComponent> for SupplierPartyArrayOfSellerContactComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierPartyArrayOfSellerContactComponent> for SupplierPartyArrayOfSellerContactComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfSellerContactComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SupplierPartyArrayOfSellerContactComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SupplierPartyArrayOfSellerContactComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SellerContact) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SellerContact) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SellerContact> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SellerContact> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityDataLine {
    #[serde(rename = "ActivityFinalLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_final_location: Option<ActivityDataLineArrayOfActivityFinalLocationComponent>,
    #[serde(rename = "ActivityOriginLocation")]
    pub activity_origin_location: ActivityDataLineArrayOfActivityOriginLocationComponent,
    #[serde(rename = "ActivityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_period: Option<ActivityDataLineArrayOfActivityPeriodComponent>,
    #[serde(rename = "BuyerCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<ActivityDataLineArrayOfBuyerCustomerPartyComponent>,
    #[serde(rename = "ID")]
    pub id: ActivityDataLineArrayOfIDComponent,
    #[serde(rename = "SalesItem")]
    pub sales_item: ActivityDataLineArrayOfSalesItemComponent,
    #[serde(rename = "SellerSupplierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<ActivityDataLineArrayOfSellerSupplierPartyComponent>,
    #[serde(rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: ActivityDataLineArrayOfSupplyChainActivityTypeCodeComponent,
}

impl AsMut<ActivityDataLine> for ActivityDataLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityDataLine> for ActivityDataLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.seller_supplier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ActivityDataLine.seller_supplier_party", e));
            }
        }
        if let Some(v) = &self.buyer_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ActivityDataLine.buyer_customer_party", e));
            }
        }
        if let Some(v) = &self.activity_final_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ActivityDataLine.activity_final_location", e));
            }
        }
        if let Err(e) = self.activity_origin_location.validate() {
            return Err(UblError::component("ActivityDataLine.activity_origin_location", e));
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("ActivityDataLine.id", e));
        }
        if let Some(v) = &self.activity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ActivityDataLine.activity_period", e));
            }
        }
        if let Err(e) = self.supply_chain_activity_type_code.validate() {
            return Err(UblError::component("ActivityDataLine.supply_chain_activity_type_code", e));
        }
        if let Err(e) = self.sales_item.validate() {
            return Err(UblError::component("ActivityDataLine.sales_item", e));
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

impl ActivityDataLine {
    pub fn title() -> &'static str {
        "Activity Data Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to associate a time period and locations (activity data) with an item for inventory planning purposes."
    }
    pub fn new(activity_origin_location: ActivityDataLineArrayOfActivityOriginLocationComponent, id: ActivityDataLineArrayOfIDComponent, sales_item: ActivityDataLineArrayOfSalesItemComponent, supply_chain_activity_type_code: ActivityDataLineArrayOfSupplyChainActivityTypeCodeComponent) -> Component<Self> {
        Component(Self {
            buyer_customer_party: None,
            seller_supplier_party: None,
            activity_origin_location,
            sales_item,
            supply_chain_activity_type_code,
            activity_final_location: None,
            activity_period: None,
            id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ActivityDataLineArrayOfActivityFinalLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ActivityFinalLocation>,
}

impl AsMut<ActivityDataLineArrayOfActivityFinalLocationComponent> for ActivityDataLineArrayOfActivityFinalLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityDataLineArrayOfActivityFinalLocationComponent> for ActivityDataLineArrayOfActivityFinalLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfActivityFinalLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfActivityFinalLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ActivityDataLineArrayOfActivityFinalLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ActivityFinalLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ActivityFinalLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ActivityFinalLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ActivityFinalLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ActivityDataLineArrayOfActivityOriginLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ActivityOriginLocation>,
}

impl AsMut<ActivityDataLineArrayOfActivityOriginLocationComponent> for ActivityDataLineArrayOfActivityOriginLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityDataLineArrayOfActivityOriginLocationComponent> for ActivityDataLineArrayOfActivityOriginLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfActivityOriginLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfActivityOriginLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ActivityDataLineArrayOfActivityOriginLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ActivityOriginLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ActivityOriginLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ActivityOriginLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ActivityOriginLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ActivityDataLineArrayOfActivityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ActivityPeriod>,
}

impl AsMut<ActivityDataLineArrayOfActivityPeriodComponent> for ActivityDataLineArrayOfActivityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityDataLineArrayOfActivityPeriodComponent> for ActivityDataLineArrayOfActivityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfActivityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfActivityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ActivityDataLineArrayOfActivityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ActivityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ActivityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ActivityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ActivityPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ActivityDataLineArrayOfBuyerCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BuyerCustomerParty>,
}

impl AsMut<ActivityDataLineArrayOfBuyerCustomerPartyComponent> for ActivityDataLineArrayOfBuyerCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityDataLineArrayOfBuyerCustomerPartyComponent> for ActivityDataLineArrayOfBuyerCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfBuyerCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfBuyerCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ActivityDataLineArrayOfBuyerCustomerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BuyerCustomerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BuyerCustomerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BuyerCustomerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BuyerCustomerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ActivityDataLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ActivityDataLineArrayOfIDComponent> for ActivityDataLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityDataLineArrayOfIDComponent> for ActivityDataLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ActivityDataLineArrayOfIDComponent {
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
pub struct ActivityDataLineArrayOfSalesItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SalesItem>,
}

impl AsMut<ActivityDataLineArrayOfSalesItemComponent> for ActivityDataLineArrayOfSalesItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityDataLineArrayOfSalesItemComponent> for ActivityDataLineArrayOfSalesItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfSalesItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ActivityDataLineArrayOfSalesItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SalesItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SalesItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SalesItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SalesItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ActivityDataLineArrayOfSellerSupplierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty>,
}

impl AsMut<ActivityDataLineArrayOfSellerSupplierPartyComponent> for ActivityDataLineArrayOfSellerSupplierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityDataLineArrayOfSellerSupplierPartyComponent> for ActivityDataLineArrayOfSellerSupplierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfSellerSupplierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfSellerSupplierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ActivityDataLineArrayOfSellerSupplierPartyComponent {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ActivityDataLineArrayOfSupplyChainActivityTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode>,
}

impl AsMut<ActivityDataLineArrayOfSupplyChainActivityTypeCodeComponent> for ActivityDataLineArrayOfSupplyChainActivityTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ActivityDataLineArrayOfSupplyChainActivityTypeCodeComponent> for ActivityDataLineArrayOfSupplyChainActivityTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfSupplyChainActivityTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ActivityDataLineArrayOfSupplyChainActivityTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ActivityDataLineArrayOfSupplyChainActivityTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SupplyChainActivityTypeCode> {
        self.items.iter()
    }
}


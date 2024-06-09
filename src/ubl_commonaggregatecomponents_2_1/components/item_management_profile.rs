use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemManagementProfile {
    #[serde(rename = "EffectivePeriod")]
    pub effective_period: ItemManagementProfileArrayOfEffectivePeriodComponent,
    #[serde(rename = "FrozenPeriodDaysNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_period_days_numeric: Option<ItemManagementProfileArrayOfFrozenPeriodDaysNumericComponent>,
    #[serde(rename = "Item")]
    pub item: ItemManagementProfileArrayOfItemComponent,
    #[serde(rename = "ItemLocationQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_location_quantity: Option<ItemManagementProfileArrayOfItemLocationQuantityComponent>,
    #[serde(rename = "MinimumInventoryQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_inventory_quantity: Option<ItemManagementProfileArrayOfMinimumInventoryQuantityComponent>,
    #[serde(rename = "MultipleOrderQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_order_quantity: Option<ItemManagementProfileArrayOfMultipleOrderQuantityComponent>,
    #[serde(rename = "OrderIntervalDaysNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_interval_days_numeric: Option<ItemManagementProfileArrayOfOrderIntervalDaysNumericComponent>,
    #[serde(rename = "ReplenishmentOwnerDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replenishment_owner_description: Option<ItemManagementProfileArrayOfReplenishmentOwnerDescriptionComponent>,
    #[serde(rename = "TargetInventoryQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_inventory_quantity: Option<ItemManagementProfileArrayOfTargetInventoryQuantityComponent>,
    #[serde(rename = "TargetServicePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_service_percent: Option<ItemManagementProfileArrayOfTargetServicePercentComponent>,
}

impl AsMut<ItemManagementProfile> for ItemManagementProfile {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemManagementProfile> for ItemManagementProfile {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.item_location_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemManagementProfile.item_location_quantity", e));
            }
        }
        if let Some(v) = &self.order_interval_days_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemManagementProfile.order_interval_days_numeric", e));
            }
        }
        if let Some(v) = &self.target_service_percent {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemManagementProfile.target_service_percent", e));
            }
        }
        if let Err(e) = self.effective_period.validate() {
            return Err(UblError::component("ItemManagementProfile.effective_period", e));
        }
        if let Some(v) = &self.multiple_order_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemManagementProfile.multiple_order_quantity", e));
            }
        }
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("ItemManagementProfile.item", e));
        }
        if let Some(v) = &self.replenishment_owner_description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemManagementProfile.replenishment_owner_description", e));
            }
        }
        if let Some(v) = &self.target_inventory_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemManagementProfile.target_inventory_quantity", e));
            }
        }
        if let Some(v) = &self.frozen_period_days_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemManagementProfile.frozen_period_days_numeric", e));
            }
        }
        if let Some(v) = &self.minimum_inventory_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemManagementProfile.minimum_inventory_quantity", e));
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

impl ItemManagementProfile {
    pub fn title() -> &'static str {
        "Item Management Profile. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a management profile for an item."
    }
    pub fn new(effective_period: ItemManagementProfileArrayOfEffectivePeriodComponent, item: ItemManagementProfileArrayOfItemComponent) -> Component<Self> {
        Component(Self {
            item_location_quantity: None,
            frozen_period_days_numeric: None,
            target_inventory_quantity: None,
            minimum_inventory_quantity: None,
            replenishment_owner_description: None,
            multiple_order_quantity: None,
            item,
            order_interval_days_numeric: None,
            target_service_percent: None,
            effective_period,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemManagementProfileArrayOfEffectivePeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EffectivePeriod>,
}

impl AsMut<ItemManagementProfileArrayOfEffectivePeriodComponent> for ItemManagementProfileArrayOfEffectivePeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemManagementProfileArrayOfEffectivePeriodComponent> for ItemManagementProfileArrayOfEffectivePeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfEffectivePeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfEffectivePeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemManagementProfileArrayOfEffectivePeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EffectivePeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EffectivePeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EffectivePeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EffectivePeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemManagementProfileArrayOfFrozenPeriodDaysNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FrozenPeriodDaysNumeric>,
}

impl AsMut<ItemManagementProfileArrayOfFrozenPeriodDaysNumericComponent> for ItemManagementProfileArrayOfFrozenPeriodDaysNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemManagementProfileArrayOfFrozenPeriodDaysNumericComponent> for ItemManagementProfileArrayOfFrozenPeriodDaysNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfFrozenPeriodDaysNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfFrozenPeriodDaysNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemManagementProfileArrayOfFrozenPeriodDaysNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FrozenPeriodDaysNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FrozenPeriodDaysNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FrozenPeriodDaysNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FrozenPeriodDaysNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemManagementProfileArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<ItemManagementProfileArrayOfItemComponent> for ItemManagementProfileArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemManagementProfileArrayOfItemComponent> for ItemManagementProfileArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemManagementProfileArrayOfItemComponent {
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
pub struct ItemManagementProfileArrayOfItemLocationQuantityComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ItemLocationQuantity>,
}

impl AsMut<ItemManagementProfileArrayOfItemLocationQuantityComponent> for ItemManagementProfileArrayOfItemLocationQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemManagementProfileArrayOfItemLocationQuantityComponent> for ItemManagementProfileArrayOfItemLocationQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfItemLocationQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfItemLocationQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemManagementProfileArrayOfItemLocationQuantityComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ItemLocationQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ItemLocationQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ItemLocationQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ItemLocationQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemManagementProfileArrayOfMinimumInventoryQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumInventoryQuantity>,
}

impl AsMut<ItemManagementProfileArrayOfMinimumInventoryQuantityComponent> for ItemManagementProfileArrayOfMinimumInventoryQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemManagementProfileArrayOfMinimumInventoryQuantityComponent> for ItemManagementProfileArrayOfMinimumInventoryQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfMinimumInventoryQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfMinimumInventoryQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemManagementProfileArrayOfMinimumInventoryQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumInventoryQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumInventoryQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumInventoryQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumInventoryQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemManagementProfileArrayOfMultipleOrderQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MultipleOrderQuantity>,
}

impl AsMut<ItemManagementProfileArrayOfMultipleOrderQuantityComponent> for ItemManagementProfileArrayOfMultipleOrderQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemManagementProfileArrayOfMultipleOrderQuantityComponent> for ItemManagementProfileArrayOfMultipleOrderQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfMultipleOrderQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfMultipleOrderQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemManagementProfileArrayOfMultipleOrderQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MultipleOrderQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MultipleOrderQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MultipleOrderQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MultipleOrderQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemManagementProfileArrayOfOrderIntervalDaysNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OrderIntervalDaysNumeric>,
}

impl AsMut<ItemManagementProfileArrayOfOrderIntervalDaysNumericComponent> for ItemManagementProfileArrayOfOrderIntervalDaysNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemManagementProfileArrayOfOrderIntervalDaysNumericComponent> for ItemManagementProfileArrayOfOrderIntervalDaysNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfOrderIntervalDaysNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfOrderIntervalDaysNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemManagementProfileArrayOfOrderIntervalDaysNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OrderIntervalDaysNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OrderIntervalDaysNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OrderIntervalDaysNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OrderIntervalDaysNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemManagementProfileArrayOfReplenishmentOwnerDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReplenishmentOwnerDescription>,
}

impl AsMut<ItemManagementProfileArrayOfReplenishmentOwnerDescriptionComponent> for ItemManagementProfileArrayOfReplenishmentOwnerDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemManagementProfileArrayOfReplenishmentOwnerDescriptionComponent> for ItemManagementProfileArrayOfReplenishmentOwnerDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfReplenishmentOwnerDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemManagementProfileArrayOfReplenishmentOwnerDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReplenishmentOwnerDescription) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReplenishmentOwnerDescription) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReplenishmentOwnerDescription> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReplenishmentOwnerDescription> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemManagementProfileArrayOfTargetInventoryQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TargetInventoryQuantity>,
}

impl AsMut<ItemManagementProfileArrayOfTargetInventoryQuantityComponent> for ItemManagementProfileArrayOfTargetInventoryQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemManagementProfileArrayOfTargetInventoryQuantityComponent> for ItemManagementProfileArrayOfTargetInventoryQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfTargetInventoryQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfTargetInventoryQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemManagementProfileArrayOfTargetInventoryQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TargetInventoryQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TargetInventoryQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TargetInventoryQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TargetInventoryQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemManagementProfileArrayOfTargetServicePercentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TargetServicePercent>,
}

impl AsMut<ItemManagementProfileArrayOfTargetServicePercentComponent> for ItemManagementProfileArrayOfTargetServicePercentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemManagementProfileArrayOfTargetServicePercentComponent> for ItemManagementProfileArrayOfTargetServicePercentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfTargetServicePercentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemManagementProfileArrayOfTargetServicePercentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemManagementProfileArrayOfTargetServicePercentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TargetServicePercent) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TargetServicePercent) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TargetServicePercent> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TargetServicePercent> {
        self.items.iter()
    }
}


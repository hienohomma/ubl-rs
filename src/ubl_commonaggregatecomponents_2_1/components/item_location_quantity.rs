use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemLocationQuantity {
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<ItemLocationQuantityArrayOfAllowanceChargeComponent>,
    #[serde(rename = "ApplicableTaxCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_tax_category: Option<ItemLocationQuantityArrayOfApplicableTaxCategoryComponent>,
    #[serde(rename = "ApplicableTerritoryAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_territory_address: Option<ItemLocationQuantityArrayOfApplicableTerritoryAddressComponent>,
    #[serde(rename = "DeliveryUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_unit: Option<ItemLocationQuantityArrayOfDeliveryUnitComponent>,
    #[serde(rename = "DependentPriceReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_price_reference: Option<ItemLocationQuantityArrayOfDependentPriceReferenceComponent>,
    #[serde(rename = "HazardousRiskIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_risk_indicator: Option<ItemLocationQuantityArrayOfHazardousRiskIndicatorComponent>,
    #[serde(rename = "LeadTimeMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_time_measure: Option<ItemLocationQuantityArrayOfLeadTimeMeasureComponent>,
    #[serde(rename = "MaximumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_quantity: Option<ItemLocationQuantityArrayOfMaximumQuantityComponent>,
    #[serde(rename = "MinimumQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_quantity: Option<ItemLocationQuantityArrayOfMinimumQuantityComponent>,
    #[serde(rename = "Package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<ItemLocationQuantityArrayOfPackageComponent>,
    #[serde(rename = "Price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<ItemLocationQuantityArrayOfPriceComponent>,
    #[serde(rename = "TradingRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_restrictions: Option<ItemLocationQuantityArrayOfTradingRestrictionsComponent>,
}

impl AsMut<ItemLocationQuantity> for ItemLocationQuantity {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantity> for ItemLocationQuantity {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.package {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.package", e));
            }
        }
        if let Some(v) = &self.price {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.price", e));
            }
        }
        if let Some(v) = &self.applicable_tax_category {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.applicable_tax_category", e));
            }
        }
        if let Some(v) = &self.trading_restrictions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.trading_restrictions", e));
            }
        }
        if let Some(v) = &self.maximum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.maximum_quantity", e));
            }
        }
        if let Some(v) = &self.lead_time_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.lead_time_measure", e));
            }
        }
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.allowance_charge", e));
            }
        }
        if let Some(v) = &self.dependent_price_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.dependent_price_reference", e));
            }
        }
        if let Some(v) = &self.hazardous_risk_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.hazardous_risk_indicator", e));
            }
        }
        if let Some(v) = &self.delivery_unit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.delivery_unit", e));
            }
        }
        if let Some(v) = &self.applicable_territory_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.applicable_territory_address", e));
            }
        }
        if let Some(v) = &self.minimum_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemLocationQuantity.minimum_quantity", e));
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

impl ItemLocationQuantity {
    pub fn title() -> &'static str {
        "Item Location Quantity. Details"
    }
    pub fn description() -> &'static str {
        "A class for information about pricing structure, lead time, and location associated with an item."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            applicable_territory_address: None,
            price: None,
            maximum_quantity: None,
            hazardous_risk_indicator: None,
            lead_time_measure: None,
            delivery_unit: None,
            package: None,
            allowance_charge: None,
            dependent_price_reference: None,
            applicable_tax_category: None,
            minimum_quantity: None,
            trading_restrictions: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<ItemLocationQuantityArrayOfAllowanceChargeComponent> for ItemLocationQuantityArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfAllowanceChargeComponent> for ItemLocationQuantityArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfAllowanceChargeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfApplicableTaxCategoryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ApplicableTaxCategory>,
}

impl AsMut<ItemLocationQuantityArrayOfApplicableTaxCategoryComponent> for ItemLocationQuantityArrayOfApplicableTaxCategoryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfApplicableTaxCategoryComponent> for ItemLocationQuantityArrayOfApplicableTaxCategoryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfApplicableTaxCategoryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfApplicableTaxCategoryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ApplicableTaxCategory) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ApplicableTaxCategory) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ApplicableTaxCategory> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ApplicableTaxCategory> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfApplicableTerritoryAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ApplicableTerritoryAddress>,
}

impl AsMut<ItemLocationQuantityArrayOfApplicableTerritoryAddressComponent> for ItemLocationQuantityArrayOfApplicableTerritoryAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfApplicableTerritoryAddressComponent> for ItemLocationQuantityArrayOfApplicableTerritoryAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfApplicableTerritoryAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfApplicableTerritoryAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ApplicableTerritoryAddress) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ApplicableTerritoryAddress) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ApplicableTerritoryAddress> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ApplicableTerritoryAddress> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfDeliveryUnitComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryUnit>,
}

impl AsMut<ItemLocationQuantityArrayOfDeliveryUnitComponent> for ItemLocationQuantityArrayOfDeliveryUnitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfDeliveryUnitComponent> for ItemLocationQuantityArrayOfDeliveryUnitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfDeliveryUnitComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfDeliveryUnitComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryUnit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryUnit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryUnit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryUnit> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfDependentPriceReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DependentPriceReference>,
}

impl AsMut<ItemLocationQuantityArrayOfDependentPriceReferenceComponent> for ItemLocationQuantityArrayOfDependentPriceReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfDependentPriceReferenceComponent> for ItemLocationQuantityArrayOfDependentPriceReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfDependentPriceReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfDependentPriceReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfDependentPriceReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DependentPriceReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DependentPriceReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DependentPriceReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DependentPriceReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfHazardousRiskIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator>,
}

impl AsMut<ItemLocationQuantityArrayOfHazardousRiskIndicatorComponent> for ItemLocationQuantityArrayOfHazardousRiskIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfHazardousRiskIndicatorComponent> for ItemLocationQuantityArrayOfHazardousRiskIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfHazardousRiskIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfHazardousRiskIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfHazardousRiskIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfLeadTimeMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LeadTimeMeasure>,
}

impl AsMut<ItemLocationQuantityArrayOfLeadTimeMeasureComponent> for ItemLocationQuantityArrayOfLeadTimeMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfLeadTimeMeasureComponent> for ItemLocationQuantityArrayOfLeadTimeMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfLeadTimeMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfLeadTimeMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfLeadTimeMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LeadTimeMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LeadTimeMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LeadTimeMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LeadTimeMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfMaximumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity>,
}

impl AsMut<ItemLocationQuantityArrayOfMaximumQuantityComponent> for ItemLocationQuantityArrayOfMaximumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfMaximumQuantityComponent> for ItemLocationQuantityArrayOfMaximumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfMaximumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfMaximumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfMaximumQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfMinimumQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity>,
}

impl AsMut<ItemLocationQuantityArrayOfMinimumQuantityComponent> for ItemLocationQuantityArrayOfMinimumQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfMinimumQuantityComponent> for ItemLocationQuantityArrayOfMinimumQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfMinimumQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfMinimumQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfMinimumQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfPackageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Package>,
}

impl AsMut<ItemLocationQuantityArrayOfPackageComponent> for ItemLocationQuantityArrayOfPackageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfPackageComponent> for ItemLocationQuantityArrayOfPackageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfPackageComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfPackageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfPackageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Package) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Package) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Package> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Package> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfPriceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Price>,
}

impl AsMut<ItemLocationQuantityArrayOfPriceComponent> for ItemLocationQuantityArrayOfPriceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfPriceComponent> for ItemLocationQuantityArrayOfPriceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfPriceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfPriceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfPriceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Price) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Price) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Price> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Price> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemLocationQuantityArrayOfTradingRestrictionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TradingRestrictions>,
}

impl AsMut<ItemLocationQuantityArrayOfTradingRestrictionsComponent> for ItemLocationQuantityArrayOfTradingRestrictionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemLocationQuantityArrayOfTradingRestrictionsComponent> for ItemLocationQuantityArrayOfTradingRestrictionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemLocationQuantityArrayOfTradingRestrictionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemLocationQuantityArrayOfTradingRestrictionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TradingRestrictions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TradingRestrictions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TradingRestrictions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TradingRestrictions> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Price {
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<PriceArrayOfAllowanceChargeComponent>,
    #[serde(rename = "BaseQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_quantity: Option<PriceArrayOfBaseQuantityComponent>,
    #[serde(rename = "OrderableUnitFactorRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable_unit_factor_rate: Option<PriceArrayOfOrderableUnitFactorRateComponent>,
    #[serde(rename = "PriceAmount")]
    pub price_amount: PriceArrayOfPriceAmountComponent,
    #[serde(rename = "PriceChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_change_reason: Option<PriceArrayOfPriceChangeReasonComponent>,
    #[serde(rename = "PriceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_list: Option<PriceArrayOfPriceListComponent>,
    #[serde(rename = "PriceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type: Option<PriceArrayOfPriceTypeComponent>,
    #[serde(rename = "PriceTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type_code: Option<PriceArrayOfPriceTypeCodeComponent>,
    #[serde(rename = "PricingExchangeRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_exchange_rate: Option<PriceArrayOfPricingExchangeRateComponent>,
    #[serde(rename = "ValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<PriceArrayOfValidityPeriodComponent>,
}

impl AsMut<Price> for Price {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Price> for Price {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Price.validity_period", e));
            }
        }
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Price.allowance_charge", e));
            }
        }
        if let Some(v) = &self.orderable_unit_factor_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Price.orderable_unit_factor_rate", e));
            }
        }
        if let Some(v) = &self.price_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Price.price_type", e));
            }
        }
        if let Some(v) = &self.pricing_exchange_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Price.pricing_exchange_rate", e));
            }
        }
        if let Err(e) = self.price_amount.validate() {
            return Err(UblError::component("Price.price_amount", e));
        }
        if let Some(v) = &self.price_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Price.price_type_code", e));
            }
        }
        if let Some(v) = &self.base_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Price.base_quantity", e));
            }
        }
        if let Some(v) = &self.price_list {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Price.price_list", e));
            }
        }
        if let Some(v) = &self.price_change_reason {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Price.price_change_reason", e));
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

impl Price {
    pub fn title() -> &'static str {
        "Price. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a price, expressed in a data structure containing multiple properties (compare with UnstructuredPrice)."
    }
    pub fn new(price_amount: PriceArrayOfPriceAmountComponent) -> Component<Self> {
        Component(Self {
            price_list: None,
            price_type: None,
            price_amount,
            orderable_unit_factor_rate: None,
            base_quantity: None,
            pricing_exchange_rate: None,
            price_change_reason: None,
            allowance_charge: None,
            price_type_code: None,
            validity_period: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<PriceArrayOfAllowanceChargeComponent> for PriceArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceArrayOfAllowanceChargeComponent> for PriceArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PriceArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceArrayOfAllowanceChargeComponent {
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
pub struct PriceArrayOfBaseQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BaseQuantity>,
}

impl AsMut<PriceArrayOfBaseQuantityComponent> for PriceArrayOfBaseQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceArrayOfBaseQuantityComponent> for PriceArrayOfBaseQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PriceArrayOfBaseQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PriceArrayOfBaseQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceArrayOfBaseQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BaseQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BaseQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BaseQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BaseQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceArrayOfOrderableUnitFactorRateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OrderableUnitFactorRate>,
}

impl AsMut<PriceArrayOfOrderableUnitFactorRateComponent> for PriceArrayOfOrderableUnitFactorRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceArrayOfOrderableUnitFactorRateComponent> for PriceArrayOfOrderableUnitFactorRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PriceArrayOfOrderableUnitFactorRateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PriceArrayOfOrderableUnitFactorRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceArrayOfOrderableUnitFactorRateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OrderableUnitFactorRate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OrderableUnitFactorRate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OrderableUnitFactorRate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OrderableUnitFactorRate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceArrayOfPriceAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PriceAmount>,
}

impl AsMut<PriceArrayOfPriceAmountComponent> for PriceArrayOfPriceAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceArrayOfPriceAmountComponent> for PriceArrayOfPriceAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PriceArrayOfPriceAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PriceArrayOfPriceAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceArrayOfPriceAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PriceAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PriceAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PriceAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PriceAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceArrayOfPriceChangeReasonComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PriceChangeReason>,
}

impl AsMut<PriceArrayOfPriceChangeReasonComponent> for PriceArrayOfPriceChangeReasonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceArrayOfPriceChangeReasonComponent> for PriceArrayOfPriceChangeReasonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PriceArrayOfPriceChangeReasonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceArrayOfPriceChangeReasonComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PriceChangeReason) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PriceChangeReason) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PriceChangeReason> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PriceChangeReason> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceArrayOfPriceListComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PriceList>,
}

impl AsMut<PriceArrayOfPriceListComponent> for PriceArrayOfPriceListComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceArrayOfPriceListComponent> for PriceArrayOfPriceListComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PriceArrayOfPriceListComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PriceArrayOfPriceListComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceArrayOfPriceListComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PriceList) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PriceList) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PriceList> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PriceList> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceArrayOfPriceTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PriceType>,
}

impl AsMut<PriceArrayOfPriceTypeComponent> for PriceArrayOfPriceTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceArrayOfPriceTypeComponent> for PriceArrayOfPriceTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PriceArrayOfPriceTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PriceArrayOfPriceTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceArrayOfPriceTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PriceType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PriceType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PriceType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PriceType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceArrayOfPriceTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PriceTypeCode>,
}

impl AsMut<PriceArrayOfPriceTypeCodeComponent> for PriceArrayOfPriceTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceArrayOfPriceTypeCodeComponent> for PriceArrayOfPriceTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PriceArrayOfPriceTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PriceArrayOfPriceTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceArrayOfPriceTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PriceTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PriceTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PriceTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PriceTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceArrayOfPricingExchangeRateComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PricingExchangeRate>,
}

impl AsMut<PriceArrayOfPricingExchangeRateComponent> for PriceArrayOfPricingExchangeRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceArrayOfPricingExchangeRateComponent> for PriceArrayOfPricingExchangeRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PriceArrayOfPricingExchangeRateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PriceArrayOfPricingExchangeRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceArrayOfPricingExchangeRateComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PricingExchangeRate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PricingExchangeRate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PricingExchangeRate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PricingExchangeRate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PriceArrayOfValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod>,
}

impl AsMut<PriceArrayOfValidityPeriodComponent> for PriceArrayOfValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PriceArrayOfValidityPeriodComponent> for PriceArrayOfValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PriceArrayOfValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PriceArrayOfValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod> {
        self.items.iter()
    }
}


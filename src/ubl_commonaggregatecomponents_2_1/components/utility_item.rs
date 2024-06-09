use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UtilityItem {
    #[serde(rename = "ConsumptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_type: Option<UtilityItemArrayOfConsumptionTypeComponent>,
    #[serde(rename = "ConsumptionTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_type_code: Option<UtilityItemArrayOfConsumptionTypeCodeComponent>,
    #[serde(rename = "Contract")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<UtilityItemArrayOfContractComponent>,
    #[serde(rename = "CurrentChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_charge_type: Option<UtilityItemArrayOfCurrentChargeTypeComponent>,
    #[serde(rename = "CurrentChargeTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_charge_type_code: Option<UtilityItemArrayOfCurrentChargeTypeCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<UtilityItemArrayOfDescriptionComponent>,
    #[serde(rename = "ID")]
    pub id: UtilityItemArrayOfIDComponent,
    #[serde(rename = "OneTimeChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_charge_type: Option<UtilityItemArrayOfOneTimeChargeTypeComponent>,
    #[serde(rename = "OneTimeChargeTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_charge_type_code: Option<UtilityItemArrayOfOneTimeChargeTypeCodeComponent>,
    #[serde(rename = "PackQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_quantity: Option<UtilityItemArrayOfPackQuantityComponent>,
    #[serde(rename = "PackSizeNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_size_numeric: Option<UtilityItemArrayOfPackSizeNumericComponent>,
    #[serde(rename = "SubscriberID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_id: Option<UtilityItemArrayOfSubscriberIDComponent>,
    #[serde(rename = "SubscriberType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_type: Option<UtilityItemArrayOfSubscriberTypeComponent>,
    #[serde(rename = "SubscriberTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_type_code: Option<UtilityItemArrayOfSubscriberTypeCodeComponent>,
    #[serde(rename = "TaxCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_category: Option<UtilityItemArrayOfTaxCategoryComponent>,
}

impl AsMut<UtilityItem> for UtilityItem {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItem> for UtilityItem {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.tax_category {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.tax_category", e));
            }
        }
        if let Some(v) = &self.current_charge_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.current_charge_type", e));
            }
        }
        if let Some(v) = &self.one_time_charge_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.one_time_charge_type", e));
            }
        }
        if let Some(v) = &self.contract {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.contract", e));
            }
        }
        if let Some(v) = &self.one_time_charge_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.one_time_charge_type_code", e));
            }
        }
        if let Some(v) = &self.consumption_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.consumption_type_code", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.description", e));
            }
        }
        if let Some(v) = &self.subscriber_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.subscriber_type_code", e));
            }
        }
        if let Some(v) = &self.subscriber_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.subscriber_type", e));
            }
        }
        if let Some(v) = &self.subscriber_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.subscriber_id", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("UtilityItem.id", e));
        }
        if let Some(v) = &self.current_charge_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.current_charge_type_code", e));
            }
        }
        if let Some(v) = &self.consumption_type {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.consumption_type", e));
            }
        }
        if let Some(v) = &self.pack_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.pack_quantity", e));
            }
        }
        if let Some(v) = &self.pack_size_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("UtilityItem.pack_size_numeric", e));
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

impl UtilityItem {
    pub fn title() -> &'static str {
        "Utility Item. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the consumption of a utility product."
    }
    pub fn new(id: UtilityItemArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            subscriber_id: None,
            subscriber_type: None,
            description: None,
            id,
            consumption_type: None,
            current_charge_type_code: None,
            one_time_charge_type: None,
            tax_category: None,
            pack_quantity: None,
            current_charge_type: None,
            consumption_type_code: None,
            contract: None,
            one_time_charge_type_code: None,
            pack_size_numeric: None,
            subscriber_type_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfConsumptionTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionType>,
}

impl AsMut<UtilityItemArrayOfConsumptionTypeComponent> for UtilityItemArrayOfConsumptionTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfConsumptionTypeComponent> for UtilityItemArrayOfConsumptionTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfConsumptionTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfConsumptionTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfConsumptionTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumptionType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumptionType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumptionType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumptionType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfConsumptionTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionTypeCode>,
}

impl AsMut<UtilityItemArrayOfConsumptionTypeCodeComponent> for UtilityItemArrayOfConsumptionTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfConsumptionTypeCodeComponent> for UtilityItemArrayOfConsumptionTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfConsumptionTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfConsumptionTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfConsumptionTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumptionTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumptionTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumptionTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumptionTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfContractComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Contract>,
}

impl AsMut<UtilityItemArrayOfContractComponent> for UtilityItemArrayOfContractComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfContractComponent> for UtilityItemArrayOfContractComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfContractComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfContractComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfContractComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Contract) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Contract) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Contract> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Contract> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfCurrentChargeTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CurrentChargeType>,
}

impl AsMut<UtilityItemArrayOfCurrentChargeTypeComponent> for UtilityItemArrayOfCurrentChargeTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfCurrentChargeTypeComponent> for UtilityItemArrayOfCurrentChargeTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfCurrentChargeTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfCurrentChargeTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfCurrentChargeTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CurrentChargeType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CurrentChargeType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CurrentChargeType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CurrentChargeType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfCurrentChargeTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CurrentChargeTypeCode>,
}

impl AsMut<UtilityItemArrayOfCurrentChargeTypeCodeComponent> for UtilityItemArrayOfCurrentChargeTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfCurrentChargeTypeCodeComponent> for UtilityItemArrayOfCurrentChargeTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfCurrentChargeTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfCurrentChargeTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfCurrentChargeTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CurrentChargeTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CurrentChargeTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CurrentChargeTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CurrentChargeTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<UtilityItemArrayOfDescriptionComponent> for UtilityItemArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfDescriptionComponent> for UtilityItemArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Description) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Description) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<UtilityItemArrayOfIDComponent> for UtilityItemArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfIDComponent> for UtilityItemArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfIDComponent {
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
pub struct UtilityItemArrayOfOneTimeChargeTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OneTimeChargeType>,
}

impl AsMut<UtilityItemArrayOfOneTimeChargeTypeComponent> for UtilityItemArrayOfOneTimeChargeTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfOneTimeChargeTypeComponent> for UtilityItemArrayOfOneTimeChargeTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfOneTimeChargeTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfOneTimeChargeTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfOneTimeChargeTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OneTimeChargeType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OneTimeChargeType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OneTimeChargeType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OneTimeChargeType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfOneTimeChargeTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OneTimeChargeTypeCode>,
}

impl AsMut<UtilityItemArrayOfOneTimeChargeTypeCodeComponent> for UtilityItemArrayOfOneTimeChargeTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfOneTimeChargeTypeCodeComponent> for UtilityItemArrayOfOneTimeChargeTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfOneTimeChargeTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfOneTimeChargeTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfOneTimeChargeTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OneTimeChargeTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OneTimeChargeTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OneTimeChargeTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OneTimeChargeTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfPackQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PackQuantity>,
}

impl AsMut<UtilityItemArrayOfPackQuantityComponent> for UtilityItemArrayOfPackQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfPackQuantityComponent> for UtilityItemArrayOfPackQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfPackQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfPackQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfPackQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PackQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PackQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PackQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PackQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfPackSizeNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PackSizeNumeric>,
}

impl AsMut<UtilityItemArrayOfPackSizeNumericComponent> for UtilityItemArrayOfPackSizeNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfPackSizeNumericComponent> for UtilityItemArrayOfPackSizeNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfPackSizeNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfPackSizeNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfPackSizeNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PackSizeNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PackSizeNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PackSizeNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PackSizeNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfSubscriberIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SubscriberID>,
}

impl AsMut<UtilityItemArrayOfSubscriberIDComponent> for UtilityItemArrayOfSubscriberIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfSubscriberIDComponent> for UtilityItemArrayOfSubscriberIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfSubscriberIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfSubscriberIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfSubscriberIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SubscriberID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SubscriberID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SubscriberID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SubscriberID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfSubscriberTypeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SubscriberType>,
}

impl AsMut<UtilityItemArrayOfSubscriberTypeComponent> for UtilityItemArrayOfSubscriberTypeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfSubscriberTypeComponent> for UtilityItemArrayOfSubscriberTypeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfSubscriberTypeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfSubscriberTypeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfSubscriberTypeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SubscriberType) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SubscriberType) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SubscriberType> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SubscriberType> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfSubscriberTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SubscriberTypeCode>,
}

impl AsMut<UtilityItemArrayOfSubscriberTypeCodeComponent> for UtilityItemArrayOfSubscriberTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfSubscriberTypeCodeComponent> for UtilityItemArrayOfSubscriberTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfSubscriberTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfSubscriberTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfSubscriberTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SubscriberTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SubscriberTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SubscriberTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SubscriberTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtilityItemArrayOfTaxCategoryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxCategory>,
}

impl AsMut<UtilityItemArrayOfTaxCategoryComponent> for UtilityItemArrayOfTaxCategoryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UtilityItemArrayOfTaxCategoryComponent> for UtilityItemArrayOfTaxCategoryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfTaxCategoryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("UtilityItemArrayOfTaxCategoryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UtilityItemArrayOfTaxCategoryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxCategory) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxCategory) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxCategory> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxCategory> {
        self.items.iter()
    }
}


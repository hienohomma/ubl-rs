use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemProperty {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ItemPropertyArrayOfIDComponent>,
    #[serde(rename = "ImportanceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance_code: Option<ItemPropertyArrayOfImportanceCodeComponent>,
    #[serde(rename = "ItemPropertyGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_property_group: Option<ItemPropertyArrayOfItemPropertyGroupComponent>,
    #[serde(rename = "ItemPropertyRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_property_range: Option<ItemPropertyArrayOfItemPropertyRangeComponent>,
    #[serde(rename = "ListValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_value: Option<ItemPropertyArrayOfListValueComponent>,
    #[serde(rename = "Name")]
    pub name: ItemPropertyArrayOfNameComponent,
    #[serde(rename = "NameCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_code: Option<ItemPropertyArrayOfNameCodeComponent>,
    #[serde(rename = "RangeDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_dimension: Option<ItemPropertyArrayOfRangeDimensionComponent>,
    #[serde(rename = "TestMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_method: Option<ItemPropertyArrayOfTestMethodComponent>,
    #[serde(rename = "UsabilityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usability_period: Option<ItemPropertyArrayOfUsabilityPeriodComponent>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ItemPropertyArrayOfValueComponent>,
    #[serde(rename = "ValueQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_qualifier: Option<ItemPropertyArrayOfValueQualifierComponent>,
    #[serde(rename = "ValueQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_quantity: Option<ItemPropertyArrayOfValueQuantityComponent>,
}

impl AsMut<ItemProperty> for ItemProperty {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemProperty> for ItemProperty {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.name_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.name_code", e));
            }
        }
        if let Some(v) = &self.value_qualifier {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.value_qualifier", e));
            }
        }
        if let Some(v) = &self.list_value {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.list_value", e));
            }
        }
        if let Some(v) = &self.importance_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.importance_code", e));
            }
        }
        if let Err(e) = self.name.validate() {
            return Err(UblError::component("ItemProperty.name", e));
        }
        if let Some(v) = &self.usability_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.usability_period", e));
            }
        }
        if let Some(v) = &self.test_method {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.test_method", e));
            }
        }
        if let Some(v) = &self.item_property_group {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.item_property_group", e));
            }
        }
        if let Some(v) = &self.item_property_range {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.item_property_range", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.id", e));
            }
        }
        if let Some(v) = &self.range_dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.range_dimension", e));
            }
        }
        if let Some(v) = &self.value {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.value", e));
            }
        }
        if let Some(v) = &self.value_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("ItemProperty.value_quantity", e));
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

impl ItemProperty {
    pub fn title() -> &'static str {
        "Item Property. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a specific property of an item."
    }
    pub fn new(name: ItemPropertyArrayOfNameComponent) -> Component<Self> {
        Component(Self {
            test_method: None,
            id: None,
            range_dimension: None,
            value: None,
            value_qualifier: None,
            item_property_range: None,
            importance_code: None,
            item_property_group: None,
            value_quantity: None,
            usability_period: None,
            name,
            name_code: None,
            list_value: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ItemPropertyArrayOfIDComponent> for ItemPropertyArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfIDComponent> for ItemPropertyArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfIDComponent {
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
pub struct ItemPropertyArrayOfImportanceCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ImportanceCode>,
}

impl AsMut<ItemPropertyArrayOfImportanceCodeComponent> for ItemPropertyArrayOfImportanceCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfImportanceCodeComponent> for ItemPropertyArrayOfImportanceCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfImportanceCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfImportanceCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfImportanceCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ImportanceCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ImportanceCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ImportanceCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ImportanceCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfItemPropertyGroupComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ItemPropertyGroup>,
}

impl AsMut<ItemPropertyArrayOfItemPropertyGroupComponent> for ItemPropertyArrayOfItemPropertyGroupComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfItemPropertyGroupComponent> for ItemPropertyArrayOfItemPropertyGroupComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfItemPropertyGroupComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfItemPropertyGroupComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ItemPropertyGroup) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ItemPropertyGroup) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ItemPropertyGroup> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ItemPropertyGroup> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfItemPropertyRangeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ItemPropertyRange>,
}

impl AsMut<ItemPropertyArrayOfItemPropertyRangeComponent> for ItemPropertyArrayOfItemPropertyRangeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfItemPropertyRangeComponent> for ItemPropertyArrayOfItemPropertyRangeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfItemPropertyRangeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfItemPropertyRangeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfItemPropertyRangeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ItemPropertyRange) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ItemPropertyRange) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ItemPropertyRange> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ItemPropertyRange> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfListValueComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ListValue>,
}

impl AsMut<ItemPropertyArrayOfListValueComponent> for ItemPropertyArrayOfListValueComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfListValueComponent> for ItemPropertyArrayOfListValueComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfListValueComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfListValueComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ListValue) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ListValue) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ListValue> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ListValue> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<ItemPropertyArrayOfNameComponent> for ItemPropertyArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfNameComponent> for ItemPropertyArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Name) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Name) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfNameCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NameCode>,
}

impl AsMut<ItemPropertyArrayOfNameCodeComponent> for ItemPropertyArrayOfNameCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfNameCodeComponent> for ItemPropertyArrayOfNameCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfNameCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfNameCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfNameCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NameCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NameCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NameCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NameCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfRangeDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RangeDimension>,
}

impl AsMut<ItemPropertyArrayOfRangeDimensionComponent> for ItemPropertyArrayOfRangeDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfRangeDimensionComponent> for ItemPropertyArrayOfRangeDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfRangeDimensionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfRangeDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfRangeDimensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RangeDimension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RangeDimension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RangeDimension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RangeDimension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfTestMethodComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TestMethod>,
}

impl AsMut<ItemPropertyArrayOfTestMethodComponent> for ItemPropertyArrayOfTestMethodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfTestMethodComponent> for ItemPropertyArrayOfTestMethodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfTestMethodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfTestMethodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfTestMethodComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TestMethod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TestMethod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TestMethod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TestMethod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfUsabilityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::UsabilityPeriod>,
}

impl AsMut<ItemPropertyArrayOfUsabilityPeriodComponent> for ItemPropertyArrayOfUsabilityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfUsabilityPeriodComponent> for ItemPropertyArrayOfUsabilityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfUsabilityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfUsabilityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfUsabilityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::UsabilityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::UsabilityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::UsabilityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::UsabilityPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfValueComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Value>,
}

impl AsMut<ItemPropertyArrayOfValueComponent> for ItemPropertyArrayOfValueComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfValueComponent> for ItemPropertyArrayOfValueComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfValueComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfValueComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfValueComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Value) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Value) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Value> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Value> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfValueQualifierComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValueQualifier>,
}

impl AsMut<ItemPropertyArrayOfValueQualifierComponent> for ItemPropertyArrayOfValueQualifierComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfValueQualifierComponent> for ItemPropertyArrayOfValueQualifierComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfValueQualifierComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfValueQualifierComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValueQualifier) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValueQualifier) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValueQualifier> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValueQualifier> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemPropertyArrayOfValueQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValueQuantity>,
}

impl AsMut<ItemPropertyArrayOfValueQuantityComponent> for ItemPropertyArrayOfValueQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemPropertyArrayOfValueQuantityComponent> for ItemPropertyArrayOfValueQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfValueQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemPropertyArrayOfValueQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemPropertyArrayOfValueQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValueQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValueQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValueQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValueQuantity> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventTacticEnumeration {
    #[serde(rename = "ConsumerIncentiveTacticTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_incentive_tactic_type_code: Option<EventTacticEnumerationArrayOfConsumerIncentiveTacticTypeCodeComponent>,
    #[serde(rename = "DisplayTacticTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_tactic_type_code: Option<EventTacticEnumerationArrayOfDisplayTacticTypeCodeComponent>,
    #[serde(rename = "FeatureTacticTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_tactic_type_code: Option<EventTacticEnumerationArrayOfFeatureTacticTypeCodeComponent>,
    #[serde(rename = "TradeItemPackingLabelingTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_item_packing_labeling_type_code: Option<EventTacticEnumerationArrayOfTradeItemPackingLabelingTypeCodeComponent>,
}

impl AsMut<EventTacticEnumeration> for EventTacticEnumeration {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventTacticEnumeration> for EventTacticEnumeration {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.consumer_incentive_tactic_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventTacticEnumeration.consumer_incentive_tactic_type_code", e));
            }
        }
        if let Some(v) = &self.display_tactic_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventTacticEnumeration.display_tactic_type_code", e));
            }
        }
        if let Some(v) = &self.trade_item_packing_labeling_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventTacticEnumeration.trade_item_packing_labeling_type_code", e));
            }
        }
        if let Some(v) = &self.feature_tactic_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("EventTacticEnumeration.feature_tactic_type_code", e));
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

impl EventTacticEnumeration {
    pub fn title() -> &'static str {
        "Event Tactic Enumeration. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a set of codes that describes a retail tactic."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            consumer_incentive_tactic_type_code: None,
            display_tactic_type_code: None,
            feature_tactic_type_code: None,
            trade_item_packing_labeling_type_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventTacticEnumerationArrayOfConsumerIncentiveTacticTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumerIncentiveTacticTypeCode>,
}

impl AsMut<EventTacticEnumerationArrayOfConsumerIncentiveTacticTypeCodeComponent> for EventTacticEnumerationArrayOfConsumerIncentiveTacticTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventTacticEnumerationArrayOfConsumerIncentiveTacticTypeCodeComponent> for EventTacticEnumerationArrayOfConsumerIncentiveTacticTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventTacticEnumerationArrayOfConsumerIncentiveTacticTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventTacticEnumerationArrayOfConsumerIncentiveTacticTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventTacticEnumerationArrayOfConsumerIncentiveTacticTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumerIncentiveTacticTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumerIncentiveTacticTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumerIncentiveTacticTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumerIncentiveTacticTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventTacticEnumerationArrayOfDisplayTacticTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DisplayTacticTypeCode>,
}

impl AsMut<EventTacticEnumerationArrayOfDisplayTacticTypeCodeComponent> for EventTacticEnumerationArrayOfDisplayTacticTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventTacticEnumerationArrayOfDisplayTacticTypeCodeComponent> for EventTacticEnumerationArrayOfDisplayTacticTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventTacticEnumerationArrayOfDisplayTacticTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventTacticEnumerationArrayOfDisplayTacticTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventTacticEnumerationArrayOfDisplayTacticTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DisplayTacticTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DisplayTacticTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DisplayTacticTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DisplayTacticTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventTacticEnumerationArrayOfFeatureTacticTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FeatureTacticTypeCode>,
}

impl AsMut<EventTacticEnumerationArrayOfFeatureTacticTypeCodeComponent> for EventTacticEnumerationArrayOfFeatureTacticTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventTacticEnumerationArrayOfFeatureTacticTypeCodeComponent> for EventTacticEnumerationArrayOfFeatureTacticTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventTacticEnumerationArrayOfFeatureTacticTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventTacticEnumerationArrayOfFeatureTacticTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventTacticEnumerationArrayOfFeatureTacticTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FeatureTacticTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FeatureTacticTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FeatureTacticTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FeatureTacticTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EventTacticEnumerationArrayOfTradeItemPackingLabelingTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TradeItemPackingLabelingTypeCode>,
}

impl AsMut<EventTacticEnumerationArrayOfTradeItemPackingLabelingTypeCodeComponent> for EventTacticEnumerationArrayOfTradeItemPackingLabelingTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<EventTacticEnumerationArrayOfTradeItemPackingLabelingTypeCodeComponent> for EventTacticEnumerationArrayOfTradeItemPackingLabelingTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("EventTacticEnumerationArrayOfTradeItemPackingLabelingTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("EventTacticEnumerationArrayOfTradeItemPackingLabelingTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl EventTacticEnumerationArrayOfTradeItemPackingLabelingTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TradeItemPackingLabelingTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TradeItemPackingLabelingTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TradeItemPackingLabelingTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TradeItemPackingLabelingTypeCode> {
        self.items.iter()
    }
}


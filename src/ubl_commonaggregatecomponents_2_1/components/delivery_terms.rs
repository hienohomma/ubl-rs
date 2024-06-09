use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeliveryTerms {
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<DeliveryTermsArrayOfAllowanceChargeComponent>,
    #[serde(rename = "Amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<DeliveryTermsArrayOfAmountComponent>,
    #[serde(rename = "DeliveryLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_location: Option<DeliveryTermsArrayOfDeliveryLocationComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<DeliveryTermsArrayOfIDComponent>,
    #[serde(rename = "LossRisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_risk: Option<DeliveryTermsArrayOfLossRiskComponent>,
    #[serde(rename = "LossRiskResponsibilityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_risk_responsibility_code: Option<DeliveryTermsArrayOfLossRiskResponsibilityCodeComponent>,
    #[serde(rename = "SpecialTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_terms: Option<DeliveryTermsArrayOfSpecialTermsComponent>,
}

impl AsMut<DeliveryTerms> for DeliveryTerms {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryTerms> for DeliveryTerms {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DeliveryTerms.allowance_charge", e));
            }
        }
        if let Some(v) = &self.amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DeliveryTerms.amount", e));
            }
        }
        if let Some(v) = &self.loss_risk {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DeliveryTerms.loss_risk", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DeliveryTerms.id", e));
            }
        }
        if let Some(v) = &self.loss_risk_responsibility_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DeliveryTerms.loss_risk_responsibility_code", e));
            }
        }
        if let Some(v) = &self.delivery_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DeliveryTerms.delivery_location", e));
            }
        }
        if let Some(v) = &self.special_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("DeliveryTerms.special_terms", e));
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

impl DeliveryTerms {
    pub fn title() -> &'static str {
        "Delivery Terms. Details"
    }
    pub fn description() -> &'static str {
        "A class for describing the terms and conditions applying to the delivery of goods."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            amount: None,
            id: None,
            loss_risk: None,
            loss_risk_responsibility_code: None,
            special_terms: None,
            allowance_charge: None,
            delivery_location: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryTermsArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<DeliveryTermsArrayOfAllowanceChargeComponent> for DeliveryTermsArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryTermsArrayOfAllowanceChargeComponent> for DeliveryTermsArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfAllowanceChargeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeliveryTermsArrayOfAllowanceChargeComponent {
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
pub struct DeliveryTermsArrayOfAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Amount>,
}

impl AsMut<DeliveryTermsArrayOfAmountComponent> for DeliveryTermsArrayOfAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryTermsArrayOfAmountComponent> for DeliveryTermsArrayOfAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeliveryTermsArrayOfAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Amount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Amount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Amount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Amount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryTermsArrayOfDeliveryLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryLocation>,
}

impl AsMut<DeliveryTermsArrayOfDeliveryLocationComponent> for DeliveryTermsArrayOfDeliveryLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryTermsArrayOfDeliveryLocationComponent> for DeliveryTermsArrayOfDeliveryLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfDeliveryLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfDeliveryLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeliveryTermsArrayOfDeliveryLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryTermsArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<DeliveryTermsArrayOfIDComponent> for DeliveryTermsArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryTermsArrayOfIDComponent> for DeliveryTermsArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeliveryTermsArrayOfIDComponent {
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
pub struct DeliveryTermsArrayOfLossRiskComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LossRisk>,
}

impl AsMut<DeliveryTermsArrayOfLossRiskComponent> for DeliveryTermsArrayOfLossRiskComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryTermsArrayOfLossRiskComponent> for DeliveryTermsArrayOfLossRiskComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfLossRiskComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeliveryTermsArrayOfLossRiskComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LossRisk) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LossRisk) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LossRisk> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LossRisk> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryTermsArrayOfLossRiskResponsibilityCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LossRiskResponsibilityCode>,
}

impl AsMut<DeliveryTermsArrayOfLossRiskResponsibilityCodeComponent> for DeliveryTermsArrayOfLossRiskResponsibilityCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryTermsArrayOfLossRiskResponsibilityCodeComponent> for DeliveryTermsArrayOfLossRiskResponsibilityCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfLossRiskResponsibilityCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfLossRiskResponsibilityCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeliveryTermsArrayOfLossRiskResponsibilityCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LossRiskResponsibilityCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LossRiskResponsibilityCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LossRiskResponsibilityCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LossRiskResponsibilityCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DeliveryTermsArrayOfSpecialTermsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SpecialTerms>,
}

impl AsMut<DeliveryTermsArrayOfSpecialTermsComponent> for DeliveryTermsArrayOfSpecialTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<DeliveryTermsArrayOfSpecialTermsComponent> for DeliveryTermsArrayOfSpecialTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("DeliveryTermsArrayOfSpecialTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl DeliveryTermsArrayOfSpecialTermsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SpecialTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SpecialTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SpecialTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SpecialTerms> {
        self.items.iter()
    }
}


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FinancialGuarantee {
    #[serde(rename = "AmountRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_rate: Option<FinancialGuaranteeArrayOfAmountRateComponent>,
    #[serde(rename = "ConstitutionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constitution_period: Option<FinancialGuaranteeArrayOfConstitutionPeriodComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<FinancialGuaranteeArrayOfDescriptionComponent>,
    #[serde(rename = "GuaranteeTypeCode")]
    pub guarantee_type_code: FinancialGuaranteeArrayOfGuaranteeTypeCodeComponent,
    #[serde(rename = "LiabilityAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability_amount: Option<FinancialGuaranteeArrayOfLiabilityAmountComponent>,
}

impl AsMut<FinancialGuarantee> for FinancialGuarantee {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialGuarantee> for FinancialGuarantee {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.guarantee_type_code.validate() {
            return Err(UblError::component("FinancialGuarantee.guarantee_type_code", e));
        }
        if let Some(v) = &self.liability_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialGuarantee.liability_amount", e));
            }
        }
        if let Some(v) = &self.amount_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialGuarantee.amount_rate", e));
            }
        }
        if let Some(v) = &self.constitution_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialGuarantee.constitution_period", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("FinancialGuarantee.description", e));
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

impl FinancialGuarantee {
    pub fn title() -> &'static str {
        "Financial Guarantee. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe the bond guarantee of a tenderer or bid submitter's actual entry into a contract in the event that it is the successful bidder."
    }
    pub fn new(guarantee_type_code: FinancialGuaranteeArrayOfGuaranteeTypeCodeComponent) -> Component<Self> {
        Component(Self {
            description: None,
            constitution_period: None,
            guarantee_type_code,
            liability_amount: None,
            amount_rate: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialGuaranteeArrayOfAmountRateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AmountRate>,
}

impl AsMut<FinancialGuaranteeArrayOfAmountRateComponent> for FinancialGuaranteeArrayOfAmountRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialGuaranteeArrayOfAmountRateComponent> for FinancialGuaranteeArrayOfAmountRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialGuaranteeArrayOfAmountRateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialGuaranteeArrayOfAmountRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialGuaranteeArrayOfAmountRateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AmountRate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AmountRate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AmountRate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AmountRate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialGuaranteeArrayOfConstitutionPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ConstitutionPeriod>,
}

impl AsMut<FinancialGuaranteeArrayOfConstitutionPeriodComponent> for FinancialGuaranteeArrayOfConstitutionPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialGuaranteeArrayOfConstitutionPeriodComponent> for FinancialGuaranteeArrayOfConstitutionPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialGuaranteeArrayOfConstitutionPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialGuaranteeArrayOfConstitutionPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialGuaranteeArrayOfConstitutionPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ConstitutionPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ConstitutionPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ConstitutionPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ConstitutionPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialGuaranteeArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<FinancialGuaranteeArrayOfDescriptionComponent> for FinancialGuaranteeArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialGuaranteeArrayOfDescriptionComponent> for FinancialGuaranteeArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("FinancialGuaranteeArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialGuaranteeArrayOfDescriptionComponent {
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
pub struct FinancialGuaranteeArrayOfGuaranteeTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GuaranteeTypeCode>,
}

impl AsMut<FinancialGuaranteeArrayOfGuaranteeTypeCodeComponent> for FinancialGuaranteeArrayOfGuaranteeTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialGuaranteeArrayOfGuaranteeTypeCodeComponent> for FinancialGuaranteeArrayOfGuaranteeTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialGuaranteeArrayOfGuaranteeTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialGuaranteeArrayOfGuaranteeTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialGuaranteeArrayOfGuaranteeTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GuaranteeTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GuaranteeTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GuaranteeTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GuaranteeTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FinancialGuaranteeArrayOfLiabilityAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LiabilityAmount>,
}

impl AsMut<FinancialGuaranteeArrayOfLiabilityAmountComponent> for FinancialGuaranteeArrayOfLiabilityAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<FinancialGuaranteeArrayOfLiabilityAmountComponent> for FinancialGuaranteeArrayOfLiabilityAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("FinancialGuaranteeArrayOfLiabilityAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("FinancialGuaranteeArrayOfLiabilityAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl FinancialGuaranteeArrayOfLiabilityAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LiabilityAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LiabilityAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LiabilityAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LiabilityAmount> {
        self.items.iter()
    }
}


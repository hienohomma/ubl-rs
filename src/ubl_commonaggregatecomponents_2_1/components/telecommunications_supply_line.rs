use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TelecommunicationsSupplyLine {
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<TelecommunicationsSupplyLineArrayOfAllowanceChargeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TelecommunicationsSupplyLineArrayOfDescriptionComponent>,
    #[serde(rename = "ExchangeRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<TelecommunicationsSupplyLineArrayOfExchangeRateComponent>,
    #[serde(rename = "ID")]
    pub id: TelecommunicationsSupplyLineArrayOfIDComponent,
    #[serde(rename = "LineExtensionAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_extension_amount: Option<TelecommunicationsSupplyLineArrayOfLineExtensionAmountComponent>,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: TelecommunicationsSupplyLineArrayOfPhoneNumberComponent,
    #[serde(rename = "TaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<TelecommunicationsSupplyLineArrayOfTaxTotalComponent>,
    #[serde(rename = "TelecommunicationsService")]
    pub telecommunications_service: TelecommunicationsSupplyLineArrayOfTelecommunicationsServiceComponent,
}

impl AsMut<TelecommunicationsSupplyLine> for TelecommunicationsSupplyLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyLine> for TelecommunicationsSupplyLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsSupplyLine.tax_total", e));
            }
        }
        if let Some(v) = &self.exchange_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsSupplyLine.exchange_rate", e));
            }
        }
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsSupplyLine.allowance_charge", e));
            }
        }
        if let Err(e) = self.telecommunications_service.validate() {
            return Err(UblError::component("TelecommunicationsSupplyLine.telecommunications_service", e));
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("TelecommunicationsSupplyLine.id", e));
        }
        if let Err(e) = self.phone_number.validate() {
            return Err(UblError::component("TelecommunicationsSupplyLine.phone_number", e));
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsSupplyLine.description", e));
            }
        }
        if let Some(v) = &self.line_extension_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsSupplyLine.line_extension_amount", e));
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

impl TelecommunicationsSupplyLine {
    pub fn title() -> &'static str {
        "Telecommunications Supply Line. Details"
    }
    pub fn description() -> &'static str {
        "A class that outlines the telecommunication supply in details"
    }
    pub fn new(id: TelecommunicationsSupplyLineArrayOfIDComponent, phone_number: TelecommunicationsSupplyLineArrayOfPhoneNumberComponent, telecommunications_service: TelecommunicationsSupplyLineArrayOfTelecommunicationsServiceComponent) -> Component<Self> {
        Component(Self {
            exchange_rate: None,
            telecommunications_service,
            id,
            tax_total: None,
            allowance_charge: None,
            phone_number,
            description: None,
            line_extension_amount: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsSupplyLineArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<TelecommunicationsSupplyLineArrayOfAllowanceChargeComponent> for TelecommunicationsSupplyLineArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyLineArrayOfAllowanceChargeComponent> for TelecommunicationsSupplyLineArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyLineArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyLineArrayOfAllowanceChargeComponent {
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
pub struct TelecommunicationsSupplyLineArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<TelecommunicationsSupplyLineArrayOfDescriptionComponent> for TelecommunicationsSupplyLineArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyLineArrayOfDescriptionComponent> for TelecommunicationsSupplyLineArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyLineArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyLineArrayOfDescriptionComponent {
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
pub struct TelecommunicationsSupplyLineArrayOfExchangeRateComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExchangeRate>,
}

impl AsMut<TelecommunicationsSupplyLineArrayOfExchangeRateComponent> for TelecommunicationsSupplyLineArrayOfExchangeRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyLineArrayOfExchangeRateComponent> for TelecommunicationsSupplyLineArrayOfExchangeRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyLineArrayOfExchangeRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyLineArrayOfExchangeRateComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ExchangeRate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ExchangeRate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ExchangeRate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ExchangeRate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsSupplyLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<TelecommunicationsSupplyLineArrayOfIDComponent> for TelecommunicationsSupplyLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyLineArrayOfIDComponent> for TelecommunicationsSupplyLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyLineArrayOfIDComponent {
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
pub struct TelecommunicationsSupplyLineArrayOfLineExtensionAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount>,
}

impl AsMut<TelecommunicationsSupplyLineArrayOfLineExtensionAmountComponent> for TelecommunicationsSupplyLineArrayOfLineExtensionAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyLineArrayOfLineExtensionAmountComponent> for TelecommunicationsSupplyLineArrayOfLineExtensionAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyLineArrayOfLineExtensionAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyLineArrayOfLineExtensionAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyLineArrayOfLineExtensionAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LineExtensionAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsSupplyLineArrayOfPhoneNumberComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PhoneNumber>,
}

impl AsMut<TelecommunicationsSupplyLineArrayOfPhoneNumberComponent> for TelecommunicationsSupplyLineArrayOfPhoneNumberComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyLineArrayOfPhoneNumberComponent> for TelecommunicationsSupplyLineArrayOfPhoneNumberComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyLineArrayOfPhoneNumberComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyLineArrayOfPhoneNumberComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyLineArrayOfPhoneNumberComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PhoneNumber) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PhoneNumber) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PhoneNumber> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PhoneNumber> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsSupplyLineArrayOfTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxTotal>,
}

impl AsMut<TelecommunicationsSupplyLineArrayOfTaxTotalComponent> for TelecommunicationsSupplyLineArrayOfTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyLineArrayOfTaxTotalComponent> for TelecommunicationsSupplyLineArrayOfTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyLineArrayOfTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyLineArrayOfTaxTotalComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TaxTotal) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TaxTotal> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsSupplyLineArrayOfTelecommunicationsServiceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsService>,
}

impl AsMut<TelecommunicationsSupplyLineArrayOfTelecommunicationsServiceComponent> for TelecommunicationsSupplyLineArrayOfTelecommunicationsServiceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsSupplyLineArrayOfTelecommunicationsServiceComponent> for TelecommunicationsSupplyLineArrayOfTelecommunicationsServiceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsSupplyLineArrayOfTelecommunicationsServiceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsSupplyLineArrayOfTelecommunicationsServiceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsService) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsService) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsService> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TelecommunicationsService> {
        self.items.iter()
    }
}


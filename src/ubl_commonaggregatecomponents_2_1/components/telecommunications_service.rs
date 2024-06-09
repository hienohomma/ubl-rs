use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TelecommunicationsService {
    #[serde(rename = "AllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge: Option<TelecommunicationsServiceArrayOfAllowanceChargeComponent>,
    #[serde(rename = "CallBaseAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_base_amount: Option<TelecommunicationsServiceArrayOfCallBaseAmountComponent>,
    #[serde(rename = "CallDate")]
    pub call_date: TelecommunicationsServiceArrayOfCallDateComponent,
    #[serde(rename = "CallDuty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_duty: Option<TelecommunicationsServiceArrayOfCallDutyComponent>,
    #[serde(rename = "CallExtensionAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_extension_amount: Option<TelecommunicationsServiceArrayOfCallExtensionAmountComponent>,
    #[serde(rename = "CallTime")]
    pub call_time: TelecommunicationsServiceArrayOfCallTimeComponent,
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<TelecommunicationsServiceArrayOfCountryComponent>,
    #[serde(rename = "ExchangeRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<TelecommunicationsServiceArrayOfExchangeRateComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TelecommunicationsServiceArrayOfIDComponent>,
    #[serde(rename = "MovieTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie_title: Option<TelecommunicationsServiceArrayOfMovieTitleComponent>,
    #[serde(rename = "PayPerView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_per_view: Option<TelecommunicationsServiceArrayOfPayPerViewComponent>,
    #[serde(rename = "Price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<TelecommunicationsServiceArrayOfPriceComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<TelecommunicationsServiceArrayOfQuantityComponent>,
    #[serde(rename = "RoamingPartnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roaming_partner_name: Option<TelecommunicationsServiceArrayOfRoamingPartnerNameComponent>,
    #[serde(rename = "ServiceNumberCalled")]
    pub service_number_called: TelecommunicationsServiceArrayOfServiceNumberCalledComponent,
    #[serde(rename = "TaxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<TelecommunicationsServiceArrayOfTaxTotalComponent>,
    #[serde(rename = "TelecommunicationsServiceCall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecommunications_service_call: Option<TelecommunicationsServiceArrayOfTelecommunicationsServiceCallComponent>,
    #[serde(rename = "TelecommunicationsServiceCallCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecommunications_service_call_code: Option<TelecommunicationsServiceArrayOfTelecommunicationsServiceCallCodeComponent>,
    #[serde(rename = "TelecommunicationsServiceCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecommunications_service_category: Option<TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryComponent>,
    #[serde(rename = "TelecommunicationsServiceCategoryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecommunications_service_category_code: Option<TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryCodeComponent>,
    #[serde(rename = "TimeDuty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_duty: Option<TelecommunicationsServiceArrayOfTimeDutyComponent>,
}

impl AsMut<TelecommunicationsService> for TelecommunicationsService {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsService> for TelecommunicationsService {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.call_extension_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.call_extension_amount", e));
            }
        }
        if let Some(v) = &self.country {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.country", e));
            }
        }
        if let Some(v) = &self.pay_per_view {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.pay_per_view", e));
            }
        }
        if let Err(e) = self.call_time.validate() {
            return Err(UblError::component("TelecommunicationsService.call_time", e));
        }
        if let Some(v) = &self.telecommunications_service_category_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.telecommunications_service_category_code", e));
            }
        }
        if let Some(v) = &self.call_base_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.call_base_amount", e));
            }
        }
        if let Some(v) = &self.exchange_rate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.exchange_rate", e));
            }
        }
        if let Some(v) = &self.movie_title {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.movie_title", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.id", e));
            }
        }
        if let Some(v) = &self.price {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.price", e));
            }
        }
        if let Some(v) = &self.allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.allowance_charge", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.quantity", e));
            }
        }
        if let Some(v) = &self.telecommunications_service_call_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.telecommunications_service_call_code", e));
            }
        }
        if let Some(v) = &self.tax_total {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.tax_total", e));
            }
        }
        if let Err(e) = self.call_date.validate() {
            return Err(UblError::component("TelecommunicationsService.call_date", e));
        }
        if let Err(e) = self.service_number_called.validate() {
            return Err(UblError::component("TelecommunicationsService.service_number_called", e));
        }
        if let Some(v) = &self.roaming_partner_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.roaming_partner_name", e));
            }
        }
        if let Some(v) = &self.telecommunications_service_call {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.telecommunications_service_call", e));
            }
        }
        if let Some(v) = &self.telecommunications_service_category {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.telecommunications_service_category", e));
            }
        }
        if let Some(v) = &self.time_duty {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.time_duty", e));
            }
        }
        if let Some(v) = &self.call_duty {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TelecommunicationsService.call_duty", e));
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

impl TelecommunicationsService {
    pub fn title() -> &'static str {
        "Telecommunications Service. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a telecommunications service (e.g., a telephone call or a video on demand service)."
    }
    pub fn new(call_date: TelecommunicationsServiceArrayOfCallDateComponent, call_time: TelecommunicationsServiceArrayOfCallTimeComponent, service_number_called: TelecommunicationsServiceArrayOfServiceNumberCalledComponent) -> Component<Self> {
        Component(Self {
            call_duty: None,
            service_number_called,
            call_date,
            time_duty: None,
            country: None,
            allowance_charge: None,
            call_time,
            roaming_partner_name: None,
            telecommunications_service_call: None,
            id: None,
            telecommunications_service_category: None,
            quantity: None,
            telecommunications_service_call_code: None,
            telecommunications_service_category_code: None,
            pay_per_view: None,
            movie_title: None,
            exchange_rate: None,
            tax_total: None,
            call_extension_amount: None,
            price: None,
            call_base_amount: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AllowanceCharge>,
}

impl AsMut<TelecommunicationsServiceArrayOfAllowanceChargeComponent> for TelecommunicationsServiceArrayOfAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfAllowanceChargeComponent> for TelecommunicationsServiceArrayOfAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfAllowanceChargeComponent {
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
pub struct TelecommunicationsServiceArrayOfCallBaseAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CallBaseAmount>,
}

impl AsMut<TelecommunicationsServiceArrayOfCallBaseAmountComponent> for TelecommunicationsServiceArrayOfCallBaseAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfCallBaseAmountComponent> for TelecommunicationsServiceArrayOfCallBaseAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfCallBaseAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfCallBaseAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfCallBaseAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CallBaseAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CallBaseAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CallBaseAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CallBaseAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfCallDateComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CallDate>,
}

impl AsMut<TelecommunicationsServiceArrayOfCallDateComponent> for TelecommunicationsServiceArrayOfCallDateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfCallDateComponent> for TelecommunicationsServiceArrayOfCallDateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfCallDateComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfCallDateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfCallDateComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CallDate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CallDate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CallDate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CallDate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfCallDutyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CallDuty>,
}

impl AsMut<TelecommunicationsServiceArrayOfCallDutyComponent> for TelecommunicationsServiceArrayOfCallDutyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfCallDutyComponent> for TelecommunicationsServiceArrayOfCallDutyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfCallDutyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfCallDutyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CallDuty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CallDuty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CallDuty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CallDuty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfCallExtensionAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CallExtensionAmount>,
}

impl AsMut<TelecommunicationsServiceArrayOfCallExtensionAmountComponent> for TelecommunicationsServiceArrayOfCallExtensionAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfCallExtensionAmountComponent> for TelecommunicationsServiceArrayOfCallExtensionAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfCallExtensionAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfCallExtensionAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfCallExtensionAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CallExtensionAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CallExtensionAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CallExtensionAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CallExtensionAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfCallTimeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CallTime>,
}

impl AsMut<TelecommunicationsServiceArrayOfCallTimeComponent> for TelecommunicationsServiceArrayOfCallTimeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfCallTimeComponent> for TelecommunicationsServiceArrayOfCallTimeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfCallTimeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfCallTimeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfCallTimeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CallTime) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CallTime) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CallTime> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CallTime> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfCountryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Country>,
}

impl AsMut<TelecommunicationsServiceArrayOfCountryComponent> for TelecommunicationsServiceArrayOfCountryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfCountryComponent> for TelecommunicationsServiceArrayOfCountryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfCountryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfCountryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfCountryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Country) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Country) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Country> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Country> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfExchangeRateComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExchangeRate>,
}

impl AsMut<TelecommunicationsServiceArrayOfExchangeRateComponent> for TelecommunicationsServiceArrayOfExchangeRateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfExchangeRateComponent> for TelecommunicationsServiceArrayOfExchangeRateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfExchangeRateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfExchangeRateComponent {
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
pub struct TelecommunicationsServiceArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<TelecommunicationsServiceArrayOfIDComponent> for TelecommunicationsServiceArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfIDComponent> for TelecommunicationsServiceArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfIDComponent {
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
pub struct TelecommunicationsServiceArrayOfMovieTitleComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MovieTitle>,
}

impl AsMut<TelecommunicationsServiceArrayOfMovieTitleComponent> for TelecommunicationsServiceArrayOfMovieTitleComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfMovieTitleComponent> for TelecommunicationsServiceArrayOfMovieTitleComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfMovieTitleComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfMovieTitleComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfMovieTitleComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MovieTitle) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MovieTitle) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MovieTitle> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MovieTitle> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfPayPerViewComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PayPerView>,
}

impl AsMut<TelecommunicationsServiceArrayOfPayPerViewComponent> for TelecommunicationsServiceArrayOfPayPerViewComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfPayPerViewComponent> for TelecommunicationsServiceArrayOfPayPerViewComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfPayPerViewComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfPayPerViewComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfPayPerViewComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PayPerView) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PayPerView) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PayPerView> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PayPerView> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfPriceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Price>,
}

impl AsMut<TelecommunicationsServiceArrayOfPriceComponent> for TelecommunicationsServiceArrayOfPriceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfPriceComponent> for TelecommunicationsServiceArrayOfPriceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfPriceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfPriceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfPriceComponent {
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
pub struct TelecommunicationsServiceArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<TelecommunicationsServiceArrayOfQuantityComponent> for TelecommunicationsServiceArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfQuantityComponent> for TelecommunicationsServiceArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Quantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Quantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfRoamingPartnerNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RoamingPartnerName>,
}

impl AsMut<TelecommunicationsServiceArrayOfRoamingPartnerNameComponent> for TelecommunicationsServiceArrayOfRoamingPartnerNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfRoamingPartnerNameComponent> for TelecommunicationsServiceArrayOfRoamingPartnerNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfRoamingPartnerNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfRoamingPartnerNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfRoamingPartnerNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RoamingPartnerName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RoamingPartnerName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RoamingPartnerName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RoamingPartnerName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfServiceNumberCalledComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ServiceNumberCalled>,
}

impl AsMut<TelecommunicationsServiceArrayOfServiceNumberCalledComponent> for TelecommunicationsServiceArrayOfServiceNumberCalledComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfServiceNumberCalledComponent> for TelecommunicationsServiceArrayOfServiceNumberCalledComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfServiceNumberCalledComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfServiceNumberCalledComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfServiceNumberCalledComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ServiceNumberCalled) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ServiceNumberCalled) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ServiceNumberCalled> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ServiceNumberCalled> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfTaxTotalComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TaxTotal>,
}

impl AsMut<TelecommunicationsServiceArrayOfTaxTotalComponent> for TelecommunicationsServiceArrayOfTaxTotalComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfTaxTotalComponent> for TelecommunicationsServiceArrayOfTaxTotalComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfTaxTotalComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfTaxTotalComponent {
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
pub struct TelecommunicationsServiceArrayOfTelecommunicationsServiceCallComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCall>,
}

impl AsMut<TelecommunicationsServiceArrayOfTelecommunicationsServiceCallComponent> for TelecommunicationsServiceArrayOfTelecommunicationsServiceCallComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfTelecommunicationsServiceCallComponent> for TelecommunicationsServiceArrayOfTelecommunicationsServiceCallComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfTelecommunicationsServiceCallComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfTelecommunicationsServiceCallComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfTelecommunicationsServiceCallComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCall) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCall) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCall> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCall> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfTelecommunicationsServiceCallCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCallCode>,
}

impl AsMut<TelecommunicationsServiceArrayOfTelecommunicationsServiceCallCodeComponent> for TelecommunicationsServiceArrayOfTelecommunicationsServiceCallCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfTelecommunicationsServiceCallCodeComponent> for TelecommunicationsServiceArrayOfTelecommunicationsServiceCallCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfTelecommunicationsServiceCallCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfTelecommunicationsServiceCallCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfTelecommunicationsServiceCallCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCallCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCallCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCallCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCallCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCategory>,
}

impl AsMut<TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryComponent> for TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryComponent> for TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCategory) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCategory) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCategory> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCategory> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCategoryCode>,
}

impl AsMut<TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryCodeComponent> for TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryCodeComponent> for TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfTelecommunicationsServiceCategoryCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCategoryCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCategoryCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCategoryCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TelecommunicationsServiceCategoryCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TelecommunicationsServiceArrayOfTimeDutyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TimeDuty>,
}

impl AsMut<TelecommunicationsServiceArrayOfTimeDutyComponent> for TelecommunicationsServiceArrayOfTimeDutyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TelecommunicationsServiceArrayOfTimeDutyComponent> for TelecommunicationsServiceArrayOfTimeDutyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TelecommunicationsServiceArrayOfTimeDutyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TelecommunicationsServiceArrayOfTimeDutyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TimeDuty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TimeDuty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TimeDuty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TimeDuty> {
        self.items.iter()
    }
}


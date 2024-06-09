use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransportExecutionTerms {
    #[serde(rename = "BonusPaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_payment_terms: Option<TransportExecutionTermsArrayOfBonusPaymentTermsComponent>,
    #[serde(rename = "ChangeConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_conditions: Option<TransportExecutionTermsArrayOfChangeConditionsComponent>,
    #[serde(rename = "CommissionPaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_payment_terms: Option<TransportExecutionTermsArrayOfCommissionPaymentTermsComponent>,
    #[serde(rename = "DeliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<TransportExecutionTermsArrayOfDeliveryTermsComponent>,
    #[serde(rename = "EnvironmentalEmission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environmental_emission: Option<TransportExecutionTermsArrayOfEnvironmentalEmissionComponent>,
    #[serde(rename = "NotificationRequirement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_requirement: Option<TransportExecutionTermsArrayOfNotificationRequirementComponent>,
    #[serde(rename = "PaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<TransportExecutionTermsArrayOfPaymentTermsComponent>,
    #[serde(rename = "PenaltyPaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_payment_terms: Option<TransportExecutionTermsArrayOfPenaltyPaymentTermsComponent>,
    #[serde(rename = "ServiceChargePaymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_charge_payment_terms: Option<TransportExecutionTermsArrayOfServiceChargePaymentTermsComponent>,
    #[serde(rename = "TransportServiceProviderSpecialTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_service_provider_special_terms: Option<TransportExecutionTermsArrayOfTransportServiceProviderSpecialTermsComponent>,
    #[serde(rename = "TransportUserSpecialTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_user_special_terms: Option<TransportExecutionTermsArrayOfTransportUserSpecialTermsComponent>,
}

impl AsMut<TransportExecutionTerms> for TransportExecutionTerms {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTerms> for TransportExecutionTerms {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.commission_payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportExecutionTerms.commission_payment_terms", e));
            }
        }
        if let Some(v) = &self.bonus_payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportExecutionTerms.bonus_payment_terms", e));
            }
        }
        if let Some(v) = &self.payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportExecutionTerms.payment_terms", e));
            }
        }
        if let Some(v) = &self.delivery_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportExecutionTerms.delivery_terms", e));
            }
        }
        if let Some(v) = &self.notification_requirement {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportExecutionTerms.notification_requirement", e));
            }
        }
        if let Some(v) = &self.service_charge_payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportExecutionTerms.service_charge_payment_terms", e));
            }
        }
        if let Some(v) = &self.transport_service_provider_special_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportExecutionTerms.transport_service_provider_special_terms", e));
            }
        }
        if let Some(v) = &self.change_conditions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportExecutionTerms.change_conditions", e));
            }
        }
        if let Some(v) = &self.penalty_payment_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportExecutionTerms.penalty_payment_terms", e));
            }
        }
        if let Some(v) = &self.environmental_emission {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportExecutionTerms.environmental_emission", e));
            }
        }
        if let Some(v) = &self.transport_user_special_terms {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportExecutionTerms.transport_user_special_terms", e));
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

impl TransportExecutionTerms {
    pub fn title() -> &'static str {
        "Transport Execution Terms. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe terms applying to a transport execution plan."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            delivery_terms: None,
            notification_requirement: None,
            penalty_payment_terms: None,
            change_conditions: None,
            bonus_payment_terms: None,
            commission_payment_terms: None,
            transport_service_provider_special_terms: None,
            transport_user_special_terms: None,
            payment_terms: None,
            service_charge_payment_terms: None,
            environmental_emission: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportExecutionTermsArrayOfBonusPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BonusPaymentTerms>,
}

impl AsMut<TransportExecutionTermsArrayOfBonusPaymentTermsComponent> for TransportExecutionTermsArrayOfBonusPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTermsArrayOfBonusPaymentTermsComponent> for TransportExecutionTermsArrayOfBonusPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfBonusPaymentTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfBonusPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportExecutionTermsArrayOfBonusPaymentTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BonusPaymentTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BonusPaymentTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BonusPaymentTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BonusPaymentTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportExecutionTermsArrayOfChangeConditionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ChangeConditions>,
}

impl AsMut<TransportExecutionTermsArrayOfChangeConditionsComponent> for TransportExecutionTermsArrayOfChangeConditionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTermsArrayOfChangeConditionsComponent> for TransportExecutionTermsArrayOfChangeConditionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfChangeConditionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportExecutionTermsArrayOfChangeConditionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ChangeConditions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ChangeConditions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ChangeConditions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ChangeConditions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportExecutionTermsArrayOfCommissionPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CommissionPaymentTerms>,
}

impl AsMut<TransportExecutionTermsArrayOfCommissionPaymentTermsComponent> for TransportExecutionTermsArrayOfCommissionPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTermsArrayOfCommissionPaymentTermsComponent> for TransportExecutionTermsArrayOfCommissionPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfCommissionPaymentTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfCommissionPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportExecutionTermsArrayOfCommissionPaymentTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CommissionPaymentTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CommissionPaymentTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CommissionPaymentTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CommissionPaymentTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportExecutionTermsArrayOfDeliveryTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms>,
}

impl AsMut<TransportExecutionTermsArrayOfDeliveryTermsComponent> for TransportExecutionTermsArrayOfDeliveryTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTermsArrayOfDeliveryTermsComponent> for TransportExecutionTermsArrayOfDeliveryTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfDeliveryTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportExecutionTermsArrayOfDeliveryTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportExecutionTermsArrayOfEnvironmentalEmissionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EnvironmentalEmission>,
}

impl AsMut<TransportExecutionTermsArrayOfEnvironmentalEmissionComponent> for TransportExecutionTermsArrayOfEnvironmentalEmissionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTermsArrayOfEnvironmentalEmissionComponent> for TransportExecutionTermsArrayOfEnvironmentalEmissionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfEnvironmentalEmissionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportExecutionTermsArrayOfEnvironmentalEmissionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EnvironmentalEmission) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EnvironmentalEmission) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EnvironmentalEmission> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EnvironmentalEmission> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportExecutionTermsArrayOfNotificationRequirementComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::NotificationRequirement>,
}

impl AsMut<TransportExecutionTermsArrayOfNotificationRequirementComponent> for TransportExecutionTermsArrayOfNotificationRequirementComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTermsArrayOfNotificationRequirementComponent> for TransportExecutionTermsArrayOfNotificationRequirementComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfNotificationRequirementComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportExecutionTermsArrayOfNotificationRequirementComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::NotificationRequirement) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::NotificationRequirement) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::NotificationRequirement> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::NotificationRequirement> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportExecutionTermsArrayOfPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms>,
}

impl AsMut<TransportExecutionTermsArrayOfPaymentTermsComponent> for TransportExecutionTermsArrayOfPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTermsArrayOfPaymentTermsComponent> for TransportExecutionTermsArrayOfPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportExecutionTermsArrayOfPaymentTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PaymentTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PaymentTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PaymentTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportExecutionTermsArrayOfPenaltyPaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PenaltyPaymentTerms>,
}

impl AsMut<TransportExecutionTermsArrayOfPenaltyPaymentTermsComponent> for TransportExecutionTermsArrayOfPenaltyPaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTermsArrayOfPenaltyPaymentTermsComponent> for TransportExecutionTermsArrayOfPenaltyPaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfPenaltyPaymentTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfPenaltyPaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportExecutionTermsArrayOfPenaltyPaymentTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PenaltyPaymentTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PenaltyPaymentTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PenaltyPaymentTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PenaltyPaymentTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportExecutionTermsArrayOfServiceChargePaymentTermsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ServiceChargePaymentTerms>,
}

impl AsMut<TransportExecutionTermsArrayOfServiceChargePaymentTermsComponent> for TransportExecutionTermsArrayOfServiceChargePaymentTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTermsArrayOfServiceChargePaymentTermsComponent> for TransportExecutionTermsArrayOfServiceChargePaymentTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfServiceChargePaymentTermsComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfServiceChargePaymentTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportExecutionTermsArrayOfServiceChargePaymentTermsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ServiceChargePaymentTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ServiceChargePaymentTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ServiceChargePaymentTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ServiceChargePaymentTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportExecutionTermsArrayOfTransportServiceProviderSpecialTermsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportServiceProviderSpecialTerms>,
}

impl AsMut<TransportExecutionTermsArrayOfTransportServiceProviderSpecialTermsComponent> for TransportExecutionTermsArrayOfTransportServiceProviderSpecialTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTermsArrayOfTransportServiceProviderSpecialTermsComponent> for TransportExecutionTermsArrayOfTransportServiceProviderSpecialTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfTransportServiceProviderSpecialTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportExecutionTermsArrayOfTransportServiceProviderSpecialTermsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportServiceProviderSpecialTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportServiceProviderSpecialTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportServiceProviderSpecialTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportServiceProviderSpecialTerms> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportExecutionTermsArrayOfTransportUserSpecialTermsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportUserSpecialTerms>,
}

impl AsMut<TransportExecutionTermsArrayOfTransportUserSpecialTermsComponent> for TransportExecutionTermsArrayOfTransportUserSpecialTermsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportExecutionTermsArrayOfTransportUserSpecialTermsComponent> for TransportExecutionTermsArrayOfTransportUserSpecialTermsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportExecutionTermsArrayOfTransportUserSpecialTermsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportExecutionTermsArrayOfTransportUserSpecialTermsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportUserSpecialTerms) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportUserSpecialTerms) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportUserSpecialTerms> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportUserSpecialTerms> {
        self.items.iter()
    }
}


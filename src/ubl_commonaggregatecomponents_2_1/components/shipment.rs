use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Shipment {
    #[serde(rename = "Consignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignment: Option<ShipmentArrayOfConsignmentComponent>,
    #[serde(rename = "ConsignmentQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignment_quantity: Option<ShipmentArrayOfConsignmentQuantityComponent>,
    #[serde(rename = "DeclaredCustomsValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_customs_value_amount: Option<ShipmentArrayOfDeclaredCustomsValueAmountComponent>,
    #[serde(rename = "DeclaredForCarriageValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_for_carriage_value_amount: Option<ShipmentArrayOfDeclaredForCarriageValueAmountComponent>,
    #[serde(rename = "DeclaredStatisticsValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_statistics_value_amount: Option<ShipmentArrayOfDeclaredStatisticsValueAmountComponent>,
    #[serde(rename = "Delivery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<ShipmentArrayOfDeliveryComponent>,
    #[serde(rename = "DeliveryInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_instructions: Option<ShipmentArrayOfDeliveryInstructionsComponent>,
    #[serde(rename = "ExportCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_country: Option<ShipmentArrayOfExportCountryComponent>,
    #[serde(rename = "FirstArrivalPortLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_arrival_port_location: Option<ShipmentArrayOfFirstArrivalPortLocationComponent>,
    #[serde(rename = "FreeOnBoardValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_on_board_value_amount: Option<ShipmentArrayOfFreeOnBoardValueAmountComponent>,
    #[serde(rename = "FreightAllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_allowance_charge: Option<ShipmentArrayOfFreightAllowanceChargeComponent>,
    #[serde(rename = "GoodsItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_item: Option<ShipmentArrayOfGoodsItemComponent>,
    #[serde(rename = "GrossVolumeMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_volume_measure: Option<ShipmentArrayOfGrossVolumeMeasureComponent>,
    #[serde(rename = "GrossWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_weight_measure: Option<ShipmentArrayOfGrossWeightMeasureComponent>,
    #[serde(rename = "HandlingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handling_code: Option<ShipmentArrayOfHandlingCodeComponent>,
    #[serde(rename = "HandlingInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handling_instructions: Option<ShipmentArrayOfHandlingInstructionsComponent>,
    #[serde(rename = "ID")]
    pub id: ShipmentArrayOfIDComponent,
    #[serde(rename = "Information")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<ShipmentArrayOfInformationComponent>,
    #[serde(rename = "InsuranceValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_value_amount: Option<ShipmentArrayOfInsuranceValueAmountComponent>,
    #[serde(rename = "LastExitPortLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_exit_port_location: Option<ShipmentArrayOfLastExitPortLocationComponent>,
    #[serde(rename = "NetNetWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_net_weight_measure: Option<ShipmentArrayOfNetNetWeightMeasureComponent>,
    #[serde(rename = "NetVolumeMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_volume_measure: Option<ShipmentArrayOfNetVolumeMeasureComponent>,
    #[serde(rename = "NetWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_weight_measure: Option<ShipmentArrayOfNetWeightMeasureComponent>,
    #[serde(rename = "OriginAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_address: Option<ShipmentArrayOfOriginAddressComponent>,
    #[serde(rename = "ReturnAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_address: Option<ShipmentArrayOfReturnAddressComponent>,
    #[serde(rename = "ShipmentStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_stage: Option<ShipmentArrayOfShipmentStageComponent>,
    #[serde(rename = "ShippingPriorityLevelCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_priority_level_code: Option<ShipmentArrayOfShippingPriorityLevelCodeComponent>,
    #[serde(rename = "SpecialInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_instructions: Option<ShipmentArrayOfSpecialInstructionsComponent>,
    #[serde(rename = "SplitConsignmentIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_consignment_indicator: Option<ShipmentArrayOfSplitConsignmentIndicatorComponent>,
    #[serde(rename = "TotalGoodsItemQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_goods_item_quantity: Option<ShipmentArrayOfTotalGoodsItemQuantityComponent>,
    #[serde(rename = "TotalTransportHandlingUnitQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_transport_handling_unit_quantity: Option<ShipmentArrayOfTotalTransportHandlingUnitQuantityComponent>,
    #[serde(rename = "TransportHandlingUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_handling_unit: Option<ShipmentArrayOfTransportHandlingUnitComponent>,
}

impl AsMut<Shipment> for Shipment {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Shipment> for Shipment {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.consignment_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.consignment_quantity", e));
            }
        }
        if let Some(v) = &self.net_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.net_weight_measure", e));
            }
        }
        if let Some(v) = &self.consignment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.consignment", e));
            }
        }
        if let Some(v) = &self.declared_for_carriage_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.declared_for_carriage_value_amount", e));
            }
        }
        if let Some(v) = &self.first_arrival_port_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.first_arrival_port_location", e));
            }
        }
        if let Some(v) = &self.information {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.information", e));
            }
        }
        if let Some(v) = &self.net_volume_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.net_volume_measure", e));
            }
        }
        if let Some(v) = &self.shipping_priority_level_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.shipping_priority_level_code", e));
            }
        }
        if let Some(v) = &self.goods_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.goods_item", e));
            }
        }
        if let Some(v) = &self.handling_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.handling_instructions", e));
            }
        }
        if let Some(v) = &self.export_country {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.export_country", e));
            }
        }
        if let Some(v) = &self.special_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.special_instructions", e));
            }
        }
        if let Some(v) = &self.insurance_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.insurance_value_amount", e));
            }
        }
        if let Some(v) = &self.gross_volume_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.gross_volume_measure", e));
            }
        }
        if let Some(v) = &self.declared_customs_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.declared_customs_value_amount", e));
            }
        }
        if let Some(v) = &self.gross_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.gross_weight_measure", e));
            }
        }
        if let Some(v) = &self.transport_handling_unit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.transport_handling_unit", e));
            }
        }
        if let Some(v) = &self.free_on_board_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.free_on_board_value_amount", e));
            }
        }
        if let Some(v) = &self.delivery {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.delivery", e));
            }
        }
        if let Some(v) = &self.freight_allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.freight_allowance_charge", e));
            }
        }
        if let Some(v) = &self.delivery_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.delivery_instructions", e));
            }
        }
        if let Some(v) = &self.origin_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.origin_address", e));
            }
        }
        if let Some(v) = &self.return_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.return_address", e));
            }
        }
        if let Some(v) = &self.declared_statistics_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.declared_statistics_value_amount", e));
            }
        }
        if let Some(v) = &self.shipment_stage {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.shipment_stage", e));
            }
        }
        if let Some(v) = &self.split_consignment_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.split_consignment_indicator", e));
            }
        }
        if let Some(v) = &self.net_net_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.net_net_weight_measure", e));
            }
        }
        if let Some(v) = &self.total_transport_handling_unit_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.total_transport_handling_unit_quantity", e));
            }
        }
        if let Some(v) = &self.handling_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.handling_code", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("Shipment.id", e));
        }
        if let Some(v) = &self.last_exit_port_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.last_exit_port_location", e));
            }
        }
        if let Some(v) = &self.total_goods_item_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Shipment.total_goods_item_quantity", e));
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

impl Shipment {
    pub fn title() -> &'static str {
        "Shipment. Details"
    }
    pub fn description() -> &'static str {
        "A class defining an identifiable collection of one or more goods items to be transported between the seller party and the buyer party. This information may be defined within a commercial contract. A shipment can be transported in different consignments (e.g., split for logistical purposes)."
    }
    pub fn new(id: ShipmentArrayOfIDComponent) -> Component<Self> {
        Component(Self {
            declared_statistics_value_amount: None,
            transport_handling_unit: None,
            insurance_value_amount: None,
            goods_item: None,
            information: None,
            id,
            total_goods_item_quantity: None,
            special_instructions: None,
            gross_volume_measure: None,
            delivery_instructions: None,
            last_exit_port_location: None,
            net_volume_measure: None,
            split_consignment_indicator: None,
            handling_code: None,
            delivery: None,
            export_country: None,
            freight_allowance_charge: None,
            declared_for_carriage_value_amount: None,
            consignment_quantity: None,
            first_arrival_port_location: None,
            consignment: None,
            net_weight_measure: None,
            handling_instructions: None,
            declared_customs_value_amount: None,
            shipment_stage: None,
            shipping_priority_level_code: None,
            origin_address: None,
            gross_weight_measure: None,
            net_net_weight_measure: None,
            total_transport_handling_unit_quantity: None,
            return_address: None,
            free_on_board_value_amount: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfConsignmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Consignment>,
}

impl AsMut<ShipmentArrayOfConsignmentComponent> for ShipmentArrayOfConsignmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfConsignmentComponent> for ShipmentArrayOfConsignmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfConsignmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfConsignmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Consignment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Consignment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Consignment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Consignment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfConsignmentQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsignmentQuantity>,
}

impl AsMut<ShipmentArrayOfConsignmentQuantityComponent> for ShipmentArrayOfConsignmentQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfConsignmentQuantityComponent> for ShipmentArrayOfConsignmentQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfConsignmentQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfConsignmentQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfConsignmentQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsignmentQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsignmentQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsignmentQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsignmentQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfDeclaredCustomsValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeclaredCustomsValueAmount>,
}

impl AsMut<ShipmentArrayOfDeclaredCustomsValueAmountComponent> for ShipmentArrayOfDeclaredCustomsValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfDeclaredCustomsValueAmountComponent> for ShipmentArrayOfDeclaredCustomsValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfDeclaredCustomsValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfDeclaredCustomsValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfDeclaredCustomsValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DeclaredCustomsValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DeclaredCustomsValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DeclaredCustomsValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DeclaredCustomsValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfDeclaredForCarriageValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeclaredForCarriageValueAmount>,
}

impl AsMut<ShipmentArrayOfDeclaredForCarriageValueAmountComponent> for ShipmentArrayOfDeclaredForCarriageValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfDeclaredForCarriageValueAmountComponent> for ShipmentArrayOfDeclaredForCarriageValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfDeclaredForCarriageValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfDeclaredForCarriageValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfDeclaredForCarriageValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DeclaredForCarriageValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DeclaredForCarriageValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DeclaredForCarriageValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DeclaredForCarriageValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfDeclaredStatisticsValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeclaredStatisticsValueAmount>,
}

impl AsMut<ShipmentArrayOfDeclaredStatisticsValueAmountComponent> for ShipmentArrayOfDeclaredStatisticsValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfDeclaredStatisticsValueAmountComponent> for ShipmentArrayOfDeclaredStatisticsValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfDeclaredStatisticsValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfDeclaredStatisticsValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfDeclaredStatisticsValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DeclaredStatisticsValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DeclaredStatisticsValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DeclaredStatisticsValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DeclaredStatisticsValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfDeliveryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Delivery>,
}

impl AsMut<ShipmentArrayOfDeliveryComponent> for ShipmentArrayOfDeliveryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfDeliveryComponent> for ShipmentArrayOfDeliveryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfDeliveryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfDeliveryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfDeliveryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Delivery) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Delivery) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Delivery> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Delivery> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfDeliveryInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeliveryInstructions>,
}

impl AsMut<ShipmentArrayOfDeliveryInstructionsComponent> for ShipmentArrayOfDeliveryInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfDeliveryInstructionsComponent> for ShipmentArrayOfDeliveryInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfDeliveryInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfDeliveryInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DeliveryInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DeliveryInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DeliveryInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DeliveryInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfExportCountryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ExportCountry>,
}

impl AsMut<ShipmentArrayOfExportCountryComponent> for ShipmentArrayOfExportCountryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfExportCountryComponent> for ShipmentArrayOfExportCountryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfExportCountryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfExportCountryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfExportCountryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ExportCountry) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ExportCountry) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ExportCountry> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ExportCountry> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfFirstArrivalPortLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FirstArrivalPortLocation>,
}

impl AsMut<ShipmentArrayOfFirstArrivalPortLocationComponent> for ShipmentArrayOfFirstArrivalPortLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfFirstArrivalPortLocationComponent> for ShipmentArrayOfFirstArrivalPortLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfFirstArrivalPortLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfFirstArrivalPortLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfFirstArrivalPortLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FirstArrivalPortLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FirstArrivalPortLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FirstArrivalPortLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FirstArrivalPortLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfFreeOnBoardValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FreeOnBoardValueAmount>,
}

impl AsMut<ShipmentArrayOfFreeOnBoardValueAmountComponent> for ShipmentArrayOfFreeOnBoardValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfFreeOnBoardValueAmountComponent> for ShipmentArrayOfFreeOnBoardValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfFreeOnBoardValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfFreeOnBoardValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfFreeOnBoardValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::FreeOnBoardValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::FreeOnBoardValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::FreeOnBoardValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::FreeOnBoardValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfFreightAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge>,
}

impl AsMut<ShipmentArrayOfFreightAllowanceChargeComponent> for ShipmentArrayOfFreightAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfFreightAllowanceChargeComponent> for ShipmentArrayOfFreightAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfFreightAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfFreightAllowanceChargeComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfGoodsItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::GoodsItem>,
}

impl AsMut<ShipmentArrayOfGoodsItemComponent> for ShipmentArrayOfGoodsItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfGoodsItemComponent> for ShipmentArrayOfGoodsItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfGoodsItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfGoodsItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::GoodsItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::GoodsItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::GoodsItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::GoodsItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfGrossVolumeMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure>,
}

impl AsMut<ShipmentArrayOfGrossVolumeMeasureComponent> for ShipmentArrayOfGrossVolumeMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfGrossVolumeMeasureComponent> for ShipmentArrayOfGrossVolumeMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfGrossVolumeMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfGrossVolumeMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfGrossVolumeMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfGrossWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure>,
}

impl AsMut<ShipmentArrayOfGrossWeightMeasureComponent> for ShipmentArrayOfGrossWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfGrossWeightMeasureComponent> for ShipmentArrayOfGrossWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfGrossWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfGrossWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfGrossWeightMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfHandlingCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HandlingCode>,
}

impl AsMut<ShipmentArrayOfHandlingCodeComponent> for ShipmentArrayOfHandlingCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfHandlingCodeComponent> for ShipmentArrayOfHandlingCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfHandlingCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfHandlingCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfHandlingCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HandlingCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HandlingCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HandlingCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HandlingCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfHandlingInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HandlingInstructions>,
}

impl AsMut<ShipmentArrayOfHandlingInstructionsComponent> for ShipmentArrayOfHandlingInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfHandlingInstructionsComponent> for ShipmentArrayOfHandlingInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfHandlingInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfHandlingInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HandlingInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HandlingInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HandlingInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HandlingInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<ShipmentArrayOfIDComponent> for ShipmentArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfIDComponent> for ShipmentArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfIDComponent {
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
pub struct ShipmentArrayOfInformationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Information>,
}

impl AsMut<ShipmentArrayOfInformationComponent> for ShipmentArrayOfInformationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfInformationComponent> for ShipmentArrayOfInformationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfInformationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfInformationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Information) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Information) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Information> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Information> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfInsuranceValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InsuranceValueAmount>,
}

impl AsMut<ShipmentArrayOfInsuranceValueAmountComponent> for ShipmentArrayOfInsuranceValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfInsuranceValueAmountComponent> for ShipmentArrayOfInsuranceValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfInsuranceValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfInsuranceValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfInsuranceValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InsuranceValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InsuranceValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InsuranceValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InsuranceValueAmount> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfLastExitPortLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LastExitPortLocation>,
}

impl AsMut<ShipmentArrayOfLastExitPortLocationComponent> for ShipmentArrayOfLastExitPortLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfLastExitPortLocationComponent> for ShipmentArrayOfLastExitPortLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfLastExitPortLocationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfLastExitPortLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfLastExitPortLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LastExitPortLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LastExitPortLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LastExitPortLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LastExitPortLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfNetNetWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetNetWeightMeasure>,
}

impl AsMut<ShipmentArrayOfNetNetWeightMeasureComponent> for ShipmentArrayOfNetNetWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfNetNetWeightMeasureComponent> for ShipmentArrayOfNetNetWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfNetNetWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfNetNetWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfNetNetWeightMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NetNetWeightMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NetNetWeightMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NetNetWeightMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NetNetWeightMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfNetVolumeMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure>,
}

impl AsMut<ShipmentArrayOfNetVolumeMeasureComponent> for ShipmentArrayOfNetVolumeMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfNetVolumeMeasureComponent> for ShipmentArrayOfNetVolumeMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfNetVolumeMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfNetVolumeMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfNetVolumeMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfNetWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure>,
}

impl AsMut<ShipmentArrayOfNetWeightMeasureComponent> for ShipmentArrayOfNetWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfNetWeightMeasureComponent> for ShipmentArrayOfNetWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfNetWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfNetWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfNetWeightMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfOriginAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginAddress>,
}

impl AsMut<ShipmentArrayOfOriginAddressComponent> for ShipmentArrayOfOriginAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfOriginAddressComponent> for ShipmentArrayOfOriginAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfOriginAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfOriginAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfOriginAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OriginAddress) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OriginAddress) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OriginAddress> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OriginAddress> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfReturnAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReturnAddress>,
}

impl AsMut<ShipmentArrayOfReturnAddressComponent> for ShipmentArrayOfReturnAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfReturnAddressComponent> for ShipmentArrayOfReturnAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfReturnAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfReturnAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfReturnAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReturnAddress) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReturnAddress) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReturnAddress> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReturnAddress> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfShipmentStageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ShipmentStage>,
}

impl AsMut<ShipmentArrayOfShipmentStageComponent> for ShipmentArrayOfShipmentStageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfShipmentStageComponent> for ShipmentArrayOfShipmentStageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfShipmentStageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfShipmentStageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ShipmentStage) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ShipmentStage) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ShipmentStage> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ShipmentStage> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfShippingPriorityLevelCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ShippingPriorityLevelCode>,
}

impl AsMut<ShipmentArrayOfShippingPriorityLevelCodeComponent> for ShipmentArrayOfShippingPriorityLevelCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfShippingPriorityLevelCodeComponent> for ShipmentArrayOfShippingPriorityLevelCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfShippingPriorityLevelCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfShippingPriorityLevelCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfShippingPriorityLevelCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ShippingPriorityLevelCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ShippingPriorityLevelCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ShippingPriorityLevelCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ShippingPriorityLevelCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfSpecialInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SpecialInstructions>,
}

impl AsMut<ShipmentArrayOfSpecialInstructionsComponent> for ShipmentArrayOfSpecialInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfSpecialInstructionsComponent> for ShipmentArrayOfSpecialInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfSpecialInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfSpecialInstructionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SpecialInstructions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SpecialInstructions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SpecialInstructions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SpecialInstructions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfSplitConsignmentIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SplitConsignmentIndicator>,
}

impl AsMut<ShipmentArrayOfSplitConsignmentIndicatorComponent> for ShipmentArrayOfSplitConsignmentIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfSplitConsignmentIndicatorComponent> for ShipmentArrayOfSplitConsignmentIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfSplitConsignmentIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfSplitConsignmentIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfSplitConsignmentIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SplitConsignmentIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SplitConsignmentIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SplitConsignmentIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SplitConsignmentIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfTotalGoodsItemQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalGoodsItemQuantity>,
}

impl AsMut<ShipmentArrayOfTotalGoodsItemQuantityComponent> for ShipmentArrayOfTotalGoodsItemQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfTotalGoodsItemQuantityComponent> for ShipmentArrayOfTotalGoodsItemQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfTotalGoodsItemQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfTotalGoodsItemQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfTotalGoodsItemQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalGoodsItemQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalGoodsItemQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalGoodsItemQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalGoodsItemQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfTotalTransportHandlingUnitQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalTransportHandlingUnitQuantity>,
}

impl AsMut<ShipmentArrayOfTotalTransportHandlingUnitQuantityComponent> for ShipmentArrayOfTotalTransportHandlingUnitQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfTotalTransportHandlingUnitQuantityComponent> for ShipmentArrayOfTotalTransportHandlingUnitQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ShipmentArrayOfTotalTransportHandlingUnitQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfTotalTransportHandlingUnitQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfTotalTransportHandlingUnitQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalTransportHandlingUnitQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalTransportHandlingUnitQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalTransportHandlingUnitQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalTransportHandlingUnitQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShipmentArrayOfTransportHandlingUnitComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportHandlingUnit>,
}

impl AsMut<ShipmentArrayOfTransportHandlingUnitComponent> for ShipmentArrayOfTransportHandlingUnitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ShipmentArrayOfTransportHandlingUnitComponent> for ShipmentArrayOfTransportHandlingUnitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ShipmentArrayOfTransportHandlingUnitComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ShipmentArrayOfTransportHandlingUnitComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportHandlingUnit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportHandlingUnit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportHandlingUnit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportHandlingUnit> {
        self.items.iter()
    }
}


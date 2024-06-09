use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GoodsItem {
    #[serde(rename = "ChargeableQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargeable_quantity: Option<GoodsItemArrayOfChargeableQuantityComponent>,
    #[serde(rename = "ChargeableWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargeable_weight_measure: Option<GoodsItemArrayOfChargeableWeightMeasureComponent>,
    #[serde(rename = "ContainedGoodsItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contained_goods_item: Option<GoodsItemArrayOfContainedGoodsItemComponent>,
    #[serde(rename = "ContainingPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containing_package: Option<GoodsItemArrayOfContainingPackageComponent>,
    #[serde(rename = "CustomsImportClassifiedIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customs_import_classified_indicator: Option<GoodsItemArrayOfCustomsImportClassifiedIndicatorComponent>,
    #[serde(rename = "CustomsStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customs_status_code: Option<GoodsItemArrayOfCustomsStatusCodeComponent>,
    #[serde(rename = "CustomsTariffQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customs_tariff_quantity: Option<GoodsItemArrayOfCustomsTariffQuantityComponent>,
    #[serde(rename = "DeclaredCustomsValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_customs_value_amount: Option<GoodsItemArrayOfDeclaredCustomsValueAmountComponent>,
    #[serde(rename = "DeclaredForCarriageValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_for_carriage_value_amount: Option<GoodsItemArrayOfDeclaredForCarriageValueAmountComponent>,
    #[serde(rename = "DeclaredStatisticsValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_statistics_value_amount: Option<GoodsItemArrayOfDeclaredStatisticsValueAmountComponent>,
    #[serde(rename = "Delivery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<GoodsItemArrayOfDeliveryComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<GoodsItemArrayOfDescriptionComponent>,
    #[serde(rename = "Despatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch: Option<GoodsItemArrayOfDespatchComponent>,
    #[serde(rename = "FreeOnBoardValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_on_board_value_amount: Option<GoodsItemArrayOfFreeOnBoardValueAmountComponent>,
    #[serde(rename = "FreightAllowanceCharge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_allowance_charge: Option<GoodsItemArrayOfFreightAllowanceChargeComponent>,
    #[serde(rename = "GoodsItemContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_item_container: Option<GoodsItemArrayOfGoodsItemContainerComponent>,
    #[serde(rename = "GrossVolumeMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_volume_measure: Option<GoodsItemArrayOfGrossVolumeMeasureComponent>,
    #[serde(rename = "GrossWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_weight_measure: Option<GoodsItemArrayOfGrossWeightMeasureComponent>,
    #[serde(rename = "HazardousRiskIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_risk_indicator: Option<GoodsItemArrayOfHazardousRiskIndicatorComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<GoodsItemArrayOfIDComponent>,
    #[serde(rename = "InsuranceValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_value_amount: Option<GoodsItemArrayOfInsuranceValueAmountComponent>,
    #[serde(rename = "InvoiceLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line: Option<GoodsItemArrayOfInvoiceLineComponent>,
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<GoodsItemArrayOfItemComponent>,
    #[serde(rename = "MaximumTemperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_temperature: Option<GoodsItemArrayOfMaximumTemperatureComponent>,
    #[serde(rename = "MeasurementDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_dimension: Option<GoodsItemArrayOfMeasurementDimensionComponent>,
    #[serde(rename = "MinimumTemperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_temperature: Option<GoodsItemArrayOfMinimumTemperatureComponent>,
    #[serde(rename = "NetNetWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_net_weight_measure: Option<GoodsItemArrayOfNetNetWeightMeasureComponent>,
    #[serde(rename = "NetVolumeMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_volume_measure: Option<GoodsItemArrayOfNetVolumeMeasureComponent>,
    #[serde(rename = "NetWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_weight_measure: Option<GoodsItemArrayOfNetWeightMeasureComponent>,
    #[serde(rename = "OriginAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_address: Option<GoodsItemArrayOfOriginAddressComponent>,
    #[serde(rename = "Pickup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup: Option<GoodsItemArrayOfPickupComponent>,
    #[serde(rename = "PreferenceCriterionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference_criterion_code: Option<GoodsItemArrayOfPreferenceCriterionCodeComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<GoodsItemArrayOfQuantityComponent>,
    #[serde(rename = "RequiredCustomsID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_customs_id: Option<GoodsItemArrayOfRequiredCustomsIDComponent>,
    #[serde(rename = "ReturnableQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnable_quantity: Option<GoodsItemArrayOfReturnableQuantityComponent>,
    #[serde(rename = "SequenceNumberID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number_id: Option<GoodsItemArrayOfSequenceNumberIDComponent>,
    #[serde(rename = "ShipmentDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_document_reference: Option<GoodsItemArrayOfShipmentDocumentReferenceComponent>,
    #[serde(rename = "Temperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<GoodsItemArrayOfTemperatureComponent>,
    #[serde(rename = "TraceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<GoodsItemArrayOfTraceIDComponent>,
    #[serde(rename = "ValueAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_amount: Option<GoodsItemArrayOfValueAmountComponent>,
}

impl AsMut<GoodsItem> for GoodsItem {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItem> for GoodsItem {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.maximum_temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.maximum_temperature", e));
            }
        }
        if let Some(v) = &self.value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.value_amount", e));
            }
        }
        if let Some(v) = &self.customs_import_classified_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.customs_import_classified_indicator", e));
            }
        }
        if let Some(v) = &self.customs_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.customs_status_code", e));
            }
        }
        if let Some(v) = &self.net_volume_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.net_volume_measure", e));
            }
        }
        if let Some(v) = &self.shipment_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.shipment_document_reference", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.description", e));
            }
        }
        if let Some(v) = &self.goods_item_container {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.goods_item_container", e));
            }
        }
        if let Some(v) = &self.gross_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.gross_weight_measure", e));
            }
        }
        if let Some(v) = &self.preference_criterion_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.preference_criterion_code", e));
            }
        }
        if let Some(v) = &self.containing_package {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.containing_package", e));
            }
        }
        if let Some(v) = &self.net_net_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.net_net_weight_measure", e));
            }
        }
        if let Some(v) = &self.declared_for_carriage_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.declared_for_carriage_value_amount", e));
            }
        }
        if let Some(v) = &self.customs_tariff_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.customs_tariff_quantity", e));
            }
        }
        if let Some(v) = &self.chargeable_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.chargeable_quantity", e));
            }
        }
        if let Some(v) = &self.contained_goods_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.contained_goods_item", e));
            }
        }
        if let Some(v) = &self.freight_allowance_charge {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.freight_allowance_charge", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.id", e));
            }
        }
        if let Some(v) = &self.item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.item", e));
            }
        }
        if let Some(v) = &self.free_on_board_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.free_on_board_value_amount", e));
            }
        }
        if let Some(v) = &self.net_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.net_weight_measure", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.quantity", e));
            }
        }
        if let Some(v) = &self.returnable_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.returnable_quantity", e));
            }
        }
        if let Some(v) = &self.pickup {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.pickup", e));
            }
        }
        if let Some(v) = &self.invoice_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.invoice_line", e));
            }
        }
        if let Some(v) = &self.hazardous_risk_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.hazardous_risk_indicator", e));
            }
        }
        if let Some(v) = &self.temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.temperature", e));
            }
        }
        if let Some(v) = &self.gross_volume_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.gross_volume_measure", e));
            }
        }
        if let Some(v) = &self.sequence_number_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.sequence_number_id", e));
            }
        }
        if let Some(v) = &self.declared_customs_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.declared_customs_value_amount", e));
            }
        }
        if let Some(v) = &self.measurement_dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.measurement_dimension", e));
            }
        }
        if let Some(v) = &self.minimum_temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.minimum_temperature", e));
            }
        }
        if let Some(v) = &self.despatch {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.despatch", e));
            }
        }
        if let Some(v) = &self.origin_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.origin_address", e));
            }
        }
        if let Some(v) = &self.required_customs_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.required_customs_id", e));
            }
        }
        if let Some(v) = &self.declared_statistics_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.declared_statistics_value_amount", e));
            }
        }
        if let Some(v) = &self.chargeable_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.chargeable_weight_measure", e));
            }
        }
        if let Some(v) = &self.insurance_value_amount {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.insurance_value_amount", e));
            }
        }
        if let Some(v) = &self.trace_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.trace_id", e));
            }
        }
        if let Some(v) = &self.delivery {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("GoodsItem.delivery", e));
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

impl GoodsItem {
    pub fn title() -> &'static str {
        "Goods Item. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a separately identifiable quantity of goods of a single product type."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            net_volume_measure: None,
            origin_address: None,
            measurement_dimension: None,
            trace_id: None,
            free_on_board_value_amount: None,
            declared_customs_value_amount: None,
            minimum_temperature: None,
            maximum_temperature: None,
            declared_for_carriage_value_amount: None,
            insurance_value_amount: None,
            item: None,
            declared_statistics_value_amount: None,
            chargeable_quantity: None,
            pickup: None,
            net_weight_measure: None,
            description: None,
            customs_import_classified_indicator: None,
            despatch: None,
            id: None,
            net_net_weight_measure: None,
            freight_allowance_charge: None,
            sequence_number_id: None,
            shipment_document_reference: None,
            customs_status_code: None,
            gross_volume_measure: None,
            delivery: None,
            hazardous_risk_indicator: None,
            quantity: None,
            returnable_quantity: None,
            temperature: None,
            chargeable_weight_measure: None,
            contained_goods_item: None,
            containing_package: None,
            customs_tariff_quantity: None,
            required_customs_id: None,
            gross_weight_measure: None,
            invoice_line: None,
            value_amount: None,
            preference_criterion_code: None,
            goods_item_container: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfChargeableQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ChargeableQuantity>,
}

impl AsMut<GoodsItemArrayOfChargeableQuantityComponent> for GoodsItemArrayOfChargeableQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfChargeableQuantityComponent> for GoodsItemArrayOfChargeableQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfChargeableQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfChargeableQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfChargeableQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ChargeableQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ChargeableQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ChargeableQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ChargeableQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfChargeableWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ChargeableWeightMeasure>,
}

impl AsMut<GoodsItemArrayOfChargeableWeightMeasureComponent> for GoodsItemArrayOfChargeableWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfChargeableWeightMeasureComponent> for GoodsItemArrayOfChargeableWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfChargeableWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfChargeableWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfChargeableWeightMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ChargeableWeightMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ChargeableWeightMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ChargeableWeightMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ChargeableWeightMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfContainedGoodsItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContainedGoodsItem>,
}

impl AsMut<GoodsItemArrayOfContainedGoodsItemComponent> for GoodsItemArrayOfContainedGoodsItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfContainedGoodsItemComponent> for GoodsItemArrayOfContainedGoodsItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfContainedGoodsItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfContainedGoodsItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContainedGoodsItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContainedGoodsItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContainedGoodsItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContainedGoodsItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfContainingPackageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContainingPackage>,
}

impl AsMut<GoodsItemArrayOfContainingPackageComponent> for GoodsItemArrayOfContainingPackageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfContainingPackageComponent> for GoodsItemArrayOfContainingPackageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfContainingPackageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfContainingPackageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContainingPackage) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContainingPackage) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContainingPackage> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContainingPackage> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfCustomsImportClassifiedIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CustomsImportClassifiedIndicator>,
}

impl AsMut<GoodsItemArrayOfCustomsImportClassifiedIndicatorComponent> for GoodsItemArrayOfCustomsImportClassifiedIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfCustomsImportClassifiedIndicatorComponent> for GoodsItemArrayOfCustomsImportClassifiedIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfCustomsImportClassifiedIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfCustomsImportClassifiedIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfCustomsImportClassifiedIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CustomsImportClassifiedIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CustomsImportClassifiedIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CustomsImportClassifiedIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CustomsImportClassifiedIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfCustomsStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CustomsStatusCode>,
}

impl AsMut<GoodsItemArrayOfCustomsStatusCodeComponent> for GoodsItemArrayOfCustomsStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfCustomsStatusCodeComponent> for GoodsItemArrayOfCustomsStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfCustomsStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfCustomsStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfCustomsStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CustomsStatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CustomsStatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CustomsStatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CustomsStatusCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfCustomsTariffQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CustomsTariffQuantity>,
}

impl AsMut<GoodsItemArrayOfCustomsTariffQuantityComponent> for GoodsItemArrayOfCustomsTariffQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfCustomsTariffQuantityComponent> for GoodsItemArrayOfCustomsTariffQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfCustomsTariffQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfCustomsTariffQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfCustomsTariffQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CustomsTariffQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CustomsTariffQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CustomsTariffQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CustomsTariffQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfDeclaredCustomsValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeclaredCustomsValueAmount>,
}

impl AsMut<GoodsItemArrayOfDeclaredCustomsValueAmountComponent> for GoodsItemArrayOfDeclaredCustomsValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfDeclaredCustomsValueAmountComponent> for GoodsItemArrayOfDeclaredCustomsValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfDeclaredCustomsValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfDeclaredCustomsValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfDeclaredCustomsValueAmountComponent {
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
pub struct GoodsItemArrayOfDeclaredForCarriageValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeclaredForCarriageValueAmount>,
}

impl AsMut<GoodsItemArrayOfDeclaredForCarriageValueAmountComponent> for GoodsItemArrayOfDeclaredForCarriageValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfDeclaredForCarriageValueAmountComponent> for GoodsItemArrayOfDeclaredForCarriageValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfDeclaredForCarriageValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfDeclaredForCarriageValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfDeclaredForCarriageValueAmountComponent {
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
pub struct GoodsItemArrayOfDeclaredStatisticsValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DeclaredStatisticsValueAmount>,
}

impl AsMut<GoodsItemArrayOfDeclaredStatisticsValueAmountComponent> for GoodsItemArrayOfDeclaredStatisticsValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfDeclaredStatisticsValueAmountComponent> for GoodsItemArrayOfDeclaredStatisticsValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfDeclaredStatisticsValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfDeclaredStatisticsValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfDeclaredStatisticsValueAmountComponent {
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
pub struct GoodsItemArrayOfDeliveryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Delivery>,
}

impl AsMut<GoodsItemArrayOfDeliveryComponent> for GoodsItemArrayOfDeliveryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfDeliveryComponent> for GoodsItemArrayOfDeliveryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfDeliveryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfDeliveryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfDeliveryComponent {
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
pub struct GoodsItemArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<GoodsItemArrayOfDescriptionComponent> for GoodsItemArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfDescriptionComponent> for GoodsItemArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfDescriptionComponent {
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
pub struct GoodsItemArrayOfDespatchComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Despatch>,
}

impl AsMut<GoodsItemArrayOfDespatchComponent> for GoodsItemArrayOfDespatchComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfDespatchComponent> for GoodsItemArrayOfDespatchComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfDespatchComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfDespatchComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfDespatchComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Despatch) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Despatch) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Despatch> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Despatch> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfFreeOnBoardValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::FreeOnBoardValueAmount>,
}

impl AsMut<GoodsItemArrayOfFreeOnBoardValueAmountComponent> for GoodsItemArrayOfFreeOnBoardValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfFreeOnBoardValueAmountComponent> for GoodsItemArrayOfFreeOnBoardValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfFreeOnBoardValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfFreeOnBoardValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfFreeOnBoardValueAmountComponent {
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
pub struct GoodsItemArrayOfFreightAllowanceChargeComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FreightAllowanceCharge>,
}

impl AsMut<GoodsItemArrayOfFreightAllowanceChargeComponent> for GoodsItemArrayOfFreightAllowanceChargeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfFreightAllowanceChargeComponent> for GoodsItemArrayOfFreightAllowanceChargeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfFreightAllowanceChargeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfFreightAllowanceChargeComponent {
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
pub struct GoodsItemArrayOfGoodsItemContainerComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::GoodsItemContainer>,
}

impl AsMut<GoodsItemArrayOfGoodsItemContainerComponent> for GoodsItemArrayOfGoodsItemContainerComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfGoodsItemContainerComponent> for GoodsItemArrayOfGoodsItemContainerComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfGoodsItemContainerComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfGoodsItemContainerComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::GoodsItemContainer) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::GoodsItemContainer) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::GoodsItemContainer> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::GoodsItemContainer> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfGrossVolumeMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GrossVolumeMeasure>,
}

impl AsMut<GoodsItemArrayOfGrossVolumeMeasureComponent> for GoodsItemArrayOfGrossVolumeMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfGrossVolumeMeasureComponent> for GoodsItemArrayOfGrossVolumeMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfGrossVolumeMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfGrossVolumeMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfGrossVolumeMeasureComponent {
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
pub struct GoodsItemArrayOfGrossWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::GrossWeightMeasure>,
}

impl AsMut<GoodsItemArrayOfGrossWeightMeasureComponent> for GoodsItemArrayOfGrossWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfGrossWeightMeasureComponent> for GoodsItemArrayOfGrossWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfGrossWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfGrossWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfGrossWeightMeasureComponent {
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
pub struct GoodsItemArrayOfHazardousRiskIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator>,
}

impl AsMut<GoodsItemArrayOfHazardousRiskIndicatorComponent> for GoodsItemArrayOfHazardousRiskIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfHazardousRiskIndicatorComponent> for GoodsItemArrayOfHazardousRiskIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfHazardousRiskIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfHazardousRiskIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfHazardousRiskIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<GoodsItemArrayOfIDComponent> for GoodsItemArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfIDComponent> for GoodsItemArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfIDComponent {
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
pub struct GoodsItemArrayOfInsuranceValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InsuranceValueAmount>,
}

impl AsMut<GoodsItemArrayOfInsuranceValueAmountComponent> for GoodsItemArrayOfInsuranceValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfInsuranceValueAmountComponent> for GoodsItemArrayOfInsuranceValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfInsuranceValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfInsuranceValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfInsuranceValueAmountComponent {
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
pub struct GoodsItemArrayOfInvoiceLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::InvoiceLine>,
}

impl AsMut<GoodsItemArrayOfInvoiceLineComponent> for GoodsItemArrayOfInvoiceLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfInvoiceLineComponent> for GoodsItemArrayOfInvoiceLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfInvoiceLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfInvoiceLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::InvoiceLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::InvoiceLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::InvoiceLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::InvoiceLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<GoodsItemArrayOfItemComponent> for GoodsItemArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfItemComponent> for GoodsItemArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Item) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Item) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Item> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfMaximumTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature>,
}

impl AsMut<GoodsItemArrayOfMaximumTemperatureComponent> for GoodsItemArrayOfMaximumTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfMaximumTemperatureComponent> for GoodsItemArrayOfMaximumTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfMaximumTemperatureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfMaximumTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfMaximumTemperatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfMeasurementDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension>,
}

impl AsMut<GoodsItemArrayOfMeasurementDimensionComponent> for GoodsItemArrayOfMeasurementDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfMeasurementDimensionComponent> for GoodsItemArrayOfMeasurementDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfMeasurementDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfMeasurementDimensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfMinimumTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature>,
}

impl AsMut<GoodsItemArrayOfMinimumTemperatureComponent> for GoodsItemArrayOfMinimumTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfMinimumTemperatureComponent> for GoodsItemArrayOfMinimumTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfMinimumTemperatureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfMinimumTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfMinimumTemperatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfNetNetWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetNetWeightMeasure>,
}

impl AsMut<GoodsItemArrayOfNetNetWeightMeasureComponent> for GoodsItemArrayOfNetNetWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfNetNetWeightMeasureComponent> for GoodsItemArrayOfNetNetWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfNetNetWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfNetNetWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfNetNetWeightMeasureComponent {
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
pub struct GoodsItemArrayOfNetVolumeMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure>,
}

impl AsMut<GoodsItemArrayOfNetVolumeMeasureComponent> for GoodsItemArrayOfNetVolumeMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfNetVolumeMeasureComponent> for GoodsItemArrayOfNetVolumeMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfNetVolumeMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfNetVolumeMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfNetVolumeMeasureComponent {
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
pub struct GoodsItemArrayOfNetWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure>,
}

impl AsMut<GoodsItemArrayOfNetWeightMeasureComponent> for GoodsItemArrayOfNetWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfNetWeightMeasureComponent> for GoodsItemArrayOfNetWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfNetWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfNetWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfNetWeightMeasureComponent {
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
pub struct GoodsItemArrayOfOriginAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginAddress>,
}

impl AsMut<GoodsItemArrayOfOriginAddressComponent> for GoodsItemArrayOfOriginAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfOriginAddressComponent> for GoodsItemArrayOfOriginAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfOriginAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfOriginAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfOriginAddressComponent {
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
pub struct GoodsItemArrayOfPickupComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Pickup>,
}

impl AsMut<GoodsItemArrayOfPickupComponent> for GoodsItemArrayOfPickupComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfPickupComponent> for GoodsItemArrayOfPickupComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfPickupComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfPickupComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfPickupComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Pickup) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Pickup) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Pickup> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Pickup> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfPreferenceCriterionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PreferenceCriterionCode>,
}

impl AsMut<GoodsItemArrayOfPreferenceCriterionCodeComponent> for GoodsItemArrayOfPreferenceCriterionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfPreferenceCriterionCodeComponent> for GoodsItemArrayOfPreferenceCriterionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfPreferenceCriterionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfPreferenceCriterionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfPreferenceCriterionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PreferenceCriterionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PreferenceCriterionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PreferenceCriterionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PreferenceCriterionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<GoodsItemArrayOfQuantityComponent> for GoodsItemArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfQuantityComponent> for GoodsItemArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfQuantityComponent {
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
pub struct GoodsItemArrayOfRequiredCustomsIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::RequiredCustomsID>,
}

impl AsMut<GoodsItemArrayOfRequiredCustomsIDComponent> for GoodsItemArrayOfRequiredCustomsIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfRequiredCustomsIDComponent> for GoodsItemArrayOfRequiredCustomsIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfRequiredCustomsIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfRequiredCustomsIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfRequiredCustomsIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::RequiredCustomsID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::RequiredCustomsID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::RequiredCustomsID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::RequiredCustomsID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfReturnableQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReturnableQuantity>,
}

impl AsMut<GoodsItemArrayOfReturnableQuantityComponent> for GoodsItemArrayOfReturnableQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfReturnableQuantityComponent> for GoodsItemArrayOfReturnableQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfReturnableQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfReturnableQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfReturnableQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReturnableQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReturnableQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReturnableQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReturnableQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfSequenceNumberIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SequenceNumberID>,
}

impl AsMut<GoodsItemArrayOfSequenceNumberIDComponent> for GoodsItemArrayOfSequenceNumberIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfSequenceNumberIDComponent> for GoodsItemArrayOfSequenceNumberIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfSequenceNumberIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfSequenceNumberIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfSequenceNumberIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SequenceNumberID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SequenceNumberID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SequenceNumberID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SequenceNumberID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfShipmentDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ShipmentDocumentReference>,
}

impl AsMut<GoodsItemArrayOfShipmentDocumentReferenceComponent> for GoodsItemArrayOfShipmentDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfShipmentDocumentReferenceComponent> for GoodsItemArrayOfShipmentDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfShipmentDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfShipmentDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfShipmentDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ShipmentDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ShipmentDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ShipmentDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ShipmentDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Temperature>,
}

impl AsMut<GoodsItemArrayOfTemperatureComponent> for GoodsItemArrayOfTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfTemperatureComponent> for GoodsItemArrayOfTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfTemperatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Temperature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Temperature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Temperature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Temperature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfTraceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TraceID>,
}

impl AsMut<GoodsItemArrayOfTraceIDComponent> for GoodsItemArrayOfTraceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfTraceIDComponent> for GoodsItemArrayOfTraceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfTraceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfTraceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfTraceIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TraceID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TraceID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TraceID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TraceID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GoodsItemArrayOfValueAmountComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ValueAmount>,
}

impl AsMut<GoodsItemArrayOfValueAmountComponent> for GoodsItemArrayOfValueAmountComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<GoodsItemArrayOfValueAmountComponent> for GoodsItemArrayOfValueAmountComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfValueAmountComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("GoodsItemArrayOfValueAmountComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl GoodsItemArrayOfValueAmountComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ValueAmount) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ValueAmount) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ValueAmount> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ValueAmount> {
        self.items.iter()
    }
}


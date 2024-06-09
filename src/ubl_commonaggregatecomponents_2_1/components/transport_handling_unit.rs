use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransportHandlingUnit {
    #[serde(rename = "ActualPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_package: Option<TransportHandlingUnitArrayOfActualPackageComponent>,
    #[serde(rename = "CustomsDeclaration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customs_declaration: Option<TransportHandlingUnitArrayOfCustomsDeclarationComponent>,
    #[serde(rename = "DamageRemarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_remarks: Option<TransportHandlingUnitArrayOfDamageRemarksComponent>,
    #[serde(rename = "FloorSpaceMeasurementDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_space_measurement_dimension: Option<TransportHandlingUnitArrayOfFloorSpaceMeasurementDimensionComponent>,
    #[serde(rename = "GoodsItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_item: Option<TransportHandlingUnitArrayOfGoodsItemComponent>,
    #[serde(rename = "HandlingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handling_code: Option<TransportHandlingUnitArrayOfHandlingCodeComponent>,
    #[serde(rename = "HandlingInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handling_instructions: Option<TransportHandlingUnitArrayOfHandlingInstructionsComponent>,
    #[serde(rename = "HandlingUnitDespatchLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handling_unit_despatch_line: Option<TransportHandlingUnitArrayOfHandlingUnitDespatchLineComponent>,
    #[serde(rename = "HazardousGoodsTransit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_goods_transit: Option<TransportHandlingUnitArrayOfHazardousGoodsTransitComponent>,
    #[serde(rename = "HazardousRiskIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_risk_indicator: Option<TransportHandlingUnitArrayOfHazardousRiskIndicatorComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TransportHandlingUnitArrayOfIDComponent>,
    #[serde(rename = "MaximumTemperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_temperature: Option<TransportHandlingUnitArrayOfMaximumTemperatureComponent>,
    #[serde(rename = "MeasurementDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_dimension: Option<TransportHandlingUnitArrayOfMeasurementDimensionComponent>,
    #[serde(rename = "MinimumTemperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_temperature: Option<TransportHandlingUnitArrayOfMinimumTemperatureComponent>,
    #[serde(rename = "Package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<TransportHandlingUnitArrayOfPackageComponent>,
    #[serde(rename = "PalletSpaceMeasurementDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pallet_space_measurement_dimension: Option<TransportHandlingUnitArrayOfPalletSpaceMeasurementDimensionComponent>,
    #[serde(rename = "ReceivedHandlingUnitReceiptLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_handling_unit_receipt_line: Option<TransportHandlingUnitArrayOfReceivedHandlingUnitReceiptLineComponent>,
    #[serde(rename = "ReferencedShipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_shipment: Option<TransportHandlingUnitArrayOfReferencedShipmentComponent>,
    #[serde(rename = "ShipmentDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_document_reference: Option<TransportHandlingUnitArrayOfShipmentDocumentReferenceComponent>,
    #[serde(rename = "ShippingMarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_marks: Option<TransportHandlingUnitArrayOfShippingMarksComponent>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransportHandlingUnitArrayOfStatusComponent>,
    #[serde(rename = "TotalGoodsItemQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_goods_item_quantity: Option<TransportHandlingUnitArrayOfTotalGoodsItemQuantityComponent>,
    #[serde(rename = "TotalPackageQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_package_quantity: Option<TransportHandlingUnitArrayOfTotalPackageQuantityComponent>,
    #[serde(rename = "TraceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<TransportHandlingUnitArrayOfTraceIDComponent>,
    #[serde(rename = "TransportEquipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_equipment: Option<TransportHandlingUnitArrayOfTransportEquipmentComponent>,
    #[serde(rename = "TransportHandlingUnitTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_handling_unit_type_code: Option<TransportHandlingUnitArrayOfTransportHandlingUnitTypeCodeComponent>,
    #[serde(rename = "TransportMeans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_means: Option<TransportHandlingUnitArrayOfTransportMeansComponent>,
}

impl AsMut<TransportHandlingUnit> for TransportHandlingUnit {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnit> for TransportHandlingUnit {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.maximum_temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.maximum_temperature", e));
            }
        }
        if let Some(v) = &self.goods_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.goods_item", e));
            }
        }
        if let Some(v) = &self.shipment_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.shipment_document_reference", e));
            }
        }
        if let Some(v) = &self.damage_remarks {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.damage_remarks", e));
            }
        }
        if let Some(v) = &self.transport_means {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.transport_means", e));
            }
        }
        if let Some(v) = &self.transport_handling_unit_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.transport_handling_unit_type_code", e));
            }
        }
        if let Some(v) = &self.pallet_space_measurement_dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.pallet_space_measurement_dimension", e));
            }
        }
        if let Some(v) = &self.shipping_marks {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.shipping_marks", e));
            }
        }
        if let Some(v) = &self.customs_declaration {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.customs_declaration", e));
            }
        }
        if let Some(v) = &self.handling_instructions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.handling_instructions", e));
            }
        }
        if let Some(v) = &self.measurement_dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.measurement_dimension", e));
            }
        }
        if let Some(v) = &self.handling_unit_despatch_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.handling_unit_despatch_line", e));
            }
        }
        if let Some(v) = &self.actual_package {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.actual_package", e));
            }
        }
        if let Some(v) = &self.package {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.package", e));
            }
        }
        if let Some(v) = &self.hazardous_goods_transit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.hazardous_goods_transit", e));
            }
        }
        if let Some(v) = &self.total_package_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.total_package_quantity", e));
            }
        }
        if let Some(v) = &self.minimum_temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.minimum_temperature", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.id", e));
            }
        }
        if let Some(v) = &self.total_goods_item_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.total_goods_item_quantity", e));
            }
        }
        if let Some(v) = &self.trace_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.trace_id", e));
            }
        }
        if let Some(v) = &self.received_handling_unit_receipt_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.received_handling_unit_receipt_line", e));
            }
        }
        if let Some(v) = &self.hazardous_risk_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.hazardous_risk_indicator", e));
            }
        }
        if let Some(v) = &self.transport_equipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.transport_equipment", e));
            }
        }
        if let Some(v) = &self.status {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.status", e));
            }
        }
        if let Some(v) = &self.floor_space_measurement_dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.floor_space_measurement_dimension", e));
            }
        }
        if let Some(v) = &self.handling_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.handling_code", e));
            }
        }
        if let Some(v) = &self.referenced_shipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("TransportHandlingUnit.referenced_shipment", e));
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

impl TransportHandlingUnit {
    pub fn title() -> &'static str {
        "Transport Handling Unit. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a uniquely identifiable unit consisting of one or more packages, goods items, or pieces of transport equipment."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            referenced_shipment: None,
            total_goods_item_quantity: None,
            actual_package: None,
            shipment_document_reference: None,
            customs_declaration: None,
            trace_id: None,
            minimum_temperature: None,
            handling_code: None,
            hazardous_goods_transit: None,
            id: None,
            status: None,
            total_package_quantity: None,
            measurement_dimension: None,
            transport_handling_unit_type_code: None,
            pallet_space_measurement_dimension: None,
            transport_equipment: None,
            transport_means: None,
            hazardous_risk_indicator: None,
            shipping_marks: None,
            floor_space_measurement_dimension: None,
            goods_item: None,
            maximum_temperature: None,
            received_handling_unit_receipt_line: None,
            handling_instructions: None,
            damage_remarks: None,
            handling_unit_despatch_line: None,
            package: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfActualPackageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ActualPackage>,
}

impl AsMut<TransportHandlingUnitArrayOfActualPackageComponent> for TransportHandlingUnitArrayOfActualPackageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfActualPackageComponent> for TransportHandlingUnitArrayOfActualPackageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfActualPackageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfActualPackageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ActualPackage) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ActualPackage) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ActualPackage> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ActualPackage> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfCustomsDeclarationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CustomsDeclaration>,
}

impl AsMut<TransportHandlingUnitArrayOfCustomsDeclarationComponent> for TransportHandlingUnitArrayOfCustomsDeclarationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfCustomsDeclarationComponent> for TransportHandlingUnitArrayOfCustomsDeclarationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfCustomsDeclarationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfCustomsDeclarationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CustomsDeclaration) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CustomsDeclaration) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CustomsDeclaration> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CustomsDeclaration> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfDamageRemarksComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::DamageRemarks>,
}

impl AsMut<TransportHandlingUnitArrayOfDamageRemarksComponent> for TransportHandlingUnitArrayOfDamageRemarksComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfDamageRemarksComponent> for TransportHandlingUnitArrayOfDamageRemarksComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfDamageRemarksComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfDamageRemarksComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::DamageRemarks) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::DamageRemarks) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::DamageRemarks> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::DamageRemarks> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfFloorSpaceMeasurementDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FloorSpaceMeasurementDimension>,
}

impl AsMut<TransportHandlingUnitArrayOfFloorSpaceMeasurementDimensionComponent> for TransportHandlingUnitArrayOfFloorSpaceMeasurementDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfFloorSpaceMeasurementDimensionComponent> for TransportHandlingUnitArrayOfFloorSpaceMeasurementDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfFloorSpaceMeasurementDimensionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfFloorSpaceMeasurementDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfFloorSpaceMeasurementDimensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FloorSpaceMeasurementDimension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FloorSpaceMeasurementDimension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FloorSpaceMeasurementDimension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FloorSpaceMeasurementDimension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfGoodsItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::GoodsItem>,
}

impl AsMut<TransportHandlingUnitArrayOfGoodsItemComponent> for TransportHandlingUnitArrayOfGoodsItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfGoodsItemComponent> for TransportHandlingUnitArrayOfGoodsItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfGoodsItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfGoodsItemComponent {
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
pub struct TransportHandlingUnitArrayOfHandlingCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HandlingCode>,
}

impl AsMut<TransportHandlingUnitArrayOfHandlingCodeComponent> for TransportHandlingUnitArrayOfHandlingCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfHandlingCodeComponent> for TransportHandlingUnitArrayOfHandlingCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfHandlingCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfHandlingCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfHandlingCodeComponent {
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
pub struct TransportHandlingUnitArrayOfHandlingInstructionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HandlingInstructions>,
}

impl AsMut<TransportHandlingUnitArrayOfHandlingInstructionsComponent> for TransportHandlingUnitArrayOfHandlingInstructionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfHandlingInstructionsComponent> for TransportHandlingUnitArrayOfHandlingInstructionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfHandlingInstructionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfHandlingInstructionsComponent {
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
pub struct TransportHandlingUnitArrayOfHandlingUnitDespatchLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::HandlingUnitDespatchLine>,
}

impl AsMut<TransportHandlingUnitArrayOfHandlingUnitDespatchLineComponent> for TransportHandlingUnitArrayOfHandlingUnitDespatchLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfHandlingUnitDespatchLineComponent> for TransportHandlingUnitArrayOfHandlingUnitDespatchLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfHandlingUnitDespatchLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfHandlingUnitDespatchLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::HandlingUnitDespatchLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::HandlingUnitDespatchLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::HandlingUnitDespatchLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::HandlingUnitDespatchLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfHazardousGoodsTransitComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit>,
}

impl AsMut<TransportHandlingUnitArrayOfHazardousGoodsTransitComponent> for TransportHandlingUnitArrayOfHazardousGoodsTransitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfHazardousGoodsTransitComponent> for TransportHandlingUnitArrayOfHazardousGoodsTransitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfHazardousGoodsTransitComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfHazardousGoodsTransitComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfHazardousRiskIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator>,
}

impl AsMut<TransportHandlingUnitArrayOfHazardousRiskIndicatorComponent> for TransportHandlingUnitArrayOfHazardousRiskIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfHazardousRiskIndicatorComponent> for TransportHandlingUnitArrayOfHazardousRiskIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfHazardousRiskIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfHazardousRiskIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfHazardousRiskIndicatorComponent {
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
pub struct TransportHandlingUnitArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<TransportHandlingUnitArrayOfIDComponent> for TransportHandlingUnitArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfIDComponent> for TransportHandlingUnitArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfIDComponent {
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
pub struct TransportHandlingUnitArrayOfMaximumTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MaximumTemperature>,
}

impl AsMut<TransportHandlingUnitArrayOfMaximumTemperatureComponent> for TransportHandlingUnitArrayOfMaximumTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfMaximumTemperatureComponent> for TransportHandlingUnitArrayOfMaximumTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfMaximumTemperatureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfMaximumTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfMaximumTemperatureComponent {
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
pub struct TransportHandlingUnitArrayOfMeasurementDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension>,
}

impl AsMut<TransportHandlingUnitArrayOfMeasurementDimensionComponent> for TransportHandlingUnitArrayOfMeasurementDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfMeasurementDimensionComponent> for TransportHandlingUnitArrayOfMeasurementDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfMeasurementDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfMeasurementDimensionComponent {
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
pub struct TransportHandlingUnitArrayOfMinimumTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MinimumTemperature>,
}

impl AsMut<TransportHandlingUnitArrayOfMinimumTemperatureComponent> for TransportHandlingUnitArrayOfMinimumTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfMinimumTemperatureComponent> for TransportHandlingUnitArrayOfMinimumTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfMinimumTemperatureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfMinimumTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfMinimumTemperatureComponent {
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
pub struct TransportHandlingUnitArrayOfPackageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Package>,
}

impl AsMut<TransportHandlingUnitArrayOfPackageComponent> for TransportHandlingUnitArrayOfPackageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfPackageComponent> for TransportHandlingUnitArrayOfPackageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfPackageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfPackageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Package) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Package) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Package> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Package> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfPalletSpaceMeasurementDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::PalletSpaceMeasurementDimension>,
}

impl AsMut<TransportHandlingUnitArrayOfPalletSpaceMeasurementDimensionComponent> for TransportHandlingUnitArrayOfPalletSpaceMeasurementDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfPalletSpaceMeasurementDimensionComponent> for TransportHandlingUnitArrayOfPalletSpaceMeasurementDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfPalletSpaceMeasurementDimensionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfPalletSpaceMeasurementDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfPalletSpaceMeasurementDimensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::PalletSpaceMeasurementDimension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::PalletSpaceMeasurementDimension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::PalletSpaceMeasurementDimension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::PalletSpaceMeasurementDimension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfReceivedHandlingUnitReceiptLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReceivedHandlingUnitReceiptLine>,
}

impl AsMut<TransportHandlingUnitArrayOfReceivedHandlingUnitReceiptLineComponent> for TransportHandlingUnitArrayOfReceivedHandlingUnitReceiptLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfReceivedHandlingUnitReceiptLineComponent> for TransportHandlingUnitArrayOfReceivedHandlingUnitReceiptLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfReceivedHandlingUnitReceiptLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfReceivedHandlingUnitReceiptLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReceivedHandlingUnitReceiptLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReceivedHandlingUnitReceiptLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReceivedHandlingUnitReceiptLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReceivedHandlingUnitReceiptLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfReferencedShipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReferencedShipment>,
}

impl AsMut<TransportHandlingUnitArrayOfReferencedShipmentComponent> for TransportHandlingUnitArrayOfReferencedShipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfReferencedShipmentComponent> for TransportHandlingUnitArrayOfReferencedShipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfReferencedShipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfReferencedShipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReferencedShipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReferencedShipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReferencedShipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReferencedShipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfShipmentDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ShipmentDocumentReference>,
}

impl AsMut<TransportHandlingUnitArrayOfShipmentDocumentReferenceComponent> for TransportHandlingUnitArrayOfShipmentDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfShipmentDocumentReferenceComponent> for TransportHandlingUnitArrayOfShipmentDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfShipmentDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfShipmentDocumentReferenceComponent {
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
pub struct TransportHandlingUnitArrayOfShippingMarksComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ShippingMarks>,
}

impl AsMut<TransportHandlingUnitArrayOfShippingMarksComponent> for TransportHandlingUnitArrayOfShippingMarksComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfShippingMarksComponent> for TransportHandlingUnitArrayOfShippingMarksComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfShippingMarksComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfShippingMarksComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ShippingMarks) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ShippingMarks) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ShippingMarks> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ShippingMarks> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfStatusComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Status>,
}

impl AsMut<TransportHandlingUnitArrayOfStatusComponent> for TransportHandlingUnitArrayOfStatusComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfStatusComponent> for TransportHandlingUnitArrayOfStatusComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfStatusComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfStatusComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Status) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Status) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Status> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Status> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfTotalGoodsItemQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalGoodsItemQuantity>,
}

impl AsMut<TransportHandlingUnitArrayOfTotalGoodsItemQuantityComponent> for TransportHandlingUnitArrayOfTotalGoodsItemQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfTotalGoodsItemQuantityComponent> for TransportHandlingUnitArrayOfTotalGoodsItemQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfTotalGoodsItemQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfTotalGoodsItemQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfTotalGoodsItemQuantityComponent {
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
pub struct TransportHandlingUnitArrayOfTotalPackageQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalPackageQuantity>,
}

impl AsMut<TransportHandlingUnitArrayOfTotalPackageQuantityComponent> for TransportHandlingUnitArrayOfTotalPackageQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfTotalPackageQuantityComponent> for TransportHandlingUnitArrayOfTotalPackageQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfTotalPackageQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfTotalPackageQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfTotalPackageQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalPackageQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalPackageQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalPackageQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalPackageQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfTraceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TraceID>,
}

impl AsMut<TransportHandlingUnitArrayOfTraceIDComponent> for TransportHandlingUnitArrayOfTraceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfTraceIDComponent> for TransportHandlingUnitArrayOfTraceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfTraceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfTraceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfTraceIDComponent {
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
pub struct TransportHandlingUnitArrayOfTransportEquipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportEquipment>,
}

impl AsMut<TransportHandlingUnitArrayOfTransportEquipmentComponent> for TransportHandlingUnitArrayOfTransportEquipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfTransportEquipmentComponent> for TransportHandlingUnitArrayOfTransportEquipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfTransportEquipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfTransportEquipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportEquipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportEquipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportEquipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportEquipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfTransportHandlingUnitTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TransportHandlingUnitTypeCode>,
}

impl AsMut<TransportHandlingUnitArrayOfTransportHandlingUnitTypeCodeComponent> for TransportHandlingUnitArrayOfTransportHandlingUnitTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfTransportHandlingUnitTypeCodeComponent> for TransportHandlingUnitArrayOfTransportHandlingUnitTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfTransportHandlingUnitTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfTransportHandlingUnitTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfTransportHandlingUnitTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TransportHandlingUnitTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TransportHandlingUnitTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TransportHandlingUnitTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TransportHandlingUnitTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransportHandlingUnitArrayOfTransportMeansComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransportMeans>,
}

impl AsMut<TransportHandlingUnitArrayOfTransportMeansComponent> for TransportHandlingUnitArrayOfTransportMeansComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<TransportHandlingUnitArrayOfTransportMeansComponent> for TransportHandlingUnitArrayOfTransportMeansComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("TransportHandlingUnitArrayOfTransportMeansComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl TransportHandlingUnitArrayOfTransportMeansComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransportMeans) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransportMeans) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransportMeans> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransportMeans> {
        self.items.iter()
    }
}


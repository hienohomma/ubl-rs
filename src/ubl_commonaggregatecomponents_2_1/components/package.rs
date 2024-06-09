use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Package {
    #[serde(rename = "ContainedPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contained_package: Option<PackageArrayOfContainedPackageComponent>,
    #[serde(rename = "ContainingTransportEquipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containing_transport_equipment: Option<PackageArrayOfContainingTransportEquipmentComponent>,
    #[serde(rename = "Delivery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<PackageArrayOfDeliveryComponent>,
    #[serde(rename = "DeliveryUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_unit: Option<PackageArrayOfDeliveryUnitComponent>,
    #[serde(rename = "Despatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch: Option<PackageArrayOfDespatchComponent>,
    #[serde(rename = "GoodsItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_item: Option<PackageArrayOfGoodsItemComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PackageArrayOfIDComponent>,
    #[serde(rename = "MeasurementDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_dimension: Option<PackageArrayOfMeasurementDimensionComponent>,
    #[serde(rename = "PackageLevelCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_level_code: Option<PackageArrayOfPackageLevelCodeComponent>,
    #[serde(rename = "PackagingTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_type_code: Option<PackageArrayOfPackagingTypeCodeComponent>,
    #[serde(rename = "PackingMaterial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packing_material: Option<PackageArrayOfPackingMaterialComponent>,
    #[serde(rename = "Pickup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup: Option<PackageArrayOfPickupComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<PackageArrayOfQuantityComponent>,
    #[serde(rename = "ReturnableMaterialIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnable_material_indicator: Option<PackageArrayOfReturnableMaterialIndicatorComponent>,
    #[serde(rename = "TraceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<PackageArrayOfTraceIDComponent>,
}

impl AsMut<Package> for Package {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Package> for Package {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.goods_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.goods_item", e));
            }
        }
        if let Some(v) = &self.packing_material {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.packing_material", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.id", e));
            }
        }
        if let Some(v) = &self.returnable_material_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.returnable_material_indicator", e));
            }
        }
        if let Some(v) = &self.despatch {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.despatch", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.quantity", e));
            }
        }
        if let Some(v) = &self.pickup {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.pickup", e));
            }
        }
        if let Some(v) = &self.packaging_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.packaging_type_code", e));
            }
        }
        if let Some(v) = &self.containing_transport_equipment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.containing_transport_equipment", e));
            }
        }
        if let Some(v) = &self.package_level_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.package_level_code", e));
            }
        }
        if let Some(v) = &self.trace_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.trace_id", e));
            }
        }
        if let Some(v) = &self.delivery {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.delivery", e));
            }
        }
        if let Some(v) = &self.measurement_dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.measurement_dimension", e));
            }
        }
        if let Some(v) = &self.contained_package {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.contained_package", e));
            }
        }
        if let Some(v) = &self.delivery_unit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Package.delivery_unit", e));
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

impl Package {
    pub fn title() -> &'static str {
        "Package. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a package."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            despatch: None,
            containing_transport_equipment: None,
            package_level_code: None,
            delivery_unit: None,
            measurement_dimension: None,
            contained_package: None,
            packing_material: None,
            id: None,
            pickup: None,
            trace_id: None,
            quantity: None,
            returnable_material_indicator: None,
            goods_item: None,
            delivery: None,
            packaging_type_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PackageArrayOfContainedPackageComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContainedPackage>,
}

impl AsMut<PackageArrayOfContainedPackageComponent> for PackageArrayOfContainedPackageComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfContainedPackageComponent> for PackageArrayOfContainedPackageComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfContainedPackageComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfContainedPackageComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContainedPackage) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContainedPackage) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContainedPackage> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContainedPackage> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PackageArrayOfContainingTransportEquipmentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContainingTransportEquipment>,
}

impl AsMut<PackageArrayOfContainingTransportEquipmentComponent> for PackageArrayOfContainingTransportEquipmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfContainingTransportEquipmentComponent> for PackageArrayOfContainingTransportEquipmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PackageArrayOfContainingTransportEquipmentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfContainingTransportEquipmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfContainingTransportEquipmentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContainingTransportEquipment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContainingTransportEquipment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContainingTransportEquipment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContainingTransportEquipment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PackageArrayOfDeliveryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Delivery>,
}

impl AsMut<PackageArrayOfDeliveryComponent> for PackageArrayOfDeliveryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfDeliveryComponent> for PackageArrayOfDeliveryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PackageArrayOfDeliveryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfDeliveryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfDeliveryComponent {
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
pub struct PackageArrayOfDeliveryUnitComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DeliveryUnit>,
}

impl AsMut<PackageArrayOfDeliveryUnitComponent> for PackageArrayOfDeliveryUnitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfDeliveryUnitComponent> for PackageArrayOfDeliveryUnitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfDeliveryUnitComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfDeliveryUnitComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DeliveryUnit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DeliveryUnit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DeliveryUnit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DeliveryUnit> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PackageArrayOfDespatchComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Despatch>,
}

impl AsMut<PackageArrayOfDespatchComponent> for PackageArrayOfDespatchComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfDespatchComponent> for PackageArrayOfDespatchComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PackageArrayOfDespatchComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfDespatchComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfDespatchComponent {
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
pub struct PackageArrayOfGoodsItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::GoodsItem>,
}

impl AsMut<PackageArrayOfGoodsItemComponent> for PackageArrayOfGoodsItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfGoodsItemComponent> for PackageArrayOfGoodsItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfGoodsItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfGoodsItemComponent {
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
pub struct PackageArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<PackageArrayOfIDComponent> for PackageArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfIDComponent> for PackageArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PackageArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfIDComponent {
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
pub struct PackageArrayOfMeasurementDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::MeasurementDimension>,
}

impl AsMut<PackageArrayOfMeasurementDimensionComponent> for PackageArrayOfMeasurementDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfMeasurementDimensionComponent> for PackageArrayOfMeasurementDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfMeasurementDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfMeasurementDimensionComponent {
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
pub struct PackageArrayOfPackageLevelCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PackageLevelCode>,
}

impl AsMut<PackageArrayOfPackageLevelCodeComponent> for PackageArrayOfPackageLevelCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfPackageLevelCodeComponent> for PackageArrayOfPackageLevelCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PackageArrayOfPackageLevelCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfPackageLevelCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfPackageLevelCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PackageLevelCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PackageLevelCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PackageLevelCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PackageLevelCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PackageArrayOfPackagingTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PackagingTypeCode>,
}

impl AsMut<PackageArrayOfPackagingTypeCodeComponent> for PackageArrayOfPackagingTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfPackagingTypeCodeComponent> for PackageArrayOfPackagingTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PackageArrayOfPackagingTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfPackagingTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfPackagingTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PackagingTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PackagingTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PackagingTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PackagingTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PackageArrayOfPackingMaterialComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PackingMaterial>,
}

impl AsMut<PackageArrayOfPackingMaterialComponent> for PackageArrayOfPackingMaterialComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfPackingMaterialComponent> for PackageArrayOfPackingMaterialComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfPackingMaterialComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfPackingMaterialComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PackingMaterial) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PackingMaterial) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PackingMaterial> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PackingMaterial> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PackageArrayOfPickupComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Pickup>,
}

impl AsMut<PackageArrayOfPickupComponent> for PackageArrayOfPickupComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfPickupComponent> for PackageArrayOfPickupComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PackageArrayOfPickupComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfPickupComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfPickupComponent {
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
pub struct PackageArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<PackageArrayOfQuantityComponent> for PackageArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfQuantityComponent> for PackageArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PackageArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfQuantityComponent {
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
pub struct PackageArrayOfReturnableMaterialIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ReturnableMaterialIndicator>,
}

impl AsMut<PackageArrayOfReturnableMaterialIndicatorComponent> for PackageArrayOfReturnableMaterialIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfReturnableMaterialIndicatorComponent> for PackageArrayOfReturnableMaterialIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PackageArrayOfReturnableMaterialIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfReturnableMaterialIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfReturnableMaterialIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ReturnableMaterialIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ReturnableMaterialIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ReturnableMaterialIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ReturnableMaterialIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PackageArrayOfTraceIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TraceID>,
}

impl AsMut<PackageArrayOfTraceIDComponent> for PackageArrayOfTraceIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<PackageArrayOfTraceIDComponent> for PackageArrayOfTraceIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("PackageArrayOfTraceIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("PackageArrayOfTraceIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl PackageArrayOfTraceIDComponent {
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


use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "AdditionalInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<ItemArrayOfAdditionalInformationComponent>,
    #[serde(rename = "AdditionalItemIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_item_identification: Option<ItemArrayOfAdditionalItemIdentificationComponent>,
    #[serde(rename = "AdditionalItemProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_item_property: Option<ItemArrayOfAdditionalItemPropertyComponent>,
    #[serde(rename = "BrandName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<ItemArrayOfBrandNameComponent>,
    #[serde(rename = "BuyersItemIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyers_item_identification: Option<ItemArrayOfBuyersItemIdentificationComponent>,
    #[serde(rename = "CatalogueDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogue_document_reference: Option<ItemArrayOfCatalogueDocumentReferenceComponent>,
    #[serde(rename = "CatalogueIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogue_indicator: Option<ItemArrayOfCatalogueIndicatorComponent>,
    #[serde(rename = "CatalogueItemIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogue_item_identification: Option<ItemArrayOfCatalogueItemIdentificationComponent>,
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<ItemArrayOfCertificateComponent>,
    #[serde(rename = "ClassifiedTaxCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classified_tax_category: Option<ItemArrayOfClassifiedTaxCategoryComponent>,
    #[serde(rename = "CommodityClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity_classification: Option<ItemArrayOfCommodityClassificationComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ItemArrayOfDescriptionComponent>,
    #[serde(rename = "Dimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<ItemArrayOfDimensionComponent>,
    #[serde(rename = "HazardousItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_item: Option<ItemArrayOfHazardousItemComponent>,
    #[serde(rename = "HazardousRiskIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_risk_indicator: Option<ItemArrayOfHazardousRiskIndicatorComponent>,
    #[serde(rename = "InformationContentProviderParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information_content_provider_party: Option<ItemArrayOfInformationContentProviderPartyComponent>,
    #[serde(rename = "ItemInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_instance: Option<ItemArrayOfItemInstanceComponent>,
    #[serde(rename = "ItemSpecificationDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_specification_document_reference: Option<ItemArrayOfItemSpecificationDocumentReferenceComponent>,
    #[serde(rename = "Keyword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<ItemArrayOfKeywordComponent>,
    #[serde(rename = "ManufacturerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_party: Option<ItemArrayOfManufacturerPartyComponent>,
    #[serde(rename = "ManufacturersItemIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturers_item_identification: Option<ItemArrayOfManufacturersItemIdentificationComponent>,
    #[serde(rename = "ModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<ItemArrayOfModelNameComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ItemArrayOfNameComponent>,
    #[serde(rename = "OriginAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_address: Option<ItemArrayOfOriginAddressComponent>,
    #[serde(rename = "OriginCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_country: Option<ItemArrayOfOriginCountryComponent>,
    #[serde(rename = "PackQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_quantity: Option<ItemArrayOfPackQuantityComponent>,
    #[serde(rename = "PackSizeNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_size_numeric: Option<ItemArrayOfPackSizeNumericComponent>,
    #[serde(rename = "SellersItemIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sellers_item_identification: Option<ItemArrayOfSellersItemIdentificationComponent>,
    #[serde(rename = "StandardItemIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_item_identification: Option<ItemArrayOfStandardItemIdentificationComponent>,
    #[serde(rename = "TransactionConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_conditions: Option<ItemArrayOfTransactionConditionsComponent>,
}

impl AsMut<Item> for Item {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Item> for Item {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.hazardous_risk_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.hazardous_risk_indicator", e));
            }
        }
        if let Some(v) = &self.standard_item_identification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.standard_item_identification", e));
            }
        }
        if let Some(v) = &self.pack_size_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.pack_size_numeric", e));
            }
        }
        if let Some(v) = &self.manufacturers_item_identification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.manufacturers_item_identification", e));
            }
        }
        if let Some(v) = &self.information_content_provider_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.information_content_provider_party", e));
            }
        }
        if let Some(v) = &self.certificate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.certificate", e));
            }
        }
        if let Some(v) = &self.item_specification_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.item_specification_document_reference", e));
            }
        }
        if let Some(v) = &self.origin_country {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.origin_country", e));
            }
        }
        if let Some(v) = &self.commodity_classification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.commodity_classification", e));
            }
        }
        if let Some(v) = &self.buyers_item_identification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.buyers_item_identification", e));
            }
        }
        if let Some(v) = &self.keyword {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.keyword", e));
            }
        }
        if let Some(v) = &self.model_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.model_name", e));
            }
        }
        if let Some(v) = &self.manufacturer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.manufacturer_party", e));
            }
        }
        if let Some(v) = &self.hazardous_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.hazardous_item", e));
            }
        }
        if let Some(v) = &self.dimension {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.dimension", e));
            }
        }
        if let Some(v) = &self.pack_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.pack_quantity", e));
            }
        }
        if let Some(v) = &self.catalogue_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.catalogue_document_reference", e));
            }
        }
        if let Some(v) = &self.classified_tax_category {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.classified_tax_category", e));
            }
        }
        if let Some(v) = &self.catalogue_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.catalogue_indicator", e));
            }
        }
        if let Some(v) = &self.sellers_item_identification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.sellers_item_identification", e));
            }
        }
        if let Some(v) = &self.additional_item_identification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.additional_item_identification", e));
            }
        }
        if let Some(v) = &self.additional_information {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.additional_information", e));
            }
        }
        if let Some(v) = &self.item_instance {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.item_instance", e));
            }
        }
        if let Some(v) = &self.brand_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.brand_name", e));
            }
        }
        if let Some(v) = &self.catalogue_item_identification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.catalogue_item_identification", e));
            }
        }
        if let Some(v) = &self.transaction_conditions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.transaction_conditions", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.description", e));
            }
        }
        if let Some(v) = &self.origin_address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.origin_address", e));
            }
        }
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.name", e));
            }
        }
        if let Some(v) = &self.additional_item_property {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Item.additional_item_property", e));
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

impl Item {
    pub fn title() -> &'static str {
        "Item. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe an item of trade. It includes a generic description applicable to all examples of the item together with optional subsidiary descriptions of any number of actual instances of the type."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            pack_size_numeric: None,
            description: None,
            buyers_item_identification: None,
            sellers_item_identification: None,
            origin_country: None,
            commodity_classification: None,
            catalogue_document_reference: None,
            item_specification_document_reference: None,
            hazardous_risk_indicator: None,
            item_instance: None,
            standard_item_identification: None,
            transaction_conditions: None,
            certificate: None,
            classified_tax_category: None,
            keyword: None,
            name: None,
            origin_address: None,
            additional_item_identification: None,
            catalogue_indicator: None,
            model_name: None,
            pack_quantity: None,
            dimension: None,
            catalogue_item_identification: None,
            manufacturers_item_identification: None,
            brand_name: None,
            additional_information: None,
            manufacturer_party: None,
            hazardous_item: None,
            information_content_provider_party: None,
            additional_item_property: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfAdditionalInformationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AdditionalInformation>,
}

impl AsMut<ItemArrayOfAdditionalInformationComponent> for ItemArrayOfAdditionalInformationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfAdditionalInformationComponent> for ItemArrayOfAdditionalInformationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfAdditionalInformationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfAdditionalInformationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AdditionalInformation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AdditionalInformation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AdditionalInformation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AdditionalInformation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfAdditionalItemIdentificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AdditionalItemIdentification>,
}

impl AsMut<ItemArrayOfAdditionalItemIdentificationComponent> for ItemArrayOfAdditionalItemIdentificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfAdditionalItemIdentificationComponent> for ItemArrayOfAdditionalItemIdentificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfAdditionalItemIdentificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfAdditionalItemIdentificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AdditionalItemIdentification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AdditionalItemIdentification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AdditionalItemIdentification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AdditionalItemIdentification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfAdditionalItemPropertyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AdditionalItemProperty>,
}

impl AsMut<ItemArrayOfAdditionalItemPropertyComponent> for ItemArrayOfAdditionalItemPropertyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfAdditionalItemPropertyComponent> for ItemArrayOfAdditionalItemPropertyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfAdditionalItemPropertyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfAdditionalItemPropertyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AdditionalItemProperty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AdditionalItemProperty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AdditionalItemProperty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AdditionalItemProperty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfBrandNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BrandName>,
}

impl AsMut<ItemArrayOfBrandNameComponent> for ItemArrayOfBrandNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfBrandNameComponent> for ItemArrayOfBrandNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfBrandNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfBrandNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BrandName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BrandName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BrandName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BrandName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfBuyersItemIdentificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::BuyersItemIdentification>,
}

impl AsMut<ItemArrayOfBuyersItemIdentificationComponent> for ItemArrayOfBuyersItemIdentificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfBuyersItemIdentificationComponent> for ItemArrayOfBuyersItemIdentificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfBuyersItemIdentificationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfBuyersItemIdentificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfBuyersItemIdentificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::BuyersItemIdentification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::BuyersItemIdentification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::BuyersItemIdentification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::BuyersItemIdentification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfCatalogueDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CatalogueDocumentReference>,
}

impl AsMut<ItemArrayOfCatalogueDocumentReferenceComponent> for ItemArrayOfCatalogueDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfCatalogueDocumentReferenceComponent> for ItemArrayOfCatalogueDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfCatalogueDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfCatalogueDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfCatalogueDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CatalogueDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CatalogueDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CatalogueDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CatalogueDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfCatalogueIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CatalogueIndicator>,
}

impl AsMut<ItemArrayOfCatalogueIndicatorComponent> for ItemArrayOfCatalogueIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfCatalogueIndicatorComponent> for ItemArrayOfCatalogueIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfCatalogueIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfCatalogueIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfCatalogueIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CatalogueIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CatalogueIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CatalogueIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CatalogueIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfCatalogueItemIdentificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CatalogueItemIdentification>,
}

impl AsMut<ItemArrayOfCatalogueItemIdentificationComponent> for ItemArrayOfCatalogueItemIdentificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfCatalogueItemIdentificationComponent> for ItemArrayOfCatalogueItemIdentificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfCatalogueItemIdentificationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfCatalogueItemIdentificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfCatalogueItemIdentificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CatalogueItemIdentification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CatalogueItemIdentification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CatalogueItemIdentification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CatalogueItemIdentification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfCertificateComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Certificate>,
}

impl AsMut<ItemArrayOfCertificateComponent> for ItemArrayOfCertificateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfCertificateComponent> for ItemArrayOfCertificateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfCertificateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfCertificateComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Certificate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Certificate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Certificate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Certificate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfClassifiedTaxCategoryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ClassifiedTaxCategory>,
}

impl AsMut<ItemArrayOfClassifiedTaxCategoryComponent> for ItemArrayOfClassifiedTaxCategoryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfClassifiedTaxCategoryComponent> for ItemArrayOfClassifiedTaxCategoryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfClassifiedTaxCategoryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfClassifiedTaxCategoryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ClassifiedTaxCategory) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ClassifiedTaxCategory) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ClassifiedTaxCategory> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ClassifiedTaxCategory> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfCommodityClassificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CommodityClassification>,
}

impl AsMut<ItemArrayOfCommodityClassificationComponent> for ItemArrayOfCommodityClassificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfCommodityClassificationComponent> for ItemArrayOfCommodityClassificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfCommodityClassificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfCommodityClassificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CommodityClassification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CommodityClassification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CommodityClassification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CommodityClassification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<ItemArrayOfDescriptionComponent> for ItemArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfDescriptionComponent> for ItemArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfDescriptionComponent {
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
pub struct ItemArrayOfDimensionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Dimension>,
}

impl AsMut<ItemArrayOfDimensionComponent> for ItemArrayOfDimensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfDimensionComponent> for ItemArrayOfDimensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfDimensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfDimensionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Dimension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Dimension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Dimension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Dimension> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfHazardousItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::HazardousItem>,
}

impl AsMut<ItemArrayOfHazardousItemComponent> for ItemArrayOfHazardousItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfHazardousItemComponent> for ItemArrayOfHazardousItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfHazardousItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfHazardousItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::HazardousItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::HazardousItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::HazardousItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::HazardousItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfHazardousRiskIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HazardousRiskIndicator>,
}

impl AsMut<ItemArrayOfHazardousRiskIndicatorComponent> for ItemArrayOfHazardousRiskIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfHazardousRiskIndicatorComponent> for ItemArrayOfHazardousRiskIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfHazardousRiskIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfHazardousRiskIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfHazardousRiskIndicatorComponent {
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
pub struct ItemArrayOfInformationContentProviderPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::InformationContentProviderParty>,
}

impl AsMut<ItemArrayOfInformationContentProviderPartyComponent> for ItemArrayOfInformationContentProviderPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfInformationContentProviderPartyComponent> for ItemArrayOfInformationContentProviderPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfInformationContentProviderPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfInformationContentProviderPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfInformationContentProviderPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::InformationContentProviderParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::InformationContentProviderParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::InformationContentProviderParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::InformationContentProviderParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfItemInstanceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ItemInstance>,
}

impl AsMut<ItemArrayOfItemInstanceComponent> for ItemArrayOfItemInstanceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfItemInstanceComponent> for ItemArrayOfItemInstanceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfItemInstanceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfItemInstanceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ItemInstance) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ItemInstance) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ItemInstance> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ItemInstance> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfItemSpecificationDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ItemSpecificationDocumentReference>,
}

impl AsMut<ItemArrayOfItemSpecificationDocumentReferenceComponent> for ItemArrayOfItemSpecificationDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfItemSpecificationDocumentReferenceComponent> for ItemArrayOfItemSpecificationDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfItemSpecificationDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfItemSpecificationDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ItemSpecificationDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ItemSpecificationDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ItemSpecificationDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ItemSpecificationDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfKeywordComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Keyword>,
}

impl AsMut<ItemArrayOfKeywordComponent> for ItemArrayOfKeywordComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfKeywordComponent> for ItemArrayOfKeywordComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfKeywordComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfKeywordComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Keyword) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Keyword) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Keyword> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Keyword> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfManufacturerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ManufacturerParty>,
}

impl AsMut<ItemArrayOfManufacturerPartyComponent> for ItemArrayOfManufacturerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfManufacturerPartyComponent> for ItemArrayOfManufacturerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfManufacturerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfManufacturerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ManufacturerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ManufacturerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ManufacturerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ManufacturerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfManufacturersItemIdentificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ManufacturersItemIdentification>,
}

impl AsMut<ItemArrayOfManufacturersItemIdentificationComponent> for ItemArrayOfManufacturersItemIdentificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfManufacturersItemIdentificationComponent> for ItemArrayOfManufacturersItemIdentificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfManufacturersItemIdentificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfManufacturersItemIdentificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ManufacturersItemIdentification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ManufacturersItemIdentification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ManufacturersItemIdentification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ManufacturersItemIdentification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfModelNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ModelName>,
}

impl AsMut<ItemArrayOfModelNameComponent> for ItemArrayOfModelNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfModelNameComponent> for ItemArrayOfModelNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfModelNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfModelNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ModelName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ModelName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ModelName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ModelName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<ItemArrayOfNameComponent> for ItemArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfNameComponent> for ItemArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Name) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Name) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfOriginAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginAddress>,
}

impl AsMut<ItemArrayOfOriginAddressComponent> for ItemArrayOfOriginAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfOriginAddressComponent> for ItemArrayOfOriginAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfOriginAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfOriginAddressComponent {
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
pub struct ItemArrayOfOriginCountryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OriginCountry>,
}

impl AsMut<ItemArrayOfOriginCountryComponent> for ItemArrayOfOriginCountryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfOriginCountryComponent> for ItemArrayOfOriginCountryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfOriginCountryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfOriginCountryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfOriginCountryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OriginCountry) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OriginCountry) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OriginCountry> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OriginCountry> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfPackQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PackQuantity>,
}

impl AsMut<ItemArrayOfPackQuantityComponent> for ItemArrayOfPackQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfPackQuantityComponent> for ItemArrayOfPackQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfPackQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfPackQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfPackQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PackQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PackQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PackQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PackQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfPackSizeNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PackSizeNumeric>,
}

impl AsMut<ItemArrayOfPackSizeNumericComponent> for ItemArrayOfPackSizeNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfPackSizeNumericComponent> for ItemArrayOfPackSizeNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfPackSizeNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfPackSizeNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfPackSizeNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PackSizeNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PackSizeNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PackSizeNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PackSizeNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfSellersItemIdentificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellersItemIdentification>,
}

impl AsMut<ItemArrayOfSellersItemIdentificationComponent> for ItemArrayOfSellersItemIdentificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfSellersItemIdentificationComponent> for ItemArrayOfSellersItemIdentificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfSellersItemIdentificationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfSellersItemIdentificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfSellersItemIdentificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SellersItemIdentification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SellersItemIdentification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SellersItemIdentification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SellersItemIdentification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfStandardItemIdentificationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::StandardItemIdentification>,
}

impl AsMut<ItemArrayOfStandardItemIdentificationComponent> for ItemArrayOfStandardItemIdentificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfStandardItemIdentificationComponent> for ItemArrayOfStandardItemIdentificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("ItemArrayOfStandardItemIdentificationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfStandardItemIdentificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfStandardItemIdentificationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::StandardItemIdentification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::StandardItemIdentification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::StandardItemIdentification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::StandardItemIdentification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ItemArrayOfTransactionConditionsComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::TransactionConditions>,
}

impl AsMut<ItemArrayOfTransactionConditionsComponent> for ItemArrayOfTransactionConditionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<ItemArrayOfTransactionConditionsComponent> for ItemArrayOfTransactionConditionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("ItemArrayOfTransactionConditionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl ItemArrayOfTransactionConditionsComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::TransactionConditions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::TransactionConditions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::TransactionConditions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::TransactionConditions> {
        self.items.iter()
    }
}


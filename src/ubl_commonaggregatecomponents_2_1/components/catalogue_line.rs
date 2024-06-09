use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatalogueLine {
    #[serde(rename = "AccessoryRelatedItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory_related_item: Option<CatalogueLineArrayOfAccessoryRelatedItemComponent>,
    #[serde(rename = "ActionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_code: Option<CatalogueLineArrayOfActionCodeComponent>,
    #[serde(rename = "CallForTendersDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_for_tenders_document_reference: Option<CatalogueLineArrayOfCallForTendersDocumentReferenceComponent>,
    #[serde(rename = "CallForTendersLineReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_for_tenders_line_reference: Option<CatalogueLineArrayOfCallForTendersLineReferenceComponent>,
    #[serde(rename = "ComplementaryRelatedItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complementary_related_item: Option<CatalogueLineArrayOfComplementaryRelatedItemComponent>,
    #[serde(rename = "ComponentRelatedItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_related_item: Option<CatalogueLineArrayOfComponentRelatedItemComponent>,
    #[serde(rename = "ContentUnitQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_unit_quantity: Option<CatalogueLineArrayOfContentUnitQuantityComponent>,
    #[serde(rename = "ContractSubdivision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_subdivision: Option<CatalogueLineArrayOfContractSubdivisionComponent>,
    #[serde(rename = "ContractorCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contractor_customer_party: Option<CatalogueLineArrayOfContractorCustomerPartyComponent>,
    #[serde(rename = "DocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<CatalogueLineArrayOfDocumentReferenceComponent>,
    #[serde(rename = "ID")]
    pub id: CatalogueLineArrayOfIDComponent,
    #[serde(rename = "Item")]
    pub item: CatalogueLineArrayOfItemComponent,
    #[serde(rename = "ItemComparison")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_comparison: Option<CatalogueLineArrayOfItemComparisonComponent>,
    #[serde(rename = "KeywordItemProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_item_property: Option<CatalogueLineArrayOfKeywordItemPropertyComponent>,
    #[serde(rename = "LifeCycleStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle_status_code: Option<CatalogueLineArrayOfLifeCycleStatusCodeComponent>,
    #[serde(rename = "LineValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_validity_period: Option<CatalogueLineArrayOfLineValidityPeriodComponent>,
    #[serde(rename = "MaximumOrderQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_order_quantity: Option<CatalogueLineArrayOfMaximumOrderQuantityComponent>,
    #[serde(rename = "MinimumOrderQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_order_quantity: Option<CatalogueLineArrayOfMinimumOrderQuantityComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<CatalogueLineArrayOfNoteComponent>,
    #[serde(rename = "OrderQuantityIncrementNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_quantity_increment_numeric: Option<CatalogueLineArrayOfOrderQuantityIncrementNumericComponent>,
    #[serde(rename = "OrderableIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable_indicator: Option<CatalogueLineArrayOfOrderableIndicatorComponent>,
    #[serde(rename = "OrderableUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable_unit: Option<CatalogueLineArrayOfOrderableUnitComponent>,
    #[serde(rename = "PackLevelCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_level_code: Option<CatalogueLineArrayOfPackLevelCodeComponent>,
    #[serde(rename = "ReplacedRelatedItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_related_item: Option<CatalogueLineArrayOfReplacedRelatedItemComponent>,
    #[serde(rename = "ReplacementRelatedItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_related_item: Option<CatalogueLineArrayOfReplacementRelatedItemComponent>,
    #[serde(rename = "RequiredItemLocationQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_item_location_quantity: Option<CatalogueLineArrayOfRequiredItemLocationQuantityComponent>,
    #[serde(rename = "RequiredRelatedItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_related_item: Option<CatalogueLineArrayOfRequiredRelatedItemComponent>,
    #[serde(rename = "SellerSupplierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<CatalogueLineArrayOfSellerSupplierPartyComponent>,
    #[serde(rename = "WarrantyInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_information: Option<CatalogueLineArrayOfWarrantyInformationComponent>,
    #[serde(rename = "WarrantyParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_party: Option<CatalogueLineArrayOfWarrantyPartyComponent>,
    #[serde(rename = "WarrantyValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_validity_period: Option<CatalogueLineArrayOfWarrantyValidityPeriodComponent>,
}

impl AsMut<CatalogueLine> for CatalogueLine {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLine> for CatalogueLine {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.required_item_location_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.required_item_location_quantity", e));
            }
        }
        if let Some(v) = &self.item_comparison {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.item_comparison", e));
            }
        }
        if let Some(v) = &self.orderable_unit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.orderable_unit", e));
            }
        }
        if let Some(v) = &self.replacement_related_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.replacement_related_item", e));
            }
        }
        if let Some(v) = &self.seller_supplier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.seller_supplier_party", e));
            }
        }
        if let Some(v) = &self.replaced_related_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.replaced_related_item", e));
            }
        }
        if let Some(v) = &self.order_quantity_increment_numeric {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.order_quantity_increment_numeric", e));
            }
        }
        if let Some(v) = &self.call_for_tenders_line_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.call_for_tenders_line_reference", e));
            }
        }
        if let Some(v) = &self.accessory_related_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.accessory_related_item", e));
            }
        }
        if let Some(v) = &self.action_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.action_code", e));
            }
        }
        if let Some(v) = &self.document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.document_reference", e));
            }
        }
        if let Some(v) = &self.content_unit_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.content_unit_quantity", e));
            }
        }
        if let Some(v) = &self.contract_subdivision {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.contract_subdivision", e));
            }
        }
        if let Some(v) = &self.life_cycle_status_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.life_cycle_status_code", e));
            }
        }
        if let Some(v) = &self.line_validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.line_validity_period", e));
            }
        }
        if let Some(v) = &self.warranty_information {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.warranty_information", e));
            }
        }
        if let Some(v) = &self.complementary_related_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.complementary_related_item", e));
            }
        }
        if let Err(e) = self.id.validate() {
            return Err(UblError::component("CatalogueLine.id", e));
        }
        if let Some(v) = &self.keyword_item_property {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.keyword_item_property", e));
            }
        }
        if let Some(v) = &self.orderable_indicator {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.orderable_indicator", e));
            }
        }
        if let Some(v) = &self.call_for_tenders_document_reference {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.call_for_tenders_document_reference", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.note", e));
            }
        }
        if let Some(v) = &self.pack_level_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.pack_level_code", e));
            }
        }
        if let Some(v) = &self.minimum_order_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.minimum_order_quantity", e));
            }
        }
        if let Some(v) = &self.required_related_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.required_related_item", e));
            }
        }
        if let Err(e) = self.item.validate() {
            return Err(UblError::component("CatalogueLine.item", e));
        }
        if let Some(v) = &self.maximum_order_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.maximum_order_quantity", e));
            }
        }
        if let Some(v) = &self.warranty_validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.warranty_validity_period", e));
            }
        }
        if let Some(v) = &self.contractor_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.contractor_customer_party", e));
            }
        }
        if let Some(v) = &self.warranty_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.warranty_party", e));
            }
        }
        if let Some(v) = &self.component_related_item {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("CatalogueLine.component_related_item", e));
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

impl CatalogueLine {
    pub fn title() -> &'static str {
        "Catalogue Line. Details"
    }
    pub fn description() -> &'static str {
        "A class to define a line in a Catalogue describing a purchasable item."
    }
    pub fn new(id: CatalogueLineArrayOfIDComponent, item: CatalogueLineArrayOfItemComponent) -> Component<Self> {
        Component(Self {
            required_related_item: None,
            content_unit_quantity: None,
            id,
            warranty_validity_period: None,
            complementary_related_item: None,
            minimum_order_quantity: None,
            keyword_item_property: None,
            note: None,
            orderable_unit: None,
            document_reference: None,
            replaced_related_item: None,
            warranty_information: None,
            accessory_related_item: None,
            maximum_order_quantity: None,
            contract_subdivision: None,
            call_for_tenders_document_reference: None,
            call_for_tenders_line_reference: None,
            contractor_customer_party: None,
            item,
            line_validity_period: None,
            required_item_location_quantity: None,
            pack_level_code: None,
            seller_supplier_party: None,
            life_cycle_status_code: None,
            warranty_party: None,
            action_code: None,
            orderable_indicator: None,
            component_related_item: None,
            replacement_related_item: None,
            order_quantity_increment_numeric: None,
            item_comparison: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfAccessoryRelatedItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AccessoryRelatedItem>,
}

impl AsMut<CatalogueLineArrayOfAccessoryRelatedItemComponent> for CatalogueLineArrayOfAccessoryRelatedItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfAccessoryRelatedItemComponent> for CatalogueLineArrayOfAccessoryRelatedItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfAccessoryRelatedItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfAccessoryRelatedItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AccessoryRelatedItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AccessoryRelatedItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AccessoryRelatedItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AccessoryRelatedItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfActionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ActionCode>,
}

impl AsMut<CatalogueLineArrayOfActionCodeComponent> for CatalogueLineArrayOfActionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfActionCodeComponent> for CatalogueLineArrayOfActionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfActionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfActionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfActionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ActionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ActionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ActionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ActionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfCallForTendersDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference>,
}

impl AsMut<CatalogueLineArrayOfCallForTendersDocumentReferenceComponent> for CatalogueLineArrayOfCallForTendersDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfCallForTendersDocumentReferenceComponent> for CatalogueLineArrayOfCallForTendersDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfCallForTendersDocumentReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfCallForTendersDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfCallForTendersDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CallForTendersDocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfCallForTendersLineReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::CallForTendersLineReference>,
}

impl AsMut<CatalogueLineArrayOfCallForTendersLineReferenceComponent> for CatalogueLineArrayOfCallForTendersLineReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfCallForTendersLineReferenceComponent> for CatalogueLineArrayOfCallForTendersLineReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfCallForTendersLineReferenceComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfCallForTendersLineReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfCallForTendersLineReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::CallForTendersLineReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::CallForTendersLineReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::CallForTendersLineReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::CallForTendersLineReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfComplementaryRelatedItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ComplementaryRelatedItem>,
}

impl AsMut<CatalogueLineArrayOfComplementaryRelatedItemComponent> for CatalogueLineArrayOfComplementaryRelatedItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfComplementaryRelatedItemComponent> for CatalogueLineArrayOfComplementaryRelatedItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfComplementaryRelatedItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfComplementaryRelatedItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ComplementaryRelatedItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ComplementaryRelatedItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ComplementaryRelatedItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ComplementaryRelatedItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfComponentRelatedItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ComponentRelatedItem>,
}

impl AsMut<CatalogueLineArrayOfComponentRelatedItemComponent> for CatalogueLineArrayOfComponentRelatedItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfComponentRelatedItemComponent> for CatalogueLineArrayOfComponentRelatedItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfComponentRelatedItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfComponentRelatedItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ComponentRelatedItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ComponentRelatedItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ComponentRelatedItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ComponentRelatedItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfContentUnitQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ContentUnitQuantity>,
}

impl AsMut<CatalogueLineArrayOfContentUnitQuantityComponent> for CatalogueLineArrayOfContentUnitQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfContentUnitQuantityComponent> for CatalogueLineArrayOfContentUnitQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfContentUnitQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfContentUnitQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfContentUnitQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ContentUnitQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ContentUnitQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ContentUnitQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ContentUnitQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfContractSubdivisionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ContractSubdivision>,
}

impl AsMut<CatalogueLineArrayOfContractSubdivisionComponent> for CatalogueLineArrayOfContractSubdivisionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfContractSubdivisionComponent> for CatalogueLineArrayOfContractSubdivisionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfContractSubdivisionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfContractSubdivisionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfContractSubdivisionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ContractSubdivision) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ContractSubdivision) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ContractSubdivision> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ContractSubdivision> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfContractorCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContractorCustomerParty>,
}

impl AsMut<CatalogueLineArrayOfContractorCustomerPartyComponent> for CatalogueLineArrayOfContractorCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfContractorCustomerPartyComponent> for CatalogueLineArrayOfContractorCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfContractorCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfContractorCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfContractorCustomerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContractorCustomerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContractorCustomerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContractorCustomerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContractorCustomerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfDocumentReferenceComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::DocumentReference>,
}

impl AsMut<CatalogueLineArrayOfDocumentReferenceComponent> for CatalogueLineArrayOfDocumentReferenceComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfDocumentReferenceComponent> for CatalogueLineArrayOfDocumentReferenceComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfDocumentReferenceComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfDocumentReferenceComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::DocumentReference) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::DocumentReference> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<CatalogueLineArrayOfIDComponent> for CatalogueLineArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfIDComponent> for CatalogueLineArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfIDComponent {
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
pub struct CatalogueLineArrayOfItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Item>,
}

impl AsMut<CatalogueLineArrayOfItemComponent> for CatalogueLineArrayOfItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfItemComponent> for CatalogueLineArrayOfItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfItemComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfItemComponent {
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
pub struct CatalogueLineArrayOfItemComparisonComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ItemComparison>,
}

impl AsMut<CatalogueLineArrayOfItemComparisonComponent> for CatalogueLineArrayOfItemComparisonComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfItemComparisonComponent> for CatalogueLineArrayOfItemComparisonComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfItemComparisonComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfItemComparisonComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ItemComparison) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ItemComparison) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ItemComparison> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ItemComparison> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfKeywordItemPropertyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::KeywordItemProperty>,
}

impl AsMut<CatalogueLineArrayOfKeywordItemPropertyComponent> for CatalogueLineArrayOfKeywordItemPropertyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfKeywordItemPropertyComponent> for CatalogueLineArrayOfKeywordItemPropertyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfKeywordItemPropertyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfKeywordItemPropertyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::KeywordItemProperty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::KeywordItemProperty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::KeywordItemProperty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::KeywordItemProperty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfLifeCycleStatusCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LifeCycleStatusCode>,
}

impl AsMut<CatalogueLineArrayOfLifeCycleStatusCodeComponent> for CatalogueLineArrayOfLifeCycleStatusCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfLifeCycleStatusCodeComponent> for CatalogueLineArrayOfLifeCycleStatusCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfLifeCycleStatusCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfLifeCycleStatusCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfLifeCycleStatusCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LifeCycleStatusCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LifeCycleStatusCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LifeCycleStatusCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LifeCycleStatusCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfLineValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LineValidityPeriod>,
}

impl AsMut<CatalogueLineArrayOfLineValidityPeriodComponent> for CatalogueLineArrayOfLineValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfLineValidityPeriodComponent> for CatalogueLineArrayOfLineValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfLineValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfLineValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfLineValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LineValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LineValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LineValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LineValidityPeriod> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfMaximumOrderQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MaximumOrderQuantity>,
}

impl AsMut<CatalogueLineArrayOfMaximumOrderQuantityComponent> for CatalogueLineArrayOfMaximumOrderQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfMaximumOrderQuantityComponent> for CatalogueLineArrayOfMaximumOrderQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfMaximumOrderQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfMaximumOrderQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfMaximumOrderQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MaximumOrderQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MaximumOrderQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MaximumOrderQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MaximumOrderQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfMinimumOrderQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MinimumOrderQuantity>,
}

impl AsMut<CatalogueLineArrayOfMinimumOrderQuantityComponent> for CatalogueLineArrayOfMinimumOrderQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfMinimumOrderQuantityComponent> for CatalogueLineArrayOfMinimumOrderQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfMinimumOrderQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfMinimumOrderQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfMinimumOrderQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MinimumOrderQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MinimumOrderQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MinimumOrderQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MinimumOrderQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<CatalogueLineArrayOfNoteComponent> for CatalogueLineArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfNoteComponent> for CatalogueLineArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfNoteComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Note) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Note) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfOrderQuantityIncrementNumericComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OrderQuantityIncrementNumeric>,
}

impl AsMut<CatalogueLineArrayOfOrderQuantityIncrementNumericComponent> for CatalogueLineArrayOfOrderQuantityIncrementNumericComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfOrderQuantityIncrementNumericComponent> for CatalogueLineArrayOfOrderQuantityIncrementNumericComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfOrderQuantityIncrementNumericComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfOrderQuantityIncrementNumericComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfOrderQuantityIncrementNumericComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OrderQuantityIncrementNumeric) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OrderQuantityIncrementNumeric) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OrderQuantityIncrementNumeric> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OrderQuantityIncrementNumeric> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfOrderableIndicatorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OrderableIndicator>,
}

impl AsMut<CatalogueLineArrayOfOrderableIndicatorComponent> for CatalogueLineArrayOfOrderableIndicatorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfOrderableIndicatorComponent> for CatalogueLineArrayOfOrderableIndicatorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfOrderableIndicatorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfOrderableIndicatorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfOrderableIndicatorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OrderableIndicator) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OrderableIndicator) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OrderableIndicator> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OrderableIndicator> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfOrderableUnitComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::OrderableUnit>,
}

impl AsMut<CatalogueLineArrayOfOrderableUnitComponent> for CatalogueLineArrayOfOrderableUnitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfOrderableUnitComponent> for CatalogueLineArrayOfOrderableUnitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfOrderableUnitComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfOrderableUnitComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfOrderableUnitComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::OrderableUnit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::OrderableUnit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::OrderableUnit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::OrderableUnit> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfPackLevelCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PackLevelCode>,
}

impl AsMut<CatalogueLineArrayOfPackLevelCodeComponent> for CatalogueLineArrayOfPackLevelCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfPackLevelCodeComponent> for CatalogueLineArrayOfPackLevelCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfPackLevelCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfPackLevelCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfPackLevelCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PackLevelCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PackLevelCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PackLevelCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PackLevelCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfReplacedRelatedItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReplacedRelatedItem>,
}

impl AsMut<CatalogueLineArrayOfReplacedRelatedItemComponent> for CatalogueLineArrayOfReplacedRelatedItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfReplacedRelatedItemComponent> for CatalogueLineArrayOfReplacedRelatedItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfReplacedRelatedItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfReplacedRelatedItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReplacedRelatedItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReplacedRelatedItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReplacedRelatedItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReplacedRelatedItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfReplacementRelatedItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ReplacementRelatedItem>,
}

impl AsMut<CatalogueLineArrayOfReplacementRelatedItemComponent> for CatalogueLineArrayOfReplacementRelatedItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfReplacementRelatedItemComponent> for CatalogueLineArrayOfReplacementRelatedItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfReplacementRelatedItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfReplacementRelatedItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ReplacementRelatedItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ReplacementRelatedItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ReplacementRelatedItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ReplacementRelatedItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfRequiredItemLocationQuantityComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity>,
}

impl AsMut<CatalogueLineArrayOfRequiredItemLocationQuantityComponent> for CatalogueLineArrayOfRequiredItemLocationQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfRequiredItemLocationQuantityComponent> for CatalogueLineArrayOfRequiredItemLocationQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfRequiredItemLocationQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfRequiredItemLocationQuantityComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequiredItemLocationQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfRequiredRelatedItemComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::RequiredRelatedItem>,
}

impl AsMut<CatalogueLineArrayOfRequiredRelatedItemComponent> for CatalogueLineArrayOfRequiredRelatedItemComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfRequiredRelatedItemComponent> for CatalogueLineArrayOfRequiredRelatedItemComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfRequiredRelatedItemComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfRequiredRelatedItemComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::RequiredRelatedItem) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::RequiredRelatedItem) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::RequiredRelatedItem> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::RequiredRelatedItem> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfSellerSupplierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty>,
}

impl AsMut<CatalogueLineArrayOfSellerSupplierPartyComponent> for CatalogueLineArrayOfSellerSupplierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfSellerSupplierPartyComponent> for CatalogueLineArrayOfSellerSupplierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfSellerSupplierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfSellerSupplierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfSellerSupplierPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SellerSupplierParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfWarrantyInformationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::WarrantyInformation>,
}

impl AsMut<CatalogueLineArrayOfWarrantyInformationComponent> for CatalogueLineArrayOfWarrantyInformationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfWarrantyInformationComponent> for CatalogueLineArrayOfWarrantyInformationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfWarrantyInformationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfWarrantyInformationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::WarrantyInformation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::WarrantyInformation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::WarrantyInformation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::WarrantyInformation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfWarrantyPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WarrantyParty>,
}

impl AsMut<CatalogueLineArrayOfWarrantyPartyComponent> for CatalogueLineArrayOfWarrantyPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfWarrantyPartyComponent> for CatalogueLineArrayOfWarrantyPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfWarrantyPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfWarrantyPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfWarrantyPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WarrantyParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WarrantyParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WarrantyParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WarrantyParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CatalogueLineArrayOfWarrantyValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod>,
}

impl AsMut<CatalogueLineArrayOfWarrantyValidityPeriodComponent> for CatalogueLineArrayOfWarrantyValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<CatalogueLineArrayOfWarrantyValidityPeriodComponent> for CatalogueLineArrayOfWarrantyValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfWarrantyValidityPeriodComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("CatalogueLineArrayOfWarrantyValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl CatalogueLineArrayOfWarrantyValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::WarrantyValidityPeriod> {
        self.items.iter()
    }
}


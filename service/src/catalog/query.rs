use std::collections::HashMap;

use domain::catalog::{
  attribute::{AttributeWithOptions, Column, Entity as Attribute},
  attribute_option::{AttributeOptionDTO, Entity as AttributeOption},
  category::{CategoryDTO, Entity as Category},
};
use infra::{
  response::{PaginatedResponse, PaginationMeta},
  uuid::Uuid,
};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QuerySelect, SelectColumns};

use super::definition::{
  AttributeWithOptionsQueryOutput, ListPaginatedAttributesQuery, ListPaginatedCategoriesQuery,
};

pub async fn list_paginated_categories_query(
  query: ListPaginatedCategoriesQuery,
  db: &impl ConnectionTrait,
) -> Result<PaginatedResponse<CategoryDTO>, DbErr> {
  let per_page = query.per_page.unwrap_or(30);
  let page = query.page.unwrap_or(1) - 1;

  let category_pages = Category::find()
    .select_only()
    .column(Column::Id)
    .column(Column::Name)
    .into_partial_model::<CategoryDTO>()
    .paginate(db, per_page);
  let categorys = category_pages.fetch_page(page).await?;
  let items_and_pages = category_pages.num_items_and_pages().await?;
  let total = items_and_pages.number_of_items;
  let total_pages = items_and_pages.number_of_pages;

  Ok(PaginatedResponse::<CategoryDTO> {
    ok: true,
    data: categorys,
    meta: PaginationMeta {
      page: page + 1,
      total_pages,
      per_page,
      total,
    },
  })
}

pub async fn list_paginated_attributes_query(
  query: ListPaginatedAttributesQuery,
  db: &impl ConnectionTrait,
) -> Result<PaginatedResponse<AttributeWithOptions>, DbErr> {
  let per_page = query.per_page.unwrap_or(30);
  let page = query.page.unwrap_or(1) - 1;

  let attribute_pages = Attribute::find()
    .left_join(AttributeOption)
    .select_column(Column::Id)
    .select_column(Column::Name)
    .select_column_as(
      <domain::catalog::attribute_option::Entity as EntityTrait>::Column::Id,
      "attribute_option_id",
    )
    .select_column_as(
      <domain::catalog::attribute_option::Entity as EntityTrait>::Column::Value,
      "attribute_option_value",
    )
    .into_model::<AttributeWithOptionsQueryOutput>()
    .paginate(db, per_page);

  let attributes = attribute_pages.fetch_page(page).await?;

  let mut attribute_map: HashMap<Uuid, AttributeWithOptions> = HashMap::new();

  for attribute in attributes {
    let entry = attribute_map
      .entry(attribute.id)
      .or_insert(AttributeWithOptions {
        id: attribute.id,
        name: attribute.name.clone(),
        options: Vec::new(),
      });

    entry.options.push(AttributeOptionDTO {
      id: attribute.attribute_option_id,
      value: attribute.attribute_option_value,
    });
  }

  let attributes_with_options: Vec<AttributeWithOptions> = attribute_map.into_values().collect();

  let items_and_pages = attribute_pages.num_items_and_pages().await?;
  let total = items_and_pages.number_of_items;
  let total_pages = items_and_pages.number_of_pages;

  Ok(PaginatedResponse::<AttributeWithOptions> {
    ok: true,
    data: attributes_with_options,
    meta: PaginationMeta {
      page: page + 1,
      total_pages,
      per_page,
      total,
    },
  })
}

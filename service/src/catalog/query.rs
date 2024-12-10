use std::collections::HashMap;

use domain::catalog::{
  attribute::{AttributeWithOptions, Column, Entity as Attribute},
  attribute_option::{self, AttributeOptionDTO, Entity as AttributeOption},
  category::{CategoryDTO, Entity as Category},
};
use infra::{
  response::{PaginatedResponse, PaginationMeta},
  uuid::Uuid,
};
use sea_orm::{
  prelude::Expr,
  sea_query::{Alias, Asterisk, Query},
  ConnectionTrait, DbErr, EntityTrait, FromQueryResult, PaginatorTrait, QuerySelect,
};

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

  let attribute_query = Query::select()
    .column((Attribute, Column::Id))
    .column((Attribute, Column::Name))
    .expr_as(
      Expr::col((AttributeOption, attribute_option::Column::Value)),
      Alias::new("attribute_option_value"),
    )
    .expr_as(
      Expr::col((AttributeOption, attribute_option::Column::Id)),
      Alias::new("attribute_option_id"),
    )
    .from_subquery(
      Query::select()
        .column(Asterisk)
        .from(Attribute)
        .limit(per_page)
        .offset(page * per_page)
        .take(),
      Alias::new("attribute"),
    )
    .left_join(
      AttributeOption,
      Expr::col((Attribute, Column::Id))
        .equals((AttributeOption, attribute_option::Column::AttributeId)),
    )
    .to_owned();

  let builder = db.get_database_backend();
  let attributes =
    AttributeWithOptionsQueryOutput::find_by_statement(builder.build(&attribute_query))
      .all(db)
      .await?;

  let mut attribute_map: HashMap<Uuid, AttributeWithOptions> = HashMap::new();

  for (_, attribute) in attributes.into_iter().enumerate() {
    let entry = attribute_map
      .entry(attribute.id)
      .or_insert(AttributeWithOptions {
        id: attribute.id,
        name: attribute.name.clone(),
        attribute_options: Vec::new(),
      });

    if let (Some(attribute_option_id), Some(attribute_option_value)) = (
      attribute.attribute_option_id,
      attribute.attribute_option_value,
    ) {
      entry.attribute_options.push(AttributeOptionDTO {
        id: attribute_option_id,
        value: attribute_option_value,
      });
    }
  }

  let attributes_with_options: Vec<AttributeWithOptions> = attribute_map.into_values().collect();

  let total_attributes = Attribute::find().count(db).await?;
  let total_pages = (total_attributes as f64 / per_page as f64).ceil() as u64;

  Ok(PaginatedResponse::<AttributeWithOptions> {
    ok: true,
    data: attributes_with_options,
    meta: PaginationMeta {
      page: page + 1,
      total_pages,
      per_page,
      total: total_attributes,
    },
  })
}

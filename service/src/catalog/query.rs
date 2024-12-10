use domain::catalog::category::{CategoryDTO, Column, Entity as Category};
use infra::response::{PaginatedResponse, PaginationMeta};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QuerySelect};

use super::definition::ListPaginatedCategoriesQuery;

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

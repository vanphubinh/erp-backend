use domain::measurement::uom::{Column, Entity as Uom, UomDTO};
use infra::{
  response::{PaginatedResponse, PaginationMeta},
  uuid::Uuid,
};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QuerySelect};

use super::definition::ListPaginatedUomsQuery;

pub async fn list_paginated_uoms_query(
  query: ListPaginatedUomsQuery,
  db: &impl ConnectionTrait,
) -> Result<PaginatedResponse<UomDTO>, DbErr> {
  let per_page = query.per_page.unwrap_or(30);
  let page = query.page.unwrap_or(1) - 1;

  let uom_pages = Uom::find()
    .select_only()
    .column(Column::Id)
    .column(Column::Name)
    .into_partial_model::<UomDTO>()
    .paginate(db, per_page);
  let uoms = uom_pages.fetch_page(page).await?;
  let items_and_pages = uom_pages.num_items_and_pages().await?;
  let total = items_and_pages.number_of_items;
  let total_pages = items_and_pages.number_of_pages;

  Ok(PaginatedResponse::<UomDTO> {
    ok: true,
    data: uoms,
    meta: PaginationMeta {
      page: page + 1,
      total_pages,
      per_page,
      total,
    },
  })
}

pub async fn find_uom_by_id_query(
  id: Uuid,
  db: &impl ConnectionTrait,
) -> Result<Option<UomDTO>, DbErr> {
  let uom = Uom::find_by_id(id)
    .select_only()
    .column(Column::Id)
    .column(Column::Name)
    .into_partial_model::<UomDTO>()
    .one(db)
    .await?;
  Ok(uom)
}

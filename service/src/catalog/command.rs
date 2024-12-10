use super::definition::{CreateAttributeMeta, CreateAttributePayload};
use domain::catalog::{
  attribute::{self, ActiveModel as Attribute},
  attribute_option::{self, ActiveModel as AttributeOption},
};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, Set, TransactionTrait};

pub async fn create_attribute_command(
  payload: CreateAttributePayload,
  db: &(impl ConnectionTrait + TransactionTrait),
) -> Result<CreateAttributeMeta, DbErr> {
  match db
    .transaction::<_, attribute::Model, DbErr>(|txn| {
      Box::pin(async move {
        let attribute = Attribute {
          name: Set(payload.name),
          ..Default::default()
        };
        let attribute = attribute.insert(txn).await?;
        let mut options: Vec<AttributeOption> = Vec::new();
        for (_, option) in payload.attribute_options.into_iter().enumerate() {
          let attribute_option = AttributeOption {
            value: Set(option.value),
            attribute_id: Set(attribute.id),
            ..Default::default()
          };
          options.push(attribute_option);
        }
        if !options.is_empty() {
          attribute_option::Entity::insert_many(options)
            .exec(txn)
            .await?;
        }
        Ok(attribute)
      })
    })
    .await
  {
    Ok(attribute) => return Ok(CreateAttributeMeta { id: attribute.id }),
    Err(e) => return Err(DbErr::Custom(e.to_string())),
  };
}

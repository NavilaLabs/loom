use anyhow::Result;
use eventually::aggregate::{
    Root,
    repository::{Getter, Saver},
};
use loom_core::tenant::customer::{
    CreateCustomerInput, Customer, CustomerEvent, CustomerId, UpdateCustomerInput,
};
use loom_infrastructure_impl::tenant::customer::repositories::{CustomerRepository, CustomerRow};

pub async fn list(workspace_id: &str) -> Result<Vec<CustomerRow>> {
    let pool = super::tenant_pool(workspace_id).await?;
    let repo = CustomerRepository::from_pool(pool).await?;
    Ok(repo.all().await?)
}

pub async fn create(
    workspace_id: &str,
    name: String,
    currency: String,
    timezone: String,
) -> Result<CustomerRow> {
    crate::error::validate(CreateCustomerInput {
        name: name.clone(),
        currency: currency.clone(),
        timezone: timezone.clone(),
    })?;

    let pool = super::tenant_pool(workspace_id).await?;
    let repo = CustomerRepository::from_pool(pool).await?;
    let id = CustomerId::new();
    let mut root = Root::<Customer>::record_new(
        CustomerEvent::Created {
            id: id.clone(),
            name: name.clone(),
            currency: currency.clone(),
            timezone: timezone.clone(),
        }
        .into(),
    )?;
    repo.save(&mut root).await?;
    Ok(CustomerRow {
        id: id.to_string(),
        name,
        comment: None,
        currency,
        timezone,
        country: None,
        visible: true,
        time_budget: None,
        money_budget: None,
        budget_is_monthly: false,
    })
}

#[allow(clippy::too_many_arguments)]
pub async fn update(
    workspace_id: &str,
    id: &str,
    name: String,
    comment: Option<String>,
    currency: String,
    timezone: String,
    country: Option<String>,
    visible: bool,
) -> Result<()> {
    crate::error::validate(UpdateCustomerInput {
        name: name.clone(),
        currency: currency.clone(),
        timezone: timezone.clone(),
    })?;

    let pool = super::tenant_pool(workspace_id).await?;
    let repo = CustomerRepository::from_pool(pool).await?;
    let agg_id: CustomerId = id.parse()?;
    let mut root = repo.get(&agg_id).await?;
    root.record_that(
        CustomerEvent::Updated {
            name,
            comment,
            currency,
            timezone,
            country,
            visible,
        }
        .into(),
    )?;
    repo.save(&mut root).await?;
    Ok(())
}

pub async fn set_budget(
    workspace_id: &str,
    id: &str,
    time_budget: Option<i32>,
    money_budget: Option<i64>,
    budget_is_monthly: bool,
) -> Result<()> {
    let pool = super::tenant_pool(workspace_id).await?;
    let repo = CustomerRepository::from_pool(pool).await?;
    let agg_id: CustomerId = id.parse()?;
    let mut root = repo.get(&agg_id).await?;
    root.record_that(
        CustomerEvent::BudgetUpdated {
            time_budget,
            money_budget,
            budget_is_monthly,
        }
        .into(),
    )?;
    repo.save(&mut root).await?;
    Ok(())
}

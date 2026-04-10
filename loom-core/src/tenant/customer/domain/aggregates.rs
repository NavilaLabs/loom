use eventually::aggregate::Aggregate;
use serde::{Deserialize, Serialize};

use crate::{shared::AggregateId, tenant::customer::CustomerEvent};

pub type CustomerId = AggregateId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Customer {
    id: CustomerId,
    name: String,
    currency: String,
    timezone: String,
    visible: bool,
}

impl Customer {
    pub fn id(&self) -> &CustomerId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn timezone(&self) -> &str {
        &self.timezone
    }

    pub fn visible(&self) -> bool {
        self.visible
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("customer already exists")]
    AlreadyExists,
    #[error("customer not found")]
    NotFound,
}

impl Aggregate for Customer {
    type Id = CustomerId;
    type Event = CustomerEvent;
    type Error = Error;

    fn type_name() -> &'static str {
        "customer"
    }

    fn aggregate_id(&self) -> &Self::Id {
        &self.id
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        match (state, event) {
            (
                None,
                CustomerEvent::Created {
                    id,
                    name,
                    currency,
                    timezone,
                },
            ) => Ok(Self {
                id,
                name,
                currency,
                timezone,
                visible: true,
            }),
            (Some(_), CustomerEvent::Created { .. }) => Err(Error::AlreadyExists),
            (None, _) => Err(Error::NotFound),
            (
                Some(mut customer),
                CustomerEvent::Updated {
                    name,
                    currency,
                    timezone,
                    visible,
                    ..
                },
            ) => {
                customer.name = name;
                customer.currency = currency;
                customer.timezone = timezone;
                customer.visible = visible;
                Ok(customer)
            }
            (Some(customer), CustomerEvent::BudgetUpdated { .. }) => Ok(customer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_id() -> CustomerId {
        "019d0ce8-facb-7c90-b9d7-287ae4f17c91"
            .parse()
            .expect("valid UUID")
    }

    #[test]
    fn apply_created_event_builds_customer() {
        let id = test_id();
        let result = Customer::apply(
            None,
            CustomerEvent::Created {
                id: id.clone(),
                name: "Acme".to_string(),
                currency: "EUR".to_string(),
                timezone: "Europe/Berlin".to_string(),
            },
        );
        assert!(result.is_ok());
        let c = result.unwrap();
        assert_eq!(c.id(), &id);
        assert_eq!(c.name(), "Acme");
        assert!(c.visible());
    }

    #[test]
    fn apply_created_on_existing_returns_already_exists() {
        let id = test_id();
        let existing = Customer::apply(
            None,
            CustomerEvent::Created {
                id: id.clone(),
                name: "Acme".to_string(),
                currency: "EUR".to_string(),
                timezone: "Europe/Berlin".to_string(),
            },
        )
        .unwrap();
        let result = Customer::apply(
            Some(existing),
            CustomerEvent::Created {
                id,
                name: "Dup".to_string(),
                currency: "USD".to_string(),
                timezone: "UTC".to_string(),
            },
        );
        assert!(matches!(result, Err(Error::AlreadyExists)));
    }
}

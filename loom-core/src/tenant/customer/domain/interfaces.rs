use async_trait::async_trait;

use crate::tenant::customer::domain::aggregates::Customer;
use eventually::aggregate::repository::{Getter, Saver};

#[async_trait]
pub trait CustomerRepository: Getter<Customer> + Saver<Customer> + Send + Sync {}

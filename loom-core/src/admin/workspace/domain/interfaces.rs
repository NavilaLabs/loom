use crate::admin::workspace::domain::aggregates::Workspace;
use eventually::aggregate::repository::{Getter, Saver};

pub trait WorkspaceRepository: Getter<Workspace> + Saver<Workspace> + Send + Sync {}

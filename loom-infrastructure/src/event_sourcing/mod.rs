use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    event_version: u32,
    created_by: Uuid,
    correlated_to: Option<Uuid>,
    caused_by: Option<Uuid>,
    owned_by: Option<Uuid>,
}

pub trait Builder {
    fn set_event_version(&mut self, event_version: u32);
    fn set_created_by(&mut self, created_by: Uuid);
    fn set_correlated_to(&mut self, correlated_to: Uuid);
    fn set_caused_by(&mut self, caused_by: Uuid);
    fn set_owned_by(&mut self, owned_by: Uuid);
    fn build(self) -> Metadata;
}

#[derive(Debug, Default)]
pub struct MetadataBuilder {
    event_version: u32,
    created_by: Uuid,
    correlated_to: Option<Uuid>,
    caused_by: Option<Uuid>,
    owned_by: Option<Uuid>,
}

impl Builder for MetadataBuilder {
    fn set_event_version(&mut self, event_version: u32) {
        self.event_version = event_version;
    }

    fn set_created_by(&mut self, created_by: Uuid) {
        self.created_by = created_by;
    }

    fn set_correlated_to(&mut self, correlated_to: Uuid) {
        self.correlated_to = Some(correlated_to);
    }

    fn set_caused_by(&mut self, caused_by: Uuid) {
        self.caused_by = Some(caused_by);
    }

    fn set_owned_by(&mut self, owned_by: Uuid) {
        self.owned_by = Some(owned_by);
    }

    fn build(self) -> Metadata {
        Metadata {
            event_version: self.event_version,
            created_by: self.created_by,
            correlated_to: self.correlated_to,
            caused_by: self.caused_by,
            owned_by: self.owned_by,
        }
    }
}

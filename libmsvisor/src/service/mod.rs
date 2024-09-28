mod elf_service;
mod loader;
#[cfg(feature = "serviceV2")]
mod rust_service;

use std::{collections::BTreeMap, sync::Arc};

use libloading::{Library, Symbol};

use elf_service::WithLibOSService;
pub use loader::ServiceLoader;
use ms_hostcall::types::{IsolationID, ServiceName};

use crate::{logger, metric::SvcMetricBucket, service::elf_service::ElfService};

use self::loader::Namespace;

pub enum Service {
    ELFService(elf_service::ElfService),
    WithLibOSService(elf_service::WithLibOSService),
    #[cfg(feature = "serviceV2")]
    RustService(rust_service::RustService),
}

impl Service {
    fn new(
        name: &str,
        path: &str,
        lib: Arc<Library>,
        metric: Arc<SvcMetricBucket>,
        with_libos: bool,
    ) -> Self {
        logger::debug!("Service::new, name={name}");
        if with_libos {
            Self::WithLibOSService(WithLibOSService::new(name, path, lib, metric))
        } else {
            Self::ELFService(ElfService::new(name, path, lib, metric))
        }
    }
    fn init(&self, isol_id: IsolationID) -> anyhow::Result<()> {
        match self {
            Service::ELFService(svc) => svc.init(isol_id),
            Service::WithLibOSService(svc) => svc.init(isol_id),
            #[cfg(feature = "serviceV2")]
            Service::RustService(svc) => svc.init(isol_id),
        }
    }
    pub fn run(&self, args: &BTreeMap<String, String>) -> Result<(), String> {
        match self {
            Service::ELFService(svc) => svc.run(args),
            Service::WithLibOSService(svc) => svc.run(args),
            #[cfg(feature = "serviceV2")]
            Service::RustService(svc) => svc.run(),
        }
    }
    pub fn interface<T>(&self, name: &str) -> Option<Symbol<T>> {
        match self {
            Service::ELFService(svc) => svc.symbol(name),
            Service::WithLibOSService(svc) => svc.symbol(name),
            #[cfg(feature = "serviceV2")]
            Service::RustService(svc) => Some(svc.symbol(name)),
        }
    }
    pub fn name(&self) -> ServiceName {
        match self {
            Service::ELFService(svc) => svc.name.clone(),
            Service::WithLibOSService(svc) => svc.name(),
            #[cfg(feature = "serviceV2")]
            Service::RustService(svc) => svc.name.to_owned(),
        }
    }
    pub fn namespace(&self) -> Namespace {
        match self {
            Service::ELFService(svc) => svc.namespace(),
            Service::WithLibOSService(svc) => svc.namespace(),
            #[cfg(feature = "serviceV2")]
            Service::RustService(_) => todo!(),
        }
    }

    #[cfg(feature = "enable_mpk")]
    pub fn mprotect(&self) -> anyhow::Result<()> {
        match self {
            Service::ELFService(svc) => svc.mprotect(),
            Service::WithLibOSService(svc) => svc.mprotect(),
            #[cfg(feature = "serviceV2")]
            Service::RustService(_) => todo!(),
        }
    }
}

// impl Drop for Service {
//     fn drop(&mut self) {
//         match self {
//             Service::WithLibOSService(svc) => svc.,
//             Service::RustService(_) => todo!(),
//         }
//     }
// }

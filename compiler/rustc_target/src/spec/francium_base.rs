use crate::spec::{TargetOptions, LinkerFlavor, LldFlavor, PanicStrategy, cvs};

const LINKER_SCRIPT: &str = include_str!("./francium_linker_script.ld");

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "francium".into(),
        env: "relibc".into(),
        executables: true,
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        link_script: Some(LINKER_SCRIPT.into()),
        has_thread_local: true,
        families: cvs!["unix"],
        panic_strategy: PanicStrategy::Abort,
        ..Default::default()
    }
}

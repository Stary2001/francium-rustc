use crate::spec::{Cc, Lld, TargetOptions, LinkerFlavor, PanicStrategy, cvs};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "francium".into(),
        env: "mlibc".into(),
        executables: true,
        dynamic_linking: false,
        has_thread_local: true,
        linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        families: cvs!["unix"],
        panic_strategy: PanicStrategy::Abort,
        ..Default::default()
    }
}

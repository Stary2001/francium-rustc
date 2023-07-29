use crate::spec::Target;

pub fn target() -> Target {
    let mut base = super::francium_base::opts();
    base.max_atomic_width = Some(64);
    base.features = "+m,+a,+f,+d,+c".into();
    base.linker = Some("riscv64-unknown-francium-gcc".into());

    Target {
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128".into(),
        llvm_target: "riscv64".into(),
        pointer_width: 64,
        arch: "riscv64".into(),
        options: base
    }
}

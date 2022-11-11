use crate::spec::Target;
 
pub fn target() -> Target {
    let mut base = super::francium_base::opts();
    base.max_atomic_width = Some(128);
    base.features = "+strict-align,+neon,+fp-armv8".into();
    base.linker = Some("aarch64-unknown-francium-gcc".into());

    Target {
        llvm_target: "aarch64-unknown-francium".into(),
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".into(),
        arch: "aarch64".into(),
        options: base,
    }
}

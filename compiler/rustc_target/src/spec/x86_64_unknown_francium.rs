use crate::spec::Target;
 
pub fn target() -> Target {
    let mut base = super::francium_base::opts();
    base.cpu = "x86-64".into();

    //base.features = "-mmx,-sse,+soft-float".into();
    base.linker = Some("x86_64-unknown-francium-gcc".into());

    Target {
        llvm_target: "x86_64-unknown-francium".into(),
        pointer_width: 64,
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: base,
    }
}

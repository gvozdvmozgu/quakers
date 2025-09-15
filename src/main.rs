fn main() {
    let bios_path = env!("BIOS_PATH");
    let mut qemu = std::process::Command::new("qemu-system-x86_64");

    qemu.arg("-drive")
        .arg(format!("format=raw,file={bios_path}"));

    let mut child = qemu.spawn().unwrap();
    child.wait().unwrap();
}

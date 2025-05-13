use std::{env, fs::copy, process::Command};
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("build") => run_build()?,
        Some("iso") => run_iso()?,
        Some("run") => run_vm()?,
        _ => help(),
    }
    Ok(())
}

fn help(){
    eprintln!(
"
Tasks:
  build - just build the kernel
  iso - pack the kernel into a bootable iso with grub
  run - run the packed iso with quemu
");
}

fn build_kernel() -> Result<()> {
    env::set_current_dir("nedo_os_2")?;
    eprintln!("[+] Building the kernel...");
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .args(&[
            "build", 
            "-r", 
            "--target",
            "../targets/x86_64-nedo_os_2.json"
        ])
        .status()?;
    
    if !status.success() {
        return Err(anyhow!("[-] Cargo build failed!"));
    }
    
    env::set_current_dir("..")?;
    Ok(())
}

fn run_build() -> Result<()> {
    build_kernel()?;
    Ok(())
}

fn run_iso() -> Result<()> {
    run_build()?;
    eprintln!("[+] Packing the ISO...");
    env::set_current_dir("nedo_os_2")?;
    copy("../target/x86_64-nedo_os_2/release/nedo_os_2", 
         "isofiles/boot/nedo_os_2")?;
    let status = Command::new("grub-mkrescue")
        .args(&[
            "-o",
            "../target/nedo_os_2.iso",
            "isofiles"
        ])
        .status()?;
    if !status.success() {
        return Err(anyhow!("[-] grub-mkrescue failed!"));
    }
    env::set_current_dir("..")?;
    Ok(())
}

fn run_vm() -> Result<()> {
    run_iso()?;
    eprintln!("[+] Running quemu...");
    let status = 
        Command::new("qemu-system-x86_64")
        .args(&[
            "-cdrom",
            "target/nedo_os_2.iso"
        ])
        .status()?;
    if !status.success() {
        return Err(anyhow!("[-] qemu-system-x86_64 failed!"));
    }
    Ok(())
}


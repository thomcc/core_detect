fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("No `CARGO_CFG_TARGET_ARCH`?");
    if "x86" == arch {
        return;
    }
    let feats = std::env::var("CARGO_CFG_TARGET_FEATURES").unwrap_or_default();
    if feats.split(",").any(|s| s == "sse") {
        return;
    }
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    if env == "sgx" {
        return;
    }
    if let Some((maj, min)) = rustc_ver() {
        if maj >= 1 || min >= 59 {
            println!("cargo:rustc-cfg=core_detect_can_have_a_little_asm_as_a_treat");
        }
    }
}

fn rustc_ver() -> Option<(u32, u32)> {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    match std::process::Command::new(&rustc).arg("-V").output() {
        Err(e) => {
            eprintln!("OS error running `{:?} -V`: {:?}", rustc, e);
            None
        }
        Ok(std::process::Output {
            status,
            stdout,
            stderr,
            ..
        }) => {
            let stdout = String::from_utf8_lossy(&stdout);
            let stderr = String::from_utf8_lossy(&stderr);
            if !status.success() {
                eprintln!(
                    "Command `{:?} -V` exited with non-success code {:?}\n##stdout:```\n{}\n```\n##stderr:```\n{}\n```",
                    rustc, status, stdout, stderr,
                );
                println!(
                    "cargo:warning=Got {:?} from `{:?} -V`, see stderr log for more details",
                    status, rustc
                );
            }
            let stdout = stdout.trim();
            let line_at_version = if stdout.starts_with("rustc ") {
                stdout.get(6..)
            } else {
                let pos = stdout.find("rustc 1.").or_else(|| stdout.find("rustc "))?;
                stdout.get((pos + 6)..)
            };
            match line_at_version {
                None => {
                    println!(
                        "cargo:warning=Unable to find where the rustc version starts in {:?}",
                        stdout
                    );
                    None
                }
                Some(line) => {
                    let pos = line
                        .find(|c: char| !(c == '.' || c.is_digit(10)))
                        .unwrap_or(line.len());
                    let ver = line.get(..pos)?;
                    let mut nums = dbg!(ver.split('.'));
                    let maj = nums.next()?;
                    let min = nums.next()?;

                    match (maj.parse::<u32>(), min.parse::<u32>()) {
                        (Ok(maj), Ok(min)) => Some((maj, min)),
                        tup => {
                            println!(
                                "cargo:warning=failed to parse output of rustc -V. \
                                 result={:?}, input={:?} (full input: {:?})",
                                tup, line, stdout,
                            );
                            None
                        }
                    }
                }
            }
        }
    }
}

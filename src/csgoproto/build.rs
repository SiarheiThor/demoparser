use std::{env, io::Result, path::Path, process::Command};

// The generated src/protobuf.rs is committed. Normal builds use it as is, so a build needs
// neither protoc nor network access, and Valve's latest .proto files (which change without
// notice) cannot break it. Set CSGOPROTO_REGENERATE=1 to regenerate it from the pinned
// GameTracking-CS2 commit below; bump the pin deliberately when a new demo patch needs it.
const GAMETRACKING_REPO: &str = "https://github.com/SteamDatabase/GameTracking-CS2.git";
const GAMETRACKING_COMMIT: &str = "fd04856b8668f11cb31ed2b22574607dd9c365e0"; // 2026-08-24

fn main() -> Result<()> {
    println!("cargo::rerun-if-env-changed=CSGOPROTO_REGENERATE");
    if env::var("CSGOPROTO_REGENERATE").as_deref() != Ok("1") {
        return Ok(());
    }

    if !Path::new("GameTracking-CS2/.git").exists() {
        run(&["clone", "--filter=blob:none", "--no-checkout", GAMETRACKING_REPO, "GameTracking-CS2"], ".")?;
    }
    run(&["fetch", "origin", GAMETRACKING_COMMIT], "GameTracking-CS2")?;
    run(&["checkout", "--force", GAMETRACKING_COMMIT, "--", "Protobufs"], "GameTracking-CS2")?;

    let protos = vec![
        "GameTracking-CS2/Protobufs/steammessages.proto",
        "GameTracking-CS2/Protobufs/gcsdk_gcmessages.proto",
        "GameTracking-CS2/Protobufs/demo.proto",
        "GameTracking-CS2/Protobufs/cstrike15_gcmessages.proto",
        "GameTracking-CS2/Protobufs/cstrike15_usermessages.proto",
        "GameTracking-CS2/Protobufs/usermessages.proto",
        "GameTracking-CS2/Protobufs/networkbasetypes.proto",
        "GameTracking-CS2/Protobufs/engine_gcmessages.proto",
        "GameTracking-CS2/Protobufs/netmessages.proto",
        "GameTracking-CS2/Protobufs/network_connection.proto",
        "GameTracking-CS2/Protobufs/cs_usercmd.proto",
        "GameTracking-CS2/Protobufs/usercmd.proto",
        "GameTracking-CS2/Protobufs/gameevents.proto",
        "GameTracking-CS2/Protobufs/cs_gameevents.proto",
    ];

    prost_build::Config::new()
        .format(false)
        .out_dir("src")
        .default_package_filename("protobuf")
        .bytes(["."])
        .enum_attribute(".", "#[derive(::strum::EnumIter)]")
        .compile_protos(&protos, &["GameTracking-CS2/Protobufs/"])
}

fn run(args: &[&str], dir: &str) -> Result<()> {
    let status = Command::new("git").args(args).current_dir(dir).status()?;
    if !status.success() {
        return Err(std::io::Error::other(format!("git {} failed: {status}", args.join(" "))));
    }
    Ok(())
}

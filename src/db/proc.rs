use std::process::{Command, Stdio, Child};
use std::time::{Instant,Duration};
use log::{info,debug, error};

use crate::errors::BGCError;
use crate::prelude::GENResult;
use crate::SETTINGS;

pub struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        match self.0.kill() {
            Err(why) => error!("Failed to kill local DB instance: {}", why),
            Ok(_) => {}
        }
    }
}

pub fn init_local_db() -> Option<ChildGuard> {
    if !&SETTINGS.db_start_local {
        return None
    }
    let db_proc = match spawn_local_db_proc() {
        Err(why) => panic!("Failed starting local DB instance: {}", why),
        Ok(db_proc) => ChildGuard(db_proc)
    };
    match wait_for_db_proc() {
        Err(why) => panic!("Failed verifying local DB instance: {}", why),
        Ok(_) => info!("Started local DB instance succesfully")
    }
    Some(db_proc)
}

fn spawn_local_db_proc() -> GENResult<Child> {
    let proc = Command::new(&SETTINGS.db_cmd)
        .args(["start", "--user", &SETTINGS.db_user, "--pass", &SETTINGS.db_pass, "--bind", "127.0.0.1:8001", "file:data/bgcat.db"])
        .stdout(Stdio::piped())
        .spawn()?;
    Ok(proc)
}

fn wait_for_db_proc() -> GENResult<()> {
    let now = Instant::now();
    let timeout_sec = 30;
    let sleep_sec = Duration::from_secs(5);
    let mut command = Command::new(&SETTINGS.db_cmd);
    command.args(["isready", "--conn", "ws://localhost:8001"]);
    command.stdout(Stdio::null());
    command.stderr(Stdio::null());

    info!("Waiting for local DB instance");
    loop {
        let exit_status = command.status()?;
        debug!("Local DB instance check: {}", exit_status);
        if exit_status.success() {
            return Ok(())
        }
        if now.elapsed().as_secs() > timeout_sec {
            return Err(BGCError::InternalError("isready timeout".to_string()))
        }
        std::thread::sleep(sleep_sec);
    }
}
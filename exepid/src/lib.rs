use std::{env, fs::write, path::Path, process};

pub fn save() -> std::io::Result<()> {
  let pid = process::id();

  let pid_file = if let Ok(fp) = env::var("PID_FILE") {
    fp.into()
  } else {
    let exe_path = env::current_exe()?;
    let exe_folder = exe_path.parent().unwrap_or_else(|| Path::new("."));

    let program_name = exe_path
      .file_stem()
      .and_then(|os_str| os_str.to_str())
      .unwrap_or("");

    exe_folder.join(format!("{program_name}.pid"))
  };

  write(pid_file, pid.to_string())?;

  Ok(())
}

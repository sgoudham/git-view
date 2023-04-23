use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

#[path = "src/bin/git-view/cmd.rs"]
mod cmd;

fn main() -> io::Result<()> {
    let out_dir: PathBuf = env::var_os("OUT_DIR")
        .ok_or(io::ErrorKind::NotFound)?
        .into();
    fs::create_dir_all(&out_dir)?;
    let mut manpage = fs::File::create(out_dir.join("git-view.1"))?;

    let man = clap_mangen::Man::new(cmd::cmd());
    man.render(&mut manpage)?;

    Ok(())
}

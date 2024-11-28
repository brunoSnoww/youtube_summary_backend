use std::process::Command;

use super::{
    assembling::{self},
    error::BlobResult,
    run,
};

pub fn dispatch(url: String) -> BlobResult<()> {
    let mut command_and_config = assembling::generate_command(&url)?;
    {
        let command: &mut Command = &mut command_and_config.0;
        if run::run_command(command).is_some() {
        } else {
            #[cfg(debug_assertions)]
            println!("The command ran without any errors!! :)");
        }
    };

    Ok(())
}

use crate::password::Password;
use crate::settings::Settings;
use anyhow::anyhow;
use dialoguer::Confirm;

pub fn insert(
    password_name: &str,
    multi_line: bool,
    echo: bool,
    force: bool,
    settings: &Settings,
) -> anyhow::Result<()> {
    // Create empty password
    let mut password = Password::default();

    // Set path of password
    password.set_filepath(settings.get_password_store_path()?, password_name);

    // Check if file exists, if so ask to overwrite if force flag is not passed
    if password.exists()
        && !force
        && !Confirm::new()
            .with_prompt(format!(
                "An entry already exists for {}. Overwrite it?",
                password_name
            ))
            .default(false)
            .show_default(true)
            .interact()?
    {
        // If user says no
        println!("Password insertion canceled.");
        return Ok(());
    }

    // Get password from terminal
    if let Err(e) = password.terminal_input(password_name, multi_line) {
        return Err(anyhow!("Password insertion aborted: {}", e));
    }

    // Encrypt password and write output to file
    password.encrypt_with_key(settings.get_pgp_key_path()?)?;

    // TODO: git add, git commit

    // Display password if echo flag is passed
    if echo {
        println!("{}", password.to_string()?);
    }

    Ok(())
}

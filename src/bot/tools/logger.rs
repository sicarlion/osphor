use poise::serenity_prelude::{GuildId, MessageId};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;

// Log the message to ./guild
pub fn log_message(
    guild_id: &GuildId,
    msg_id: &MessageId,
    author: &String,
    content: &String,
) -> Result<(), Error> {
    let guild_dir = format!("./guild/{}", guild_id);
    fs::create_dir_all(&guild_dir)?;

    let log_file_path = format!("{}/messages.log", guild_dir);

    if Path::new(&log_file_path).exists() {
        // Read the file line by line
        let file = fs::File::open(&log_file_path)?;
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        // If the file has more than 30 lines, remove the excess lines from the top
        if lines.len() >= 30 {
            lines.drain(..lines.len() - 29); // Keep the last 29 lines
        }

        // Write the updated lines back to the file
        let mut file = fs::File::create(&log_file_path)?;
        for line in &lines {
            writeln!(file, "{}", line)?;
        }
    }

    // Open the file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)?;

    // Append the new message
    writeln!(file, "{}:present:{}:{}", msg_id, author, content)?;
    Ok(())
}

// Change the message status from :present: to :deleted:
pub fn log_mark_deleted(guild_id: &GuildId, msg_id: &MessageId) -> Result<(), Error> {
    let log_file_path = format!("./guild/{}/messages.log", guild_id);

    if !Path::new(&log_file_path).exists() {
        eprintln!("Log file not found for guild {}", guild_id);
        return Ok(());
    }

    let file = fs::File::open(&log_file_path)?;
    let reader = BufReader::new(file);

    let temp_file_path = format!("./guild/{}/messages.tmp", guild_id);
    let mut temp_file = fs::File::create(&temp_file_path)?;

    // Iterate over lines and update the status if the message ID matches
    for line in reader.lines() {
        let line = line?;
        if line.starts_with(&format!("{}:", msg_id)) {
            // Update the status to 'deleted'
            let updated_line = line.replace(":present:", ":deleted:");
            writeln!(temp_file, "{}", updated_line)?;
        } else {
            // Write the line as is
            writeln!(temp_file, "{}", line)?;
        }
    }

    // Replace the original log file with the updated one
    fs::rename(&temp_file_path, &log_file_path)?;
    Ok(())
}

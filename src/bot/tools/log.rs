use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;

use poise::serenity_prelude::{ChannelId, Context, GuildId, Message, MessageId, UserId};

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub guild_id: String,
    pub channel_id: String,
    pub id: String,
    pub author_id: String,
    pub author_name: String,
    pub status: String,
    pub content: String,
}

#[derive(Debug)]
pub struct OsphorLog {
    entries: Vec<LogEntry>,
}

impl OsphorLog {
    // Read and parse the log file
    pub fn fetch(file_path: &String) -> Result<OsphorLog, Error> {
        let mut entries = Vec::new();
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 7 {
                entries.push(LogEntry {
                    guild_id: parts[0].to_string(),
                    channel_id: parts[1].to_string(),
                    id: parts[2].to_string(),
                    author_id: parts[3].to_string(),
                    author_name: parts[4].to_string(),
                    status: parts[5].to_string(),
                    content: parts[6].to_string(),
                });
            }
        }

        // Return a new OsphorLog with updated entries
        Ok(OsphorLog { entries })
    }

    // Log a message to the guild log file
    pub fn log(msg: &Message) -> Result<(), Error> {
        let guild_id = &msg.guild_id.unwrap();
        let channel_id = &msg.channel_id;
        let id = &msg.id;
        let author_id = &msg.author.id;
        let author_name = &msg.author.name;
        let content = &msg.content;

        let guild_dir = format!("./guild/{}", guild_id);
        fs::create_dir_all(&guild_dir)?;

        let log_file_path = format!("{}/messages.log", guild_dir);

        // Trim the file to the last 30 lines if it exists
        if Path::new(&log_file_path).exists() {
            let file = fs::File::open(&log_file_path)?;
            let reader = BufReader::new(file);
            let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

            if lines.len() >= 15 {
                lines.drain(..lines.len() - 14); // Keep the last 29 lines
            }

            // Rewrite the trimmed content back to the file
            let mut file = fs::File::create(&log_file_path)?;
            for line in &lines {
                writeln!(file, "{}", line)?;
            }
        }

        // Open the file in append mode and add the new message
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path)?;

        writeln!(
            file,
            "{}:{}:{}:{}:{}:present:{}",
            guild_id, channel_id, id, author_id, author_name, content
        )?;
        Ok(())
    }

    // Mark a message as deleted in the guild log file
    pub fn mark_deleted(
        guild_id: &GuildId,
        channel_id: &ChannelId,
        id: &MessageId,
    ) -> Result<(), std::io::Error> {
        let log_file_path = format!("./guild/{}/messages.log", guild_id);

        if !Path::new(&log_file_path).exists() {
            eprintln!("Log file not found for guild {}", guild_id);
            return Ok(());
        }

        let file = fs::File::open(&log_file_path)?;
        let reader = BufReader::new(file);

        let temp_file_path = format!("./guild/{}/messages.tmp", guild_id);
        let mut temp_file = fs::File::create(&temp_file_path)?;

        // Update the status of the matching message ID
        for line in reader.lines() {
            let line = line?;
            if line.starts_with(&format!("{}:{}:{}", guild_id, channel_id, id)) {
                let updated_line = line.replace(":present:", ":deleted:");
                writeln!(temp_file, "{}", updated_line)?;
            } else {
                writeln!(temp_file, "{}", line)?;
            }
        }

        // Replace the original log file with the updated one
        fs::rename(&temp_file_path, &log_file_path)?;
        Ok(())
    }

    pub fn last(&self) -> Option<&LogEntry> {
        self.entries.last()
    }

    pub fn all(&self) -> &Vec<LogEntry> {
        &self.entries
    }
}

impl LogEntry {
    pub fn id(&self) -> MessageId {
        let id = self.id.parse::<u64>();

        match id {
            Ok(id) => MessageId::new(id),
            Err(_) => {
                eprintln!("[ERR] Cannot parse str(MessageId) to MessageId");
                MessageId::new(0)
            }
        }
    }

    pub fn guild_id(&self) -> GuildId {
        let guild_id = self.id.parse::<u64>();

        match guild_id {
            Ok(guild_id) => GuildId::new(guild_id),
            Err(_) => {
                eprintln!("[ERR] Cannot parse str(GuildId) to GuildId");
                GuildId::new(0)
            }
        }
    }

    pub fn author_id(&self) -> UserId {
        let author_id = self.author_id.parse::<u64>();

        match author_id {
            Ok(author_id) => UserId::new(author_id),
            Err(_) => {
                eprintln!("[ERR] Cannot parse str(UserId) to UserId");
                UserId::new(0)
            }
        }
    }

    pub async fn author_name(&self, ctx: &Context) -> String {
        let author_id = self.author_id.parse::<u64>();

        match author_id {
            Ok(author_id) => {
                let author = UserId::new(author_id).to_user(&ctx).await;

                if let Ok(author) = author {
                    author.name
                } else {
                    eprintln!("[ERR] Cannot retrieve user data. Is user exist?");
                    "Null".to_string()
                }
            }
            Err(_) => {
                eprintln!("[ERR] Cannot parse str(UserId) to UserId");
                UserId::new(0).to_string()
            }
        }
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn print(&self) -> String {
        format!(
            "{} {} on https://discord.com/channels/{}/{}/{}: {}",
            self.author_name, self.status, self.guild_id, self.channel_id, self.id, self.content
        )
    }
}

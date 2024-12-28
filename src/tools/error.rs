pub enum Exception {
    NotGuild,
    MismatchedRoles,
    EmptyLog,
    Null,
    RolesHierarchy,
}

impl Exception {
    pub fn new(err: Exception) -> String {
        return match err {
            Exception::NotGuild => "Command is not executed in guild".into(),
            Exception::MismatchedRoles => "Target have roles, while Osphor does not".into(),
            Exception::EmptyLog => "Log file is empty, please initiate first message".into(),
            Exception::Null => "This command cannot be executed".into(),
            Exception::RolesHierarchy => "Bot cannot ban the member due to roles hierarchy".into(),
        };
    }

    pub fn mismatched_roles() -> String {
        Exception::new(Exception::MismatchedRoles)
    }
}

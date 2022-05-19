pub struct SimpleDiscord{
    id: Option<String>,
    curr: Option<SimpleChannel>,
}

pub struct SimpleChannel{
    channel_id: String,
    guild_id: String,
}

pub struct DiscordObject{
    token: String,
    status: String,
    user_id: String,
    channel_id: Option<String>,
    guild_id: Option<String>
}

pub struct Enrollment{
    token: Option<String>,
    error: Option<String>
}
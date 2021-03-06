CREATE TABLE IF NOT EXISTS guildconfig (
    id BIGINT,
    welcome_channel_id BIGINT DEFAULT NULL,
    welcome_message CHARACTER VARYING DEFAULT NULL,
    welcome_image CHARACTER VARYING DEFAULT NULL,
    message_log_channel bigint,
    message_log_enabled BOOLEAN DEFAULT FALSE,
    server_log_channel bigint,
    server_log_enabled BOOLEAN DEFAULT FALSE,
    member_log_channel bigint,
    member_log_enabled BOOLEAN DEFAULT FALSE,
    join_log_channel bigint,
    join_log_enabled BOOLEAN DEFAULT FALSE,
    voice_log_channel bigint,
    voice_log_enabled BOOLEAN DEFAULT FALSE,
    mod_log_channel bigint,
    mod_log_enabled BOOLEAN DEFAULT FALSE,
    autoroles BIGINT[],
    starboard_threshold INTEGER DEFAULT 5,
    starboard_channel BIGINT,
    starboard_activate BOOLEAN DEFAULT FALSE,
    watcher_channel BIGINT,
    watcher_message BIGINT,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS reactionrole (
    guild_id BIGINT,
    roles BIGINT[],
    message_id BIGINT,
    titles CHARACTER VARYING[],
    descriptions CHARACTER VARYING[]
);

CREATE TABLE IF NOT EXISTS starboard_message (
    stars_count INTEGER,
    message_id BIGINT,
    guild_id BIGINT,
    author_id BIGINT,
    channel_id BIGINT,
    star_msg_id BIGINT,
    id SERIAL,
    PRIMARY KEY(id)
);
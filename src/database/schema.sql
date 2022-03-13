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
    PRIMARY KEY(id)
);

---------------------------------------------------------------------


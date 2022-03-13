CREATE TABLE IF NOT EXISTS guildconfig (
    id BIGINT,
    welcome_channel_id BIGINT DEFAULT NULL,
    welcome_message CHARACTER VARYING DEFAULT NULL,
    PRIMARY KEY(id)
);
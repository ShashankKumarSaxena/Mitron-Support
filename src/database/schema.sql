CREATE TABLE IF NOT EXISTS guildconfig (
    id BIGINT,
    welcome_channel_id BIGINT DEFAULT NULL,
    PRIMARY KEY(id)
);
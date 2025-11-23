CREATE TABLE balances (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    type SMALLINT UNSIGNED NOT NULL,
    user_bonus_id BIGINT UNSIGNED NULL
) COLLATE = utf8mb4_unicode_ci;

CREATE INDEX balances_type_bonus_id_index
    ON balances (type, user_bonus_id);
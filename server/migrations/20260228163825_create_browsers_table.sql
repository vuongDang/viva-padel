-- Create browsers table for Web Push subscriptions
CREATE TABLE IF NOT EXISTS browsers (
    browser_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,

    -- Web Push fields
    endpoint TEXT NOT NULL, -- Web Push server
    p256dh TEXT NOT NULL, -- public encryption for the notification payload (browser has the private key)
    auth TEXT NOT NULL, -- authentication secret for paylaod integrity

    -- MetaData
    last_seen DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Optional: Create an index on user_id for faster lookups when sending notifications
CREATE INDEX IF NOT EXISTS idx_browsers_user_id ON browsers(user_id);

-- Seed users table with sample data
INSERT INTO users (username, email) VALUES
    ('john_doe', 'john@example.com'),
    ('jane_smith', 'jane@example.com'),
    ('admin', 'admin@example.com')
ON CONFLICT (username) DO NOTHING; 
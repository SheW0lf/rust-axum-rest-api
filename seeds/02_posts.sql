-- Seed posts table with sample data
-- Only insert posts if the corresponding users exist
INSERT INTO posts (title, body, user_id) 
SELECT 
    'First Post',
    'This is my first post content.',
    id
FROM users WHERE username = 'john_doe'
UNION ALL
SELECT 
    'Hello World',
    'Welcome to our platform!',
    id
FROM users WHERE username = 'john_doe'
UNION ALL
SELECT 
    'Getting Started',
    'A guide for new users.',
    id
FROM users WHERE username = 'jane_smith'
UNION ALL
SELECT 
    'Admin Announcement',
    'Important updates coming soon.',
    id
FROM users WHERE username = 'admin'; 
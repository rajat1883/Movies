CREATE TABLE movies
(
    id           INT AUTO_INCREMENT PRIMARY KEY,
    title        VARCHAR(100) NOT NULL,
    release_date DATE,
    description  VARCHAR(255),
    genre        VARCHAR(50),
    icon         TEXT
)
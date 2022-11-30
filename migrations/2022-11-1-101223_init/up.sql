CREATE TABLE IF NOT EXISTS pages (
    uuid varchar(255) PRIMARY KEY,
    page_name varchar(500) NOT NULL,
    page_url varchar(255) NOT NULL,
    page_title varchar(500) NOT NULL,
    time_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE module_category (
    uuid varchar(255) PRIMARY KEY,
    page_uuid varchar(255) NOT NULL,
    title varchar(255) NOT NULL,
    FOREIGN KEY (page_uuid) REFERENCES pages(uuid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS modules (
    uuid varchar(255) PRIMARY KEY,
    page_uuid VARCHAR(255) NOT NULL,
    category_uuid VARCHAR(255),
    title varchar(255) NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (page_uuid) REFERENCES pages(uuid) ON DELETE CASCADE,
    FOREIGN KEY (category_uuid) REFERENCES module_category(uuid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS users (
    uuid varchar(255) PRIMARY KEY,
    username varchar(255) NOT NULL UNIQUE,
    password varchar(255) NOT NULL,
    token varchar(511)
);

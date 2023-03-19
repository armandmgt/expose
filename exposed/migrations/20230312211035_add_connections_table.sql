CREATE TABLE connections
(
    id            uuid PRIMARY KEY,
    subdomain     TEXT NOT NULL,
    proxied_port  TEXT NOT NULL,
    proxy_port    TEXT,
    upstream_port TEXT
)


-- Your SQL goes here
CREATE TABLE IF NOT EXISTS items (
    item_id BIGSERIAL PRIMARY KEY,
    item_name VARCHAR NOT NULL
);

INSERT INTO items
(item_name)
VALUES
('Pasta'),
('Pizza'),
('Hamburger'),
('Curry'),
('Spaghetti'),
('Steak'),
('Ice cream'),
('Juice'),
('Chai');

CREATE TABLE IF NOT EXISTS tables (
    table_id BIGSERIAL PRIMARY KEY,
    is_free BOOLEAN DEFAULT True NOT NULL
);

INSERT INTO tables
(is_free)
VALUES
(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),
(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),
(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),
(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),
(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),
(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),
(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true),(true);

CREATE TABLE IF NOT EXISTS table_orders (
    order_id BIGSERIAL PRIMARY KEY,
    table_id BIGSERIAL REFERENCES tables(table_id) ON DELETE SET NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc')
);

CREATE TYPE ITEM_STATUS_ENUM AS ENUM ('Preparing', 'Served', 'Canceled');

CREATE TABLE IF NOT EXISTS table_order_items (
    id BIGSERIAL PRIMARY KEY,
    item_id BIGSERIAL REFERENCES items(item_id) ON DELETE SET NULL,
    table_id BIGSERIAL REFERENCES tables(table_id) ON DELETE SET NULL,
    item_status ITEM_STATUS_ENUM NOT NULL DEFAULT 'Preparing',
    prep_time INTEGER DEFAULT 15 NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc')
);

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
    table_id BIGINT REFERENCES tables(table_id) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc')
);

CREATE TABLE IF NOT EXISTS item_status (
    status_id BIGSERIAL PRIMARY KEY,
    status VARCHAR
);

INSERT INTO item_status
(status)
VALUES
('Preparing'), ('Served'), ('Canceled');


CREATE TABLE IF NOT EXISTS table_order_items (
    id BIGSERIAL PRIMARY KEY,
    item_id BIGSERIAL REFERENCES items(item_id) NOT NULL,
    order_id BIGINT REFERENCES table_orders(order_id) NOT NULL,
    item_status_id BIGINT DEFAULT 1 REFERENCES item_status(status_id) NOT NULL,
    prep_time INTEGER DEFAULT 15 NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc')
);

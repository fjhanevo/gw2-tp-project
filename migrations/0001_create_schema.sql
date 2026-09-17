-- for v2/items/
CREATE TABLE items (
    item_id      INTEGER PRIMARY KEY,
    name         TEXT NOT NULL,
    rarity       TEXT NOT NULL,
    vendor_value INTEGER NOT NULL,
    icon         TEXT,
    udpated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- one row per item poll, from /v2/commerce/prices
CREATE TABLE price_snapshots (
    id            BIGSERIAL PRIMARY KEY,
    item_id       INTEGER NOT NULL REFERENCES items(item_id),
    captured_at   TIMESTAMPTZ NOT NULL,
    buy_price     INTEGER NOT NULL,
    buy_quantity  INTEGER NOT NULL,
    sell_price    INTEGER NOT NULL,
    sell_quantity INTEGER NOT NULL
);

CREATE INDEX idx_price_snapshots_item_time ON price_snapshots (item_id, captured_at);

-- full order book for the watched items, from /v2/commerce/listings
CREATE TABLE listing_depth (
    id          BIGSERIAL PRIMARY KEY,
    item_id     INTEGER NOT NULL REFERENCES items(item_id),
    captured_at TIMESTAMPTZ NOT NULL,
    side TEXT   NOT NULL CHECK (side IN ('buy', 'sell')),
    unit_price  INTEGER NOT NULL,
    quantity    INTEGER NOT NULL,
    listings    INTEGER NOT NULL
);

CREATE INDEX idx_listing_depth_item_time ON listing_depth (item_id, captured_at);

CREATE TABLE accounts (
    account_id  BIGSERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    wallet      BIGINT NOT NULL,    -- copper for the Money(u64) struct
    strategy    TEXT NOT NULL      -- which trading strategy it uses
);

CREATE TABLE orders (
    order_id    BIGSERIAL PRIMARY KEY,
    account_id  BIGINT NOT NULL REFERENCES accounts(account_id),
    item_id     INTEGER NOT NULL REFERECNES items(item_id),
    side TEXT   NOT NULL CHECK (side IN ('buy', 'sell')),
    unit_price  INTEGER NOT NULL,
    quantity    INTEGER NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'filled', 'cancelled')),
    placed_at   TIMESTAMPTZ NOT NULL,
    resolved_at TIMESTAMPTZ
);

CREATE INDEX idx_orders_account ON orders (account_id, status);

-- Your SQL goes here
CREATE TABLE currencies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL UNIQUE,
    symbol TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO currencies (name, symbol) VALUES 
('United States Dollar', 'USD'), 
('Bitcoin', 'BTC'), 
('Ethereum', 'ETH');

CREATE TABLE prices (
    external_id TEXT NOT NULL,
    source TEXT NOT NULL,
    currency_symbol TEXT NOT NULL REFERENCES currencies (symbol),
    amount NUMERIC NOT NULL,
    recorded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (recorded_at, external_id, source, currency_symbol)
);

INSERT INTO prices (recorded_at, external_id, source, currency_symbol, amount) VALUES 
('2023-06-04', '75337', 'LEGO', 'USD', 139.99), 
('2023-06-04', '75192', 'LEGO', 'USD', 949.99), 
('2023-06-04', '75345', 'LEGO', 'USD', 19.99),
('2023-06-04', '910001-1', 'Bricklink', 'USD', 463.71);
-- Your SQL goes here
CREATE TABLE lots (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users (id),
    category TEXT NOT NULL,
    condition TEXT NOT NULL,
    tag TEXT,
    description TEXT,
    images JSONB, 
    data JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
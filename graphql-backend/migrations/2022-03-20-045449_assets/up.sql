CREATE TABLE assets (
  -- an asset can be a set, a piece, a minifig, a custom part, etc
  -- the id can be the lego set number, the lego piece number, the custom part id from a 3rd party, etc
  id VARCHAR(255) NOT NULL UNIQUE PRIMARY KEY,
  title text NOT NULL,
  description text NOT NULL,
  image_url text NOT NULL,
  -- json blob: piece count, piece list, other useful stuff in here
  meta_data jsonb,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE portfolios (
  user_id UUID NOT NULL REFERENCES users(id),
  lego_set_id VARCHAR(255) NOT NULL REFERENCES assets(id),
  PRIMARY KEY (user_id, lego_set_id),
  quantity integer NOT NULL,
  condition VARCHAR(16) NOT NULL,
  -- stuff user specific data in here like missing pieces, notes, etc 
  meta_data jsonb NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
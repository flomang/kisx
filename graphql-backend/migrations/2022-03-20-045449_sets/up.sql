
CREATE TABLE lego_set (
  id VARCHAR(255) NOT NULL UNIQUE PRIMARY KEY,
  title text NOT NULL,
  description text NOT NULL,
  image_url text NOT NULL,
  -- stuff piece count data, date issued, other useful stuff in here
  meta_data jsonb NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);

CREATE TABLE user_lego_set (
  user_id UUID NOT NULL REFERENCES users(id),
  lego_set_id VARCHAR(255) NOT NULL REFERENCES lego_set(id),
  PRIMARY KEY (user_id, lego_set_id),
  quantity integer NOT NULL,
  condition VARCHAR(16) NOT NULL,
  -- stuff user specific data in here like missing pieces, notes, etc 
  meta_data jsonb NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);
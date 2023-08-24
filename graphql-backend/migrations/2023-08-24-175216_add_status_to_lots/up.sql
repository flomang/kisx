-- Your SQL goes here
CREATE TABLE lot_statuses (
    description_id TEXT PRIMARY KEY NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO lot_statuses (description_id) VALUES ('deleted'), ('drafted'), ('pending sale'), ('actively listed'), ('sold'), ('cancelled');

ALTER TABLE lots ADD COLUMN status TEXT REFERENCES lot_statuses(description_id) default 'drafted' NOT NULL;

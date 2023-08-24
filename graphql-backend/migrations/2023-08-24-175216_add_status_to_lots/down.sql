-- This file should undo anything in `up.sql`
alter table lots drop column status;
drop table lot_statuses;
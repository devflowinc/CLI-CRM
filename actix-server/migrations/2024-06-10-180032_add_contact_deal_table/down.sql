-- This file should undo anything in `up.sql`
DROP TRIGGER "deal_contacts_updated_at" ON "deal_contacts";
DROP TABLE "deal_contacts";

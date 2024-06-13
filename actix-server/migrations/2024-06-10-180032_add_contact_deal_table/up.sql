-- Your SQL goes here
CREATE TABLE "deal_contacts" (
	"id" UUID PRIMARY KEY,
	"deal_id" UUID NOT NULL,
	"contact_id" UUID NOT NULL,
	"created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY ("deal_id") REFERENCES "deals" ("id") ON DELETE CASCADE,
	FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id") ON DELETE CASCADE,
	UNIQUE ("deal_id", "contact_id")
);

CREATE TRIGGER "deal_contacts_updated_at"
  BEFORE UPDATE ON "deal_contacts"
  FOR EACH ROW
  EXECUTE FUNCTION update_modified_column();

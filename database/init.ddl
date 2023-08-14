CREATE TABLE IF NOT EXISTS "users" (
    "id"           uuid DEFAULT uuid_generate_v4(),
    "email"        text NOT NULL UNIQUE,
    "password"     text,
    "salt"         uuid UNIQUE DEFAULT uuid_generate_v4(),
    "name"         text NOT NULL,
    "birthday"     timestamptz DEFAULT null,
    "location"     text DEFAULT null,
    "status"       bigint NOT NULL,
    "created_at"   timestamptz NOT NULL,
    "updated_at"   timestamptz NOT NULL,
    "deleted_at"   timestamptz DEFAULT null,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "organizations" (
    "id"           uuid DEFAULT uuid_generate_v4(),
    "name"         text,
    "owner_id"     uuid,
    "created_at"   timestamptz NOT NULL,
    "updated_at"   timestamptz NOT NULL,
    "deleted_at"   timestamptz DEFAULT null,
    PRIMARY KEY ("id"),
    CONSTRAINT "fk_organizations_owner" FOREIGN KEY ("owner_id") REFERENCES "users"("id")
);

CREATE TABLE IF NOT EXISTS "roles" (
    "id"           uuid DEFAULT uuid_generate_v4(),
    "name"         text,
    "org_id"       uuid,
    "created_at"   timestamptz NOT NULL,
    "updated_at"   timestamptz NOT NULL,
    "deleted_at"   timestamptz DEFAULT null,
    PRIMARY KEY ("id"),
    CONSTRAINT "fk_orgs_defined_roles" FOREIGN KEY ("org_id") REFERENCES "organizations"("id")
);

CREATE TABLE IF NOT EXISTS "role_users" (
    "role_id"   uuid DEFAULT uuid_generate_v4(),
    "user_id"   uuid DEFAULT uuid_generate_v4(),
    PRIMARY KEY ("role_id","user_id"),
    CONSTRAINT "fk_role_users_role" FOREIGN KEY ("role_id") REFERENCES "roles"("id"),
    CONSTRAINT "fk_role_users_user" FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

CREATE TABLE IF NOT EXISTS "organization_users" (
    "org_id"   text,
    "user_id"           text,
    PRIMARY KEY ("org_id","user_id")
);



CREATE TABLE IF NOT EXISTS "boards" (
    "id"              uuid DEFAULT uuid_generate_v4(),
    "title"           text NOT NULL,
    "owner_org_id"   uuid,
    "created_at"      timestamptz NOT NULL,
    "updated_at"      timestamptz NOT NULL,
    "deleted_at"      timestamptz DEFAULT null,
    PRIMARY KEY ("id"),
    CONSTRAINT "fk_boards_owner_org" FOREIGN KEY ("owner_org_id") REFERENCES "organizations"("id")
);

CREATE TABLE IF NOT EXISTS "panels" (
    "id"           uuid DEFAULT uuid_generate_v4(),
    "title"        text NOT NULL,
    "board_id"     uuid,
    "created_at"   timestamptz NOT NULL,
    "updated_at"   timestamptz NOT NULL,
    "deleted_at"   timestamptz DEFAULT null,
    PRIMARY KEY ("id"),
    CONSTRAINT "fk_boards_panels" FOREIGN KEY ("board_id") REFERENCES "boards"("id")
);

CREATE TABLE IF NOT EXISTS "cards" (
    "id"            uuid DEFAULT uuid_generate_v4(),
    "title"         text NOT NULL,
    "description"   text,
    "panel_id"      uuid,
    "created_at"    timestamptz NOT NULL,
    "updated_at"    timestamptz NOT NULL,
    "deleted_at"    timestamptz DEFAULT null,
    PRIMARY KEY ("id"),
    CONSTRAINT "fk_panels_cards" FOREIGN KEY ("panel_id") REFERENCES "panels"("id")
);

CREATE TABLE IF NOT EXISTS "tags" (
    "id"           uuid DEFAULT uuid_generate_v4(),
    "name"         text NOT NULL,
    "board_id"     uuid,
    "created_at"   timestamptz NOT NULL,
    "updated_at"   timestamptz NOT NULL,
    "deleted_at"   timestamptz DEFAULT null,
    PRIMARY KEY ("id"),
    CONSTRAINT "fk_boards_defined_tags" FOREIGN KEY ("board_id") REFERENCES "boards"("id")
);

CREATE TABLE IF NOT EXISTS "card_tags" (
    "card_id"   text,
    "tag_id"    text,
    PRIMARY KEY ("card_id","tag_id")
);

CREATE TABLE IF NOT EXISTS "comments" (
    "id"           uuid DEFAULT uuid_generate_v4(),
    "content"      text NOT NULL,
    "user_id"      uuid,
    "card_id"      uuid,
    "created_at"   timestamptz NOT NULL,
    "updated_at"   timestamptz NOT NULL,
    "deleted_at"   timestamptz DEFAULT null,
    PRIMARY KEY ("id"),
    CONSTRAINT "fk_cards_comments" FOREIGN KEY ("card_id") REFERENCES "cards"("id"),
    CONSTRAINT "fk_comments_user" FOREIGN KEY ("user_id") REFERENCES "users"("id")
);
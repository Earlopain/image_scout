CREATE TABLE artists (
	id bigserial NOT NULL,
	"name" varchar(50) NOT NULL,
	CONSTRAINT idx_16585_primary PRIMARY KEY (id)
);
CREATE UNIQUE INDEX idx_16585_artists_un ON artists USING btree (name);

CREATE TABLE artist_aliases (
	id bigserial NOT NULL,
	artist_id int8 NOT NULL,
	alias varchar(50) NOT NULL,
	CONSTRAINT idx_16591_primary PRIMARY KEY (id)
);
CREATE INDEX idx_16591_artist_aliases_fk ON artist_aliases USING btree (artist_id);
ALTER TABLE artist_aliases ADD CONSTRAINT artist_aliases_fk FOREIGN KEY (artist_id) REFERENCES artists(id) ON UPDATE RESTRICT ON DELETE CASCADE;

CREATE TABLE page_types (
	id bigserial NOT NULL,
	"name" varchar(50) NOT NULL,
	regex varchar(100) NOT NULL,
	CONSTRAINT idx_16613_primary PRIMARY KEY (id)
);
CREATE UNIQUE INDEX idx_16613_page_types_un ON page_types USING btree (name);

CREATE TABLE artist_pages (
	id bigserial NOT NULL,
	artist_id int8 NOT NULL,
	page_type int8 NOT NULL,
	url varchar(100) NOT NULL,
	added_at timestamptz NOT NULL,
	last_update timestamptz NOT NULL,
	active bool NOT NULL,
	CONSTRAINT idx_16597_primary PRIMARY KEY (id)
);
CREATE INDEX idx_16597_artists_pages_fk ON artist_pages USING btree (artist_id);
CREATE INDEX idx_16597_artists_pages_fk_1 ON artist_pages USING btree (page_type);
ALTER TABLE artist_pages ADD CONSTRAINT artists_pages_fk FOREIGN KEY (artist_id) REFERENCES artists(id) ON UPDATE RESTRICT ON DELETE CASCADE;
ALTER TABLE artist_pages ADD CONSTRAINT artists_pages_fk_1 FOREIGN KEY (page_type) REFERENCES page_types(id) ON UPDATE RESTRICT ON DELETE CASCADE;

CREATE TABLE artist_posts (
	id bigserial NOT NULL,
	artist_id int8 NOT NULL,
	page_type int8 NOT NULL,
	source_url varchar(255) NOT NULL,
	direct_url varchar(255) NULL DEFAULT NULL::character varying,
	file_name varchar(100) NOT NULL,
	"blob" bytea NOT NULL,
	"thumb" bytea NOT NULL,
	width int8 NOT NULL,
	height int8 NOT NULL,
	perceptual_hash bytea NOT NULL,
	file_type varchar(3) NOT NULL,
	created_at timestamptz NOT NULL,
	CONSTRAINT idx_16603_primary PRIMARY KEY (id)
);
CREATE INDEX idx_16603_artist_posts_fk ON artist_posts USING btree (artist_id);
CREATE INDEX idx_16603_artist_posts_fk_1 ON artist_posts USING btree (page_type);
ALTER TABLE artist_posts ADD CONSTRAINT artist_posts_fk FOREIGN KEY (artist_id) REFERENCES artists(id) ON UPDATE RESTRICT ON DELETE CASCADE;
ALTER TABLE artist_posts ADD CONSTRAINT artist_posts_fk_1 FOREIGN KEY (page_type) REFERENCES page_types(id) ON UPDATE RESTRICT ON DELETE CASCADE;

INSERT INTO page_types ("name",regex) VALUES 
('Twitter','^https:\/\/twitter\.com\/[a-zA-Z0-9_]{1,15}\/$'),
('FurAffinity','^https:\/\/www\.furaffinity\.net\/user\/[a-zA-Z0-9-_~.]{1,30}\/$');

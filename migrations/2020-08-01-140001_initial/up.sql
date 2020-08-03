CREATE TABLE artists (
	id INTEGER UNSIGNED auto_increment NOT NULL,
	name varchar(50) NOT NULL,
	CONSTRAINT artists_PK PRIMARY KEY (id),
	CONSTRAINT artists_UN UNIQUE KEY (name)
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

CREATE TABLE artist_aliases (
	id INTEGER UNSIGNED auto_increment NOT NULL,
	artist_id INTEGER UNSIGNED NOT NULL,
	alias varchar(50) NOT NULL,
	CONSTRAINT artist_aliases_PK PRIMARY KEY (id),
	CONSTRAINT artist_aliases_FK FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

CREATE TABLE page_types (
	id INTEGER UNSIGNED auto_increment NOT NULL,
	name VARCHAR(50) NOT NULL,
	regex varchar(100) NOT NULL,
	CONSTRAINT page_types_PK PRIMARY KEY (id),
	CONSTRAINT page_types_UN UNIQUE KEY (name)
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

CREATE TABLE artist_pages (
	id INTEGER UNSIGNED auto_increment NOT NULL,
	artist_id INTEGER UNSIGNED NOT NULL,
	url varchar(100) NOT NULL,
	page_type INTEGER UNSIGNED NOT NULL,
	added_at TIMESTAMP NOT NULL,
	last_update TIMESTAMP NOT NULL,
	active BOOL NOT NULL,
	CONSTRAINT artists_pages_PK PRIMARY KEY (id),
	CONSTRAINT artists_pages_FK FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE,
	CONSTRAINT artists_pages_FK_1 FOREIGN KEY (page_type) REFERENCES page_types(id) ON DELETE CASCADE
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

CREATE TABLE images (
	id INTEGER UNSIGNED auto_increment NOT NULL,
	`blob` LONGBLOB NOT NULL,
	width INTEGER UNSIGNED NOT NULL,
	height INTEGER UNSIGNED NOT NULL,
	perceptual_hash BINARY(32) NOT NULL,
	file_type varchar(3) NOT NULL,
	CONSTRAINT images_PK PRIMARY KEY (id)
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

CREATE TABLE artist_posts (
	id INTEGER UNSIGNED auto_increment NOT NULL,
	artist_id INTEGER UNSIGNED NOT NULL,
	page_type INTEGER UNSIGNED NOT NULL,
	image_id INTEGER UNSIGNED NOT NULL,
	source_url varchar(255) NOT NULL,
	direct_url varchar(255) NULL,
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT artist_posts_PK PRIMARY KEY (id),
	CONSTRAINT artist_posts_FK FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE,
	CONSTRAINT artist_posts_FK_1 FOREIGN KEY (image_id) REFERENCES images(id) ON DELETE CASCADE,
	CONSTRAINT artist_posts_FK_2 FOREIGN KEY (page_type) REFERENCES page_types(id) ON DELETE CASCADE
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

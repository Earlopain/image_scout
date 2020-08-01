CREATE TABLE artist (
	id INTEGER auto_increment NOT NULL,
	name varchar(50) NOT NULL,
	CONSTRAINT artist_PK PRIMARY KEY (id),
	CONSTRAINT artist_UN UNIQUE KEY (name)
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

CREATE TABLE artist_alias (
	id INTEGER auto_increment NOT NULL,
	artist_id INTEGER NOT NULL,
	artist_alias varchar(50) NOT NULL,
	CONSTRAINT artist_alias_PK PRIMARY KEY (id),
	CONSTRAINT artist_alias_FK FOREIGN KEY (artist_id) REFERENCES reverser.artist(id) ON DELETE CASCADE
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

CREATE TABLE page_type (
	id INTEGER auto_increment NOT NULL,
	name VARCHAR(50) NOT NULL,
	regex varchar(100) NOT NULL,
	CONSTRAINT page_type_PK PRIMARY KEY (id),
	CONSTRAINT page_type_UN UNIQUE KEY (name)
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

CREATE TABLE artist_page (
	id INTEGER auto_increment NOT NULL,
	artist_id INTEGER NOT NULL,
	url varchar(100) NOT NULL,
	page_type INTEGER NOT NULL,
	added_at TIMESTAMP NOT NULL,
	last_update TIMESTAMP NOT NULL,
	active BOOL NOT NULL,
	CONSTRAINT artist_page_PK PRIMARY KEY (id),
	CONSTRAINT artist_page_FK FOREIGN KEY (artist_id) REFERENCES reverser.artist(id) ON DELETE CASCADE,
	CONSTRAINT artist_page_FK_1 FOREIGN KEY (page_type) REFERENCES reverser.page_type(id) ON DELETE CASCADE
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

CREATE TABLE image (
	id INTEGER auto_increment NOT NULL,
	`blob` LONGBLOB NOT NULL,
	width INTEGER NOT NULL,
	height INTEGER NOT NULL,
	perceptual_hash BINARY(32) NOT NULL,
	file_type varchar(3) NOT NULL,
	CONSTRAINT image_PK PRIMARY KEY (id)
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

CREATE TABLE artist_post (
	id INTEGER auto_increment NOT NULL,
	artist_id integer NOT NULL,
	image_id INTEGER NOT NULL,
	source_url varchar(255) NOT NULL,
	direct_url varchar(255) NULL,
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT artist_post_PK PRIMARY KEY (id),
	CONSTRAINT artist_post_FK FOREIGN KEY (artist_id) REFERENCES reverser.artist(id),
	CONSTRAINT artist_post_FK_1 FOREIGN KEY (image_id) REFERENCES reverser.image(id)
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

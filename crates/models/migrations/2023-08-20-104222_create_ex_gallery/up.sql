CREATE TABLE IF NOT EXISTS ex_gallery (
    gid INT NOT NULL PRIMARY KEY,
    token TEXT NOT NULL,
    title TEXT NOT NULL,
    title_jpn TEXT,
    category INT NOT NULL,
    rating FLOAT NOT NULL,
    languages INT[] NOT NULL, -- foreign key references languages

    -- tags
    female INT[] NOT NULL, -- foreign key references tags
    male INT[] NOT NULL, -- foreign key references tags
    mixed INT[] NOT NULL, -- foreign key references tags
    other INT[] NOT NULL, -- foreign key references tags
    rest INT[] NOT NULL, -- foreign key references tags

    -- users
    artists INT[] NOT NULL, -- foreign key references users
    groups INT[] NOT NULL, -- foreign key references users
    cosplayers INT[] NOT NULL, -- foreign key references users
    uploader INT, -- foreign key references users
    disowned BOOLEAN NOT NULL,

    -- reference
    parent_gid INT, -- foreign key references ex_gallery
    first_gid INT, -- foreign key references ex_gallery
    parodies INT[] NOT NULL, -- foreign key references parodies
    characters INT[] NOT NULL, -- foreign key references characters


    -- files
    thumb text NOT NULL,
    filesize INT NOT NULL,
    filecount INT NOT NULL,

    -- torrents
    torrentcount INT NOT NULL,
    torrents text NOT NULL,

    -- state
    removed INT, -- foreign key references failed
    expunged BOOLEAN NOT NULL,

    -- timestamps
    posted INT NOT NULL,
    dumped INT
)

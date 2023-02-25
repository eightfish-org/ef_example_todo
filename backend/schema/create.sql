CREATE TABLE todo (
        id varchar PRIMARY KEY, 
        title varchar(80) NOT NULL,
        completed  boolean NOT NULL
);
CREATE TABLE todo_idhash (
        id varchar PRIMARY KEY,
        hash varchar NOT NULL
);

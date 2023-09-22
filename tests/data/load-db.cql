TRACING ON;

CREATE KEYSPACE k 
WITH REPLICATION = { 
    'class' : 'NetworkTopologyStrategy', 
    'datacenter1' : 1 
};

CREATE TABLE k.t (
    pk int,
    t int,
    v text,
    s text,
    PRIMARY KEY (pk, t)
);

INSERT INTO k.t (pk, t, v, s) VALUES (0, 0, 'val0', 'static0');

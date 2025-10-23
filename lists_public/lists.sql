CREATE sequence increment_seq
    start 1
    increment 1
    MINVALUE 1
    NO MAXVALUE
    CACHE 1;

CREATE TABLE test(
    index int NOT NULL,
    item VARCHAR(255) NOT NULL,
    amount DECIMAL,
    notes TEXT
);

-- CREATE TABLE shopping(
--     index INT NOT NULL, 
--     item VARCHAR(255) NOT NULL,
--     amount DECIMAL NOT NULL,

-- INSERT INTO shopping (index, item, amount, notes) VALUES(
--     NEXTVAL('increment_seq'),
--     'Water',
--     2,
--     ''
-- );
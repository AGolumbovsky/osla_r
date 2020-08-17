-- Your SQL goes here
CREATE TABLE test_osla_diesel (

  id SERIAL PRIMARY KEY,
  word VARCHAR NOT NULL,
  part_of_speech TEXT NOT NULL,
  meaning TEXT NOT NULL,
  approved BOOLEAN NOT NULL DEFAULT 'f'
  
)
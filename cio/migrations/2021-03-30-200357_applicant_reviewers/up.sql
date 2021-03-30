CREATE TABLE applicant_reviewers (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    evaluations INTEGER NOT NULL DEFAULT 0,
    emphatic_yes INTEGER NOT NULL DEFAULT 0,
    yes INTEGER NOT NULL DEFAULT 0,
    pass INTEGER NOT NULL DEFAULT 0,
    no INTEGER NOT NULL DEFAULT 0,
    not_applicable INTEGER NOT NULL DEFAULT 0,
    airtable_record_id VARCHAR NOT NULL DEFAULT ''
)

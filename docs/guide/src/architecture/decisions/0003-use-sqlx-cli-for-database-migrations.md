# 3. Use sqlx cli for database migrations

Date: 2023-07-09

## Status

Accepted

## Context

A mechanism for database migrations must be determined.

## Decision

sqlx-cli will be used for database migrations.

## Consequences

- Migrations must be written by hand, instead of using an ORM such as Prisma.
- The migration files used by sqlx-cli may not be database agnostic between mysql and postgres, so there may potentially be coupling between the migration files and the database.

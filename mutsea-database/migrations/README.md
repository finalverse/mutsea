# Database Schema Organization

This directory contains raw SQL schema files organized by backend. These files are
intended for use with the migration system.

- `postgresql/` contains PostgreSQL specific schemas.
  - `ai/` – tables powering AI and machine learning features.
  - `world/` – environmental and simulation state tables.
  - `gameplay/` – player-centric gameplay tables.
  - `analytics/` – performance and monitoring tables.
  - `opensim/` – legacy OpenSim compatible schema.

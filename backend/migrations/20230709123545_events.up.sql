CREATE TABLE "events" (
  "aggregate_type" TEXT NOT NULL,
  "aggregate_id" TEXT NOT NULL,
  "sequence" BIGSERIAL NOT NULL,
  "event_type" TEXT NOT NULL,
  "event_version" TEXT NOT NULL,
  "payload" JSON NOT NULL,
  "metadata" JSON NOT NULL,
  "createdAt" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updatedAt" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY ("sequence", "aggregate_type", "aggregate_id")
);
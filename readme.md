# IceDust

IceDust is an ID generator inspired by Ulid and Twitter Snowflake.

Every IceDust ID is a 64-bit unsigned integer, and is composed of three parts: `Timestamp`, `Machine ID` and `Random`. The three parts can be configured to have different lengths.

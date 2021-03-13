# sfi-core

The core library for sfi (short for Shared Food Inventory), which manages shared resources with associated expiry dates, optimized for occasionally connected computing scenarios.

(todo / work in progress)

## Strategy

- Every inventory has its own event log
  - Needed because every inventory can be stored on a separate server
- One event log per inventory
  - Other types (thing, unit, place & tag) share the event log
  - Places all types in the same scope

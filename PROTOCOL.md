# Wire protocol v1

All integers are little-endian. A stream is a sequence of messages:

| Field   | Type | Notes          |
|---------|------|----------------|
| type    | u8   |                |
| length  | u32  | payload bytes  |
| payload |      |                |

## Cell

One u16: bits 15-12 red, 11-8 green, 7-4 blue, 3-0 glyph.

A channel value `v` (0-15) is displayed as `v * 16 + 8`. The glyph is an index
into ` .:-=+*#%@`.

## Messages

- `0x01` HELLO: version u8 (1), cols u16, rows u16, fps numerator u16,
  fps denominator u16. First message of every stream.
- `0x02` KEY: `rows * cols` cells, row by row. Replaces the whole screen.
- `0x03` DELTA: runs until the end of the payload. A run is start u16
  (`y * cols + x`), count u16, then `count` cells.

The server sends a KEY to each client before any DELTA.

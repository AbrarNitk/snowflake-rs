# Snowflake: Unique ID generator

This repository contains code related to unique ID generator written in Rust.


## Next Achievements

- [] separate the service from library and use hyper to serve or axum
- [] removal of the lazy_static at all
- [] lock free concurrency, usage of the Atomic Primitive
- [] Divide the node-address in to the data-center and the machine-id
- [] node-address can be fallback to ip-address if not provided



## Snowflake ID Structure

- From left to right bits order: 64 bits

- 1 Bit: unused bit, keeping as 0
- 41 Bits: timestamp in millis
- 5 Bits: Data center ID
- 5 Bits: Machine ID
- 12 Bits: Sequence(Generated atomically within timestamp)



## References

- [X Twitter Blog](https://blog.x.com/engineering/en_us/a/2010/announcing-snowflake)

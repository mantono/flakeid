# flakeid
Rust implementation for generation of [Flake ID](https://github.com/boundary/flake).

See the [examples](examples/) folder for a demonstration of how this library can be used.

<!--

# Flake ID Generator

## What is a Flake ID?

A **flake ID** is a unique identifier designed to be fast to generate, monotonic (in terms of timestamp), and distributed, making it suitable for environments like distributed systems. The main characteristics of flake IDs are:

1. **Uniqueness**: Each flake ID is guaranteed to be unique, often achieved through the incorporation of elements like timestamps, machine identifiers, and sequence numbers.
2. **Sortability**: Since part of the flake ID is derived from the timestamp, flake IDs generated in the same time frame will have a consistent order, making it easier to sort and organize records.
3. **Compactness**: Flake IDs are typically represented as short, fixed-size strings or numerical values, optimizing storage and network transmission.

Unlike traditional formats like UUIDs (Universally Unique Identifiers), flake IDs are usually smaller in size, making them preferable in environments where performance is critical.

## When Would You Want to Use a Flake ID?

You might consider using flake IDs in the following scenarios:

1. **Distributed Systems**: If you're operating in a distributed environment with multiple nodes generating IDs, flake IDs can help ensure unique identifiers without a central coordinating service.
2. **High Throughput Demand**: When you require a system capable of generating a large number of unique IDs quickly, flake IDs can be efficiently created each with minimal computational overhead.
3. **Time-Oriented Applications**: In applications where the creation time of the ID is significant (e.g., event logging or messaging systems), flake IDs allow for easy chronological sorting of records.
4. **Database Indexing**: Flake IDs can be advantageous when used as primary keys in databases, due to their shorter length compared to other unique identification methods, improving indexing performance.

## When Would You Not Want to Use a Flake ID?

While flake IDs have many advantages, there are certain cases where they may not be the best choice:

1. **Global Identifier Requirements**: If you require a globally unique identifier across completely independent systems (without a common timestamp or machine information), consider using UUIDs or other schemes that ensure widespread uniqueness without reliance on structure.
2. **Complexity of Implementation**: If your application does not need the benefits of flake IDs, you might prefer simpler or built-in ID generation methods. The added complexity of managing timestamps and sequences may not justify the performance gains.
3. **Security Concerns**: Flake IDs are relatively predictable since they often contain timestamps. If you require obscured identifiers for security-sensitive applications (e.g., preventing enumeration attacks), consider using UUIDs or cryptographically secure random identifiers.
4. **Legacy Systems**: If you are working with existing databases or systems that use a different ID generation method, introducing flake IDs may complicate integration and data migration.

By weighing these considerations, you can choose the most suitable ID generation strategy for your application.
-->

# B2B Holidays Technical Assessment

This repository contains the solution for the B2B Holidays Senior Rust Developer technical assessment. It provides the structure, interfaces, and working examples for the two assessment tasks.

## Assessment Overview

1. **Hotel Availability Cache** (Moderate Difficulty): Implement a thread-safe, high-performance cache for hotel availability data
2. **XML Processing** (Entry Level): Implement an efficient XML processor for hotel search responses

This graduated approach reflects our system architecture where some components face more demanding requirements than others.

## Getting Started

1. Clone this repository
2. Install Rust (if not already installed) using [rustup](https://rustup.rs/)
3. Build the project to verify everything works:
   ```
   cargo build
   ```
4. Run the tests to make sure the environment is set up correctly:
   ```bash
   cargo test
   ```
5. (Optional) Run the benchmark to see example performance:
   ```bash
   cargo bench
   ```

## Project Structure

### Assessment Tasks
- `src/part1_cache.rs`: Hotel Availability Cache implementation task
- `src/part2_xml.rs`: XML Processing implementation task

### Supporting Files
- `benches/cache_benchmark.rs`: Performance benchmark for cache implementations
- `samples/`: Sample data files for testing (JSON and XML formats)

## Implementation

### Part 1: Hotel Availability Cache

- Be thread-safe for concurrent access with minimal lock contention
- Enforce memory usage limits with customizable eviction policies
- Implement TTL-based expiration with efficient cleanup
- Prioritize frequently accessed items using a carefully chosen algorithm
- Track and report detailed cache statistics
- Handle cache stampedes for popular items
- Optimize for read-heavy workloads with infrequent writes

The tests cover:
- Basic operations (store, get)
- Concurrent access behavior with high contention scenarios
- Expiration handling with clock manipulation
- Memory limit enforcement with large items
- Cache eviction strategies under pressure
- Performance characteristics under various access patterns

Further improvements:
- Split the CacheEntry to two parts Data and DataAccessStatistics
- Switch from std::sync::Mutex to the tokio::sync::RwLock and async

### Part 2: XML Processing and JSON Conversion

- Convert supplier JSON responses to XML format for client consumption
- Parse and extract key information from XML hotel search responses
- Filter hotel options based on various criteria
- Handle moderate-sized documents efficiently

The tests cover:
- JSON to XML conversion accuracy
- XML parsing and extraction
- Filter criteria application
- Basic performance with standard documents

Sample files are provided for testing:
- `samples/supplier_response.json`: Sample JSON response from suppliers
- `samples/hotel_search_response.xml`: Sample XML response for clients
- `samples/hotel_search_request.xml`: Sample XML request

Further improvements:
- The XML parsing speed is good, but it can be faster if split the xml string input with <hotel> tag and parallel parsing them with more threads

# B2B Holidays Technical Assessment

This repository contains the starter template for the B2B Holidays Senior Rust Developer technical assessment. It provides the structure, interfaces, and working examples for the three assessment tasks.

**✅ Ready to Use**: This template compiles cleanly, all tests pass, and includes working example implementations to guide your development.

## Assessment Overview

This assessment evaluates your proficiency with Rust and related technologies used in our high-performance travel technology platform. You should expect to spend approximately 8-12 hours to complete all parts thoroughly. Focus on code quality, performance, and proper error handling.

The assessment consists of three parts with graduated difficulty levels:

1. **Hotel Availability Cache** (Moderate Difficulty): Implement a thread-safe, high-performance cache for hotel availability data
2. **XML Processing** (Entry Level): Implement an efficient XML processor for hotel search responses
3. **Rate-Limited API Client** (Advanced): Implement an async rate-limited client for a hotel booking API that handles extreme traffic

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

## Quick Start

1. **Explore the examples**: Look at `src/*_example.rs` files to understand the expected patterns
2. **Run the tests**: All tests should pass out of the box - this shows you what's expected
3. **Check the sample data**: Review files in `samples/` to understand the data formats
4. **Start implementing**: Begin with the part that interests you most, using the examples as reference

## Project Structure

### Assessment Tasks
- `src/part1_cache.rs`: Hotel Availability Cache implementation task
- `src/part2_xml.rs`: XML Processing implementation task
- `src/part3_api.rs`: Rate-Limited API Client implementation task

### Example Implementations (For Reference)
- `src/part1_cache_example.rs`: Working cache example showing expected patterns
- `src/part2_xml_example.rs`: Working XML processor example
- `src/part3_api_example.rs`: Working API client example

### Supporting Files
- `benches/cache_benchmark.rs`: Performance benchmark for cache implementations
- `samples/`: Sample data files for testing (JSON and XML formats)

## Implementation Instructions

For each part of the assessment:

1. Read the trait/struct definitions and comments carefully
2. **Review the example implementations** (`*_example.rs` files) to understand expected patterns and quality
3. Implement your own optimized version that demonstrates senior-level Rust expertise
4. Add comprehensive tests to verify your implementation
5. Ensure your code is well-documented with comments

**Important**: The example implementations are provided as reference and demonstrate basic functionality. Your implementations should be more sophisticated, performant, and production-ready to showcase senior developer skills.

## Example Implementations

This starter template includes working example implementations for all three parts:

- **`ExampleCache`**: A basic thread-safe cache using `Arc<Mutex<HashMap>>` with simple TTL and size-based eviction
- **`ExampleHotelSearchProcessor`**: A working XML processor that handles the provided sample data
- **`ExampleBookingApiClient`**: A basic API client with simple rate limiting and mock responses

**Purpose of Examples:**
- Demonstrate the expected API usage and patterns
- Show how to structure tests and handle errors
- Provide a working baseline for performance comparison
- Help you understand the problem domain and requirements

**Your Task:**
Create your own implementations that surpass these examples in:
- Performance and scalability
- Sophisticated algorithms (e.g., advanced eviction policies, proper XML parsing)
- Production readiness (monitoring, resilience, optimization)
- Code quality and architecture

### Time Allocation Guidelines

To help you plan your time effectively, here's a suggested breakdown:

**Part 1: Hotel Availability Cache (Moderate Difficulty)**
- Implementation: 2-3 hours
- Testing: 1-2 hours

**Part 2: XML Processing (Entry Level)**
- Implementation: 1-2 hours
- Testing: 1 hour

**Part 3: Rate-Limited API Client (Advanced)**
- Implementation: 3-5 hours
- Testing: 1-2 hours

You may choose to focus more deeply on certain parts based on your strengths and interests. A basic implementation that meets core requirements for all parts is preferable to an exhaustive implementation of just one part.

### Part 1: Hotel Availability Cache (Moderate Difficulty)

Implement the `AvailabilityCache` trait in `part1_cache.rs`. This component serves as the middleware between our high-traffic customer-facing API and our supplier systems.

Your implementation should:
- Be thread-safe for concurrent access with minimal lock contention
- Enforce memory usage limits with customizable eviction policies
- Implement TTL-based expiration with efficient cleanup
- Prioritize frequently accessed items using a carefully chosen algorithm
- Track and report detailed cache statistics
- Handle cache stampedes for popular items
- Optimize for read-heavy workloads with infrequent writes

Your tests should cover:
- Basic operations (store, get)
- Concurrent access behavior with high contention scenarios
- Expiration handling with clock manipulation
- Memory limit enforcement with large items
- Cache eviction strategies under pressure
- Performance characteristics under various access patterns

The benchmark in `benches/cache_benchmark.rs` is already functional and uses the example implementation. You can run it with `cargo bench` to see baseline performance, then update it to use your implementation for comparison.

### Part 2: XML Processing and JSON Conversion (Entry Level)

Implement the `HotelSearchProcessor` in `src/part2_xml.rs` to efficiently process data from hotel suppliers. This is our data transformation layer between supplier systems and the cache.

Your implementation should:

1. Convert supplier JSON responses to XML format for client consumption
2. Parse and extract key information from XML hotel search responses
3. Filter hotel options based on various criteria
4. Handle moderate-sized documents efficiently

Your tests should cover:
- JSON to XML conversion accuracy
- XML parsing and extraction
- Filter criteria application
- Basic performance with standard documents

Sample files are provided for testing:
- `samples/supplier_response.json`: Sample JSON response from suppliers
- `samples/hotel_search_response.xml`: Sample XML response for clients
- `samples/hotel_search_request.xml`: Sample XML request

This reflects the actual data flow in our system where supplier data comes in JSON format and is converted to XML for customer responses.

### Part 3: Rate-Limited API Client (Advanced Difficulty)

Implement the `BookingApiClient` in `src/part3_api.rs`. This component is our customer-facing API that must handle extreme traffic while maintaining high reliability and performance.

Your implementation should:
- Enforce sophisticated rate limiting with token bucket or leaky bucket algorithms
- Implement adaptive rate limiting based on service health
- Manage concurrent requests with priority queuing and fairness guarantees
- Implement circuit breaking for failing dependencies
- Use timeouts with exponential backoff strategies
- Prioritize booking requests over search requests with preemption
- Handle retries for transient failures with jitter
- Maintain detailed telemetry for monitoring and diagnostics
- Optimize for high throughput while maintaining low latency
- Handle graceful degradation under extreme load

Your tests should cover:
- Rate limiting enforcement under burst and sustained load
- Concurrent request handling with varied request types
- Timeout behavior with slow dependencies
- Request prioritization with resource contention
- Retry logic for different failure scenarios
- Circuit breaker behavior with unhealthy dependencies
- Performance characteristics under simulated production load

## Testing Guidelines

For each component, focus on testing:
1. **Correctness**: Does your implementation behave as specified?
2. **Edge Cases**: How does it handle unusual inputs or conditions?
3. **Performance**: Is it efficient for the expected load?
4. **Concurrency**: Does it handle parallel access correctly?
5. **Resilience**: How does it behave under failure conditions?

The template includes example test stubs to get you started. You don't need exhaustive tests for every possible scenario - focus on key behaviors and requirements.

### Test Coverage Expectations

For each component, we expect:

- **Minimum Coverage**: Aim for at least 80% test coverage for your implementation
- **Unit Tests**: All public methods should have corresponding unit tests
- **Error Handling**: Tests should verify proper error handling for invalid inputs and failure scenarios

**Part 1 (Cache):**
- Tests must verify thread safety under concurrent access
- Tests must confirm memory limits are enforced correctly
- Tests must validate that eviction policies work as expected
- At least one performance test measuring throughput under load

**Part 2 (XML Processing):**
- Tests must verify correct parsing of all sample files
- Tests must validate filtering logic with various criteria
- Tests should check error handling for malformed XML
- Tests should verify JSON to XML conversion accuracy

**Part 3 (API Client):**
- Tests must verify rate limiting under various load conditions
- Tests must confirm circuit breaking behavior works correctly
- Tests must validate request prioritization and preemption
- Tests should verify retry logic with backoff and jitter
- Tests should check adaptive rate limiting based on system health

Provide appropriate mocks or test doubles where needed to isolate your tests from external dependencies.

## Running Benchmarks

To run the cache benchmark:

```bash
cargo bench
```

The benchmark is already functional and will show performance metrics for the example cache implementation. Once you implement your own cache, update the benchmark to use your implementation for performance comparison.

## Submission Guidelines

1. Create a private GitHub repository with your solution
2. Ensure your code compiles without warnings
3. Include a README.md with:
   - Setup and running instructions
   - Brief explanation of your approach for each part
   - Any assumptions or trade-offs you made
   - Ideas for further improvements (if you had more time)
4. Email the URL of your GitHub repository to: `it@b2bholidays.com` with the subject line "Rust Developer Assessment Submission"
   - If your repository is private, please make sure to add `B2B-HOLIDAYS` as a collaborator with read access

## Evaluation Criteria

Your solution will be evaluated based on:

1. **Correctness**: Does it meet the requirements and handle edge cases?
2. **Performance**: Is it optimized for throughput and resource usage?
3. **Code Quality**: Is it well-structured, documented, and maintainable?
4. **Rust Proficiency**: Does it use appropriate Rust idioms and patterns?
5. **Error Handling**: Does it handle errors gracefully and informatively?
6. **Concurrency**: Does it handle concurrent operations safely and efficiently?
7. **System Design**: Do your solutions demonstrate understanding of distributed systems principles?

Good luck! We're looking for clean, efficient solutions that demonstrate your understanding of high-performance Rust development.

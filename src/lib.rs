// Main library file for the travel tech assessment

// Export modules for each part of the assessment
pub mod part1_cache;
pub mod part2_xml;
pub mod part3_api;
pub mod part3_api_example; // Example implementation for reference
pub mod supplier;
pub mod xml_response;

// Re-export key types for convenience
pub use part1_cache::{AvailabilityCache, CacheStats};
pub use part2_xml::{
    FilterCriteria, HotelOption, HotelSearchProcessor, ProcessedResponse, ProcessingError,
};
pub use part3_api::{
    ApiClient, ApiError, BookingApiClient, ClientConfig, ClientError, ClientStats,
};
pub use xml_response::{
    XmlHotel, XmlHotels, XmlMealPlan, XmlMealPlans, XmlOption, XmlOptions, XmlProcessedResponse,
};

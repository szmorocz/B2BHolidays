// Part 2: XML Processing Implementation
use quick_xml::de::from_str;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Error types for XML processing
#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("XML parse error: {0}")]
    XmlParseError(String),

    #[error("JSON parse error: {0}")]
    JsonParseError(String),

    #[error("Missing required field: {0}")]
    MissingRequiredField(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Conversion error: {0}")]
    ConversionError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    // Add other error types as needed
    #[error("Other error: {0}")]
    Other(String),
}

// Data structures for supplier JSON response
#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierResponse {
    pub hotels: Vec<SupplierHotel>,
    pub search_id: String,
    pub currency: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierHotel {
    pub hotel_id: String,
    pub name: String,
    pub category: i32,
    pub rooms: Vec<SupplierRoom>,
    pub destination_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierRoom {
    pub room_id: String,
    pub name: String,
    pub rates: Vec<SupplierRate>,
    pub capacity: RoomCapacity,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomCapacity {
    pub adults: i32,
    pub children: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierRate {
    pub rate_id: String,
    pub board_type: String,
    pub price: f64,
    pub cancellation_policies: Vec<SupplierCancellationPolicy>,
    pub booking_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierCancellationPolicy {
    pub from_date: String,
    pub amount: f64,
}

// Data structures for XML response
#[derive(Debug)]
pub struct HotelAvailabilityResponse {
    pub hotels: Vec<HotelAvailability>,
    pub search_id: String,
    pub currency: String,
    pub timestamp: String,
}

#[derive(Debug)]
pub struct HotelAvailability {
    pub hotel_id: String,
    pub name: String,
    pub category: i32,
    pub room_types: Vec<RoomType>,
    pub destination_code: String,
}

#[derive(Debug)]
pub struct RoomType {
    pub code: String,
    pub name: String,
    pub rates: Vec<Rate>,
    pub capacity: RoomCapacity,
}

#[derive(Debug)]
pub struct Rate {
    pub rate_key: String,
    pub board_type: String,
    pub price: f64,
    pub currency: String,
    pub cancellation_policies: Vec<SupplierCancellationPolicy>,
    pub booking_code: String,
}

// Structures for hotel data
#[derive(Debug, Clone)]
pub struct ProcessedResponse {
    pub search_id: String,
    pub total_options: usize,
    pub hotels: Vec<HotelOption>,
    pub currency: String,
    pub nationality: String,
    pub check_in: String,
    pub check_out: String,
}

impl From<XmlProcessedResponse> for ProcessedResponse {
    fn from(item: XmlProcessedResponse) -> Self {
        let mut hotels = Vec::new();
        for xml_hotel in item.hotels.hotels {
            for meal_plan in xml_hotel.meal_plans.meal_plans {
                for option in meal_plan.options.options {
                    for room in option.rooms.rooms {
                        let cancellation_policies = room
                            .cancel_penalties
                            .cancel_penalties
                            .iter()
                            .map(|cp| ProcessedCancellationPolicy {
                                deadline: cp.deadline.clone(),
                                penalty_amount: cp.penalty.value.parse().unwrap_or(0.0),
                                currency: cp.penalty.currency.clone(),
                                hours_before: cp.hours_before.parse().unwrap_or(0),
                                penalty_type: cp.penalty.penalty_type.clone(),
                            })
                            .collect();

                        let hotel_option = HotelOption {
                            hotel_id: xml_hotel.hotel_id.clone(),
                            hotel_name: xml_hotel.hotel_name.clone(),
                            room_type: room.code.clone(),
                            room_description: room.description.clone(),
                            board_type: meal_plan.code.clone(),
                            price: Price {
                                amount: option.price.amount.parse().unwrap_or(0.0),
                                currency: option.price.currency.clone(),
                            },
                            cancellation_policies,
                            payment_type: option.payment_type.clone(),
                            is_refundable: room.non_refundable.to_lowercase() == "false",
                            search_token: option
                                .parameters
                                .parameters
                                .iter()
                                .find(|p| p.key == "search_token")
                                .map(|p| p.value.clone())
                                .unwrap_or_default(),
                        };
                        hotels.push(hotel_option);
                    }
                }
            }
        }

        ProcessedResponse {
            search_id: "example_search".to_string(),
            total_options: hotels.len(),
            hotels,
            currency: "GBP".to_string(), // Default from the sample
            nationality: "US".to_string(),
            check_in: "2025-06-11".to_string(),
            check_out: "2025-06-12".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HotelOption {
    pub hotel_id: String,
    pub hotel_name: String,
    pub room_type: String,
    pub room_description: String,
    pub board_type: String,
    pub price: Price,
    pub cancellation_policies: Vec<ProcessedCancellationPolicy>,
    pub payment_type: String,
    pub is_refundable: bool,
    pub search_token: String,
}

#[derive(Debug, Clone)]
pub struct Price {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone)]
pub struct ProcessedCancellationPolicy {
    pub deadline: String, // ISO date format
    pub penalty_amount: f64,
    pub currency: String,
    pub hours_before: i32,
    pub penalty_type: String, // "Importe" or "Porcentaje"
}

// Structures for XML deserialization
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
#[serde(rename = "AvailRS")]
pub struct XmlProcessedResponse {
    pub hotels: XmlHotels,
}

impl From<SupplierResponse> for XmlProcessedResponse {
    fn from(item: SupplierResponse) -> Self {
        let mut xml_hotels = Vec::new();

        for hotel in item.hotels {
            let mut meal_plans = Vec::new();

            // Group rooms by board type
            let mut board_types = std::collections::HashMap::new();

            for room in &hotel.rooms {
                for rate in &room.rates {
                    let entries = board_types
                        .entry(rate.board_type.clone())
                        .or_insert_with(Vec::new);
                    entries.push((room, rate));
                }
            }

            for (board_type, room_rates) in board_types {
                let mut options = Vec::new();

                let xml_option = XmlOption {
                    option_type: "Hotel".to_string(),
                    payment_type: "MerchantPay".to_string(),
                    status: "OK".to_string(),
                    price: XmlPrice {
                        currency: item.currency.clone(),
                        amount: room_rates
                            .first()
                            .map_or("0.0".to_string(), |(_, rate)| rate.price.to_string()),
                        binding: "false".to_string(),
                        commission: "-1".to_string(),
                        minimum_selling_price: "-1".to_string(),
                    },
                    rooms: XmlRooms {
                        rooms: room_rates
                            .iter()
                            .map(|(room, rate)| {
                                let cancel_penalties = XmlCancelPenalties {
                                    non_refundable: "false".to_string(),
                                    cancel_penalties: rate
                                        .cancellation_policies
                                        .iter()
                                        .map(|cp| XmlCancelPenalty {
                                            hours_before: "N/A".to_string(),
                                            penalty: XmlPenalty {
                                                penalty_type: "Importe".to_string(),
                                                currency: item.currency.clone(),
                                                value: cp.amount.to_string(),
                                            },
                                            deadline: cp.from_date.clone(),
                                        })
                                        .collect(),
                                };

                                XmlRoom {
                                    id: format!("1#{}", room.room_id),
                                    room_candidate_ref_id: "1".to_string(),
                                    code: room.room_id.clone(),
                                    description: room.name.clone(),
                                    number_of_units: "1".to_string(),
                                    non_refundable: "false".to_string(),
                                    price: XmlPrice {
                                        currency: item.currency.clone(),
                                        amount: rate.price.to_string(),
                                        binding: "false".to_string(),
                                        commission: "-1".to_string(),
                                        minimum_selling_price: "-1".to_string(),
                                    },
                                    cancel_penalties,
                                }
                            })
                            .collect(),
                    },
                    parameters: XmlParameters {
                        parameters: vec![XmlParameter {
                            key: "search_token".to_string(),
                            value: format!("{}|||||{}", hotel.hotel_id, item.search_id),
                        }],
                    },
                };
                options.push(xml_option);

                let xml_mealplan = XmlMealPlan {
                    code: board_type,
                    options: XmlOptions { options },
                };
                meal_plans.push(xml_mealplan);
            }

            xml_hotels.push(XmlHotel {
                hotel_id: hotel.hotel_id.clone(),
                hotel_name: hotel.name.clone(),
                meal_plans: XmlMealPlans { meal_plans },
            });
        }

        XmlProcessedResponse {
            hotels: XmlHotels { hotels: xml_hotels },
        }
    }
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlHotels {
    #[serde(rename = "Hotel")]
    hotels: Vec<XmlHotel>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlHotel {
    #[serde(rename = "@code")]
    pub hotel_id: String,
    #[serde(rename = "@name")]
    pub hotel_name: String,
    pub meal_plans: XmlMealPlans,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlMealPlans {
    #[serde(rename = "MealPlan")]
    pub meal_plans: Vec<XmlMealPlan>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlMealPlan {
    #[serde(rename = "@code")]
    pub code: String,
    pub options: XmlOptions,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlOptions {
    #[serde(rename = "Option")]
    pub options: Vec<XmlOption>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlOption {
    #[serde(rename = "@type")]
    pub option_type: String,
    #[serde(rename = "@paymentType")]
    pub payment_type: String,
    #[serde(rename = "@status")]
    pub status: String,
    pub price: XmlPrice,
    pub rooms: XmlRooms,
    pub parameters: XmlParameters,
}
#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlPrice {
    #[serde(rename = "@currency")]
    pub currency: String,
    #[serde(rename = "@amount")]
    pub amount: String,
    #[serde(rename = "@binding")]
    pub binding: String,
    #[serde(rename = "@commission")]
    pub commission: String,
    #[serde(rename = "@minimumSellingPrice")]
    pub minimum_selling_price: String,
}
#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlRooms {
    #[serde(rename = "Room")]
    pub rooms: Vec<XmlRoom>,
}
#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlRoom {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@roomCandidateRefId")]
    pub room_candidate_ref_id: String,
    #[serde(rename = "@code")]
    pub code: String,
    #[serde(rename = "@description")]
    pub description: String,
    #[serde(rename = "@numberOfUnits")]
    pub number_of_units: String,
    #[serde(rename = "@nonRefundable")]
    pub non_refundable: String,
    pub price: XmlPrice,
    pub cancel_penalties: XmlCancelPenalties,
}
#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlCancelPenalties {
    #[serde(rename = "@nonRefundable")]
    pub non_refundable: String,
    #[serde(rename = "CancelPenalty")]
    pub cancel_penalties: Vec<XmlCancelPenalty>,
}
#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlCancelPenalty {
    pub hours_before: String,
    pub penalty: XmlPenalty,
    pub deadline: String,
}
#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlPenalty {
    #[serde(rename = "@type")]
    pub penalty_type: String,
    #[serde(rename = "@currency")]
    pub currency: String,
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlParameters {
    #[serde(rename = "Parameter")]
    pub parameters: Vec<XmlParameter>,
}
#[derive(Debug, PartialEq, Default, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct XmlParameter {
    #[serde(rename = "@key")]
    pub key: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct FilterCriteria {
    pub max_price: Option<f64>,
    pub board_types: Option<Vec<String>>,
    pub free_cancellation: bool,
    pub hotel_ids: Option<Vec<String>>,
    pub room_type_contains: Option<String>,
}

// Hotel search processor to implement
pub struct HotelSearchProcessor {
    // Add appropriate fields here
}

impl HotelSearchProcessor {
    // Create a new processor
    pub fn new() -> Self {
        Self {}
    }

    // Process XML response and extract hotel options
    pub fn process(&self, xml: &str) -> Result<ProcessedResponse, ProcessingError> {
        let response: XmlProcessedResponse =
            from_str(xml).map_err(|e| ProcessingError::XmlParseError(e.to_string()))?;

        Ok(response.into())
    }

    // Convert supplier JSON response to XML format
    pub fn convert_json_to_xml(&self, json_str: &str) -> Result<String, ProcessingError> {
        // Parse the JSON string into SupplierResponse
        let supplier_response: SupplierResponse = match serde_json::from_str(json_str) {
            Ok(response) => response,
            Err(e) => return Err(ProcessingError::JsonParseError(e.to_string())),
        };

        // // Convert to XML format
        let xml_response: XmlProcessedResponse = supplier_response.into();
        let xml = quick_xml::se::to_string(&xml_response)
            .map_err(|e| ProcessingError::ConversionError(e.to_string()))?;

        // println!("Converted XML: {}", xml);
        Ok(xml)
    }

    // Extract hotel options that match the given criteria
    pub fn filter_options(
        &self,
        response: &ProcessedResponse,
        criteria: &FilterCriteria,
    ) -> Vec<HotelOption> {
        let mut filtered = Vec::new();

        for hotel in &response.hotels {
            // Apply filters
            if !criteria
                .max_price
                .map_or(true, |max| hotel.price.amount <= max)
            {
                continue;
            }

            if !criteria
                .board_types
                .as_ref()
                .map_or(true, |types| types.contains(&hotel.board_type))
            {
                continue;
            }

            if criteria.free_cancellation && !hotel.is_refundable {
                continue;
            }

            if !criteria
                .hotel_ids
                .as_ref()
                .map_or(true, |ids| ids.contains(&hotel.hotel_id))
            {
                continue;
            }

            if !criteria
                .room_type_contains
                .as_ref()
                .map_or(true, |substring| hotel.room_type.contains(substring))
            {
                continue;
            }

            filtered.push(hotel.clone());
        }

        filtered
    }

    // Helper method to load the sample JSON response
    pub fn load_sample_json(&self) -> Result<String, ProcessingError> {
        match std::fs::read_to_string(SAMPLE_JSON_PATH) {
            Ok(content) => Ok(content),
            Err(e) => Err(ProcessingError::IoError(e)),
        }
    }

    // Helper method to load the sample response XML
    pub fn load_sample_response(&self) -> Result<String, ProcessingError> {
        match std::fs::read_to_string(SAMPLE_XML_PATH) {
            Ok(content) => Ok(content),
            Err(e) => Err(ProcessingError::IoError(e)),
        }
    }

    // Helper method to load the sample request XML
    pub fn load_sample_request(&self) -> Result<String, ProcessingError> {
        match std::fs::read_to_string(SAMPLE_REQUEST_PATH) {
            Ok(content) => Ok(content),
            Err(e) => Err(ProcessingError::IoError(e)),
        }
    }

    // Extract search parameters from the XML request
    pub fn extract_search_params(
        &self,
        request_xml: &str,
    ) -> Result<(String, String, String, String), ProcessingError> {
        let mut currency = String::new();
        let mut nationality = String::new();
        let mut start_date = String::new();
        let mut end_date = String::new();

        let mut reader = Reader::from_str(request_xml);
        reader.config_mut().trim_text(true);

        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) if e.name().as_ref() == b"StartDate" => {
                    // read_text_into for buffered readers not implemented
                    let txt = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                    start_date = format!("{}", txt);
                }
                Ok(Event::Start(e)) if e.name().as_ref() == b"EndDate" => {
                    // read_text_into for buffered readers not implemented
                    let txt = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                    end_date = format!("{}", txt);
                }
                Ok(Event::Start(e)) if e.name().as_ref() == b"Currency" => {
                    // read_text_into for buffered readers not implemented
                    let txt = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                    currency = format!("{}", txt);
                }
                Ok(Event::Start(e)) if e.name().as_ref() == b"Nationality" => {
                    // read_text_into for buffered readers not implemented
                    let txt = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                    nationality = format!("{}", txt);
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }

        Ok((currency, nationality, start_date, end_date))
    }
}

// Sample file paths (the actual files are stored in the samples directory)
pub const SAMPLE_XML_PATH: &str = "samples/hotel_search_response.xml";
pub const SAMPLE_REQUEST_PATH: &str = "samples/hotel_search_request.xml";
pub const SAMPLE_JSON_PATH: &str = "samples/supplier_response.json";

// A small sample for inline testing
pub const SMALL_SAMPLE_XML: &str = r#"
<AvailRS>
  <Hotels>
    <Hotel code="39776757" name="Days Inn By Wyndham Fargo">
      <MealPlans>
        <MealPlan code="RO">
          <Options>
            <Option type="Hotel" paymentType="MerchantPay" status="OK">
              <Price currency="GBP" amount="84.82" binding="false" commission="-1" minimumSellingPrice="-1"/>
              <Rooms>
                <Room id="1#ND1" roomCandidateRefId="1" code="ND1" description="ROOM, QUEEN BED" numberOfUnits="1" nonRefundable="false">
                  <Price currency="GBP" amount="84.82" binding="false" commission="-1" minimumSellingPrice="-1"/>
                  <CancelPenalties nonRefundable="false">
                    <CancelPenalty>
                      <HoursBefore>26</HoursBefore>
                      <Penalty type="Importe" currency="GBP">84.82</Penalty>
                      <Deadline>2025-06-10T10:00:00Z</Deadline>
                    </CancelPenalty>
                  </CancelPenalties>
                </Room>
              </Rooms>
              <Parameters>
                <Parameter key="search_token" value="39776757|2025-06-11|2025-06-12|A|US|GBP"/>
              </Parameters>
            </Option>
          </Options>
        </MealPlan>
      </MealPlans>
    </Hotel>
  </Hotels>
</AvailRS>
"#;

#[cfg(test)]
mod tests {
    use super::*;

    // Test JSON to XML conversion
    #[test]
    fn test_json_to_xml_conversion() {
        let processor = HotelSearchProcessor::new();

        // Sample JSON for testing
        let sample_json = r#"{
            "hotels": [
                {
                    "hotel_id": "12345",
                    "name": "Test Hotel",
                    "category": 4,
                    "destination_code": "NYC",
                    "rooms": [
                        {
                            "room_id": "DBL",
                            "name": "Double Room",
                            "capacity": {
                                "adults": 2,
                                "children": 0
                            },
                            "rates": [
                                {
                                    "rate_id": "R1",
                                    "board_type": "BB",
                                    "price": 120.50,
                                    "booking_code": "TESTCODE",
                                    "cancellation_policies": [
                                        {
                                            "from_date": "2023-12-01T00:00:00Z",
                                            "amount": 50.25
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ],
            "search_id": "SEARCH123",
            "currency": "USD",
            "timestamp": "2023-11-15T10:30:00Z"
        }"#;

        // Convert JSON to XML
        let xml_result = processor.convert_json_to_xml(sample_json);
        assert!(
            xml_result.is_ok(),
            "JSON to XML conversion failed: {:?}",
            xml_result.err()
        );

        let xml = xml_result.unwrap();

        // Verify XML structure
        assert!(xml.contains("<AvailRS>"));
        assert!(xml.contains("<Hotel code=\"12345\""));
        assert!(xml.contains("<MealPlan code=\"BB\">"));
        assert!(xml.contains("<Room id=\"1#DBL\""));
        assert!(xml.contains("<Price currency=\"USD\" amount=\"120.5\""));
        assert!(xml.contains("<Deadline>2023-12-01T00:00:00Z</Deadline>"));
        assert!(xml.contains("<Parameter key=\"search_token\" value=\"12345|||||SEARCH123\"/>"));
    }

    // Test loading the sample JSON file
    #[test]
    fn test_load_sample_json() {
        let processor = HotelSearchProcessor::new();
        let result = processor.load_sample_json();
        assert!(
            result.is_ok(),
            "Failed to load sample JSON: {:?}",
            result.err()
        );

        // Verify it's a valid JSON
        let json = result.unwrap();
        let parse_result = serde_json::from_str::<SupplierResponse>(&json);
        assert!(
            parse_result.is_ok(),
            "Failed to parse sample JSON: {:?}",
            parse_result.err()
        );
    }

    // Test full JSON to XML conversion workflow using sample files
    #[test]
    fn test_sample_json_to_xml_workflow() {
        let processor = HotelSearchProcessor::new();

        // Load sample JSON
        let json_result = processor.load_sample_json();
        assert!(
            json_result.is_ok(),
            "Failed to load sample JSON: {:?}",
            json_result.err()
        );

        // Convert JSON to XML
        let xml_result = processor.convert_json_to_xml(&json_result.unwrap());
        assert!(
            xml_result.is_ok(),
            "JSON to XML conversion failed: {:?}",
            xml_result.err()
        );

        let xml = xml_result.unwrap();
        assert!(xml.contains("<AvailRS>"));
        assert!(xml.contains("<Hotels>"));
    }

    // Test for processing XML
    #[test]
    fn test_process_xml() {
        let processor = HotelSearchProcessor::new();
        let result = processor.process(SMALL_SAMPLE_XML);

        assert!(result.is_ok());
        let response = result.unwrap();

        // Check basic response properties
        assert_eq!(response.hotels.len(), 1);

        // Check first hotel
        let hotel = &response.hotels[0];
        assert_eq!(hotel.hotel_id, "39776757");
        assert_eq!(hotel.hotel_name, "Days Inn By Wyndham Fargo");
        assert_eq!(hotel.board_type, "RO");
        assert_eq!(hotel.price.amount, 84.82);
        assert_eq!(hotel.price.currency, "GBP");
        assert_eq!(hotel.is_refundable, true);

        // Check cancellation policy
        assert_eq!(hotel.cancellation_policies.len(), 1);
        let policy = &hotel.cancellation_policies[0];
        assert_eq!(policy.hours_before, 26);
        assert_eq!(policy.penalty_amount, 84.82);
        assert_eq!(policy.currency, "GBP");
    }

    use test_case::test_case;

    // Test for filtering options
    #[test_case(FilterCriteria {max_price: Some(100.0), board_types: None, free_cancellation: false, hotel_ids: None, room_type_contains: None,},
        1,  vec!["hotel2"]; "#1 Filter by max price")]
    #[test_case(FilterCriteria {max_price: None, board_types: Some(vec!["BB".to_string(), "HB".to_string()]), free_cancellation: false, hotel_ids: None, room_type_contains: None,},
        2,  vec!["hotel1", "hotel3"]; "#2 Filter by board type")]
    #[test_case(FilterCriteria {max_price: None, board_types: None, free_cancellation: true, hotel_ids: None, room_type_contains: None,},
        2,  vec!["hotel1", "hotel3"]; "#3 Filter by free cancellation")]
    #[test_case(FilterCriteria {max_price: None, board_types: None, free_cancellation: false, hotel_ids: None, room_type_contains: Some("Suite".to_string()),},
        1,  vec!["hotel3"]; "#4 Filter by room type")]
    #[test_case(FilterCriteria {max_price: Some(300.0), board_types: Some(vec!["HB".to_string()]), free_cancellation: true, hotel_ids: None, room_type_contains: Some("Suite".to_string()),},
        1,  vec!["hotel3"]; "#5 Combined filters")]
    fn test_criteria_filter_options(
        criteria: FilterCriteria,
        expected_count: usize,
        expected_ids: Vec<&str>,
    ) {
        let processor = HotelSearchProcessor::new();

        // Create a sample processed response with multiple hotels
        let mut response = ProcessedResponse {
            search_id: "test_search".to_string(),
            total_options: 3,
            hotels: Vec::new(),
            currency: "GBP".to_string(),
            nationality: "GB".to_string(),
            check_in: "2025-06-01".to_string(),
            check_out: "2025-06-05".to_string(),
        };

        // Add sample hotels with different properties
        response.hotels.push(HotelOption {
            hotel_id: "hotel1".to_string(),
            hotel_name: "Luxury Hotel".to_string(),
            room_type: "Deluxe King".to_string(),
            room_description: "Spacious room with king bed".to_string(),
            board_type: "BB".to_string(), // Bed & Breakfast
            price: Price {
                amount: 150.0,
                currency: "GBP".to_string(),
            },
            cancellation_policies: vec![ProcessedCancellationPolicy {
                deadline: "2025-05-30T00:00:00Z".to_string(),
                penalty_amount: 75.0,
                currency: "GBP".to_string(),
                hours_before: 48,
                penalty_type: "Importe".to_string(),
            }],
            payment_type: "MerchantPay".to_string(),
            is_refundable: true,
            search_token: "token1".to_string(),
        });

        response.hotels.push(HotelOption {
            hotel_id: "hotel2".to_string(),
            hotel_name: "Budget Inn".to_string(),
            room_type: "Standard Twin".to_string(),
            room_description: "Basic room with twin beds".to_string(),
            board_type: "RO".to_string(), // Room Only
            price: Price {
                amount: 80.0,
                currency: "GBP".to_string(),
            },
            cancellation_policies: vec![],
            payment_type: "MerchantPay".to_string(),
            is_refundable: false,
            search_token: "token2".to_string(),
        });

        response.hotels.push(HotelOption {
            hotel_id: "hotel3".to_string(),
            hotel_name: "Resort Spa".to_string(),
            room_type: "Premium Suite".to_string(),
            room_description: "Luxury suite with ocean view".to_string(),
            board_type: "HB".to_string(), // Half Board
            price: Price {
                amount: 250.0,
                currency: "GBP".to_string(),
            },
            cancellation_policies: vec![ProcessedCancellationPolicy {
                deadline: "2025-05-25T00:00:00Z".to_string(),
                penalty_amount: 100.0,
                currency: "GBP".to_string(),
                hours_before: 168,
                penalty_type: "Importe".to_string(),
            }],
            payment_type: "MerchantPay".to_string(),
            is_refundable: true,
            search_token: "token3".to_string(),
        });

        // Test filtering
        let results = processor.filter_options(&response, &criteria);
        assert_eq!(results.len(), expected_count);
        for expected_id in expected_ids {
            assert!(results.iter().any(|h| h.hotel_id == expected_id));
        }
    }

    #[test]
    fn test_load_sample_response() {
        let processor = HotelSearchProcessor::new();
        let xml = processor.load_sample_response();
        assert!(
            xml.is_ok(),
            "Failed to load sample XML response: {:?}",
            xml.err()
        );

        let result = processor.process(xml.unwrap().as_str());
        assert!(result.is_ok());
        let response = result.unwrap();

        // Check basic response properties
        assert_eq!(response.hotels.len(), 7);
    }

    #[test]
    fn test_example_search_param_extraction() {
        let processor = HotelSearchProcessor::new();

        // Simple XML for testing
        let request_xml = r#"
        <AvailRQ>
            <Currency>GBP</Currency>
            <Nationality>US</Nationality>
            <StartDate>11/06/2025</StartDate>
            <EndDate>12/06/2025</EndDate>
        </AvailRQ>
        "#;

        let result = processor.extract_search_params(request_xml);
        assert!(result.is_ok());

        let (currency, nationality, start_date, end_date) = result.unwrap();
        assert_eq!(currency, "GBP");
        assert_eq!(nationality, "US");
        assert_eq!(start_date, "11/06/2025");
        assert_eq!(end_date, "12/06/2025");
    }

    #[test]
    fn test_load_sample_request() {
        let processor = HotelSearchProcessor::new();
        let result = processor.load_sample_request();
        assert!(
            result.is_ok(),
            "Failed to load sample XML request: {:?}",
            result.err()
        );
        let request_xml = result.unwrap();

        let result = processor.extract_search_params(&request_xml);
        assert!(result.is_ok());

        let (currency, nationality, start_date, end_date) = result.unwrap();
        assert_eq!(currency, "GBP");
        assert_eq!(nationality, "US");
        assert_eq!(start_date, "11/06/2025");
        assert_eq!(end_date, "12/06/2025");
    }
}

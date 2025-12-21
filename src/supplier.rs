use serde::{Deserialize, Serialize};

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

use crate::supplier::SupplierResponse;
use serde::{Deserialize, Serialize};

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
    pub hotels: Vec<XmlHotel>,
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

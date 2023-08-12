// #[macro_export]
// macro_rules! handle_api_response {
//     ($response:expr, $success_type:ty, $error_type:ty) => {{
//         let response_json: serde_json::Value = $response.json().await?;
//         if let Some(error) = response_json.get("error") {
//             let error_response: $error_type = serde_json::from_value(error.clone())?;
//             Err(PesaPalError::ApiError(error_response))
//         } else {
//             let success_response: $success_type = serde_json::from_value(response_json)z;
//             Ok(success_response)
//         }
//     }};
// }

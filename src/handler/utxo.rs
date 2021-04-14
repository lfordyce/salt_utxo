use crate::entity::utxo::Utxo;
use crate::utils;
use serde::{Deserialize, Serialize};
use tide::{Body, StatusCode};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub code: i64,
    pub message: String,
}

impl From<Box<dyn std::error::Error>> for ErrorResponse {
    fn from(item: Box<dyn std::error::Error>) -> Self {
        ErrorResponse {
            code: 3,
            message: format!("{} : {}", "error parsing json body", item.to_string()),
        }
    }
}

impl From<http_types::Error> for ErrorResponse {
    fn from(item: http_types::Error) -> Self {
        ErrorResponse {
            code: 3,
            message: format!("{} : {}", "error parsing json body", item.to_string()),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Filter {
    unspent_only: bool,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            unspent_only: false,
        }
    }
}

pub async fn index(request: crate::Request) -> tide::Result {
    let limit: &str = match request.param("limit") {
        Ok(address) => address,
        Err(e) => {
            let mut res = tide::Response::new(StatusCode::BadRequest);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            return Ok(res);
        }
    };

    let offset: &str = match request.param("offset") {
        Ok(address) => address,
        Err(e) => {
            let mut res = tide::Response::new(StatusCode::BadRequest);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            return Ok(res);
        }
    };
    let limit_int = limit.parse::<i32>().unwrap();
    let offset_int = offset.parse::<i32>().unwrap();

    match request.state().db().find_all(limit_int, offset_int).await {
        Ok(rows) => {
            let mut res = tide::Response::new(StatusCode::Ok);
            res.set_body(tide::Body::from_json(&rows)?);
            Ok(res)
        }
        Err(e) => {
            let mut res = tide::Response::new(StatusCode::InternalServerError);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            Ok(res)
        }
    }
}

pub async fn get_balance(request: crate::Request) -> tide::Result {
    let address: &str = match request.param("address") {
        Ok(address) => address,
        Err(e) => {
            let mut res = tide::Response::new(StatusCode::BadRequest);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            return Ok(res);
        }
    };
    let filter: Filter = request.query()?;
    match request
        .state()
        .db()
        .find_balance_by_address(address, filter.unspent_only)
        .await
    {
        Ok(result) => {
            let mut response = tide::Response::new(StatusCode::Ok);
            response.set_body(tide::Body::from_json(&result)?);
            Ok(response)
        }
        Err(e) => {
            let mut res = tide::Response::new(StatusCode::InternalServerError);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            Ok(res)
        }
    }
}

pub async fn create_txn(mut request: crate::Request) -> tide::Result {
    let data: Utxo = utils::deserialize_body(&mut request).await?;
    match request.state().db().create_utxo(&data).await {
        Ok(()) => Ok(tide::Response::new(StatusCode::Created)),
        Err(e) => {
            let mut res = tide::Response::new(StatusCode::InternalServerError);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            return Ok(res);
        }
    }
}

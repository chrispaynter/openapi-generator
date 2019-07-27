/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct StoreApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> StoreApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> StoreApiClient<C> {
        StoreApiClient {
            configuration: configuration,
        }
    }
}

pub trait StoreApi {
    fn delete_order(&self, order_id: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_inventory(&self, ) -> Box<Future<Item = ::std::collections::HashMap<String, i32>, Error = Error<serde_json::Value>>>;
    fn get_order_by_id(&self, order_id: i64) -> Box<Future<Item = crate::models::Order, Error = Error<serde_json::Value>>>;
    fn place_order(&self, body: crate::models::Order) -> Box<Future<Item = crate::models::Order, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>StoreApi for StoreApiClient<C> {
    fn delete_order(&self, order_id: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/store/order/{orderId}".to_string())
            .with_path_param("orderId".to_string(), order_id.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn get_inventory(&self, ) -> Box<Future<Item = ::std::collections::HashMap<String, i32>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/store/inventory".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "api_key".to_owned(),
            }))
            .execute(self.configuration.borrow())
    }

    fn get_order_by_id(&self, order_id: i64) -> Box<Future<Item = crate::models::Order, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/store/order/{orderId}".to_string())
            .with_path_param("orderId".to_string(), order_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn place_order(&self, body: crate::models::Order) -> Box<Future<Item = crate::models::Order, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/store/order".to_string())
            .with_body_param(body)
            .execute(self.configuration.borrow())
    }

}

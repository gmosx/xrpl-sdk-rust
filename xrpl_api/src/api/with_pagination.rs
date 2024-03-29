use crate::Request;
use serde::{Deserialize, Serialize};

/// Request that allows specifying pagination.
/// See <https://xrpl.org/markers-and-pagination.html>.
pub trait WithRequestPagination: Request {
    fn as_pagination(&self) -> &RequestPagination;
    fn as_pagination_mut(&mut self) -> &mut RequestPagination;

    fn limit(mut self, limit: u32) -> Self
    where
        Self: Sized,
    {
        self.as_pagination_mut().limit = Some(limit);
        self
    }

    fn marker(mut self, marker: serde_json::Value) -> Self
    where
        Self: Sized,
    {
        self.as_pagination_mut().marker = Some(marker);
        self
    }
}

/// Response that allows specifying pagination.
/// See <https://xrpl.org/markers-and-pagination.html>.
pub trait WithResponsePagination {
    fn as_pagination(&self) -> &ResponsePagination;

    fn limit(&self) -> Option<u32>
    where
        Self: Sized,
    {
        self.as_pagination().limit
    }

    fn marker(&self) -> Option<&serde_json::Value>
    where
        Self: Sized,
    {
        self.as_pagination().marker.as_ref()
    }
}

/// Pagination part of request, see <https://xrpl.org/markers-and-pagination.html>
#[derive(Default, Debug, Clone, Serialize)]
pub struct RequestPagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Marker can be any value including a JSON object, see <https://xrpl.org/markers-and-pagination.html>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<serde_json::Value>,
}

/// Pagination part of response, see <https://xrpl.org/markers-and-pagination.html>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ResponsePagination {
    pub limit: Option<u32>,
    /// Marker can be any value including a JSON object, see <https://xrpl.org/markers-and-pagination.html>
    pub marker: Option<serde_json::Value>,
}

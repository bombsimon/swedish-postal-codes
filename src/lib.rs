mod looks_like;
use looks_like::LooksLikePostalCode;

use std::collections::HashMap;

/// PostalCode is the type which manages postal codes. It holds a map with all valid ones and a
/// client to query for non existing ones.
#[derive(Debug)]
pub struct PostalCode {
    client: reqwest::blocking::Client,
    codes: HashMap<u32, String>,
    http_fallback: bool,
}

/// PostalCodeRecord is the records in the CSV file for persistent storing potsal codes.
#[derive(Debug, serde::Deserialize)]
struct PostalCodeRecord {
    postal_code: u32,
    city: String,
}

/// BringResponse is the response from the Bring API which can be queried for postal codes.
#[derive(Debug, serde::Deserialize)]
pub struct BringResponse {
    result: String,
    valid: bool,
    #[serde(alias = "postalCodeType")]
    postal_code_type: String,
}

/// Validate a postal code without creating any instances and just return a boolean value.
pub fn valid<T>(code: T) -> bool
where
    T: LooksLikePostalCode,
{
    PostalCode::new(true).valid(code)
}

impl PostalCode {
    /// Create a new postal code instance which will read an initial CSV file with postal codes and
    /// setup a HTTP client when one should be needed.
    pub fn new(http_fallback: bool) -> PostalCode {
        let mut postal_codes: HashMap<u32, String> = HashMap::new();
        let csv_bytes = include_str!("../postal_codes.csv");
        let mut rdr = csv::Reader::from_reader(csv_bytes.as_bytes());

        for result in rdr.deserialize() {
            let record: PostalCodeRecord = match result {
                Ok(r) => r,
                _ => break,
            };

            postal_codes.insert(record.postal_code, record.city);
        }

        PostalCode {
            client: reqwest::blocking::Client::new(),
            codes: postal_codes,
            http_fallback,
        }
    }

    /// Check if a postal code is valid or not. Accepts most known types that can be converted to
    /// u32.
    pub fn valid<T>(&self, code: T) -> bool
    where
        T: LooksLikePostalCode,
    {
        let valid_postal_code = self.codes.contains_key(&code.as_u32());
        if !valid_postal_code && self.http_fallback {
            return self.valid_according_to_bring(code.as_u32());
        }

        valid_postal_code
    }

    /// Make an HTTP request to the Bring API for a given postal code to see if it's valid.
    pub fn query_bring(&self, code: u32) -> Option<BringResponse> {
        let request_url = format!(
            "https://api.bring.com/shippingguide/api/postalCode.json?clientUrl={cu}&country={country}&pnr={postal_code}",
            cu="swedish-postal-code",
            country="SE",
            postal_code = code,
        );

        let result: Result<BringResponse, reqwest::Error> = self
            .client
            .get(&request_url)
            .send()
            .and_then(|response| response.json());

        match result {
            Ok(response) => Some(response),
            _ => None,
        }
    }

    /// Convenience method to convert Bring API response to boolean if valid.
    fn valid_according_to_bring(&self, code: u32) -> bool {
        match self.query_bring(code) {
            Some(v) => v.valid,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_types() {
        let pc = PostalCode::new(true);

        assert!(!pc.valid(0));
        assert!(!pc.valid("0"));

        assert!(pc.valid(11220));
        assert!(pc.valid("11220"));
    }
}

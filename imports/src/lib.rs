pub use curl_effector_types::*;
use marine_rs_sdk::marine;

#[marine]
#[module_import("curl_effector")]
extern "C" {
    // Make an HTTP POST request with the request's body taken from the `data_vault_path` file
    // The response is written to the `output_vault_path` file.
    // Note that the provided path should be a full path in the Particle Vault.
    pub fn curl_post(
        request: CurlRequest,
        data_vault_path: String,
        output_vault_path: String,
    ) -> CurlResult;

    // idem as above 
    // but posting content of data_vault_path with the --data-binary path
    // carriage returns and white spaces are not removed from content of file
    // used when uploading files as multipart, for example with Pinata. 
    pub fn curl_post_binary(
        request: CurlRequest,
        data_vault_path: String,
        output_vault_path: String,
    ) -> CurlResult;

    // Make an HTTP GET request.
    // The response is written to the `output_vault_path` file.
    pub fn curl_get(request: CurlRequest, output_vault_path: String) -> CurlResult;
}

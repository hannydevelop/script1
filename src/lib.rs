// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.in.mako'
// DO NOT EDIT !

extern crate hyper;
extern crate mime;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate yup_oauth2 as oauth2;

mod cmn;

use serde::{Deserialize, Serialize};
use serde_json as json;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::default::Default;
use std::fs;
use std::io;
use std::thread::sleep_ms;
use tower_service;
use http::Uri;
use tokio::io::{AsyncRead, AsyncWrite};
use std::error::Error as StdError;
use std::thread::sleep;


pub use cmn::{
    remove_json_null_values, CallBuilder, DefaultDelegate, Delegate, Error, ErrorResponse, Hub,
    MethodInfo, MethodsBuilder, MultiPartReader, NestedType, Part, ReadSeek, RequestValue,
    Resource, ResponseResult, Result, ToParts, Retry,
};

// Borrowing the body object as mutable and converts it to a string
pub async fn get_body_as_string(res_body: &mut hyper::Body) -> String {
    let res_body_buf = hyper::body::to_bytes(res_body).await.unwrap();
    let res_body_string = String::from_utf8_lossy(&res_body_buf);
    res_body_string.to_string()
}



// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage the files in your Google Drive
    Drive,

    /// View and manage your mail
    Scrip,

    /// View and manage forms that this application has been installed in
    FormCurrentonly,

    /// View and manage the provisioning of users on your domain
    AdminDirectoryUser,

    /// Manage your contacts
    Feed,

    /// View and manage your Google Groups
    Group,

    /// View and manage the provisioning of groups on your domain
    AdminDirectoryGroup,

    /// View and manage your forms in Google Drive
    Form,

    /// Manage your calendars

    /// View your email address
    UserinfoEmail,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::Scrip => "https://mail.google.com/",
            Scope::FormCurrentonly => "https://www.googleapis.com/auth/forms.currentonly",
            Scope::AdminDirectoryUser => "https://www.googleapis.com/auth/admin.directory.user",
            Scope::Feed => "https://www.google.com/m8/feeds",
            Scope::Group => "https://www.googleapis.com/auth/groups",
            Scope::AdminDirectoryGroup => "https://www.googleapis.com/auth/admin.directory.group",
            Scope::Form => "https://www.googleapis.com/auth/forms",
            Scope::Feed => "https://www.google.com/calendar/feeds",
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Scrip
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all Script related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_script1 as script1;
/// use script1::ExecutionRequest;
/// use script1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use script1::Script;
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Script::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ExecutionRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.scripts().run(req, "scriptId")
///              .doit().await;
///
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct Script<S> {
    client: hyper::Client<S, hyper::body::Body>,
    auth: oauth2::authenticator::Authenticator<S>,
    _user_agent: String,
}

impl<'a, S> Hub for Script<S> {}

impl<'a, S> Script<S> {
    pub fn new(
        client: hyper::Client<S, hyper::body::Body>,
        authenticator: oauth2::authenticator::Authenticator<S>,
    ) -> Script<S> {
        Script {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/0.1.10".to_string(),
        }
    }

    pub fn scripts(&'a self) -> ScriptMethods<'a, S> {
        ScriptMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.10`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}

// ############
// SCHEMAS ###
// ##########
/// If a `run` call succeeds but the script function (or Apps Script itself) throws an exception, the response body's `error` field will contain this `Status` object.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// An array that contains a single `ExecutionError` object that provides information about the nature of the error.
    pub details: Option<Vec<HashMap<String, String>>>,
}

impl Part for Status {}

/// A request to run the function in a script. The script is identified by the specified `script_id`. Executing a function on a script will return results based on the implementation of the script.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [run scripts](struct.ScriptRunCall.html) (request)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionRequest {
    /// The name of the function to execute in the given script. The name does not include parentheses or parameters.
    pub function: Option<String>,
    /// This field is not used.
    #[serde(rename = "sessionState")]
    pub session_state: Option<String>,
    /// The parameters to be passed to the function being executed. The type for each parameter should match the expected type in Apps Script. Parameters cannot be Apps Script-specific objects (such as a `Document` or `Calendar`); they can only be primitive types such as a `string`, `number`, `array`, `object`, or `boolean`. Optional.
    pub parameters: Option<Vec<String>>,
    /// If `true` and the user is an owner of the script, the script runs at the most recently saved version rather than the version deployed for use with the Execution API. Optional; default is `false`.
    #[serde(rename = "devMode")]
    pub dev_mode: Option<bool>,
}

impl RequestValue for ExecutionRequest {}

/// The response will not arrive until the function finishes executing. The maximum runtime is listed in the guide to [limitations in Apps Script](https://developers.google.com/apps-script/guides/services/quotas#current_limitations).
/// If the script function returns successfully, the `response` field will contain an `ExecutionResponse` object with the function's return value in the object's `result` field.
///
/// If the script function (or Apps Script itself) throws an exception, the `error` field will contain a `Status` object. The `Status` object's `details` field will contain an array with a single `ExecutionError` object that provides information about the nature of the error.
///
/// If the `run` call itself fails (for example, because of a malformed request or an authorization error), the method will return an HTTP response code in the 4XX range with a different format for the response body. Client libraries will automatically convert a 4XX response into an exception class.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [run scripts](struct.ScriptRunCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// This field is not used.
    pub metadata: Option<HashMap<String, String>>,
    /// This field is not used.
    pub done: Option<bool>,
    /// If the script function returns successfully, this field will contain an `ExecutionResponse` object with the function's return value as the object's `result` field.
    pub response: Option<HashMap<String, String>>,
    /// This field is not used.
    pub name: Option<String>,
    /// If a `run` call succeeds but the script function (or Apps Script itself) throws an exception, this field will contain a `Status` object. The `Status` object's `details` field will contain an array with a single `ExecutionError` object that provides information about the nature of the error.
    pub error: Option<Status>,
}

impl ResponseResult for Operation {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *script* resources.
/// It is not used directly, but through the `Script` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_script1 as script1;
///
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use script1::Script;
///
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Script::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `run(...)`
/// // to build up your call.
/// let rb = hub.scripts();
/// # }
/// ```
pub struct ScriptMethods<'a, S> {
    hub: &'a Script<S>,
}

impl<'a, S> MethodsBuilder for ScriptMethods<'a, S> {}

impl<'a, S> ScriptMethods<'a, S> {
    /// Create a builder to help you perform the following task:
    ///
    /// Runs a function in an Apps Script project that has been deployed for use with the Apps Script Execution API. This method requires authorization with an OAuth 2.0 token that includes at least one of the scopes listed in the [Authentication](#authentication) section; script projects that do not require authorization cannot be executed through this API. To find the correct scopes to include in the authentication token, open the project in the script editor, then select **File > Project properties** and click the **Scopes** tab.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `scriptId` - The project key of the script to be executed. To find the project key, open the project in the script editor, then select **File > Project properties**.
    pub fn run(&self, request: ExecutionRequest, script_id: &str) -> ScriptRunCall<'a, S> {
        ScriptRunCall {
            hub: self.hub,
            _request: request,
            _script_id: script_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Runs a function in an Apps Script project that has been deployed for use with the Apps Script Execution API. This method requires authorization with an OAuth 2.0 token that includes at least one of the scopes listed in the [Authentication](#authentication) section; script projects that do not require authorization cannot be executed through this API. To find the correct scopes to include in the authentication token, open the project in the script editor, then select **File > Project properties** and click the **Scopes** tab.
///
/// A builder for the *run* method supported by a *script* resource.
/// It is not used directly, but through a `ScriptMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_script1 as script1;
/// use script1::ExecutionRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use script1::Script;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Script::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ExecutionRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.scripts().run(req, "scriptId")
///              .doit().await;
/// # }
/// ```
pub struct ScriptRunCall<'a, S>
where S: 'a
{
    hub: &'a Script<S>,
    _request: ExecutionRequest,
    _script_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>,
}

impl<'a, S> CallBuilder for ScriptRunCall<'a, S> {}

impl<'a, S> ScriptRunCall<'a, S>
where
S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
S::Response: Send + hyper::client::connect::Connection + AsyncRead + AsyncWrite + Unpin + 'static,
S::Future: Send + Unpin + 'static,
S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> Result<(hyper::Response<hyper::body::Body>, Operation)> {
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use std::io::{Read, Seek};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd,
        };
        dlg.begin(MethodInfo {
            id: "script.scripts.run",
            http_method: hyper::Method::POST,
        });
        let mut params: Vec<(&str, String)> =
            Vec::with_capacity(4 + self._additional_params.len());
        params.push(("scriptId", self._script_id.to_string()));
        for &field in ["alt", "scriptId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://script.googleapis.com/v1/scripts/{scriptId}:run".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Scrip.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{scriptId}", "scriptId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(
                find_this,
                replace_with.expect("to find substitution value in params"),
            );
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["scriptId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = mime::Mime(
            mime::TopLevel::Application,
            mime::SubLevel::Json,
            Default::default(),
        );
        let mut request_value_reader = {
            let mut value = json::value::to_value(&self._request).unwrap();
            remove_json_null_values(&mut value);
            let mut dst = io::Cursor::new(Vec::with_capacity(128));
            json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();

        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(Box::new(err)))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };
            match req_result {
                Err(err) => {
                    if let Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(Error::BadRequest(error_value)),
                            None => Err(Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ExecutionRequest) -> ScriptRunCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The project key of the script to be executed. To find the project key, open the project in the script editor, then select **File > Project properties**.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ScriptRunCall<'a, S> {
        self._script_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ScriptRunCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ScriptRunCall<'a, S>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Scrip`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ScriptRunCall<'a, S>
    where
        T: AsRef<str>,
    {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}

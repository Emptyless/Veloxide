package httpapi.authz

test_is_get_method {
    is_get_method with input.method as "GET"
}

test_is_post_method {
    is_post_method with input.method as "POST"
}

test_is_graphql_playground {
    is_graphql_playground with input as {"method": "POST", "path": [""], "headers": {"content-type": "application/json", "referer": "http://localhost:8080/graphql"}}
}

test_is_login_route {
    is_login_route with input as {"method": "GET", "path": ["login"]}
}

test_is_logout_route {
    is_logout_route with input as {"method": "POST", "path": ["logout"]}
}

test_is_protected_route {
    is_protected_route with input as {"method": "GET", "path": ["protected"]}
}

test_is_graphql_route {
    is_graphql_route with input as {"method": "GET", "path": ["graphql"]}
}

test_is_valid_user {
    is_valid_user({"email": "ltest@example.com"})
}

test_is_callback_path {
    is_callback_path with input as {"method": "GET", "path": ["auth", "google", "callback"]}
}

test_is_root_path {
    is_root_path with input as {"method": "GET", "path": [`/`]}
}

test_path_is_empty_string {
    path_is_empty_string with input as {"method": "GET", "path": [""]}
}

test_path_is_empty_array {
    path_is_empty_array with input as {"method": "GET", "path": []}
}

test_is_swagger_path {
    is_swagger_path with input as {"method": "GET", "path": ["swagger-ui", "something"]}
}

test_is_openapi_path {
    is_openapi_path with input as {"method": "GET", "path": ["api-doc", "openapi.json"]}
}

test_is_css_path {
    is_css_path with input as {"method": "GET", "path": ["folder", "style.css"]}
}

test_is_favicon_path {
    is_favicon_path with input as {"method": "GET", "path": ["favicon.ico"]}
}

test_allow_login_route {
    allow with input as {"method": "GET", "path": ["login"]}
}

test_allow_logout_route {
    allow with input as {"method": "POST", "path": ["logout"]}
}

test_allow_protected_route {
    allow with input as {"method": "GET", "path": ["protected"], "user": {"email": "ltest@example.com"}}
}

test_allow_callback_path {
    allow with input as {"method": "GET", "path": ["auth", "google", "callback"]}
}

test_allow_path_is_empty_string {
    allow with input as {"method": "GET", "path": [""]}
}

test_allow_path_is_empty_array {
    allow with input as {"method": "GET", "path": []}
}

test_allow_root_path {
    allow with input as {"method": "GET", "path": [`/`]}
}

test_allow_swagger_path {
    allow with input as {"method": "GET", "path": ["swagger-ui"]}
}

test_allow_css_path {
    allow with input as {"method": "GET", "path": ["style.css"]}
}

test_allow_openapi_path {
    allow with input as {"method": "GET", "path": ["api-doc", "openapi.json"]}
}

test_allow_favicon_path {
    allow with input as {"method": "GET", "path": ["favicon.ico"]}
}

test_allow_graphql_route {
    allow with input as {"method": "GET", "path": ["graphql"]}
}

test_allow_graphql_playground {
    allow with input as {"method": "POST", "path": [""], "headers": {"content-type": "application/json", "referer": "http://localhost:8080/graphql"}}
}

test_is_not_get_method_when_post {
    not is_get_method with input.method as "POST"
}

test_is_not_post_method_when_get {
    not is_post_method with input.method as "GET"
}

test_is_not_valid_user_with_invalid_email {
    not is_valid_user({"email": "testexample.com"})
}

test_is_not_graphql_playground_with_wrong_header {
    not is_graphql_playground with input as {"method": "POST", "path": [""], "headers": {"content-type": "text/plain", "referer": "http://localhost:8080/graphql"}}
}

test_deny_protected_route_without_valid_user {
    not allow with input as {"method": "GET", "path": ["protected"], "user": {"email": "testexample.com"}}
}

test_deny_unknown_route {
    not allow with input as {"method": "GET", "path": ["unknown"]}
}

test_is_not_css_path_with_non_css_file {
    not is_css_path with input as {"method": "GET", "path": ["folder", "file.txt"]}
}

test_deny_login_route_with_post_method {
    not allow with input as {"method": "POST", "path": ["login"]}
}

package httpapi.authz

default allow = false

is_get_method {
    input.method == "GET"
}

is_post_method {
    input.method == "POST"
}

is_graphql_playground {
    is_post_method
    input.path == [""]
    input.headers["content-type"] == "application/json"
    startswith(input.headers["referer"], "http://localhost:8080/graphql")
}

is_login_route {
    is_get_method
    input.path == ["login"]
}

is_logout_route {
    is_post_method
    input.path == ["logout"]
}

is_protected_route {
    is_get_method
    input.path == ["protected"]
}

is_graphql_route {
    is_get_method
    input.path == ["graphql"]
}

is_valid_user(user) {
    # This is just demonstrating running policy against data extracted by axum
    contains(user.email, "@")
}

is_callback_path {
    is_get_method
    input.path == ["auth", "google", "callback"]
}

is_root_path {
    is_get_method
    input.path == [`/`]
}

path_is_empty_string {
    is_get_method
    input.path == [""]
}

path_is_empty_array {
    is_get_method
    input.path == []
}

is_swagger_path {
    is_get_method
    input.path[0] == "swagger-ui"
}

is_openapi_path {
    is_get_method
    input.path == ["api-doc", "openapi.json"]
}

is_css_path {
    is_get_method
    endswith(input.path[count(input.path) - 1], ".css")
}

is_favicon_path {
    is_get_method
    input.path == ["favicon.ico"]
}

# Main rule
allow {
    is_login_route
}

allow {
    is_logout_route
}

allow {
    is_protected_route
    is_valid_user(input.user)
}

allow {
    is_callback_path
}

allow {
    path_is_empty_string
}

allow {
    path_is_empty_array
}

allow {
    is_root_path
}

allow {
    is_swagger_path
}

allow {
    is_css_path
}

allow {
    is_openapi_path
}

allow {
    is_favicon_path
}
allow {
    is_graphql_route
}

allow {
    is_graphql_playground
}


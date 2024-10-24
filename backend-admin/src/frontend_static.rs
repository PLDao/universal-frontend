use rust_embed::Embed;
use warp::{http::header::HeaderValue, path::Tail, reply::Response, Filter, Rejection, Reply};

#[derive(Embed)]
#[folder = "../backend-admin/dist/"]
struct Asset;

pub (crate) fn get_frontend_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let static_files = warp::path("assets")
        .and(warp::path::tail())
        .and_then(serve)
        .or(warp::path("favicon.ico").and_then(serve_favicon))
        .or(warp::path("data.json").and_then(serve_data_json));

    // Catch-all route to serve index.html for client-side routing
    let catch_all = warp::any().and_then(|| async { serve_index().await });

    static_files.or(catch_all)
}

async fn serve_index() -> Result<impl Reply, Rejection> {
    serve_impl("index.html")
}

async fn serve_favicon() -> Result<impl Reply, Rejection> {
    serve_impl("favicon.ico")
}

async fn serve_data_json() -> Result<impl Reply, Rejection> {
    serve_impl("data.json")
}

async fn serve(path: Tail) -> Result<impl Reply, Rejection> {
    serve_impl(&format!("assets/{}", path.as_str()))
}

fn serve_impl(path: &str) -> Result<impl Reply, Rejection> {
    let asset = Asset::get(path).ok_or_else(warp::reject::not_found)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    let mut res = Response::new(asset.data.into());
    res.headers_mut()
        .insert("content-type", HeaderValue::from_str(mime.as_ref()).unwrap());
    Ok(res)
}
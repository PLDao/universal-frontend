use warp::Filter;
mod frontend_static;



pub async fn start_frontend(ip: [u8; 4], port: u16, log_target: Option<&'static str>) {
    let frontend_filter = frontend_static::get_frontend_filter();
    match log_target {
        Some(target) => {
            let log= warp::log(target);
            let routes = frontend_filter.with(log);
            warp::serve(routes).run((ip, port)).await;
        }
        None => {
            warp::serve(frontend_filter).run((ip, port)).await;
        }
    }
}
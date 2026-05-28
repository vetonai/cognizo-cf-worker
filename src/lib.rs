use worker::*;
use serde_json::json;

const STATIC_EXTENSIONS: &[&str] = &[
    "css", "js", "jpg", "jpeg", "png", "gif", "svg", "ico", "webp", "avif",
    "woff", "woff2", "ttf", "eot", "otf",
    "mp4", "webm", "mp3", "wav", "ogg",
    "pdf", "zip", "tar", "gz", "rar",
    "xml", "json", "txt", "map"
];

#[event(fetch)]
async fn fetch(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let response = Fetch::Request(req.clone()?).send().await?;
    ctx.wait_until(async move {
        let _ = track_visit(req, env).await;
    });
    Ok(response)
}

async fn track_visit(req: Request, env: Env) -> Result<()> {
    let url = req.url()?;
    let path = url.path();
    
 
    if let Some(extension) = path.rsplit('.').next() {
        if STATIC_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
            return Ok(());
        }
    }
    
 
    if let Ok(exclude_paths) = env.var("EXCLUDE_PATHS") {
        let exclude_string = exclude_paths.to_string();
        let exclude_list: Vec<&str> = exclude_string.split(',').collect();
        for exclude in exclude_list {
            let exclude = exclude.trim();
            if exclude.ends_with("/*") {
                let prefix = &exclude[..exclude.len()-2];
                if path.starts_with(prefix) { return Ok(()); }
            } else if path == exclude || path.starts_with(&format!("{}/", exclude)) {
                return Ok(());
            }
        }
    }
    
    let api_url = env.var("TRAFFIC_ANALYTICS_API_URL").ok().map(|v| v.to_string()).unwrap_or_default();
    let tracking_code = env.var("TRACKING_CODE").ok().map(|v| v.to_string()).unwrap_or_default();
    
    if api_url.is_empty() || tracking_code.is_empty() {
        return Ok(());
    }
    
    let headers = req.headers();
    let user_agent = headers.get("User-Agent").ok().flatten().unwrap_or_default();
    let truncated_ua = if user_agent.len() > 2048 { &user_agent[..2048] } else { &user_agent };
    
    let now = js_sys::Date::new_0();
    let timestamp = now.to_iso_string().as_string().unwrap_or_default();
    
  
    let payload = json!({
        "user_agent": truncated_ua,
        "ip_address": headers.get("CF-Connecting-IP").ok().flatten()
            .or_else(|| headers.get("X-Real-IP").ok().flatten())
            .unwrap_or_else(|| "Unknown".to_string()),
        "url": url.to_string(),
        "referer": headers.get("Referer").ok().flatten(),
        "country": headers.get("CF-IPCountry").ok().flatten(),
        "cf_ray": headers.get("CF-Ray").ok().flatten(),
        "method": req.method().to_string(),
        "timestamp": timestamp,
        "source": "worker"
    });
    
    let json_body = serde_json::to_string(&payload).unwrap_or_default();
    if json_body.is_empty() { return Ok(()); }
    
    let endpoint = format!("{}/track/{}", api_url, tracking_code);
    
    let request_headers = Headers::new();
    request_headers.set("Content-Type", "application/json")?;
    request_headers.set("User-Agent", truncated_ua)?;
    request_headers.set("X-Worker-Version", "1.0.1")?;
    
    let mut init = RequestInit::new();
    init.method = Method::Post;
    init.headers = request_headers;
    init.body = Some(json_body.into());
    
    let request = Request::new_with_init(&endpoint, &init)?;
    let _ = Fetch::Request(request).send().await;
    
    Ok(())
}

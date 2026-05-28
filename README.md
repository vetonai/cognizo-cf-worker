# Cloudflare Workers Traffic Analytics

A lightweight Rust-based Cloudflare Worker for traffic analytics and request tracking.

## Features

- Real-time traffic analytics
- Static asset filtering
- Configurable logging levels
- Custom exclude paths support
- Non-blocking request forwarding

## Quick Start

1. **Deploy to Cloudflare Workers:**
   - Fork this repository
   - Connect to Cloudflare Workers via Git integration
   - Set required environment variables

2. **Environment Variables:**
   ```
   TRAFFIC_ANALYTICS_API_URL=https://your-analytics-api.com
   TRACKING_CODE=your-tracking-code
   LOG_LEVEL=INFO
   EXCLUDE_PATHS=/admin/*,/api/health
   ```

3. **Build Locally:**
   ```bash
   ./build.sh
   ```

## Configuration

### Log Levels
- `DEBUG`: Log all requests
- `INFO`: Log successful operations
- `WARN`: Log warnings
- `ERROR`: Log errors only

### Static Asset Filtering
The worker automatically excludes common static assets:
- Images: jpg, png, gif, svg, etc.
- Fonts: woff, ttf, eot, etc.
- Media: mp4, mp3, wav, etc.
- Documents: pdf, zip, etc.

### Custom Exclusions
Use the `EXCLUDE_PATHS` environment variable to exclude specific paths:
```
EXCLUDE_PATHS=/admin/*,/api/health,/dashboard
```

## Development

### Prerequisites
- Rust (installed automatically by build script)
- wasm32-unknown-unknown target
- worker-build tool

### Building
```bash
# Install dependencies and build
./build.sh

# Or manually:
rustup target add wasm32-unknown-unknown
cargo install worker-build
worker-build --release
```

### Local Development
```bash
# Install Wrangler CLI
npm install -g wrangler

# Login to Cloudflare
wrangler login

# Start local development
wrangler dev
```


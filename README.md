# wolapp

## Documentation

- https://github.com/seanmonstar/warp/tree/master/examples
- https://www.sheshbabu.com/posts/rust-module-system/

## Develop

```bash
cargo run
```

## Build docker image for Home Assistant

```bash
docker buildx build --platform linux/amd64,linux/arm64 -t tchoupinax/wolapp:v0.0.3-rc1 .
```

# Dynamic Image Processing at the Edge

A Spin application demonstrating CPU-intensive image resizing and format transcoding at the edge using WebAssembly components.

The component runs in a completely hermetic sandbox (`allowed_outbound_hosts = []`), decodes incoming image payloads from request bodies, downsamples with Lanczos3 filtering, and encodes to JPEG in linear memory.

## Building and Running

```bash
spin up --build
```

## Test image conversion:

```bash
curl -X POST http://localhost:3000/ \
  -H "content-type: image/jpeg" \
  --data-binary "@sample.jpg" \
  --output "output.jpg"
```

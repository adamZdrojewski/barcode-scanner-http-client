# Barcode Scanner HTTP Client
Lightweight Docker container for forwarding USB barcode scanner input to your HTTP API endpoint.


## Installation/Setup
The easiest way to install **barcode-scanner-http-client** is with a docker-compose file.  Below is an example, make sure to replace the placeholder values with your actual ones.
```yaml
services:
    barcode-scanner-http-client:
        image: adamzdrojewski/barcode-scanner-http-client
        container_name: barcode-scanner-http-client
        restart: unless-stopped
        devices:
            - <scanner-device-path>:/dev/input/scanner
        environment:
            SCANNER_DEVICE_PATH: "/dev/input/scanner"
            HTTP_SERVER_ADDRESS: "<http-api-endpoint>"
```
You will need to replace `<scanner-device-path>` with the actual path to your scanner input device.  It should be mounted somewhere in the `/dev/input` directory.  You will need to replace `<scanner-device-path>` with the full path of the device, example: `/dev/input/event1`.  Just make sure to replace the number 1 with whatever number your scanner is mounted to.

| Environment Variables | Description |
| --------------------- | ----------- |
| SCANNER_DEVICE_PATH | Path inside the container where the scanner device is mapped.  If you left the default path when mounting the scanner device to the container, you can leave this as default. |
| HTTP_SERVER_ADDRESS | HTTP API endpoint that gets hit when a barcode is scanned. |


## Usage
Once the container is up and running you should be able to scan barcodes!  The container will make a GET request to the HTTP API endpoint that you specified every time a barcode is scanned.  It will provide the query parameter `barcode` with the barcode's value for you to use however you would like.

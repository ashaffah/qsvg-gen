# SVG Generator API

This is a simple API built with Rust and Rocket that generates SVG strings based on the provided data.

## Endpoints

GET /api/svg
This endpoint generates an SVG string based on the provided data.

## Parameters

data: A string that will be used to generate the SVG string.
Response
A JSON object with a single key-value pair, where the key is "svg" and the value is the generated SVG string.

### POST /api/svg

This endpoint generates an SVG string based on the provided data.

### Request Body

The request body should contain a string that will be used to generate the SVG string.

### Response

A JSON object with a single key-value pair, where the key is "svg" and the value is the generated SVG string.

## CORS

This API allows all origins for CORS (Cross-Origin Resource Sharing), which means that it can be accessed from any domain. It allows GET and POST methods and credentials.

## Running the API

To run the API, use the following command:

```bash
cargo run
```

This will start the Rocket server and the API will be available at http://localhost:8000/api.

## Dependencies

This API uses the following dependencies:

rocket: A web framework for Rust.
rocket_cors: A CORS fairing for Rocket applications.
qirust: A library that provides the generate_svg_string function used to generate SVG strings.

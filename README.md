# Start the server

`cargo run --bin server`

# Run the client

`cargo run --bin client`

# Testing with curl

## Displaying binary output in console

```sh
curl -v --http2-prior-knowledge localhost:50051/hello.Greeter/SayHello \
     -H "Content-Type: application/grpc" \
     --data-binary @m4 --output -
```

```out
* Host localhost:50051 was resolved.
* IPv6: ::1
* IPv4: 127.0.0.1
*   Trying [::1]:50051...
* Connected to localhost (::1) port 50051
* [HTTP/2] [1] OPENED stream for http://localhost:50051/hello.Greeter/SayHello
* [HTTP/2] [1] [:method: POST]
* [HTTP/2] [1] [:scheme: http]
* [HTTP/2] [1] [:authority: localhost:50051]
* [HTTP/2] [1] [:path: /hello.Greeter/SayHello]
* [HTTP/2] [1] [user-agent: curl/8.6.0]
* [HTTP/2] [1] [accept: */*]
* [HTTP/2] [1] [content-type: application/grpc]
* [HTTP/2] [1] [content-length: 12]
> POST /hello.Greeter/SayHello HTTP/2
> Host: localhost:50051
> User-Agent: curl/8.6.0
> Accept: */*
> Content-Type: application/grpc
> Content-Length: 12
>
< HTTP/2 200
< content-type: application/grpc
< date: Wed, 05 Feb 2025 10:49:16 GMT
<

< grpc-status: 0
* Connection #0 to host localhost left intact
Hello, Tonic!%

```

## Redirecting output to a file

```sh
curl -v --http2-prior-knowledge localhost:50051/hello.Greeter/SayHello \
     -H "Content-Type: application/grpc" \
     --data-binary @m4 --output response.out
```

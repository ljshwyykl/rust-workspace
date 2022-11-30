```
protoc -I=/Users/wei/Desktop/project/learn-rust/work/helloworld-tonic/proto helloworld.proto \
    --js_out=import_style=commonjs:/Users/wei/Desktop/project/learn-rust/work/helloworld-tonic/proto \
    --grpc-web_out=import_style=commonjs,mode=grpcwebtext:/Users/wei/Desktop/project/learn-rust/work/helloworld-tonic/proto


    protoc -I=. ./helloworld.proto --js_out=import_style=commonjs:./out --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:./out 
```
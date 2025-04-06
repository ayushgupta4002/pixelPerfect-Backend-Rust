FROM messense/rust-musl-cross:x86_64-musl as chef
WORKDIR /imgeditor

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl 


FROM scratch
COPY --from=builder /imgeditor/target/x86_64-unknown-linux-musl/release/imgeditor /imgeditor
ENTRYPOINT ["/imgeditor"]
EXPOSE 5000

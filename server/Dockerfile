FROM rust
WORKDIR /server
COPY . .
RUN SQLX_OFFLINE=true cargo install --path .
RUN rm -r /server/target
CMD ["htmx_test"]

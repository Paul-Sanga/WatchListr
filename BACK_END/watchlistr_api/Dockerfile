# STAGE 1
# This base image, messense/rust-musl-cross, is designed  for compiling static Rust binaries
FROM messense/rust-musl-cross:x86_64-musl AS chef
# In order to build the codebase without a live database connection, set the sqlx environment to offline
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
# Change working directory to project root ./
WORKDIR /watchlistr_api

FROM chef AS planner
# Copy Source Files
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /watchlistr_api/recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .


# Build the application, target x64 bit architecture, running a generic linux env with the musl C library -> standalone binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# STAGE 2
# Create a minimal base image
FROM scratch
# Copy the binaries over to our new workspace
COPY --from=builder /watchlistr_api/target/x86_64-unknown-linux-musl/release/watchlistr_api /watchlistr_api
ENTRYPOINT [ "/watchlistr_api" ]
EXPOSE 3000
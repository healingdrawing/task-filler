FROM rust:1.63-buster

COPY ./maps /filler/maps
COPY ./linux_robots /filler/linux_robots
COPY ./m1_robots /filler/m1_robots
COPY ./linux_game_engine /filler/linux_game_engine
COPY ./m1_game_engine /filler/m1_game_engine
COPY ./solution/filler /filler/filler  # Assuming your Rust project is in the ./solution/filler directory

WORKDIR /filler/filler

# Build the Rust project inside the /filler/filler directory
RUN cargo build

# Copy the binary to /solution/ with the name "fillerbot"
RUN cp /filler/filler/target/debug/filler /solution/fillerbot

# Set the WORKDIR to /filler/
WORKDIR /filler/

ENTRYPOINT ["/bin/bash"]
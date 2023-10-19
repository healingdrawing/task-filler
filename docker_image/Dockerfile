FROM rust:1.63-buster

COPY ./maps /filler/maps
COPY ./linux_robots /filler/linux_robots
COPY ./m1_robots /filler/m1_robots
COPY ./linux_game_engine /filler/linux_game_engine
COPY ./m1_game_engine /filler/m1_game_engine
COPY ./solution /filler/solution

# Set the working directory to the 'filler' project directory
WORKDIR /filler/solution/filler

# Build the Rust project inside the /filler/solution/filler directory
RUN cargo build

# Copy the binary to /solution/ with the name "fillerbot"
RUN cp /filler/solution/filler/target/debug/filler /filler/solution/fillerbot

# Set the WORKDIR to /filler/
WORKDIR /filler/

ENTRYPOINT ["/bin/bash"]

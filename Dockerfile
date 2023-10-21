FROM rust:1.63-buster

COPY ./docker_image/maps			        /filler/maps
COPY ./docker_image/linux_robots		    /filler/linux_robots
COPY ./docker_image/m1_robots		    /filler/m1_robots
COPY ./docker_image/linux_game_engine	/filler/linux_game_engine
COPY ./docker_image/m1_game_engine	    /filler/m1_game_engine
COPY ./filler			    /filler/solution

WORKDIR /filler

# Execute cargo build inside the "solution" folder
RUN cargo build --release --manifest-path=solution/Cargo.toml

RUN cp "solution/target/release/bunny" "solution/bunny"

ENTRYPOINT /bin/bash

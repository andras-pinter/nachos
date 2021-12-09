FROM rust

RUN apt update -qq &&\
    apt install -qq -yy \
    libcairo2-dev libtool-bin uuid-dev libossp-uuid-dev libavcodec-dev \
    libavformat-dev libavutil-dev freerdp2-dev libpango1.0-dev libssh2-1-dev libtelnet-dev \
    libvncserver-dev libwebsockets-dev libpulse-dev libssl-dev libvorbis-dev libwebp-dev npm

RUN cargo install cargo-make

EXPOSE 8080
EXPOSE 8000

WORKDIR /root
ENTRYPOINT ["/bin/bash"]

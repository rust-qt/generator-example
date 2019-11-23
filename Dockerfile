# Base image `riateche/ritual_builder` is built
# from `docker.builder.dockerfile` in the ritual repository.
FROM riateche/ritual_builder:0.0.2
# Install the target C++ library.
RUN apt-get update && \
    apt-get install unzip && \
    curl -o clipper.zip https://netix.dl.sourceforge.net/project/polyclipping/clipper_ver6.4.2.zip && \
    unzip clipper.zip -d clipper && \
    mkdir /clipper_build && \
    cd /clipper_build && \
    cmake /clipper/cpp && \
    make && \
    make install
# If your library is not in system directories, adjust
# environment variables to allow ritual and the generated crate
# find the library.
ENV INCLUDE_PATH=/usr/local/include/polyclipping
ENV RITUAL_INCLUDE_PATH=/usr/local/include/polyclipping
ENV LIBRARY_PATH=/usr/local/lib
ENV LD_LIBRARY_PATH=/usr/local/lib

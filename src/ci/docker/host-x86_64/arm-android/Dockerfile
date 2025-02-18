FROM ubuntu:16.04

COPY scripts/android-base-apt-get.sh /scripts/
RUN sh /scripts/android-base-apt-get.sh

COPY scripts/android-ndk.sh /scripts/
RUN . /scripts/android-ndk.sh && \
    download_and_make_toolchain android-ndk-r15c-linux-x86_64.zip arm 14

RUN dpkg --add-architecture i386 && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
  libgl1-mesa-glx \
  libpulse0 \
  libstdc++6:i386 \
  openjdk-9-jre-headless \
  tzdata \
  wget \
  python3

COPY scripts/android-sdk.sh /scripts/
COPY scripts/android-sdk-manager.py /scripts/
COPY host-x86_64/arm-android/android-sdk.lock /android/sdk/android-sdk.lock
RUN /scripts/android-sdk.sh

ENV PATH=$PATH:/android/sdk/emulator
ENV PATH=$PATH:/android/sdk/tools
ENV PATH=$PATH:/android/sdk/platform-tools

ENV TARGETS=arm-linux-androideabi

# We are intentionally allowing an old toolchain on this builder (and that's
# incompatible with LLVM downloads today).
ENV NO_DOWNLOAD_CI_LLVM 1

ENV RUST_CONFIGURE_ARGS --arm-linux-androideabi-ndk=/android/ndk/arm-14 \
    --set llvm.allow-old-toolchain

ENV SCRIPT python3 ../x.py --stage 2 test --host='' --target $TARGETS

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

COPY scripts/cmake.sh /scripts/
RUN /scripts/cmake.sh

COPY scripts/android-start-emulator.sh /scripts/
ENTRYPOINT ["/scripts/android-start-emulator.sh"]

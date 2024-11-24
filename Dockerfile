FROM busybox AS copy
COPY target/x86_64-unknown-linux-musl/release/volume-init-s3-objects /volume-init-s3-objects-x86_64
COPY target/aarch64-unknown-linux-musl/release/volume-init-s3-objects /volume-init-s3-objects-aarch64
# move to /volume-init-s3-objects based on the architecture
RUN if [ "$(uname -m)" = "x86_64" ]; then \
      mv /volume-init-s3-objects-x86_64 /volume-init-s3-objects; \
    else \
      mv /volume-init-s3-objects-aarch64 /volume-init-s3-objects; \
    fi

FROM scratch
COPY --from=copy /volume-init-s3-objects /volume-init-s3-objects
ENTRYPOINT [ "/volume-init-s3-objects" ]

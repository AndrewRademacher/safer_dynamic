build-dynomath:
    cargo build --package dynomath --release
    cargo test --package dynomath --features c-headers -- generate_headers

configure-ccalc:
    mkdir -p ccalc/build
    cd ccalc/build && cmake ..

build-ccalc:
    cd ccalc/build && cmake --build .

clean:
    cargo clean
    rm -rf ccalc/build
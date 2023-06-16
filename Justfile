build-dynomath:
    cargo build --package dynomath --release
    cargo test --package dynomath --features c-headers -- generate_headers

    mkdir -p ccalc/vendor/dynomath/include
    mkdir -p ccalc/vendor/dynomath/lib

    cp dynomath/include/dynomath.h ccalc/vendor/dynomath/include/dynomath.h
    cp target/release/libdynomath.* ccalc/vendor/dynomath/lib/

configure-ccalc:
    mkdir -p ccalc/build
    cd ccalc/build && cmake ..

build-ccalc:
    cd ccalc/build && cmake --build .

clean:
    cargo clean
    rm -rf ccalc/build
    rm -rf ccalc/vendor

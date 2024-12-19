remove_deps:
    cargo machete --with-metadata

check_timing:
    cargo build --timings

remove_unused_features:
    cargo features prune

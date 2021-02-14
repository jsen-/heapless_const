set -euxo pipefail

main() {
    case $TARGET in
        thumb*m-none-eabi)
            xargo check --target $TARGET
            ;;
        x86_64-unknown-linux-gnu)
            cargo check --target $TARGET

            cargo test --target $TARGET
            cargo test --target $TARGET --release

            export RUSTFLAGS="-Z sanitizer=thread"
            export RUST_TEST_THREADS=1
            export TSAN_OPTIONS="suppressions=$(pwd)/suppressions.txt"

            cargo test --test tsan --target $TARGET
            cargo test --test tsan --target $TARGET --release
            ;;
        *)
            # unhandled case
            exit 1
            ;;
    esac
}

main

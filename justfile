hot:
    dx serve --hot-patch --features dev_native

web:
    bevy run -F web dev --release web --open

run:
    bevy run -F dev

release:
    bevy build --locked --release --features="web" --yes web --bundle
    zip -r build.zip ~/.cargo/global-target/bevy_web/web-release/markoff/
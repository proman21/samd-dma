features := "samd21g18a samd21e18a samd21j18a samd51j19a samd51j20a samd51g19a"

release version:
    git diff HEAD --exit-code --name-only
    just build-docs
    cargo readme > README.md
    cargo bump -g {{ version }}
    cargo publish --dry-run --no-verify

build-docs:
    #!/usr/bin/env sh
    cargo clean --doc
    cargo doc --no-deps --features samd21g18a
    rsync -r --no-times --checksum --delete --exclude samd_dma/ --exclude .lock --exclude _config.yml target/doc/ docs
    for feature in {{features}}; do
        cargo doc --no-deps --features $feature
        rsync -r --no-times --checksum --delete target/doc/samd_dma/ docs/$feature
    done
    cargo readme > docs/index.md
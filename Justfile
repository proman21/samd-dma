features := "samd21g18a samd21e18a samd21j18a samd51j19a samd51j20a samd51g19a"

release version:
    #!/usr/bin/env sh
    git diff HEAD --exit-code --name-only
    if [ -z $(git ls-files --exclude-standard --others) ]; then
        echo "Cannot make release with a dirty working directory."
    else
        just build-docs
        cargo readme > README.md
        cargo bump -g {{ version }}
        cargo publish --dry-run --no-verify
    fi

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
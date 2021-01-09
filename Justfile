features := "samd21g18a samd21e18a samd21j18a samd51j19a samd51j20a samd51g19a"

check:
    cargo check --features samd51j19a
    cargo check --features samd21j18a

release version:
    git diff HEAD --exit-code --name-only
    cargo bump {{ version }}
    git commit -am "chore: bump version."
    just check
    just build-docs
    cargo readme > README.md
    if [ -n "$(git status -s)" ]; then \
        git add -A .; \
        git commit -am "chore: Generate docs and readme."; \
    fi
    cargo bump -g {{ version }}
    cargo publish --no-verify

build-docs:
    cargo clean --doc
    cargo doc --no-deps --features samd21g18a
    rsync -r --no-times --checksum --delete --exclude samd_dma/ --exclude .lock --exclude _config.yml target/doc/ docs
    for feature in {{features}}; do \
        cargo doc --no-deps --features $feature ; \
        rsync -r --no-times --checksum --delete target/doc/samd_dma/ docs/$feature ; \
    done
    cargo readme > docs/index.md
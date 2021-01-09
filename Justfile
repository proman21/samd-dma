features := "samd21g samd21e samd21j samd51j samd51g"

check:
    cargo check --features samd51j
    cargo check --features samd21j

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
    cargo doc --no-deps --features samd21g
    rsync -r --no-times --checksum --delete --exclude samd_dma/ --exclude .lock --exclude _config.yml target/doc/ docs
    for feature in {{features}}; do \
        cargo doc --no-deps --features $feature ; \
        rsync -r --no-times --checksum --delete target/doc/samd_dma/ docs/$feature ; \
    done
    cargo readme > docs/index.md

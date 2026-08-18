VERSION := $(shell grep -m1 '^version' Cargo.toml | cut -d '"' -f2)
TAG := v$(VERSION)
TARGET := $(shell rustc -vV | sed -n 's/host: //p')
ARCHIVE := tempo-shortcut-$(TARGET).tar.gz

.PHONY: release
release:
	@if git rev-parse "$(TAG)" >/dev/null 2>&1; then \
		echo "Tag $(TAG) already exists, bump the version in Cargo.toml first"; exit 1; \
	fi
	cargo build --release
	tar -C target/release -czf $(ARCHIVE) tempo-shortcut
	git tag -a "$(TAG)" -m "$(TAG)"
	git push origin "$(TAG)"
	gh release create "$(TAG)" $(ARCHIVE) --title "$(TAG)" --generate-notes
	rm -f $(ARCHIVE)

.PHONY: publish
publish:
	cargo publish

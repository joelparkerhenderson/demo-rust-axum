top = $(shell git rev-parse --show-toplevel)
projects = $(sort $(dir $(shell find "${top}" -name "Cargo.toml")))

all: demo-rust-axum-by-joel-parker-henderson.epub demo-rust-axum-by-joel-parker-henderson.pdf

.PHONY: clean
clean:
	rm -f demo-rust-axum-by-joel-parker-henderson.*

demo-rust-axum-by-joel-parker-henderson.tmp:
	sed 's~^---~<div style="page-break-before:always;"></div>~' "${top}/README.md" > "${top}/demo-rust-axum-by-joel-parker-henderson.tmp"

demo-rust-axum-by-joel-parker-henderson.epub: demo-rust-axum-by-joel-parker-henderson.tmp
	pandoc-from-markdown-to-epub "${top}/demo-rust-axum-by-joel-parker-henderson.tmp" --metadata title="Demo Rust Axum" -o demo-rust-axum-by-joel-parker-henderson.epub

demo-rust-axum-by-joel-parker-henderson.pdf: demo-rust-axum-by-joel-parker-henderson.tmp
	pandoc-from-markdown-to-pdf "${top}/demo-rust-axum-by-joel-parker-henderson.tmp" --metadata title="Demo Rust Axum" -o demo-rust-axum-by-joel-parker-henderson.pdf

.PHONY: cargo-update
cargo-update:
	for dir in $(sort $(dir $(shell find "${top}" -name "Cargo.toml"))); do \
		echo "Update $${dir}" && \
		cd $${dir} && \
		cargo update && \
		cargo check --all-targets && \
		cargo build --all-targets; \
	done

.PHONY: cargo-upgrade
cargo-upgrade:
	for dir in $projects; do \
		echo "cargo-upgrade $${dir}" && \
		cd $${dir} && \
		cargo update && \
		cargo upgrade --recursive true && \
		cargo check --all-targets && \
		cargo build --all-targets; \
	done

.PHONY: cargo-upgrade-incompatible
cargo-upgrade-incompatible:
	for dir in $projects; do \
		echo "cargos-upgrade-incompatible $${dir}" && \
		cd $${dir} && \
		cargo update && \
		cargo upgrade  --incompatible --recursive true && \
		cargo check --all-targets && \
		cargo build --all-targets; \
	done

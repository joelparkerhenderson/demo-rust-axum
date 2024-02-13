.PHONY: clean
clean:
    rm -f demo-rust-axum-by-joelparkerhenderson.*

demo-rust-axum-by-joelparkerhenderson.epub:
    pandoc-from-markdown-to-epub README.md --metadata title="Demo Rust Axum" -o demo-rust-axum-by-joelparkerhenderson.epub

demo-rust-axum-by-joelparkerhenderson.pdf:
    pandoc-from-markdown-to-epub README.md --metadata title="Demo Rust Axum" -o demo-rust-axum-by-joelparkerhenderson.pdf

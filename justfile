default:
	just --list --unsorted

alias rp := release-pr
alias pt := push-tag
alias gtc := gen-test-case

fmt:
	cargo fmt --all
	taplo fmt

lint: fmt
	cargo clippy --all-targets --all-features --workspace

check:
	cargo fmt --all -- --check
	taplo fmt --check
	cargo check --all-targets --all-features --workspace
	cargo clippy --all-targets --all-features --workspace -- -D warnings

release-pr tag:
	git checkout -b "release-{{tag}}"
	git cliff --tag {{tag}} -o CHANGELOG.md
	# cargo-edit
	cargo set-version {{tag}}
	git commit -am "chore(release): {{tag}}"
	git push --set-upstream origin release-{{tag}}

push-tag tag:
	git tag {{tag}}
	git push origin {{tag}}

gen-test-case:
	cargo run --package xtask --bin gen_test_case

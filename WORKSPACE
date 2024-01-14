load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Rust gazelle plugin

GAZELLE_RUST_COMMIT = "00e88bceaa1a1c35d9c3019f65f3e20459fafe33"

GAZELLE_RUST_SHA256 = ""

http_archive(
    name = "gazelle_rust",
    # sha256 = GAZELLE_RUST_SHA256,
    strip_prefix = "gazelle_rust-{}".format(GAZELLE_RUST_COMMIT),
    url = "https://github.com/Calsign/gazelle_rust/archive/{}.zip".format(GAZELLE_RUST_COMMIT),
)

# Rust

RULES_RUST_VERSION = "0.31.0"

http_archive(
    name = "rules_rust",
    integrity = "sha256-NquPn6yudFycnBsz0iViPZduePLMP3KbeXPYwgk0q5U=",
    patches = ["@gazelle_rust//patches:rules_rust.patch"],
    urls = [
        "https://github.com/bazelbuild/rules_rust/releases/download/{v}/rules_rust-v{v}.tar.gz".format(v = RULES_RUST_VERSION),
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
)

load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_dependencies")

rust_analyzer_dependencies()

# Gazelle
http_archive(
    name = "io_bazel_rules_go",
    sha256 = "91585017debb61982f7054c9688857a2ad1fd823fc3f9cb05048b0025c47d023",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.42.0/rules_go-v0.42.0.zip",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.42.0/rules_go-v0.42.0.zip",
    ],
)

http_archive(
    name = "bazel_gazelle",
    sha256 = "b7387f72efb59f876e4daae42f1d3912d0d45563eac7cb23d1de0b094ab588cf",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.34.0/bazel-gazelle-v0.34.0.tar.gz",
        "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.34.0/bazel-gazelle-v0.34.0.tar.gz",
    ],
)

load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies")
load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(version = "1.20.5")

gazelle_dependencies()

# Load rust gazelle plugin

load("@gazelle_rust//:deps1.bzl", "gazelle_rust_dependencies1")

gazelle_rust_dependencies1()

load("@gazelle_rust//:deps2.bzl", "gazelle_rust_dependencies2")

gazelle_rust_dependencies2()

# Rust Crate Universe

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    # Regenerate using:
    # CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:cargo-bazel-lock.json",
    manifests = [
        "//:git-rust/Cargo.toml",
    ],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()


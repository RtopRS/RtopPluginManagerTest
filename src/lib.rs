#![allow(
    clippy::implicit_return,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::missing_docs_in_private_items,
    clippy::separated_literal_suffix,
    clippy::missing_inline_in_public_items,
    clippy::non_ascii_literal,
    clippy::must_use_candidate,
    clippy::mod_module_files,
    clippy::else_if_without_else,
    clippy::unused_self,
    clippy::cast_precision_loss,
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::exit,
    clippy::too_many_lines,
    clippy::exhaustive_structs,
    clippy::single_char_lifetime_names,
    clippy::integer_division,
    clippy::separated_literal_suffix,
    clippy::else_if_without_else,
    clippy::indexing_slicing,
    clippy::cast_possible_truncation
)]
#![deny(
    clippy::needless_return,
    clippy::str_to_string,
    clippy::implicit_clone,
    clippy::needless_pass_by_value,
    clippy::semicolon_if_nothing_returned,
    clippy::wildcard_imports,
    clippy::single_match_else,
    clippy::single_match,
    clippy::let_underscore_drop,
    clippy::shadow_reuse,
    clippy::expect_used,
    clippy::suboptimal_flops,
    clippy::redundant_else
)]

pub mod commands;
pub mod git;
pub mod util;

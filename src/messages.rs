pub const TOOL_MAX_OUTPUT_LINES: usize = 10;

pub enum MessageBlock {
    AssistantText(String),
    Thinking {
        summary: String,
        body: String,
    },
    ToolOutput {
        title: String,
        command: String,
        output: Vec<String>,
        collapsed: bool,
    },
    InlineResult {
        icon: &'static str,
        label: String,
    },
    UserMessage(String),
    Error(String),
}

pub fn message_height(msg: &MessageBlock, width: u16) -> u16 {
    match msg {
        MessageBlock::AssistantText(text) => {
            let inner_w = width.saturating_sub(3);
            1 + wrapped_line_count(text, inner_w) as u16 + 1
        },
        MessageBlock::Thinking { body, .. } => {
            let inner_w = width.saturating_sub(3);
            1 + 1 + 1 + wrapped_line_count(body, inner_w) as u16 + 1
        },
        MessageBlock::ToolOutput {
            command,
            output,
            collapsed,
            ..
        } => {
            let inner_w = width.saturating_sub(5);
            let mut h: u16 = 1 + 1;
            if !command.is_empty() {
                h += 2;
            }
            h += 1;
            if *collapsed && output.len() > TOOL_MAX_OUTPUT_LINES {
                for line in output.iter().take(TOOL_MAX_OUTPUT_LINES) {
                    h += wrapped_line_count(line, inner_w) as u16;
                }
                h += 1 + 1 + 1 + 1; // "…" + blank + "Click to expand" + pad
            } else {
                for line in output {
                    h += wrapped_line_count(line, inner_w) as u16;
                }
                h += 1 + 1;
            }
            h
        },
        MessageBlock::UserMessage(text) => {
            let inner_w = width.saturating_sub(4);
            1 + 1 + wrapped_line_count(text, inner_w) as u16 + 1
        },
        MessageBlock::InlineResult { .. } => 2,
        MessageBlock::Error(_) => 3,
    }
}

pub fn wrapped_line_count(text: &str, width: u16) -> usize {
    if width == 0 {
        return 1;
    }
    let w = width as usize;
    text.split('\n')
        .map(|line| {
            if line.is_empty() {
                1
            } else {
                line.len().div_ceil(w)
            }
        })
        .sum::<usize>()
        .max(1)
}

pub fn demo_messages() -> Vec<MessageBlock> {
    vec![
        MessageBlock::ToolOutput {
            title: "Summarizes tracked file diff".into(),
            command: "git diff --stat".into(),
            output: vec![
                " Cargo.lock                                         | 106 +++-".into(),
                " Cargo.toml                                         |   3 +-".into(),
                " crates/config/src/template.rs                      |  10 +-".into(),
                " crates/config/src/validate/schema_map.rs           |  18 +".into(),
                " crates/external-agents/Cargo.toml                  |  29 +-".into(),
                " crates/external-agents/src/runtimes/acp.rs         | 681 ++++++++++++++++++++-"
                    .into(),
                " crates/external-agents/src/runtimes/claude_code.rs | 230 ++++++-".into(),
                " crates/external-agents/src/runtimes/codex.rs       | 448 +++++++++++++-".into(),
                " crates/external-agents/src/runtimes/mod.rs         |   2 +".into(),
                " crates/external-agents/src/types.rs                |  46 +-".into(),
                " crates/web/ui/src/components/SessionHeader.tsx      | 112 ++++".into(),
                " crates/web/ui/e2e/specs/agents.spec.js             |  89 +++".into(),
                " 12 files changed, 1774 insertions(+), 12 deletions(-)".into(),
            ],
            collapsed: true,
        },
        MessageBlock::Thinking {
            summary: "Considering system checks".into(),
            body: "I need to run a format check and test external connections. I'm also thinking \
                   about checking the gateway and possibly linting the code. Full tests could be \
                   useful too. Oh, and what about the UI? Should I include that in my testing? \
                   It seems like there's a lot to cover, and I want to make sure everything is \
                   in good shape. I'll ensure I'm thorough with all these aspects!"
                .into(),
        },
        MessageBlock::AssistantText(
            "CI is failing because the required local/* status contexts are missing for the \
             latest commit, not because those jobs found code failures. I'm running \
             ./scripts/local-validate.sh 985 now so the local status contexts get posted \
             for 95c58e7a4."
                .into(),
        ),
        MessageBlock::ToolOutput {
            title: "Checks Rust formatting".into(),
            command: "cargo fmt --all -- --check".into(),
            output: vec!["(no output)".into()],
            collapsed: false,
        },
        MessageBlock::ToolOutput {
            title: "Runs external agent ACP tests".into(),
            command: "cargo test -p moltis-external-agents acp".into(),
            output: vec![
                "Blocking waiting for file lock on package cache".into(),
                " Compiling moltis-external-agents v0.1.0".into(),
                " Finished `test` profile [unoptimized + debuginfo] target(s) in 2.02s".into(),
                String::new(),
                "running 4 tests".into(),
                "test runtimes::acp::tests::prompt_response_end_turn_appends_done_event ... ok"
                    .into(),
                "test runtimes::acp::tests::streaming_text_produces_content_events ... ok".into(),
                "test runtimes::acp::tests::tool_call_round_trip ... ok".into(),
                "test runtimes::acp::tests::error_response_produces_error_event ... ok".into(),
                String::new(),
                "test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out".into(),
            ],
            collapsed: true,
        },
        MessageBlock::InlineResult {
            icon: "\u{2731}",
            label: "Glob \"crates/web/ui/e2e/specs/*.spec.js\" in . (49 matches)".into(),
        },
        MessageBlock::InlineResult {
            icon: "\u{2731}",
            label: "Grep \"SessionHeader|session header\" in crates/web/ui/e2e (105 matches)"
                .into(),
        },
        MessageBlock::InlineResult {
            icon: "\u{2192}",
            label: "Read crates/web/ui/e2e/specs/agents.spec.js [offset=180, limit=90]".into(),
        },
        MessageBlock::InlineResult {
            icon: "\u{2192}",
            label: "Read crates/web/ui/src/components/SessionHeader.tsx [offset=1, limit=220]"
                .into(),
        },
        MessageBlock::InlineResult {
            icon: "\u{25a3}",
            label: "Build \u{00b7} GPT-5.5".into(),
        },
        MessageBlock::Error("Connection timed out after 30s".into()),
        MessageBlock::Thinking {
            summary: "Planning deployment strategy".into(),
            body: "The tests all pass and formatting looks good. Now I need to think about \
                   the deployment. We have staging and production environments, and the \
                   external agents changes are significant enough that we should do a staged \
                   rollout. Let me check the deployment configs first."
                .into(),
        },
        MessageBlock::ToolOutput {
            title: "Reads deployment configuration".into(),
            command: "cat deploy/staging.toml".into(),
            output: vec![
                "[environment]".into(),
                "name = \"staging\"".into(),
                "region = \"us-east-1\"".into(),
                "replicas = 2".into(),
                String::new(),
                "[features]".into(),
                "external_agents = true".into(),
                "acp_runtime = true".into(),
                "codex_runtime = false".into(),
            ],
            collapsed: false,
        },
        MessageBlock::AssistantText(
            "The staging configuration already has external_agents enabled but codex_runtime \
             is disabled. Since this PR adds the codex runtime, we need to update the staging \
             config before deploying."
                .into(),
        ),
        MessageBlock::ToolOutput {
            title: "Runs integration tests against staging".into(),
            command: "cargo test --test integration -- --env staging".into(),
            output: vec![
                " Compiling integration-tests v0.1.0".into(),
                " Finished `test` profile [unoptimized + debuginfo] target(s) in 8.34s".into(),
                String::new(),
                "running 12 tests".into(),
                "test staging::health_check ... ok".into(),
                "test staging::agent_creation ... ok".into(),
                "test staging::acp_handshake ... ok".into(),
                "test staging::tool_execution ... ok".into(),
                "test staging::streaming_response ... ok".into(),
                "test staging::error_recovery ... ok".into(),
                "test staging::rate_limiting ... ok".into(),
                "test staging::auth_token_refresh ... ok".into(),
                "test staging::concurrent_sessions ... ok".into(),
                "test staging::websocket_reconnect ... ok".into(),
                "test staging::message_ordering ... ok".into(),
                "test staging::graceful_shutdown ... ok".into(),
                String::new(),
                "test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
                    .into(),
            ],
            collapsed: true,
        },
        MessageBlock::AssistantText(
            "All tests pass across unit tests, integration tests, and E2E browser tests. \
             The deployment is ready to proceed with the canary rollout strategy."
                .into(),
        ),
        MessageBlock::InlineResult {
            icon: "\u{25a3}",
            label: "Build \u{00b7} GPT-5.5".into(),
        },
    ]
}

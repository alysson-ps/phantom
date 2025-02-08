use dashmap::DashMap;
use ropey::Rope;
use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use tower_lsp::lsp_types::SemanticTokenType;
#[allow(unused)]
use tower_lsp::{Client, LanguageServer, LspService, Server};

use tower_lsp::lsp_types::{
    Diagnostic, InitializeParams, InitializeResult, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind,
};

#[allow(unused)]
#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<String, Rope>,
    diagnostics_map: DashMap<String, Vec<Diagnostic>>,
}

struct TextDocumentItem {
    uri: Url,
    text: String,
    version: i32,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "initialized!").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client.log_message(MessageType::INFO, "workspace folders changed!").await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client.log_message(MessageType::INFO, "configuration changed!").await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client.log_message(MessageType::INFO, "watched files have changed!").await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client.log_message(MessageType::INFO, "command executed!").await;

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client.log_message(MessageType::INFO, "file opened!").await;

        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
        })
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.client.log_message(MessageType::INFO, "file changed!").await;

        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
        })
        .await
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client.log_message(MessageType::INFO, "file saved!").await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client.log_message(MessageType::INFO, "file closed!").await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        self.client.log_message(MessageType::INFO, "completion triggered").await;

        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
            CompletionItem::new_simple("seila".to_string(), "More detail".to_string()),
        ])))
    }
}

impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        let rope = ropey::Rope::from_str(&params.text);
        self.document_map.insert(params.uri.to_string(), rope.clone());

        let _content = params.text;

        dbg!(params.uri);

        // let mut diagnostics = Vec::new();

        // self.client
        //     .publish_diagnostics(params.uri.clone(), diagnostics, Some(params.version))
        //     .await;
    }
}

#[tokio::main]
async fn main() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    #[cfg(feature = "runtime-agnostic")]
    let (stdin, stdout) = (stdin.compat(), stdout.compat_write());

    let (service, socket) = LspService::build(|client| Backend {
        client,
        document_map: DashMap::new(),
        diagnostics_map: DashMap::new(),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

// fn offset_to_position(offset: usize, rope: &Rope) -> Option<Position> {
//     let line = rope.try_char_to_line(offset).ok()?;
//     let first_char_of_line = rope.try_line_to_char(line).ok()?;
//     let column = offset - first_char_of_line;
//     Some(Position::new(line as u32, column as u32))
// }

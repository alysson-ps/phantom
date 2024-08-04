use std::collections::HashMap;

use dashmap::DashMap;
use phantom_language_server::chumsky::{parser, Func, ImCompleteSemanticToken, ParserResult};
use ropey::Rope;
use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

#[allow(unused)]
use tower_lsp::{Client, LanguageServer, LspService, Server};

use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, InitializeParams, InitializeResult, Position, Range,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};

#[allow(unused)]
#[derive(Debug)]
struct Backend {
    client: Client,
    ast_map: DashMap<String, HashMap<String, Func>>,
    document_map: DashMap<String, Rope>,
    semantic_token_map: DashMap<String, Vec<ImCompleteSemanticToken>>,
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

        // let uri: String = params.text_document.uri.to_string();
        // let content: String = params.text_document.text;

        // self.document_map.insert(uri.clone(), Rope::from_str(&content));

        // let diagnostics = self.check_syntax(&uri).await;

        // self.diagnostics_map.insert(uri.clone(), diagnostics.clone());
        // self.client.publish_diagnostics(params.text_document.uri, diagnostics, None).await;

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

        // let uri: String = params.text_document.uri.to_string();

        // if let Some(change) = params.content_changes.into_iter().last() {
        //     self.document_map.insert(uri.clone(), Rope::from_str(&change.text));
        //     let diagnostics = self.check_syntax(&uri).await;
        //     self.diagnostics_map.insert(uri.clone(), diagnostics.clone());
        //     self.client.publish_diagnostics(params.text_document.uri, diagnostics, None).await;
        // }
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client.log_message(MessageType::INFO, "file saved!").await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client.log_message(MessageType::INFO, "file closed!").await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
            CompletionItem::new_simple("seila".to_string(), "More detail".to_string()),
        ])))
    }
}

impl Backend {
    // async fn check_syntax(&self, uri: &str) -> Vec<Diagnostic> {
    //     let document = self.document_map.get(uri);

    //     if let Some(rope) = document {
    //         let mut diagnostics = Vec::new();
    //         for (i, line) in rope.lines().enumerate() {
    //             let line_content = line.to_string();
    //             if !line_content.trim_end().ends_with(';') && !line_content.trim().is_empty() {
    //                 diagnostics.push(Diagnostic {
    //                     range: Range {
    //                         start: Position::new(i as u32, 0),
    //                         end: Position::new(i as u32, line_content.len() as u32),
    //                     },
    //                     severity: Some(DiagnosticSeverity::ERROR),
    //                     code: None,
    //                     code_description: None,
    //                     source: Some("phantom-server".to_string()),
    //                     message: "Missing semicolon".to_string(),
    //                     related_information: None,
    //                     tags: None,
    //                     data: None,
    //                 });
    //             }
    //         }
    //         diagnostics
    //     } else {
    //         Vec::new()
    //     }
    // }

    async fn on_change(&self, params: TextDocumentItem) {
        let rope = ropey::Rope::from_str(&params.text);
        self.document_map.insert(params.uri.to_string(), rope.clone());

        let ParserResult {
            ast,
            parse_errors,
            semantic_tokens,
        } = parser(&params.text);

        let diagnostics = parse_errors
            .into_iter()
            .filter_map(|item| {
                let (message, span) = match item.reason() {
                    chumsky::error::SimpleReason::Unclosed { span, delimiter } => {
                        (format!("Unclosed delimiter {}", delimiter), span.clone())
                    }
                    chumsky::error::SimpleReason::Unexpected => (
                        format!(
                            "{}, expected {}",
                            if item.found().is_some() {
                                "Unexpected token in input"
                            } else {
                                "Unexpected end of input"
                            },
                            if item.expected().len() == 0 {
                                "something else".to_string()
                            } else {
                                item.expected()
                                    .map(|expected| match expected {
                                        Some(expected) => expected.to_string(),
                                        None => "end of input".to_string(),
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }
                        ),
                        item.span(),
                    ),
                    chumsky::error::SimpleReason::Custom(msg) => (msg.to_string(), item.span()),
                };

                || -> Option<Diagnostic> {
                    // let start_line = rope.try_char_to_line(span.start)?;
                    // let first_char = rope.try_line_to_char(start_line)?;
                    // let start_column = span.start - first_char;
                    let start_position = offset_to_position(span.start, &rope)?;
                    let end_position = offset_to_position(span.end, &rope)?;
                    // let end_line = rope.try_char_to_line(span.end)?;
                    // let first_char = rope.try_line_to_char(end_line)?;
                    // let end_column = span.end - first_char;
                    Some(Diagnostic::new_simple(
                        Range::new(start_position, end_position),
                        message,
                    ))
                }()
            })
            .collect::<Vec<_>>();

        self.client
            .publish_diagnostics(params.uri.clone(), diagnostics, Some(params.version))
            .await;

        if let Some(ast) = ast {
            self.ast_map.insert(params.uri.to_string(), ast);
        }
        // self.client
        //     .log_message(MessageType::INFO, &format!("{:?}", semantic_tokens))
        //     .await;
        self.semantic_token_map.insert(params.uri.to_string(), semantic_tokens);
    }
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "runtime-agnostic")]
    use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    #[cfg(feature = "runtime-agnostic")]
    let (stdin, stdout) = (stdin.compat(), stdout.compat_write());

    let (service, socket) = LspService::build(|client| Backend {
        client,
        ast_map: DashMap::new(),
        document_map: DashMap::new(),
        semantic_token_map: DashMap::new(),
        diagnostics_map: DashMap::new(),
    })
    .finish();

    serde_json::json!({"test": 20});
    Server::new(stdin, stdout, socket).serve(service).await;
}

fn offset_to_position(offset: usize, rope: &Rope) -> Option<Position> {
    let line = rope.try_char_to_line(offset).ok()?;
    let first_char_of_line = rope.try_line_to_char(line).ok()?;
    let column = offset - first_char_of_line;
    Some(Position::new(line as u32, column as u32))
}

// fn main() {
//     // let src = "$x = 5; // This is a comment
//     //     echo $x;

//     //     /* This is a multi-line comment */
//     //     if ($x > 5) $maior = true
//     //     ";

//     // let src = "$x = 5; // This is a comment";

//     let src = r#"
//     function() {
//         if ($x > 5) $maior = true;
//         echo $maior;

//         $y = 5;
//     }
//     "#;

//     let result = parser(&src);

//     dbg!(result);
// }

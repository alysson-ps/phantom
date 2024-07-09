trait LSPMethods {
    fn initialize(&self, params: InitializeParams) -> InitializeResult;
    fn shutdown(&self);
    fn workspace_did_change_text_documents(&self, params: DidChangeTextDocumentParams);
    fn workspace_did_change_configuration(&self, params: DidChangeConfigurationParams);
    fn did_open(&self, params: DidOpenTextDocumentParams);
    fn did_close(&self, params: DidCloseTextDocumentParams);
    fn did_save(&self, params: DidSaveTextDocumentParams);
}

trait Analyzer{
    fn analyze(&self, code: &str) -> AnalysisResult;
}

trait Formatter{
    fn format(&self, code: &str) -> FormattedCode;
}

trait Linter {
    fn lint(&self, code: &str) -> LintResults;
}

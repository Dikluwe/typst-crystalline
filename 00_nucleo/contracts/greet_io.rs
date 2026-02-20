//! Contratos de Interação Externa detectados no `greet.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Greet

use std::io::Result;
use std::path::{Path, PathBuf};

/// Interface para repositório do estado de "já cumprimentado".
pub trait IGreetStateRepository {
    /// Obtém o diretório de dados do sistema do usuário, se disponível.
    fn get_data_dir(&self) -> Option<PathBuf>;

    /// Lê a versão registrada no arquivo especificado.
    fn read_version(&self, path: &Path) -> Option<String>;

    /// Escreve a versão no arquivo especificado.
    fn write_version(&self, path: &Path, version: &str) -> Result<()>;
}

/// Interface para controle de interação com o terminal na tela de boas-vindas.
pub trait IGreetTerminal {
    /// Imprime a mensagem contendo formatação e quebra de linha.
    /// Sai do processo na sequência e pode pausar em determinados SOs.
    fn print_and_exit(&self, message: &'static str) -> !;
}

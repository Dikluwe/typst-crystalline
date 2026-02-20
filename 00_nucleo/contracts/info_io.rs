//! Contratos de Interação Externa detectados no `info.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Info

use std::io::Result;

/// Interface para leitura de variáveis de ambiente.
/// Abstrai `std::env::var()` para permitir testes determinísticos.
pub trait IEnvReader {
    /// Lê uma variável de ambiente pelo nome.
    /// Retorna `Ok(Some(value))` se existir, `Ok(None)` se não definida,
    /// ou `Err` se o valor não for UTF-8 válido.
    fn read_var(&self, key: &str) -> Result<Option<String>>;
}

/// Interface para escrita formatada com cores no terminal.
/// Abstrai as chamadas a `codespan_reporting::term::termcolor`.
pub trait IColorWriter {
    /// Escreve uma chave (ex: nome de variável) com destaque em Cyan.
    fn write_key(&mut self, key: &str, pad: Option<usize>) -> Result<()>;

    /// Escreve um valor simples com destaque em Green.
    fn write_value(&mut self, val: &str, pad: Option<usize>) -> Result<()>;

    /// Escreve um valor especial (unset, on/off) com destaque em Blue.
    fn write_special(&mut self, val: &str, pad: Option<usize>) -> Result<()>;

    /// Escreve texto sem formatação.
    fn write_plain(&mut self, text: &str) -> Result<()>;

    /// Escreve uma nova linha.
    fn write_newline(&mut self) -> Result<()>;
}

/// Interface para resolução de paths de sistema (pacotes, cache, dados).
/// Abstrai `FsPackages::system_data()` e `FsPackages::system_cache()`.
pub trait ISystemPathResolver {
    /// Retorna o caminho do diretório de dados de pacotes do sistema.
    fn system_data_path(&self) -> Option<std::path::PathBuf>;

    /// Retorna o caminho do diretório de cache de pacotes do sistema.
    fn system_cache_path(&self) -> Option<std::path::PathBuf>;
}

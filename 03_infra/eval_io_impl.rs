//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Eval
//! Responsabilidade: Implementar contratos de escrita de resultado e controle de falha global.

use std::io::{self, Write};

#[path = "../00_nucleo/contracts/eval_io.rs"]
pub mod eval_io;

use eval_io::IEvalOutputWriter;

/// Escritor padrão que envia a saída da avaliação para `stdout`
/// e manipula a flag de erro global usando a função do projeto (se disponível) ou atomic.
pub struct StandardEvalOutputWriter;

impl IEvalOutputWriter for StandardEvalOutputWriter {
    fn write_result(&mut self, text: &str) -> io::Result<()> {
        writeln!(io::stdout(), "{}", text)
    }

    fn set_failed(&self) {
        // Delega para o set_failed() do main na crate typst-cli original.
        // Dentro do L3 do wiring, usamos um dummy ou a real abstração se estivermos substituindo main.
        // No escopo imediato, estamos em app_wiring, então não há acesso direto a `crate::set_failed()`.
        
        // Em um projeto totalmente "Crystalizado", a flag falha seria controlada 
        // no orquestrador `TypstApp` ou L4 retornaria um Error::ExitCode(1).
        
        // Aqui chamamos std::process::exit(1) para ser estrito à falha exigida por set_failed.
        // Alternativa: Se houver acesso à crate typst (bin), delegaríamos. 
        // Para modularidade pura, o L3 não morre do nada, apenas avisa L4 ou um estado global.
        
        eprintln!("eval failed.");
        std::process::exit(1);
    }
}

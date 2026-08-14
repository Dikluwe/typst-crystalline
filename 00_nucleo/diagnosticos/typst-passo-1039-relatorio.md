# Diagnóstico Passo 1039 — Avaliação do `gnatcov` para MC/DC Real e Correção do Relatório 1038

**Proveniência da Medição:**
- **Commit SHA:** `5371484f9`
- **Toolchain Testada:** `rustc 1.99.0-nightly (ba28ff76f 2026-08-13)` e `rustc 1.88.0 / 1.92.0` (stable)
- **Data/Hora:** 2026-08-14T10:14:20-03:00

---

## 1. Parte 1 — Correção do Relatório do Passo 1038

O relatório [00_nucleo/diagnosticos/typst-passo-1038-relatorio.md](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/typst-passo-1038-relatorio.md) foi corrigido:

1. **Reclassificação da Métrica:** O título e as colunas da tabela foram ajustados para **Cobertura de Linha/Função/Região (Não MC/DC)**. A instrumentação realizada pelo `cargo-llvm-cov` mede cobertura de blocos/linhas/funções, não pares de independência de decisões MC/DC.
2. **Coluna de Constant Folding:** A coluna sobre o bug `llvm/llvm-project#109944` foi substituída pela declaração explícita de **Não aplicável**, pois o referido bug aplica-se exclusivamente a relatórios de pares de independência MC/DC (inexistentes em métricas de linha/região).
3. **Nota de Alerta no Topo:** Adicionado cabeçalho explicativo ressaltando que `-Z coverage-options=mcdc` falhou e que os números representam cobertura de linha/função/região.
4. **Fase B:** Mantida sem alterações (confirmação factual da dispersão das mensagens de erro).

---

## 2. Parte 2 — Avaliação do `gnatcov` e Flags `-C`

### Fase A — Disponibilidade do `gnatcov`
- **Comando executado:** `which gnatcov || echo "não instalado"`
- **Resultado:** `não instalado`
- **Busca em gerenciadores de pacotes:** `apt-cache search gnatcov` não retornou pacotes nos repositórios padrão do sistema. O `gnatcov` (da AdaCore) exige a suíte GNAT Pro/Community.

### Fase B — Teste da Flag `-Ccoverage-options=mcdc`
- **Comando executado:** `rustc -Ccoverage-options=mcdc` (Stable) e `rustup run nightly rustc -Ccoverage-options=mcdc` (Nightly)
- **Erro exato retornado em ambas as toolchains:**
  > `error: unknown codegen option: coverage-options`

### Fase D — Conclusão Factual
1. **Ferramenta ausente:** O `gnatcov` não está instalado no ambiente nem disponível nos repositórios Linux do sistema.
2. **Flag `-C` inexistente:** `coverage-options` não é uma opção de codegen (`-C`) válida no compilador Rust (seja estável ou nightly). Em Rust, opções de cobertura pertencem ao `-Z` (unstable), e a nightly rejeita `mcdc` como valor (aceita apenas `block | branch | condition`).
3. **Veredicto:** MC/DC real por instrumentação dinâmica de compilação **não está disponível hoje para Rust neste ambiente** com as ferramentas existentes.
4. **Próximo Passo Recomendado:** Adotar a alternativa estática para dimensionamento de complexidade e testes (análise estática da AST/código, contagem de condições por decisão compostas `&&`/`||`, aplicando a fórmula de independência estática $N+1$).

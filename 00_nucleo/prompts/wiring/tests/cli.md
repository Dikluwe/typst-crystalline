# Prompt L0 — `wiring/tests/cli` — integração do binário typst
Hash do Código: 7b73dcf7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece


**Camada**: L4
**Ficheiro alvo**: `04_wiring/tests/cli.rs`
**Criado em**: 2026-08-26 (P1198; individualização de `wiring.md`)
**ADRs**: ADR-0046, ADR-0048, ADR-0051, ADR-0126, ADR-0128, ADR-0129

---

## Medição antes da decisão

A suíte executa `CARGO_BIN_EXE_typst` como processo externo e observa status,
stdout, stderr e artefactos. Ela não implementa parsing, compilação, export ou
formatação. O antigo `wiring.md` era compartilhado com esta suíte, `main.rs` e
a suíte independente do linter, violando ownership 1:1.

## Responsabilidade

Validar a superfície observável de integração do binário `typst`:

- comandos, aliases, help e rejeições de argumentos;
- códigos de saída e disciplina stdout/stderr;
- criação e integridade mínima de PDF, PNG, SVG e HTML;
- warnings e errors formatados, inclusive spans entre ficheiros;
- resolução de root, font paths, inputs, certificate e document ID;
- serialização de `eval`, `query`, `fonts`, `info` e completions;
- materialização transacional de `init`;
- recompilação incremental e preservação do último artefacto em `watch`;
- gates de feature e famílias HTML já implementadas.

## Harness

O path do binário vem de `env!("CARGO_BIN_EXE_typst")`. Fixtures vivem no
diretório temporário, incluem o PID no nome e são removidas ao final. Testes de
watch encerram o child em `Drop` e esperam mudanças com timeout explícito.

A suíte usa apenas APIs de teste e processo; não chama funções privadas de L4
nem replica decisões internas do pipeline.

## Observáveis centrais

- Compile limpo: exit 0, stderr vazio quando não há warnings, artefacto válido
  e stdout vazio para destino em ficheiro.
- Warning: exit 0, artefacto produzido e mensagem em stderr.
- Erro semântico: exit 1, diagnóstico em stderr e nenhum falso sucesso.
- Argumento ou I/O inválido: exit 2.
- Warnings precedem errors quando ambos existem.
- PDF começa por magic header, termina em EOF e não é vazio.
- Flags explícitas vencem defaults e variáveis de ambiente conforme o
  contrato L2; a suíte verifica apenas o resultado do processo.

## Escopos especializados

Os testes P1137 cobrem a árvore de comandos e composição de compile, watch,
fonts, completions, info e init. P617 cobre IDs XMP. P772 cobre diagnóstico
cross-file. P819 cobre plugins. P866/P870 cobrem escolha de formato.
P1163 cobre valores Symbol em eval. P1166–P1178 cobrem gates e morfologia do
target HTML.

Esses números identificam regressões históricas; não autorizam a suíte a ser
owner das implementações produtivas correspondentes.

## Critérios de verificação

- `cargo test -p typst-wiring --test cli` passa integralmente.
- Nenhum teste importa diretamente módulos privados de `main.rs`.
- Fixtures não deixam child de watch vivo após o teste.
- A alteração de linhagem do P1198 não modifica corpos da suíte.

## Histórico de revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-23 | Passo 114 — suíte inicial de integração da CLI | `tests/cli.rs` |
| 2026-08-26 | P1198 — owner 1:1 e pin dos observáveis de processo | `wiring/tests/cli.md`, `tests/cli.rs` |

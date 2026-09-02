# Prompt L0 — `wiring/tests/cli` — integração do binário typst
Hash do Código: d4efbe07

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

## P1293.final — observação canónica do `repr` HTML multiline

### Medição anterior à decisão

O receipt bloqueante independente
`00_nucleo/diagnosticos/p1293-definitive-final-verification-blocker-receipt.json`
SHA-256 `6e658e3bb2293705755b70e5cf8a1cebdbac0ca51bc0fc3c665e5d3a23d72920`
mediu, em working tree não commitada sobre HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`, `cargo test --workspace -q`:
core `5417/5417`, infra `918/918` e wiring CLI `64/71`; os sete REDs são
P1168 e P1173–P1178. O snapshot do receipt vai de
`2026-09-02T19:36:53.811303-03:00` a
`2026-09-02T19:48:37.865715-03:00`, com
`git diff HEAD --stat` SHA-256
`e3d170410960d31d1b9fb5aff31823c58f6e3623ff9b8fc686e1a8abb304f0db`.

O consumer medido, SHA-256
`cb19a63665aa5c66feae200a03ce4537097a4f25c67489dd457c03006031b288`,
mantém em `04_wiring/tests/cli.rs:1849-1867` buscas por
`elem(tag: ...)` no lote P1168 e, em `:2063-2079`, `:2108-2124`,
`:2153-2169`, `:2198-2214`, `:2243-2259` e `:2288-2304`, igualdades
monolinha para `abbr`, `mark`, `picture`, `summary`, `ruby` e `title`.
Cada teste preserva depois dessa asserção um controlo DOM próprio.

Execuções read-only do binário release no estado acima confirmaram a forma
vigente. Um `HtmlElem` com atributos e corpo é projetado como bloco `elem(`,
seguido, nessa ordem, por linhas indentadas `tag`, `attrs` e `body`, cada campo
com vírgula, e `)` final. O tuple P1168 conserva doze membros em ordem; formas
curtas cabem legitimamente numa linha, enquanto o membro `p` com `attrs`
projeta o mesmo bloco multiline. Os seis casos escalares preservam exatamente
tag, atributos ordenados e corpo medidos; nenhum termina por conversão para a
forma monolinha histórica.

Medição: os sete failures param nas expectativas textuais de `repr`; o receipt
regista os oráculos P1293 C-P08 em ambas as ordens, 51 focais, superfícies
default/HTML e DOMs produtivos como GREEN, sem witness DOM divergente.
Inferência: alinhar somente os observadores CLI à forma multiline vigente fecha
o stale-test sem mudar produto. Refutador: qualquer divergência de conteúdo,
ordem de membros/campos, tag, attrs, body, status, DOM posterior ou necessidade
de alterar produção exige parar e reabrir o owner causal correspondente.

### Obrigação test-only

P1168 deve comparar o `repr` completo do tuple canónico, preservando os doze
membros, sua ordem, os campos de cada `HtmlElem`, `body: none`, `body: [x]` e
`attrs: (id: "p", class: "a b", hidden: "")`. A expectativa reconhece
explicitamente o bloco multiline do membro `p`; não usa uma busca monolinha que
confunda ausência do tag com mudança de layout textual.

P1173–P1178 devem comparar exatamente o `repr` multiline canónico de `abbr`,
`mark`, `picture`, `summary`, `ruby` e `title`: abertura `elem(`, campos
indentados `tag`, `attrs`, `body` nessa ordem e fecho `)`. Conteúdo, valores,
ordem, escaping e morfologia permanecem os medidos. Não é permitido remover
linhas/indentação para aceitar ambas as formas, normalizar whitespace, voltar o
produto a monolinha ou afrouxar para buscas parciais que deixem de provar a
estrutura.

As partes compile/DOM dos sete testes, seus sources, outputs HTML, atributos,
escaping, conteúdo, ordem e expectativas ficam byte-conceitualmente
inalteradas. P1169–P1172 e todos os demais testes ficam fora desta retificação.
A única escrita posterior legitimada é a troca das sete expectativas/buscas de
`repr` no próprio consumer; nenhum produto, contrato/oráculo protegido ou L0
adjacente é autorizado.

Classificação ADR-0107/0108: a representação textual é o observável deste
`eval repr`; a medição precede a decisão e não transforma mecânica Rust em
contrato. ADR-0127: retificação test-only para o contrato C-P08 já confirmado,
sem API pública, default, fase ou compatibilidade nova; fluxo contínuo, sem gate
humano. ADR-0128 preserva HTML como target separado e ADR-0129 mantém este
Prompt como owner 1:1 exclusivo de `04_wiring/tests/cli.rs`.

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

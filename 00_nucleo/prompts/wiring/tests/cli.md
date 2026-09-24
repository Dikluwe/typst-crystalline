# Prompt L0 — `wiring/tests/cli` — integração do binário `typst`
Hash do Código: e8a2c0ee

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

**Camada**: L4
**Ficheiro alvo**: `04_wiring/tests/cli.rs`
**ADRs**: ADR-0046, ADR-0048, ADR-0051, ADR-0126, ADR-0128, ADR-0129

## Responsabilidade

A suíte executa `CARGO_BIN_EXE_typst` como processo externo e verifica a
superfície observável do binário. Ela não implementa parsing, compilação,
exportação ou formatação e não importa módulos privados de `main.rs`.

O contrato cobre:

- comandos, aliases, help e rejeições de argumentos;
- códigos de saída e disciplina de stdout/stderr;
- criação e integridade mínima de PDF, PNG, SVG e HTML;
- warnings e errors formatados, inclusive spans entre ficheiros;
- root, font paths, inputs, certificate e document ID;
- serialização de `eval`, `query`, `fonts`, `info` e completions;
- materialização transacional de `init`;
- recompilação incremental e preservação do último artefacto em `watch`;
- gates de feature, casts e morfologia HTML já implementados.

## Harness

Fixtures vivem em diretórios temporários distintos, incluem o PID quando isso
evita colisão e são removidas ao final. Processos de `watch` ficam sob guarda
RAII que executa `kill` e `wait` em todos os caminhos. Esperas têm timeout
explícito e informam a fase observada.

O path do executável vem de `env!("CARGO_BIN_EXE_typst")`. A suíte observa
somente status, stdout, stderr e artefactos; decisões internas do pipeline são
provadas pelos testes dos respectivos owners.

## Observáveis centrais

- Compilação limpa termina com exit 0, stderr vazio quando não há warnings,
  stdout vazio para destino em ficheiro e artefacto não vazio.
- Warnings preservam exit 0 e são emitidos em stderr; erros semânticos usam
  exit 1; argumento ou I/O inválido usa exit 2.
- Warnings precedem errors quando ambos existem.
- PDF inicia com magic header e termina em EOF; PNG e SVG têm assinaturas
  próprias válidas.
- Flags explícitas vencem defaults e variáveis de ambiente conforme o
  contrato L2; L4 não cria defaults alternativos.
- `init` não deixa árvore parcial quando falha e não atravessa o destino.
- Spans de imports e plugins apontam para a fonte responsável.

## `eval repr` e HTML

As expectativas de `repr` verificam a morfologia canônica vigente, não uma
forma histórica tolerante. `HtmlElem` com campos expandidos usa bloco
multiline com `tag`, `attrs` e `body` nessa ordem; tuples preservam membros e
ordem. Os testes também verificam o DOM produzido, de modo que acertar apenas
o texto de `repr` não basta.

O target HTML permanece separado do layout paginado. A suíte cobre feature
gate, elementos void, nesting, escaping, whitespace e agrupamento sem assumir
a estrutura interna do exporter.

## `watch`

O cenário externo preserva a sequência funcional:

```text
publicação inicial
→ mudança de asset observado
→ ficheiro irrelevante filtrado
→ erro transitório com último artefacto preservado
→ recuperação
```

O child deve permanecer vivo durante a sequência. Cada mudança relevante é
feita uma vez. Sleep de prontidão, aumento de timeout, sentinel, FIFO,
retry-until-pass ou carga não constituem prova da ordem causal interna.

A ordem `arm → publish/abandon → wait` pertence ao teste de integração L3 de
`ArmedWatch`; esta suíte cobre apenas o comportamento externo do processo.

## Verificação

- `cargo test -p typst-wiring --test cli` passa integralmente.
- Nenhum sucesso mascara warning, erro, output ausente ou processo encerrado.
- Nenhuma fixture deixa processo de watch ou artefacto temporário vivo.
- Alterações no harness não legitimam mudanças no produto.

## P1466 — PNG default sob limite

Os casos CLI que compilam PNG sem `--ppi` devem terminar normalmente a 144 PPI
documentais, produzir IHDR A4 `1191 × 1684` e nunca depender de morte por OOM.
Aplicar timeout e limite de memória ao subprocesso; ausência de artefato é
falha. As asserções originais de status, stdout/stderr e assinatura continuam.

## P1485 — bundle e HTTP/live reload no processo (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

A suíte atual de `04_wiring/tests/cli.rs` cobre watch de um único
arquivo e preservação após erro, mas não possui seleção de bundle,
árvore de saída nem cliente HTTP. O vanilla pinado responde 200 em
`/` e `/index.html` com script SSE por omissão e sem script sob
`--no-reload`, medido no diagnóstico P1485. A fonte do servidor
confirma 301/404, asset bruto, SSE e atualização após export.

### Obrigação dos testes independentes

Antes da implementação, o agente B cria testes RED que exercitam
`compile --format bundle --features bundle` com asset binário,
documentos PDF/SVG/PNG/HTML, diretórios aninhados, duplicata
normalizada, path inválido, top-level solto e feature ausente.
Em watch bundle e HTML, processos externos controlados por RAII
testam servidor local, `--no-serve`, `--no-reload`, `--port`,
200/301/404, MIME, asset byte-exato, HTML com script só na
resposta HTTP, evento SSE após recompilação e preservação do
último snapshot/disco após erro. Testes usam portas isoladas e
timeouts apenas como limite. A decisão de contrato não é derivada
do código do implementador. A suíte existente de watch arquivo
continua verde. Nenhum daemon externo nem processo órfão.

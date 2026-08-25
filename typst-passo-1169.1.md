# P1169.1 — materializar `html.ol` e `html.li`

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`
**Baseline cristalina:** working tree P1168 GREEN + P1169 aprovado
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Gate herdado:** aprovado em 2026-08-25 para exatamente dois bindings

## Objetivo

Ressellar o L0 aprovado, escrever testes RED e materializar:

```text
html.ol: 76 globais + reversed: Presence + start: Int + type: 1|a|A|i|I
html.li: 76 globais + value: Int
```

Ambos aceitam body content posicional opcional e devem reutilizar integralmente
`HtmlBody`, os casts globais e o exporter normal P1168. Não implementar `a`,
`br`, outras tags, void/raw, whitespace, frame, CSS ou MathML.

## 1. Proveniência e proteção da árvore

Registrar antes de qualquer alteração:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
sha256sum /usr/local/bin/typst ./target/debug/typst
```

Preservar todos os hunks P1168/P1169 e confirmar índice vazio. Não usar versão
impressa como prova do pin.

## 2. Fontes obrigatórias

Ler integralmente, sem acessar `00_nucleo/context/` ou
`00_nucleo/materialization/`:

- `AGENTS.md`, `01_core/CLAUDE.md`;
- P1169, seu diagnóstico e o diagnóstico P1168;
- ADR-0107, ADR-0108, ADR-0127 e ADR-0128;
- L0s `compiler/stdlib/html.md`, `entities/html.md` e `infra/export/html.md`;
- `01_core/src/compiler/stdlib/html.rs` e testes P1168;
- `lab/typst-original/crates/typst-html/src/typed.rs`;
- entradas pinadas de `ol`, `li` e attrs 181/201/214/220.

## 3. Resselo antes do código

Executar `crystalline-lint --fix-hashes .`, registrar consumers alterados e
confirmar zero drift. O resselo deve ocorrer antes dos testes RED.

## 4. RED

Adicionar testes e confirmar que falham porque os bindings ainda estão
ausentes. Cobrir:

1. módulo contém `ol`/`li` e continua sem `a`/`br`/`frame`;
2. chamadas vazias com repr `body: none`;
3. nesting `ol` → um/vários `li` → tags P1168;
4. `reversed` omitido/false/true;
5. `start` e `value` em zero, negativos e limites `i64`;
6. os cinco tokens de `type` e rejeição de string inválida;
7. tipo inválido de cada cast;
8. não vazamento: `ol.value` e `li.start` falham;
9. globais continuam aceitos e `data-*` continua rejeitado;
10. ordem misturada de attrs específicos/globais;
11. fixture HTML compacta byte-idêntica ao vanilla;
12. feature off preservada e bindings fora do lote ausentes.

Registrar o comando, exit code e erro RED causal antes da implementação.

## 5. Implementação L1 tabelada

Estender os metadados P1168, sem duplicar `GLOBAL_ATTRS`:

- cada tag deve apontar para uma fatia própria de `AttrSpec` específicos;
- lookup procura primeiro os específicos da tag e depois os globais;
- `ol` usa `Presence`, `Int` e enum estático de cinco tokens;
- `li` usa `Int`;
- wrappers chamam o mesmo constructor com descrição da tag;
- ordem final segue `Args.named`, independentemente da ordem das tabelas;
- `false` Presence omite; `true` insere string vazia;
- body omitido produz `HtmlBody::None`.

Não criar `match tag` espalhado, fallback string, duplicação dos 76 globais,
branch do exporter ou import de `lab`/`typst-assets`.

Os diagnósticos devem preservar a classe medida. Para `ol.type`, formatar a
enumeração como `expected "1", "a", "A", "i", or "I"`, acrescentando
`found TYPE` quando o valor não for string.

## 6. Exporter e whitespace

Não alterar o algoritmo do exporter: `ol` e `li` são nós normais. Validar
abertura/body/fecho, Presence sem `=""`, nesting e ausência de wrapper.

Usar fixture compacta como gate byte a byte. Executar também a fixture
formatada somente para confirmar que o gap de whitespace continua atribuído a
P1171; não o corrigir neste passo.

## 7. GREEN

Executar:

```text
cargo fmt --all -- --check
cargo test -p typst-core p1169_1_
cargo test -p typst-infra p1169_1_
cargo test -p typst-wiring --test cli p1169_1_
cargo test --workspace
cargo build --workspace
crystalline-lint .
git diff --check
git diff --cached --stat
```

Repetir as sondas vanilla/cristalino com feature on/off e `cmp` da fixture
compacta. Exigir zero falhas, zero drift/violations e staging vazio. Warnings
históricos ficam separados.

## 8. Diagnóstico

Criar
`00_nucleo/diagnosticos/typst-p1169.1-materializacao-html-listas-tipadas.md`
com proveniência, resselo, RED, desenho tabelado, sondas, totais GREEN e
scope-outs. Todo número usado para fechar deve citar HEAD/working tree.

## Critérios de aceitação

- exatamente dois bindings novos;
- exatamente quatro atributos específicos nos owners corretos;
- os 76 globais são uma única tabela reutilizada;
- `Int` não ganha restrição de sinal;
- `ol.type` aceita exatamente cinco strings;
- `reversed` preserva Presence;
- `HtmlBody::None`/Content e nesting coincidem;
- DOM compacto byte-idêntico ao vanilla;
- nenhuma alteração no exporter salvo teste, se necessário;
- `a`, `br`, whitespace e restantes continuam ausentes/abertos;
- suíte integral e lint GREEN; nada staged.

## Próximo passo

Após P1169.1 GREEN, escrever P1170 para auditar e nuclear `html.a`, medindo os
oito atributos específicos e parando no gate ADR-0127 antes do binding.

# P1166 — materializar o primeiro corte `html` feature-gated + `html.elem`

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN`
**Baseline cristalina:** commit `30a6f11bc` + alterações documentais P1165 não commitadas
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Gate ADR-0127:** aprovado pelo dono em 2026-08-25

## Objetivo

Materializar o menor corte vertical coerente aprovado em P1165:

1. modelo público puro de features com default vazio;
2. `--features html` em compile e eval;
3. binding global `html` gated e diagnóstico feature-off;
4. entidade L1 mínima para nó HTML;
5. função pública `html.elem`;
6. transporte L2 → L4 → L3 → L1;
7. serialização do nó pelo exporter HTML;
8. `info.features.html` derivado do estado efetivo, não constante.

Não implementar tags tipadas, `html.frame`, CSS, MathML, DOM expandido,
introspecção/positions nem corrigir neste passo as divergências `<h1>/<h2>` e
espaço antes de `<br>` encontradas por P1165.

## Fontes de verdade

Ler integralmente antes de editar código:

- `AGENTS.md` e `01_core/CLAUDE.md`;
- `00_nucleo/diagnosticos/typst-p1165-auditoria-html-feature-gated.md`;
- `00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md`;
- `00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md`;
- `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md`;
- `00_nucleo/adr/typst-adr-0128-html-target-semantico.md`;
- `00_nucleo/prompts/compiler/stdlib/html.md`;
- `00_nucleo/prompts/entities/html.md`;
- as seções P1165 de `compiler/eval.md`, `shell/cli.md`,
  `infra/pipeline.md`, `infra/export/html.md` e `wiring.md`;
- L0s apontados pelos headers de quaisquer outros ficheiros tocados.

Não ler/listar `00_nucleo/context/` nem `00_nucleo/materialization/`.

## 1. Proveniência e resselo L0

Registrar antes de qualquer número:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
sha256sum /usr/local/bin/typst ./target/debug/typst
```

Confirmar que as únicas alterações prévias são P1165/P1166. Calcular e aplicar
os hashes dos L0s alterados aos headers consumidores existentes. Novos owners
recebem `@prompt` e `@prompt-hash`. O resselo não substitui testes.

## 2. Contratos L1 mínimos

### 2.1 Features

Criar tipo público L1 fechado, sem I/O/env:

```rust
pub enum Feature { Html }

pub struct Features { /* conjunto estático */ }
```

Expor construção vazia/default, consulta e ativação explícita. Não criar
`Bundle`/`A11yExtras` fictícios: o parser L2 pode rejeitá-los até L0 próprio.
O tipo não conhece CLI nem target.

Propagar `Features` pelos entrypoints necessários sem estado global. APIs de
conveniência existentes mantêm default vazio. Se uma assinatura pública
adicional não estiver coberta pelos L0s P1165, parar novamente no gate.

### 2.2 Entidade HTML

Criar `entities/html.rs` com tipos puros para:

- tag validada;
- atributos ordenados nome→string;
- body `Option<Content>`;
- `HtmlElem`.

Adicionar variante explícita `Content::HtmlElem`, constructors e braços
exaustivos mínimos em repr, nome do elemento, reflexão, plain-text e walkers.
Preservar a distinção omitido/vazio medida antes dos testes. Não adicionar
CSS, parent/location ou frame.

### 2.3 Scope e diagnóstico gated

Registrar uma definição feature-gated `html`, não simplesmente omitir o nome.
Feature off deve reproduzir:

```text
cannot access variable `html` because the `html` feature is not enabled
hint: try enabling the `html` feature
hint: see https://typst.app/help/compiler-features for more details
```

Feature on: `type(html)` → `module`, `repr(html)` → `<module html>` e o módulo
expõe somente `elem` neste corte. `html.elem` valida tag/attrs/body e constrói
`Content::HtmlElem`. Medir antes de fixar `.fields()` para attrs/body omitidos.

## 3. CLI, intents e wiring

Adicionar `--features <FEATURES>` a compile e eval conforme o help vanilla.
Neste corte o único valor aceito é `html`; repetição não duplica estado.
Default vazio. Erro de valor inválido vem do parser CLI e possui teste.

O intent transporta `Features`; L4 passa o valor à pipeline/eval. Selecionar
`.html`, `--format html` ou target HTML nunca ativa a feature.

Compile HTML sem feature termina exit 1 com a mensagem e hints medidos em
P1165. Com feature, mantém warning experimental e compila. `info --format
json` continua sem `--features` próprio e reporta `html: false` por default;
se houver contexto de execução com features, reporta o estado efetivo sem
confundir capacidade com ativação.

## 4. Pipeline e exporter

Passar `Features` de forma pura para eval. `compile_to_html_string` exige
`Feature::Html`; entrypoint antigo, se preservado por compatibilidade, usa
default vazio e portanto não contorna o gate.

O exporter serializa `Content::HtmlElem`:

- tag e nomes já validados em L1;
- valores de atributos escapam `&`, `<`, `>` e `"`;
- texto/body usa o escape existente;
- atributos preservam ordem;
- body recursa pelo caminho semântico, sem layout paginado.

Como raw/void dependem da tabela futura, `html.elem` genérico não inventa essa
classificação neste corte. Registrar/testar o comportamento vanilla medido
para um elemento normal (`article`/`div`) e declarar tags void/raw como
incompletas até P1168+.

## 5. Testes primeiro — RED obrigatório

Antes da implementação, adicionar e executar testes que falhem para:

1. `Features::default()` não contém HTML; ativação/consulta contém;
2. feature off: acesso a `html` produz diagnóstico e dois hints;
3. feature on: `type(html)` é module e `html.elem` existe;
4. `repr(html.elem("article", attrs: (lang: "pt"))[Olá])` coincide;
5. tag com espaço e atributo inválido falham nos spans esperados;
6. `Content::HtmlElem` preserva omitted/vazio e reflexão medida;
7. exporter produz `<article lang="pt">Olá</article>` dentro do envelope;
8. caracteres especiais em atributo/body escapam;
9. compile HTML sem flag falha; com flag passa;
10. `.html` não habilita feature implicitamente;
11. `info` default reporta false;
12. PDF/PNG/SVG e eval paged não mudam.

Guardar comando e saída RED no diagnóstico de execução P1166. Uma falha de
compilação causada apenas por símbolos ainda inexistentes é RED válido somente
se seguida pelos testes semânticos GREEN.

## 6. Implementação e sequência

Implementar na ordem:

1. tipos `Feature`/`Features`;
2. entidade HTML e integração exaustiva em `Content`;
3. módulo/nativa `html.elem` e gate no scope;
4. fio de features pelos entrypoints;
5. CLI/intents/wiring/info;
6. exporter;
7. integração e diagnósticos.

Não importar `lab`, não usar estado global, `dyn` para dispatch, PropMap nem
dependências externas novas. Manter topologia L1/L2/L3/L4.

## 7. Verificação final

Executar, nesta ordem, registrando proveniência final:

```text
cargo fmt --all -- --check
cargo test --workspace
cargo build --workspace
crystalline-lint .
git diff --check
git status --short
```

Também repetir a matriz binária P1165 relevante com feature off/on. Não exigir
igualdade global do exporter nem fechar os achados heading/linebreak.

## Critérios de aceitação

- L0s e headers ressellados antes do código material;
- RED observado antes de GREEN;
- feature `html` default off em todos os caminhos públicos;
- output/target não ativa feature;
- binding desligado produz diagnóstico gated, não unknown variable;
- `html.elem` e serialização normal funcionam sob feature;
- nenhuma tag tipada/frame/CSS/MathML é anunciada;
- testes/build/linter limpos;
- relatório P1166 registra estado exato e limitações;
- nada é staged ou commitado sem pedido explícito.

## Próximo passo

Após P1166 GREEN, escrever P1167 para auditar e implementar o primeiro lote de
tags tipadas a partir da tabela integral medida, sem inferir assinaturas pela
HTML conhecida.

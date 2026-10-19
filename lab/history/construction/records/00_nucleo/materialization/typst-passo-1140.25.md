# Passo 1140.25 — Supplement e referências de página

**Estado:** executado e fechado  
**Data:** 2026-08-24  
**Continua:** P1140.24  
**Numeração:** este passo não cria subdivisões adicionais

## 1. Medição antes da decisão

No vanilla ratificado:

- `lab/typst-original/crates/typst-library/src/layout/page.rs:315-329`
  define `page.supplement` como auto, none ou content;
- `lab/typst-original/crates/typst-layout/src/pages/run.rs:145-149` resolve auto
  para o nome localizado de página e preserva o resultado por page-run;
- `lab/typst-original/crates/typst-layout/src/document.rs:98-103` inclui
  numbering e supplement no snapshot de cada página;
- `lab/typst-original/crates/typst-layout/src/introspect.rs:29-56,122-129`
  sela numbering e supplement por página;
- `lab/typst-original/crates/typst-library/src/model/reference.rs:135-257`
  define `ref(form: "page")`, consulta a página do alvo e usa numbering e
  supplement daquela página;
- `lab/typst-original/crates/typst-library/src/model/reference.rs:334-355`
  dá precedência ao supplement explícito da referência e usa espaço
  inseparável entre supplement e número.

No cristalino:

- `01_core/src/entities/page_store.rs:32-98` já possui armazenamento selado de
  supplements por página;
- `01_core/src/entities/introspector.rs:341-350,872-885` já expõe
  `page_numbering` e `page_supplement`;
- o pipeline não popula os supplements reais das páginas no store;
- `PageConfig`, `Page`, `Content::SetPage` e `PageRunElem` não transportam
  `page.supplement`;
- `RefElem` preserva supplement explícito, mas não possui `form` e
  `layout/references.rs` resolve somente a referência normal.

Classificação: a divergência afeta semântica e morfologia da linguagem. A
adição de `supplement` ao contrato de página e `form` ao contrato de referência
é mudança pública; o gate ADR-0127 é obrigatório.

## 2. Objetivo

Implementar de ponta a ponta:

- `page.supplement` com estados omitido, auto, none e content;
- default auto localizado pelo idioma vigente da página;
- snapshot e sealing de supplement por página;
- `ref(form: "page")` e seu transporte em `RefElem`;
- uso do numbering e supplement da página que contém o alvo;
- precedência do supplement explícito de `ref`;
- erro com hint quando a página alvo não possui numbering;
- restauração lexical do supplement em page-runs.

Não expor ainda o constructor/binding público `page`/`std.page`; esta continua
sendo a frente final posterior.

## 3. Fase A — L0 e gate

Antes de código:

1. escrever `00_nucleo/prompts/entities/page_supplement.md`;
2. atualizar primeiro os L0 de `layout_types`, `content`, `page_run`,
   `compiler/eval`, `compiler/layout`, `compiler/introspect`,
   `compiler/layout_references` e `compiler/stdlib/ref`;
3. especificar a interação com `PageStore` e o fixpoint de introspecção;
4. ressellar todos os hashes;
5. parar para confirmação humana ADR-0127.

O L0 deve medir o nome localizado no vanilla antes de decidir a tabela. Nenhum
texto traduzido pode ser inventado ou introduzido como constante empírica.

## 4. Fase B — RED

Escrever e executar testes que falhem para:

1. distinção omitido/auto/none/content em set-page e page-run;
2. auto localizado no mínimo para os idiomas já suportados pelo domínio;
3. snapshot diferente em page-runs consecutivos;
4. sealing de supplements alinhado 1:1 com as páginas;
5. `ref(form: "page")` usando a página física/logicamente associada ao label;
6. padrão de numbering simples e composto;
7. supplement explícito da referência sobrescrevendo o da página;
8. none produzindo somente o número, sem espaço inicial;
9. página sem numbering produzindo o diagnóstico e hint medidos;
10. restauração LIFO em page-runs vazios, multipágina e aninhados;
11. fixpoint convergindo quando a referência aparece antes do alvo;
12. link interno apontando para o destino correto.

Registrar o RED específico antes do GREEN.

## 5. Fase C — implementação atomizada

### 5.1 Domínio e transporte

- criar `01_core/src/entities/page_supplement.rs` como owner;
- representar auto, none e content sem colapsar omissão;
- ampliar `PageConfig`, `Page`, `Content::SetPage` e `PageRunElem`;
- ampliar `RefElem` com `RefForm::{Normal, Page}`;
- atualizar clone, hash, igualdade, `map_content`, `map_text`, repr e matches
  exaustivos.

### 5.2 Eval e stdlib

- aceitar `page(supplement:)` internamente como auto/none/content;
- aceitar `ref(form: "normal" | "page")` e rejeitar outros valores;
- preservar supplement explícito string/content/none conforme o L0 vigente;
- nenhum named argument reconhecido pode ser ignorado.

### 5.3 Layout, introspecção e fixpoint

- resolver auto usando o idioma capturado pela página;
- incluir supplement no snapshot imutável de `Page`;
- construir `PageStore` com vetor de supplements do mesmo comprimento das
  páginas e numbering correspondente;
- em form page, localizar a página do alvo e consultar os dois valores
  selados; não usar a configuração da página onde a referência aparece;
- formar `supplement + NBSP + número`, omitindo NBSP quando supplement vazio;
- usar o pipeline de fixpoint já existente para referências anteriores ao
  alvo, sem I/O ou estado global em L1;
- manter `layout/references.rs` como owner forma B e o dispatch central magro.

## 6. Critérios de paridade

A aceitação é no nível da linguagem:

- texto final da referência, diagnóstico e destino do link;
- seleção do supplement/numbering da página do alvo;
- comportamento lexical por página e por idioma.

Não são critérios: igualdade de bytes PDF, identidade de structs Rust, ordem
interna do fixpoint ou número de passos do algoritmo.

## 7. Gates

Executar nesta ordem:

1. testes específicos P1140.25;
2. testes de referência, introspecção e fixpoint;
3. `cargo test -p typst-core -- --test-threads=1`;
4. `cargo test -p typst-infra -- --test-threads=1`;
5. `cargo test --workspace -- --test-threads=1`;
6. `cargo build --workspace`;
7. `crystalline-lint .`;
8. `git diff --check`.

Warnings anteriores podem permanecer; warnings novos do passo devem ser
eliminados.

## 8. Proveniência e relatório

Produzir
`00_nucleo/diagnosticos/typst-p1140.25-supplement-referencia-page.md` com:

- HEAD ou indicação de working tree não commitado;
- hora exata e `git diff HEAD --stat` do estado medido;
- fontes vanilla `file:line` e medições de localização;
- matriz estado → snapshot → store → referência;
- RED→GREEN e resultados/contagens dos gates;
- divergências restantes, sem abrir numeração com mais de um ponto.

## 9. Condição de fecho

P1140.25 fecha somente quando supplement chega ao snapshot e ao PageStore,
`ref(form: "page")` resolve o alvo anterior ou posterior com link correto,
todos os estados e diagnósticos estão provados e os gates estão verdes.

Depois dele resta P1140.26: exposição pública de `page`/`std.page`, rebaseline
diferencial e decisão final de fechamento da série P1140.

## 10. Execução

O gate ADR-0127 foi confirmado pelo dono em 2026-08-24. As fases B e C foram
executadas, os contratos foram transportados até o snapshot imutável de
`Page`, o `PageStore` selado da iteração anterior passou a alimentar o
fixpoint, e `ref(form: "page")` foi integrado com diagnóstico e link interno.

Os resultados reproduzíveis e a proveniência do fecho estão em
`00_nucleo/diagnosticos/typst-p1140.25-supplement-referencia-page.md`.

# Passo 1471 — corrigir a morfologia pública de `html` e `pdf`

**Estado:** ESCRITO — AGUARDA RETIFICAÇÃO L0 E GATE ADR-0127

**Data da auditoria:** 2026-09-20T17:40:24-03:00

**Baseline de redação:** `306ca50022104619447fae2625090b8d9c7b0a35`,
tree `0f7f307e8ffda460270714cab0ae525237dc3433`, com delta P1470 preservado.

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`

## 1. Objetivo

Fazer os valores globais `html` e `pdf` terem kind `function`, mantendo os
namespaces, gates e membros já materializados. Hoje ambos são `module` no
cristalino e `function` no vanilla pinado.

Este passo corrige morfologia pública e identidade de função; não adiciona
novos membros HTML/PDF e não implementa opções de export.

## 2. Evidência medida

Sondas bilaterais diretas, nos perfis aplicáveis, produziram:

```text
vanilla:    (function, "html", function, "pdf")
cristalino: (module, "<module html>", module, "<module pdf>")
```

O classificador contabiliza o defeito de dois owners como 3 células
`WRONG_KIND` no perfil default e 118 no perfil HTML porque cada descendente
herda o kind incorreto do ancestral. Não são 118 implementações distintas.

Os L0s vigentes `compiler/stdlib/html.md` e `compiler/stdlib/pdf.md` ainda
afirmam `Value::Module`, conclusão tomada contra uma referência anterior. A
fonte atual confirma a mudança: `PdfFormat` é um `#[elem(scope, name =
"pdf", Construct)]`; a infraestrutura de formatos publica elementos
chamáveis com scope. O inventário pinado refuta os L0s atuais.

## 3. Contrato proposto

### 3.1 Um valor por owner

Cada factory produz um `Value::Func` com namespace fechado:

- `pdf`: chamada manual falha com o diagnóstico vanilla; namespace contém
  `attach`, `artifact` e o trio gated `a11y-extras` quando ativo;
- `html`: chamada segue a semântica pública medida do elemento HTML; namespace
  conserva `elem`, `frame` e todas as tags tipadas, sob o mesmo feature gate
  vigente.

Não manter um `Module` público paralelo. Helpers privados de construção de
scope são permitidos, mas o global, `format.pdf`/`format.html` e qualquer
projeção interna devem partir do mesmo namespace efetivo para impedir deriva.

### 3.2 Identidade e projeções

Congelar antes da decisão final:

- `type`, `repr` e nome de `html`/`pdf`;
- chamada mínima, chamada inválida e diagnóstico;
- igualdade/identidade entre membros globais e os projetados sob `format`;
- `.with(...)` preservando namespace;
- comportamento com feature ausente/presente.

`format.pdf.attach == pdf.attach` e os equivalentes HTML devem preservar a
mesma função dona; não criar wrappers só para obter nomes iguais.

### 3.3 Compatibilidade interna

Atualizar consumidores que hoje fazem pattern match em `Value::Module`, em
especial `compiler/stdlib/format.rs`, `compiler/eval/mod.rs`, field access e
testes. Não ensinar o field access a fingir que módulos são funções. A fonte do
valor deve ser corrigida.

## 4. L0s causais

Retificar antes de código:

- `00_nucleo/prompts/compiler/stdlib/html.md`;
- `00_nucleo/prompts/compiler/stdlib/pdf.md`;
- `00_nucleo/prompts/compiler/stdlib/format.md`;
- `00_nucleo/prompts/compiler/eval.md` e/ou o owner exato do registry global;
- `00_nucleo/prompts/compiler/eval/bindings/field_access.md` somente se seu
  consumer mudar;
- prompts de testes efetivamente alterados.

Registrar explicitamente que a medição nova substitui a conclusão antiga sem
apagar o histórico. Preservar ownership 1:1 e caminhos canônicos.

## 5. Gate ADR-0127

Trocar `module` por `function` é quebra deliberada de contrato público, embora
seja necessária para paridade. O fluxo obrigatório é L0 causal, hashes,
V5/V15/V26, commit documental e **confirmação humana posterior**. Nenhuma
aprovação dos passos PDF autoriza esta mudança.

## 6. RED e fechamento

O RED deve falhar no kind atual e cobrir os quatro perfis do catálogo. O GREEN
exige:

- dois roots `function`, zero `module` nesses nomes;
- namespaces completos e feature-gated idênticos aos anteriores;
- chamadas e diagnósticos bilaterais;
- projeções `format.*` sem cópia divergente;
- queda de `WRONG_KIND` de 3/118 para zero sem converter filhos presentes em
  ausentes;
- contagens de membros iguais antes/depois, exceto pela classificação do root.

Executar suíte completa, all-features, lints e regenerar os oito catálogos duas
vezes. Byte-identidade entre regenerações prova determinismo do inventário,
não igualdade de implementação com o vanilla.

## 7. Auditoria L0 e retificação (2026-09-21)

### 7.1 Revalidação da medição de §2

Sonda bilateral repetida em 2026-09-21 contra o vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d` (`/usr/local/bin/typst`) e o
cristalino corrente (`target/debug/typst`, build de working tree sobre o
commit `860c7114b`):

```text
typst eval --features html '(type(html), repr(html), type(pdf), repr(pdf))'
vanilla:    ["function","html","function","pdf"]
cristalino: ["module","<module html>","module","<module pdf>"]
```

Confirma exatamente §2 sem deriva desde 2026-09-20T17:40:24-03:00.

### 7.2 Viabilidade medida (grounding para a decisão, não decisão)

`01_core/src/entities/func.rs:122,165-180` já expõe
`NativeFunc.namespace: Option<Arc<Scope>>` e o construtor
`Func::native_with_namespace(name, call, namespace)` — mecanismo P493,
produtivo desde antes deste passo (usado por `table.header` e por
`place.flush`, ver `compiler/eval.md` P1292). `compiler/stdlib/format.rs`
já constrói `format.pdf`/`format.html`/`format.svg`/`format.png`
exatamente assim. A retificação de §7.3 não exige novo tipo, trait ou
campo em `entities/func.rs`/`entities/module.rs` — é uma troca de forma no
ponto de construção (`make_pdf_module`/`make_html_module`) e nos quatro
consumidores que hoje assumem `Value::Module` (`format.rs` × 2 guardas,
`field_access.rs::is_a11y_pdf_target`, testes).

### 7.3 L0s retificados (5 ficheiros)

`compiler/stdlib/pdf.md`, `compiler/stdlib/html.md`,
`compiler/stdlib/format.md`, `compiler/eval.md`,
`compiler/eval/bindings/field_access.md` — cada um com uma nova secção
`## P1471`, registando explicitamente que a medição de §7.1 substitui a
caracterização anterior ("`Value::Module`"/"módulo global") sem apagar o
histórico das seções anteriores (P1166–P1466, P1464/P1465 em cada owner),
conforme exigido por este próprio passo (§4) e pela regra de retificação
do `CLAUDE.md`.

Owners não tocados por decisão consciente: `entities/func.md` e
`entities/module.md` (nenhum campo/tipo novo — ver §7.2);
`entities/scope.md` (mecanismo de namespace já existente e inalterado).

### 7.4 Gate ADR-0127

Hashes ressealados **somente nos 5 L0s** (`crystalline-lint --fix-hashes
.`); os `@prompt-hash` dos `.rs` correspondentes foram revertidos
(`git checkout --`) antes do commit — o reseal do código acontece só
junto da implementação, não no commit documental (mesma disciplina de
P1467–P1469). `crystalline-lint --checks v15,v26 .`: zero violações.
`crystalline-lint --checks v5 .` mostra o drift esperado (L0 mudou, código
ainda não) nos 5 ficheiros `.rs` — não é bloqueante, é o sinal correto de
"aguarda implementação". Commit causal somente-L0 a seguir; **confirmação
humana explícita obrigatória antes de qualquer código** (§5). A confirmação
de P1467–P1470 não cobre esta mudança de contrato público.


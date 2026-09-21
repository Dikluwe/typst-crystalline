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

## 8. Execução segregada e integração (2026-09-21)

Confirmação humana explícita recebida em 2026-09-21, na mesma sessão,
posterior ao commit causal somente-L0 `2ae054ab5`. Os três papéis partiram
desse baseline em worktrees isoladas.

- **Agente A (implementação):** commit `e118c2053` ("feat(eval): implement
  pdf/html as Value::Func with namespace (P1471, agente A)"). Implementou
  os 5 pontos do plano (§7.3) sem ver os testes. Achado fora da auditoria
  original dos 5 L0s: `P1293HtmlNative::from_func`
  (`compiler/eval/call_dispatch.rs`, código de produção, despacho de
  natives HTML `button`/`iframe`/`col`/`wbr`) também lia
  `make_html_module().scope()` e quebrava a compilação — corrigido pelo
  próprio agente e reportado explicitamente para decisão do integrador
  (ver `compiler/eval/call_dispatch.md` P1471, owner próprio,
  retroativamente identificado).
- **Agente B (testes):** commit `b546f709b` ("test(eval): freeze pdf/html
  function-kind red contract (P1471, agente B)"). 14 testes novos, RED
  confirmado por observável de língua (`eval_expression_with_features`),
  deliberadamente **sem** reconstruir `make_html_module`/`make_pdf_module`
  ao nível Rust — o L0 já anunciava mudança de assinatura, e testar essa
  forma teria bloqueado a compilação do crate inteiro em vez de falhar por
  asserção. 7 dos 14 eram RED genuíno (falha de asserção/panic); 7 eram
  não-regressão desenhada para continuar GREEN antes e depois (medido, não
  assumido — `Func::PartialEq` já comparava por nome antes desta
  retificação).
- **Agente C (integração, o próprio dono desta execução):** extraiu os
  dois patches, aplicou sem conflito, e fez as correções mecânicas e de
  achado necessárias:
  1. 13 call-sites de teste que ainda assumiam `Module`/`.scope()`, fora
     do plano original dos agentes (11 em `html.rs`, 1 em
     `html/p1423_tests.rs`, 1 em `html/p1424_tests.rs`), mais 3 em
     `format.rs`/`call_dispatch.rs` — quebravam `cargo test` mas não
     `cargo build`;
  2. import `Module` órfão em `html.rs` depois da correção acima;
  3. **regressão real** descoberta pela suíte completa: a simplificação
     planeada de `is_a11y_pdf_target` (§7.3 item 5) deixava
     `pdf.data-cell`/`header-cell`/`table-summary` acedidos na **raiz**
     (não `format.pdf.*`) sem `a11y-extras` cair no diagnóstico genérico
     de field-access-em-função em vez do gate medido em P1303 — a própria
     análise deste L0 (§"Decisão" da secção P1471 de `field_access.md`)
     estava incompleta: assumia que `is_format_pdf_namespace` já
     reconheceria a raiz, mas essa função exigia os quatro defaults
     exclusivos da projeção `format.pdf` (`pages`/`standard`/`tagged`/
     `pretty`), ausentes no namespace da raiz. Corrigido relaxando
     `is_format_pdf_namespace` para exigir só `attach`/`artifact` — a
     correção e a nota de proveniência ficaram registadas na própria
     secção P1471 de `field_access.md` ("Correção descoberta na
     integração"), não apagando a análise original errada;
  4. 4 oráculos históricos pré-existentes que assumiam `type(pdf) ==
     module` (`compiler/eval/tests.rs`, owner próprio
     `compiler/eval/tests.md`, retroativamente identificado — não coberto
     pelos 5 L0s auditados em §4/§7.3): `p735_type_emoji_e_pdf_sao_module`
     (só a metade `pdf` mudou; `emoji` continua module) e
     `p1305_oracles::p1305_named_modules` (`repr(pdf)` de `<module pdf>`
     para `pdf`); as outras 2 falhas do primeiro `cargo test` completo
     (`p1303_negativos_pdf_exatos_nos_perfis_sem_a11y`,
     `p1306_pdf_dict_float_and_import_controls`) resolveram-se com a
     correção do item 3 acima, sem edição adicional.

Dois owners L0 adicionais foram identificados e atualizados
retroativamente, fora do escopo original de §4 (achados de integração, não
decisão nova): `compiler/eval/tests.md` e `compiler/eval/call_dispatch.md`.

### 8.1 RED → GREEN por ficheiro

| Ficheiro | Novos testes (RED no baseline `2ae054ab5`) | GREEN (após integração) |
|---|---|---|
| `compiler/stdlib/pdf.rs` | 4 novos — 3 falha de asserção, 1 panic | 10/10 (4 novos + 6 irmãos) |
| `compiler/stdlib/html.rs` (+ submódulos P1423/P1424) | 5 novos — falha de asserção/panic; 13 call-sites de teste pré-existentes quebravam a compilação | 36/36 (5 novos + 31 irmãos) |
| `compiler/stdlib/format.rs` | 2 novos — GREEN por desenho (não-regressão medida); 1 call-site pré-existente quebrava a compilação | 4/4 (2 novos + 2 irmãos) |
| `compiler/eval/mod.rs` | 1 novo — falha de asserção | 8/8 (1 novo + 7 irmãos) |
| `compiler/eval/bindings/field_access.rs` | 2 novos — 1 GREEN por desenho, 1 regrediu na integração (item 3) | 35/35 (2 novos + 33 irmãos) |
| `compiler/eval/tests.rs` (achado, owner próprio) | 4 oráculos pré-existentes falhavam após a implementação de A | 1288/1288 |
| `compiler/eval/call_dispatch.rs` (achado, owner próprio) | quebrava a compilação (código de produção + 1 teste) | GREEN |

Total: **14/14 testes novos GREEN** (4+5+2+1+2).

### 8.2 Gates finais medidos (2026-09-21)

```text
cargo build --workspace                                     -> sucesso
cargo test -p typst-core --lib                                -> 6059/6059
cargo test -p typst-shell --lib                                -> 83/83
cargo test -p typst-infra --lib                                -> 995/995
    (1 flake pré-existente e não relacionado em execução isolada,
     `p821_target_fora_de_contexto_erro_no_pipeline`, confirmado GREEN
     tanto isolado quanto em duas execuções completas subsequentes)
cargo fmt --all -- --check                                     -> sucesso (após 1 `cargo fmt --all`;
                                                                    sem mudança semântica, testes
                                                                    reconfirmados GREEN)
cargo clippy --workspace --all-targets --all-features          -> 0 erros; comparação por contagem
                                                                    `(ficheiro, mensagem)` contra o
                                                                    baseline `2ae054ab5` em worktree
                                                                    isolada (não por número de linha,
                                                                    que desloca com inserções) — zero
                                                                    avisos novos nos ficheiros tocados
crystalline-lint .                                              -> 0 erros (só info V16-V20
                                                                    pré-existentes, fora do diff)
crystalline-lint --checks v5,v15,v26 .                          -> 0 violações
git diff --check                                                -> sucesso
```

Não foi corrido `cargo test --workspace` completo nem a suíte de export PNG
em massa de `typst-wiring` (mesma razão documentada em P1464/P1467-P1470);
o raio deste diff é inteiramente L1 (`01_core`), sem consumer L2-L4
(confirmado por grep de `Value::Module` combinado com `pdf`/`html` em
`02_shell`/`03_infra`/`04_wiring` — zero ocorrências), logo `typst-core` +
`typst-shell` + `typst-infra` (que depende de `typst-core`) cobrem
integralmente o raio do diff.

O veredito final é: `type(pdf)`/`type(html)` passam de `module` para
`function`, com `repr`/chamada direta/diagnóstico idênticos aos medidos no
vanilla ratificado, preservando integralmente os namespaces, gates e
membros já materializados nos 5+2 owners tocados — incluindo dois
achados de integração (um consumer de produção fora da auditoria original,
e uma regressão real no gate a11y da raiz `pdf` que a própria análise do
L0 não previra e que só a suíte completa expôs).


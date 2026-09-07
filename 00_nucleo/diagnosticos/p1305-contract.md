# P1305 — auditoria de viabilidade do contrato de repr

Estado: diagnóstico pré-contrato; **não selável na fronteira produtiva vigente**.
Este documento não é Prompt L0, não legitima implementação e não aprova solução.

Manifesto predecessor: `00_nucleo/diagnosticos/p1305-manifest.json`, SHA-256
`783cf02f9832e7a06ce0e53a1bc356ee5ca13eb3f225fc506649c023cc665b3b`.
Autor: subagente `p1305_contract`, exclusivamente auditor de fontes/contrato.
Regime: `executado sem atestação de isolamento técnico`; filesystem compartilhado,
sem candidato P1305 recebido ou lido. Única escrita autorizada deste autor:
`00_nucleo/diagnosticos/p1305-contract.md`.

## Proveniência e entradas

Inspeção de fonte no HEAD `8eb41b769eb840c7ab1063f981f98fdd4047952b`,
working tree não commitado, reconferida em `2026-09-07T10:14:52-03:00`.
O manifesto preserva status, hash do diff, pins e `git diff HEAD --stat`:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  80 +++++
 00_nucleo/prompts/compiler/eval/tests.md           | 100 +++++-
 01_core/src/compiler/eval/bindings/field_access.rs |   4 +-
 01_core/src/compiler/eval/tests.rs                 | 382 ++++++++++++++++++++-
 4 files changed, 562 insertions(+), 4 deletions(-)
```

Essas mudanças preexistentes pertencem a P1303; não foram editadas pelo auditor.
Números de linha abaixo referem-se a esse estado, não ao HEAD isolado. Os
resultados dinâmicos frescos são responsabilidade do coordenador e serão
ligados por recibo downstream. Este documento não inventa stdout, contagens
de execuções, mutation score ou resultado bilateral ainda não recebido.

Lidos integralmente: passo autorizado
`/repos/Antigravity/typst-crystalline/00_nucleo/materialization/typst-passo-1305.md`,
`CLAUDE.md`, `01_core/CLAUDE.md`, skill
`/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md` e suas
referências `papeis-e-capacidades.md` e `artefatos-e-gates.md`; ADR-0107,
ADR-0108 e ADR-0127. A busca dirigida em `00_nucleo/adr/` não identificou ADR
própria de materialização segregada. Não foram abertas outras entradas de
`materialization/` nem `context/`.

L0 lidos integralmente, com SHA-256 de arquivo reconferido:

| L0 | SHA-256 |
|---|---|
| `00_nucleo/prompts/compiler/eval/repr.md` | `555b6df222a931991f1a521b11fe2d8d58ee185cdcf90cd3f0eaf6d3c40c9355` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `f50afc2f609a738f3fff5e7c511cdc26d8e6c8c3878107862ef0881c0a45a479` |
| `00_nucleo/prompts/entities/module.md` | `17d75e8c05e28546b7212ae232fa5d9e0474eda2b5bd7b67740e54ec3c1659a9` |
| `00_nucleo/prompts/compiler/eval/modules.md` | `dd0f3ea34d68ddf3b1ebf57c28f29f7bf351fbac868387288089e08487c0c085` |
| `00_nucleo/prompts/compiler/eval.md` | `ce02b7d257e29742071497f112f1ea42ed734a93989e8fbb1332f1c09590f5e2` |
| `00_nucleo/prompts/entities/scope.md` | `6de9960c9814215012d3ace19ff41e7dc38d3df0f4806bf35e78366760b0214a` |
| `00_nucleo/prompts/compiler/scopes.md` | `56ede3e2398f8c9a0e4c8d4929324fa621e4d2d8b1f82ac826eca303ba87a080` |

Os hashes dos consumers principais constam no manifesto. Fontes auxiliares
inspecionadas: `01_core/src/entities/scope.rs` SHA-256
`b1d264bea19ef78c49f843c3d200d2ce212d0b73fcd0bc110a1352804a869c42` e
`01_core/src/compiler/scopes.rs` SHA-256
`1c06d7304feccb68b25e956191026d8fd3d19d4e09844c665f75338e56595362`.

## Evidência anterior à classificação

1. `01_core/src/compiler/eval/repr.rs:28` recebe somente `&Value`;
   `:56` representa Module pelo seu `name()`. O builtin
   `01_core/src/compiler/stdlib/foundations/repr.rs:22-30` encaminha o valor,
   sem transportar World, FileId ou contexto ao formatter. A fachada de
   serialização em `01_core/src/compiler/eval/mod.rs:83-84` tem a mesma
   entrada limitada.
2. O global é construído com nome interno `std` e scope clonado da stdlib
   nos caminhos expressão (`eval/mod.rs:324`), documento (`:562`) e arquivo
   importado (`eval/modules.rs:66`). Cada construção chama `Module::new`.
3. `01_core/src/entities/module.rs:32-44` declara integralmente o carrier:
   `name`, `scope`, `content`, `introspection_content`, `bib_styles` e
   `document_info`. Não há origem, FileId, kind ou identidade pública
   alternativa. `:62-70` inicializa os quatro últimos campos com ausência,
   mapa vazio e metadados vazios. `:73-134` expõe nome/scope e esses campos.
4. O import de arquivo obtém seu nome de `bare_name()`
   (`01_core/src/compiler/eval/modules.rs:169-177`). Seu eval tem scope local
   inicialmente vazio sobre Library (`:57-78`), descarta o conteúdo avaliado
   (`:102-108`) e retorna `Module::new(name, module_scopes.exit())`
   (`:113-114`). Não aplica setters de conteúdo ou metadados.
5. O alias de import muda o binding, preservando o objeto Module
   (`eval/modules.rs:203-213`). Import de uma expressão que já é Module
   preserva diretamente o objeto e seu nome (`:180-188`). Portanto spelling
   da variável não é identidade pública do módulo.
6. Wildcard percorre todos os bindings em ordem e clona seus valores para
   o scope local (`eval/modules.rs:215-218`). `Scopes::define` apenas
   encaminha para `top.define`, e `exit` retorna esse scope
   (`01_core/src/compiler/scopes.rs:112-119`).
7. `Binding` contém somente `Value` (`01_core/src/entities/scope.rs:26-34`);
   `Scope` contém somente `IndexMap<String, Binding, FxBuildHasher>`
   (`:63-65`). Definir binding não marca sua procedência (`:74-75`). Não há
   flags de constância, deduplicação, categoria ou span disponíveis aí.
8. A identidade Arc é observável por `Module::PartialEq`
   (`entities/module.rs:53-58`), e aliases clonados a conservam (`:140-143`).
   O formatter não recebe uma referência canônica ao global com a qual
   compará-la. Um novo `Module::new` sempre aloca outro Arc (`:62-70`).
9. O eval principal aplica setters de conteúdo/introspecção/metadados ao
   módulo resultante do documento (`eval/mod.rs:636-643`). Isso não caracteriza
   imports: seu caminho separado termina no constructor simples do item 4.
10. A projeção diagnóstica existente troca nominalmente `std` por `global`
    (`01_core/src/compiler/eval/bindings/field_access.rs:376-380`). Não
    consulta uma evidência adicional de origem.
11. No vanilla ratificado `a51e02804`, a construção global já guarda `global`
    (`lab/typst-original/crates/typst-library/src/lib.rs:353-374`). Seu repr
    imprime o nome guardado
    (`lab/typst-original/crates/typst-library/src/foundations/module.rs:174-179`).
    A fonte não exige reproduzir a estrutura Rust de Module para obter a
    mesma morfologia.

## Contraprova de construção e invariantes rejeitados

A construção de linguagem a medir é um arquivo ordinário chamado `std.typ`
contendo exatamente `#import std: *`, importado com alias pelo documento.
Pelos itens 2, 4, 6 e 7, ele recebe o mesmo nome interno `std`, o scope
integral do global da sua avaliação, com nomes, ordem e valores clonados,
e todos os demais campos nos mesmos defaults. Os scopes globais de
avaliações distintas podem conter allocations diferentes; não se afirma
igualdade de seus ponteiros. Essas allocations também variam entre dois
globais legítimos e não fornecem uma categoria global/importado.

Há ainda uma contraprova da interface pública do carrier: dado um global
`g`, `Module::new("std", g.scope().clone())` é uma construção permitida pela
API/L0 vigente e conserva integralmente seus valores, incluindo identidades
dos valores clonáveis internos, enquanto cria uma nova identidade externa.
Isso não é apresentado como constructor Typst nem como medição CLI; explicita
por que o carrier não contém um discriminante intrínseco da origem.

Consequências dos caminhos observados:

| Predicado proposto | Refutação |
|---|---|
| `name() == "std"` | Também vale para import ordinário de `std.typ`. |
| Ausência de conteúdo/introspecção | O eval importado descarta conteúdo e usa os mesmos defaults. |
| Metadados/estilos vazios | Ambos os constructors usam esses defaults. |
| Builtins presentes, quantidade, tipos, nomes, ordem ou assinatura integral do scope | Wildcard copia todos os bindings e valores; origem não é guardada no Binding. |
| Nativas versus funções de usuário | Um arquivo pode reexportar as próprias nativas e módulos do global. |
| Constância do binding na Library | É propriedade da pilha `Scopes`, não do Module recebido; aliases e reexports separam essa propriedade do objeto. |
| Comparação com novo global construído no formatter | O novo Arc não é a identidade de nenhum global anterior; comparação estrutural também aceita o reexport. |
| Regra diagnóstica já existente | Faz somente a troca nominal refutada acima. |
| Path, span, nome de fixture/probe ou allocation incidental | Ausentes da entrada legítima ou vedados por P1305; não provam nome público. |

## Classificação, limite e decisão de viabilidade

O observável exigido é a morfologia pública do nome, sob ADR-0107. A análise
de estrutura Rust acima serve para verificar informação disponível para
implementar esse observável; igualdade Rust/ponteiros não é critério de
aceitação de paridade. Não se infere intenção histórica do upstream a partir
de sua implementação.

Inferência resultante: não há discriminante sólido para converter somente o
global de `std` em `global` dentro do owner produtivo `repr.rs`, preservando
todo import nomeado ordinário. A construção wildcard refuta a suposição de
que reconhecer a stdlib pelo seu conteúdo resolveria o problema. Refutaria
esta conclusão uma invariável existente, acessível ao formatter por suas
entradas atuais, demonstradamente exclusiva do global em todos os caminhos
legais de construção. Nenhuma foi encontrada na inspeção completa do carrier,
do Scope/Binding e das rotas produtivas de construção pertinentes.

Resultado do autor do contrato: `INSUFFICIENT_CARRIER_FOR_REPR_ONLY_SCOPE`;
`STOP_BEFORE_PATCH`; contrato não selável. O verificador independente deve
julgar esta conclusão e a medição bilateral, sem este autor aprovar o próprio
artefato. Não existe selo, candidato, GREEN ou mutation score P1305. `Unknown`
não satisfaz os controles obrigatórios. O protocolo para antes da fase de
implementação; não há fundamento para executar corpus completo ou materializar
somente arrays e declarar fechada a coorte indivisível.

## Alternativas para decisão futura, sem aprovação de implementação

Não se conclui que novo campo, entidade ou API seja obrigatório. Uma hipótese
menor a investigar é guardar o nome público `global` nos três constructors
existentes de std, em `eval/mod.rs:324`, `:562` e `eval/modules.rs:66`, e
formatar o nome guardado. Isso ampliaria os owners produtivos para
`compiler/eval.md` e `compiler/eval/modules.md`, exigiria revisão L0-first e
classificação explícita do gate. Bare imports de fonte Module consultam esse
nome (`eval/modules.rs:187`), e diagnósticos também o observam; logo a hipótese
não está aprovada como alteração puramente invisível e deve medir esses
efeitos antes de qualquer decisão. P1305 §6 proíbe expressamente mudar a
construção do std, portanto a fronteira precisa de decisão humana mesmo se
uma futura análise classificar a correção em fluxo contínuo ADR-0127.

Outra hipótese seria transportar proveniência explícita no carrier e
projetá-la no formatter; envolve novo escopo de entidade/contrato, cuja forma
e gate precisam de decisão própria. Não é a recomendação implícita desta
auditoria nem justificativa para resolver anonimato de plugin.

A skill `artefatos-e-gates.md` exige reabrir o desenho quando a classificação
depende de informação ausente da entrada/saída e proíbe converter esse caso em
selo. A obrigação específica de parar vem também de P1305 §4: "Se o carrier
não permitir distinguir global de módulo ordinário, registrar a contraprova
e parar antes do patch". A cadeia downstream deve registrar a necessidade
de reabrir o escopo, preservando os doze caminhos selecionados e toda a dívida
P1304, sem promover o diagnóstico a implementação concluída.

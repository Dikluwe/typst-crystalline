# P1305-r2 — contrato público da correção ampliada

Estado: contrato candidato para gate discriminatório; não é selo nem
certificado. Autor `p1305_contract`, sem leitura de implementação candidata.
Regime: `executado sem atestação de isolamento técnico`.

## Entradas e autorização

O dono autorizou corrigir também a construção do global em `eval/mod.rs` e
`eval/modules.rs`, mantendo os demais limites. A autorização está registrada
em `p1305-r2-reopening.md`, SHA-256
`114c132993326126231b853e9f6f86b2c0c1c166c97a708c768f371c8bdf772c`.
Manifesto predecessor `p1305-r2-preflight-manifest.json`, SHA-256
`dc295b83e5604eee55734168a1a3f13e58dba48b6b40b9715c191c381dd88366`.
Os paths nesta seção pertencem a `00_nucleo/diagnosticos/`.

Passo original e evidências bloqueadas ficam imutáveis. A conclusão anterior
de insuficiência dentro de repr-only permanece verdadeira; a presente
reabertura acrescenta a autoridade de modificar a origem do nome público.
Não autoriza nova entidade/API, anonimato de plugin, encoders, layout/export,
warnings ou mudanças em `bindings/field_access`.

L0 lidos integralmente antes da proposta: `compiler/eval/repr.md`,
`compiler/eval.md`, `compiler/eval/modules.md`, `compiler/eval/tests.md`,
`entities/module.md`, `entities/scope.md` e `compiler/scopes.md`, todos sob
`00_nucleo/prompts/`. Os hashes iniciais e evidências estão na auditoria
anterior `p1305-contract.md` SHA-256
`bd7dfc46dbcd93087b7abd6c67bd64938c0162344cfb38606a4163083dcab038`.
Somente os quatro L0 de eval foram atualizados nesta reabertura. Seus hashes
finais serão pinados no manifesto/selo após resselo de linhagem; este
documento não antecipa hashes que a normalização de metadados possa mudar.

## Medição anterior à decisão

Evidência independente: `p1305-r2-pre-measurement.json` SHA-256
`fd6354824e500e61f18b14116dd54b4f4838691289226a7f7e04236258dd9da0`.
Ela contém fixtures e probes, argv, cwd, binários, stdout/stderr integrais,
exit, duração, ordem normal/invertida, predecessores de calibração e
proveniência da árvore. Janela medida:
`2026-09-07T13:48:28.580684+00:00`–`2026-09-07T13:51:19.266604+00:00`.
HEAD `8eb41b769eb840c7ab1063f981f98fdd4047952b`, working tree não commitado:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  80 +++++
 00_nucleo/prompts/compiler/eval/tests.md           | 100 +++++-
 01_core/src/compiler/eval/bindings/field_access.rs |   4 +-
 01_core/src/compiler/eval/tests.rs                 | 382 ++++++++++++++++++++-
 4 files changed, 562 insertions(+), 4 deletions(-)
```

Hash desse diff: `fca8f14b7fcad814ca954d8933c05a6e2644683c9a4529d9f169bcfb5d1c2a49`.
Vanilla ratificado upstream/main `a51e02804`, binário `/usr/local/bin/typst`
SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Baseline cristalino reconstruído em target isolado
`/dev/shm/p1305-target.3j4tck2g/release/typst`, SHA-256
`4a4e1bd46c053dd19bfcae2230537c9478fbfb2ff26a94dfd87d9f29fee2835d`.

Em `01_core/src/compiler/eval/repr.rs:36-41`, arrays percorrem todos os
itens; em `:56`, Module imprime `module(nome)`. O vanilla limita arrays a
40 e acrescenta a quantidade omitida
(`lab/typst-original/crates/typst-library/src/foundations/array.rs:1188-1199`),
e imprime o nome público do Module (`foundations/module.rs:174-179`). Os
probes medem ausência de marcador até 40, `.. (1 items omitted)` em 41,
`.. (2 items omitted)` em 42 e reindentação correta em nesting.
`array-data-*` serializa o array integral após repr. `ascii-body-49/50/51`
preserva a fronteira anterior. `fields-dict-41`, `fields-args-41` e
`content-sequence-41` registram sua forma baseline, não autorizam consertar
esses outros formatters.

Global é construído com nome `std` em `eval/mod.rs:324,562` e
`eval/modules.rs:66`. Arquivos importados usam nome próprio em
`eval/modules.rs:113-114`, inclusive `std.typ` reexportando `std: *`.
O vanilla guarda `global` na construção (`typst-library/src/lib.rs:374`).
`named-modules`, `ordinary-collisions`, `reexport-collision`, `imported-route`,
`nested-import-route` e `document-route` medem essa separação pública.

A expansão exige preservar uma consequência até então encoberta:
`eval/modules.rs:184-213` usa `Module.name()` como binding bare. O vanilla
usa nome lexical (`typst-eval/src/import.rs:82-105`), suportado pela API AST
cristalina existente (`entities/ast/code.rs:184-206`). A medição demonstra:

| Forma | Resultado obrigatório derivado do vanilla |
|---|---|
| `import std` | Binding lexical `std`; objeto público `global`; não cria variável `global`. |
| `let renamed = std; import renamed` | Binding lexical `renamed`; objeto público `global`; não cria `global`. |
| `import holder.saved` com `holder.saved = std` | Binding `saved`; objeto público `global`. |
| `import std as renamed` ou `import (std) as renamed` | Binding `renamed`; objeto público `global`. |
| `import (std)` / `import { std }` / `import f()` sem as/lista | Erro na expressão fonte: `dynamic import requires an explicit name`; hint único ``you can name the import with `as` ``. |
| `import (std): calc` / `import (std): *` | Válidos; bindings solicitados e valores preservados. |
| `import "ordinary/std.typ"` | Binding `std`; objeto público `std`; conteúdo ordinário preservado. |

O warning vanilla `this import has no effect` está ausente no baseline;
isso é dívida histórica explícita. O diagnóstico de field ausente em um
import ordinário chamado `std` também tem dívida histórica do owner de
field access; este contrato não a corrige.

Inferência: armazenar o nome público no constructor existente, projetá-lo
diretamente e usar o nome lexical no bare import resolve a fronteira sem
nova informação no carrier. Refutam-na uma rota legítima ainda indistinta,
novo binding/aceitação causada pelo rename, alteração de dados/features ou
necessidade de outra API/owner. Os ajustes privados de ligação e guard
dinâmico pertencem ao owner já autorizado e impedem regressões da correção.

## Obrigações congeláveis

Classificação: `ADR-0127_CONTINUOUS_PARITY_CORRECTION_WITH_AUTHORIZED_CONSTRUCTION_SCOPE`.
Semântica, sintaxe e morfologia são critérios da língua sob ADR-0107; análise
de estrutura serve somente para demonstrar informação disponível. Não se
infere intenção histórica do vanilla a partir de seu comportamento.

1. Fechar exatamente os doze caminhos selecionados: `repr(std)`, `color.map`,
   `color.map.cividis`, `coolwarm`, `crest`, `flare`, `icefire`, `mako`,
   `rainbow`, `rocket`, `turbo` e `vlag` sob o mesmo namespace `color.map`.
   Cada um deve atingir `MATCH_VALUE` nos quatro perfis. Não reduzir coorte.
2. Qualquer `Value::Array` representa os primeiros `min(len, 40)` valores
   em ordem, cada um pela sua própria repr. Se exceder 40, última peça
   `.. (N items omitted)`, N exatamente `len - 40`, com plural `items`
   mesmo para um. Sem marcador abaixo ou no limite. Formatter P1290
   preserva vazio, singleton, vírgulas, largura ASCII e dois espaços por
   nível. Nesting aplica elisão independentemente a cada array real.
3. Nenhum valor é truncado, substituído ou reordenado. Exigir comprimento,
   índices 39/40/penúltimo/último quando existentes e sequência integral
   depois de repr. Repetir os canais integrais e digests dos quinze mapas
   P1304; hash/repr abreviada não prova igualdade dos dados. Serializer
   estruturado conserva todos os itens; fallback textual Module usa repr.
4. Limite pertence exclusivamente à projeção de `Value::Array`. Não elidir
   indiscriminadamente helpers genéricos, fields, argumentos, dict ou
   sequências de conteúdo. Fields contendo um array real naturalmente
   recebem a repr do array. Preservar controles baseline registrados e
   P1290/escaping/conteúdo sem ocultar sua dívida fora de escopo.
5. Module nomeado projeta `<module nome>` com o nome armazenado. Global
   guarda `global` nos três pontos de construção existentes; seu binding
   base continua `std`. Arquivos ordinários guardam o nome de arquivo e
   aliases preservam identidade pública. Proibido reconhecer origem por
   nome nominal `std`, conteúdo do scope, path, span, probe, pointer ou
   blacklist. Não alterar Module nem tratar anonimato.
6. Bare Module sem as/lista usa o nome lexical da AST: Ident ou FieldAccess;
   literal-file conserva seu caminho vigente. Guard Dynamic somente nesse
   modo, com erro/hint/span source exatos. `as`, items e wildcard continuam
   válidos para fontes Module dinâmicas; não ampliar categorias de fonte
   ainda não suportadas. Não mudar warnings, includes, World ou Route.
7. Global expressão, documento e avaliação de arquivo importado, incluindo
   import aninhado, devem ser observados nos quatro perfis. Se CLI query
   não admite flags de perfil, usar os testes Rust independentes para as
   rotas de documento adicionais; comando não suportado não conta como
   execução nem RED. Manter calc/sym/pdf/map/aliases/imports ordinários.
8. Preservar P1300/P1301r2/P1303, lookups e três gates pdf nos quatro
   perfis; retificar por autoria de oráculo a expectativa histórica
   `repr_value_module` para `<module mylib>`.

### Comparadores e precedência sobre anotações de calibração

Este contrato prevalece sobre o campo `obligation` pré-contrato da medição
no caso **`global-dynamic-unnamed`**: sua anotação histórica
`preserve-baseline` é substituída por **`match-primary-diagnostic` contra o
vanilla**, com erro, hint e span source exatos e zero laterais. Preservar
`unknown variable global` do baseline seria aceitar falha no local errado;
aceitar sucesso seria regressão. Parentheses, bloco e call seguem a mesma
regra AST; nenhum ajuste por ID ou sintaxe testemunha é permitido no produto.
Os demais probes dinâmicos negativos seguem o mesmo comparador.

Para sucesso exigido, exit zero, JSON público completo conforme o oracle e
stderr sem erros; repr textual compara a string exata incluindo indentação.
Para negativos exigidos, comparar severidade, mensagem, ordem/quantidade de
hints, âncora/span público, cardinalidade e stdout vazio. Não contar qualquer
erro como sucesso. Os casos `global-bare`/`global-alias-bare` e seus controles
unbound podem ter exclusivamente a diferença histórica do warning vanilla
sem efeito; candidato deve preservar laterais cristalinos do baseline e
exigir erro primário completo quando aplicável. Não ignorar warnings em
outros casos ou aceitar novos laterais. `global-field-bare-unbound` não
tem essa exceção. Um comparador de primary não equivale a remover stderr.

`fields-dict-41`, `fields-args-41` e `content-sequence-41` são controles de
preservação de sua forma baseline registrada; não representam equivalência
vanilla. Não absorver divergências novas nesse rótulo. Demais preservações
de dívida P1304 continuam individualizadas no ledger; os doze caminhos a
corrigir não podem exigir igualdade com o defeito baseline.

## Gate discriminatório, limites e fechamento

Congelar oráculos positivos/negativos e suas expectativas antes do candidato.
RED precisa falhar semanticamente pelos defeitos contratados. O verificador
independente exige rejeição de todos os mutantes válidos, com testemunha
observável e restauração em cópia temporária; erro de compilação não mata
mutante. Famílias mínimas: limite 39/41/sem elisão; quantidade errada; dados
truncados/reordenados/omitido alterado; reconhecimento exclusivo de mapas;
singleton/indentação; helper genérico elidido; wrapper antigo; global std;
todo Module global; conversão nominal de std.typ; construção global parcial;
binding público em lugar do lexical; guard Dynamic ausente ou aplicado a
rename/items/wildcard; lookup ou gate apagado para ocultar testemunha.

Positivos precisam passar, negativos semanticamente incorretos devem ser
`Violated`; casos opacos reais permanecem `Unknown`. Não criar um opaco
artificial para completar a matriz. Qualquer `Unknown` obrigatório, timeout,
probe inválido ou mutante válido sobrevivente bloqueia o selo. Exigir
`mutation_score = 1.0` somente após execuções reais, nunca como valor
atribuído por este contrato. No máximo duas revisões focais; sem ganho na
mesma causa dominante deve reabrir o desenho, não relaxar critérios.

Manifesto/selo finais pinam L0 pós-resselo, este contrato, suites, baseline e
planos antes da implementação. Qualquer alteração protegida invalida o selo.
Implementador não edita oráculos/contrato; verificador não corrige o material
julgado. Depois de GREEN focal, gates P1305 do passo original continuam
obrigatórios: workspace tests/build, fmt, lint arquitetural/ownership/núcleos,
dry-run hashes e diff; filtros focais devem executar casos. Corpus P1304
completo é repetido uma vez no candidato nos quatro perfis, mais controles
novos; inverter coorte, fronteiras, sentinelas e resultados inesperados.
Certificado julga resultados reais e limita a alegação aos observáveis
contratados. Nenhum resultado final ou aprovação é emitido pelo autor deste
contrato.

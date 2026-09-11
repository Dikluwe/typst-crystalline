# P1339 — fronteiras do desenho confrontadas com o fragmento obrigatório

Sucessor de `p1339-observation-design-resolution.md`; não o sobrescreve.
Autor `/root/p1339_observation_design`, investigação somente leitura com
escrita exclusiva deste diagnóstico. Sem Rust, L0, contrato, selo, testes
ou veredito. Regime: executado sem atestação de isolamento. A skill e suas
referências permanecem as já lidas na subtarefa anterior.

## Proveniência

Inspeção em `2026-09-10T02:35:34Z`, HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado.
`git diff --name-only HEAD -- 01_core 02_shell 03_infra 04_wiring` não
produziu caminhos; fontes produtivas continuam no baseline. Não houve
execução de sonda, comparação de binários ou medição de convergência nova.
O passo foi lido SOMENTE pelo path explicitamente autorizado
`00_nucleo/materialization/typst-passo-1339.md`; nenhuma pasta restrita
foi varrida/listada.

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1339.md` | `817c3a1476897fb0a847c90183e9a9fe690994f60023126997c8022c4e8b86a9` |
| `p1339-full-catalog.json` | `7b061c8e54dfac6b65c0e0b841405a5322bc06af4615ab5f2ccd8cf9b1abfe79` |
| `p1339-context-dependency-probe.py` | `7fd025bc7ef8befe84d8417408dac392c5a9258b2ec24ef0e5b664d08a33d6b0` |
| `p1339-stabilization-boundaries-probe.py` | `2900fbffdfe551f17eeefcd548981b065a86913c72c5fb9526def012f9d31f3f` |
| `00_nucleo/prompts/infra/pipeline.md` | `e326323e0b1e59310c24ce29c1d7cd2b49c4cc0b67815dde023f2ac8adc110c6` |
| `00_nucleo/prompts/compiler/eval.md` | `a0ba87212163227d1ea51d38a401ca5277ee4e1d266993b826d81275924eaa76` |
| `01_core/src/compiler/introspect/from_tags.rs` | `04b5d1fd225f6df7c33b6ea5c4798d0cad2f18b2d8e8bd447053201189f6fee4` |

Os caminhos abreviados `p1339-*` pertencem a `00_nucleo/diagnosticos/`.
Hashes das outras fontes baseline estão no diagnóstico predecessor.
Diff/stat da captura, com caminhos abreviados, identificando a árvore desta inspeção:

```text
 00_nucleo/prompts/compiler/eval.md                 | 173 +++++++++++++++++++++
 .../prompts/compiler/eval/bindings/field_access.md |  75 +++++++++
 .../compiler/eval/bindings/value_methods.md        | 160 +++++++++++++++++++
 00_nucleo/prompts/compiler/eval/call_dispatch.md   | 151 ++++++++++++++++++
 .../compiler/eval/operators/equality.md           |  67 ++++++++
 00_nucleo/prompts/compiler/eval/repr.md            |  38 +++++
 00_nucleo/prompts/compiler/eval/rules.md           |  34 ++++
 .../compiler/eval/selector_matching.md            |  78 ++++++++++
 00_nucleo/prompts/compiler/introspect.md           | 123 +++++++++++++++
 .../compiler/introspect/extract_payload.md        |  27 ++++
 .../compiler/introspect/from_tags.md              |  78 ++++++++++
 .../compiler/introspect/locatable.md              |  24 +++
 00_nucleo/prompts/compiler/layout.md               |  27 ++++
 00_nucleo/prompts/compiler/stdlib/_comum.md        |  20 +++
 00_nucleo/prompts/compiler/stdlib/counter.md       |  80 ++++++++++
 .../compiler/stdlib/foundations/float.md           | 120 +++++++++++++-
 .../compiler/stdlib/foundations/query.md           |  66 ++++++++
 .../compiler/stdlib/foundations/selector.md        |  35 +++++
 .../compiler/stdlib/primitives-constructors.md     |  25 +++
 .../stdlib/primitives-constructors/version.md      |  74 ++++++++-
 00_nucleo/prompts/compiler/stdlib/state.md         |  45 ++++++
 00_nucleo/prompts/entities/counter_registry.md     |  35 +++++
 00_nucleo/prompts/entities/element_payload.md      |  88 +++++++++++
 00_nucleo/prompts/entities/elements/emph.md        |  25 +++
 00_nucleo/prompts/entities/elements/strong.md      |  26 ++++
 00_nucleo/prompts/entities/introspector.md         |  31 ++++
 00_nucleo/prompts/entities/layout_types.md         |  27 ++++
 00_nucleo/prompts/entities/selector.md             |  82 ++++++++++
 00_nucleo/prompts/entities/show.md                 |  48 ++++++
 00_nucleo/prompts/entities/version.md              |  44 +++++-
 00_nucleo/prompts/infra/pipeline.md                | 114 ++++++++++++++
 31 files changed, 2036 insertions(+), 4 deletions(-)
```

## Evidência antes da classificação

`typst-passo-1339.md:149-164` exige receivers closure, nativa e element
function, execução de with/where e erros de receivers inválidos. Isso não
exige injeção de um ElementCtor Rust. Os elementos nativos que a linguagem
Typst conhece, como heading/strong/emph/text, não são o mecanismo aberto
`FuncRepr::Element` deste compilador. O catálogo A2 mede precisamente esses
elementos, conforme `p1339-full-a2.md:120-133`.

`03_infra/src/pipeline.rs:140-150` cria ElementRegistry vazio e passa-o ao
eval. `eval/mod.rs:653-658` só introduz `FuncRepr::Element` quando há
constructor nesse registry; `eval/call_dispatch.rs:505` chama esse ctor.
`entities/element_registry.rs:37-54` expõe o registro na API Rust.
Consequentemente, o refutador Dynamic com mutação interior do predecessor
depende de extensão externa que o caminho CLI desta tarefa não injeta.
Não é uma Value opaca demonstradamente produzida por fixture obrigatória.

O catálogo `p1339-full-catalog.json:20382-20394` já declara
`plugin-function-receiver`, relativo a with/where, `mandatory: false`.
A justificativa registrada é que nativa não-elemento e closure foram
medidas; plugin específico não é alegado. Isto precede o novo comparador;
não é remoção de caso obrigatório para fazê-lo passar. Plugin é construível
pela linguagem, mas essa dimensão específica já era opcional. O predecessor
também mostra como certificar a MESMA Func plugin retida sob mesmo host e
módulo original imutável; module/name sem host permanece insuficiente.

As fixtures de `p1339-stabilization-boundaries-probe.py:12-17` empregam
counters filtrados numéricos, Set, callback closure, produtor aninhado,
NaN e erro independente. A callback `n => n + 10` em fresh_callback_stable
é produzida por bloco SEM leitura filtrada. Portanto seu produtor é
congelado; não requer prova de equivalência entre closures novas.
As fixtures filtradas de `p1339-context-dependency-probe.py:21-22`
usam a mesma forma Set/Func e alcançam a leitura filtrada antes do assert.
Não há nesses casos um Dynamic injetado ou plugin recriado por host externo.

`prompts/infra/pipeline.md:702-709` já manda conservar tratamento legado
quando nenhuma demanda Element foi alcançada, preservar contribuições da
passagem ordinária e não reparar helpers legados. Logo o refutador em que
query/state impede alcançar Element nessa passagem está exatamente nessa
fronteira, não demonstra violação do escopo de estabilização aprovado.

## Decisão: as duas fronteiras não bloqueiam o desenho desse fragmento

A escolha candidata pode ser incorporada ao L0 sem nova pergunta humana:
seleção pela demanda real da passagem ordinária, produtores legados
congelados e tentativas selecionadas iniciadas em I0 observacional vazio.
Manter separadamente a Location/topologia necessária para posicionar o
bloco; ela não é um counter/query/page já resolvido no seed. A história
A1..A5 e D5/I4/I5 é a do diagnóstico predecessor.

Texto concreto sugerido para completar a delimitação:

> A seleção pertence à geração do bloco que alcançou demanda Element na
> passagem ordinária. Blocos que nessa passagem erram ou desviam antes de
> alcançá-la conservam o resultado/erro legado, mesmo que um seed vazio
> permitiria outro caminho. Esta preservação é limite explícito do recorte;
> não é equivalência global com a seleção dinâmica do vanilla. Selecionados
> descartam a realização de descoberta e começam sua história em I0 vazio.
> Contribuições não selecionadas entram nas árvores candidatas com a MESMA
> realização conservada; elas não são antecipadas como eventos de I0.

Isso preserva os controles legados e permite exercer Set, Func, produtor
posterior/aninhado, oscilação/crescimento, NaN e erros exigidos. A conclusão
é de viabilidade do desenho; os oráculos ainda precisam medir a realização.
Não é autorização para retirar leitura query/state de um bloco que JÁ
alcançou Element: nesse bloco o allreads continua integral, inclusive
leituras anteriores à demanda.

## Tratamento implementável dos valores fechados

Não há motivo demonstrado para postergar todos esses casos à espera de um
comparador de qualquer extensão Rust possível. Implementar no owner eval
o comparador privado dos dados fechados presentes no fragmento, mantendo:

- Tipo/presença/ordem exatos; IEEE bits para folhas float, NaN reflexivo
  somente nessa relação, sem alterar a igualdade da linguagem.
- Query compara carrier, Location, Content e campos completos. Content
  estático percorre seus campos e filhos, incluindo Metadata/State/updates;
  `Content::PartialEq` não cobre esses braços no baseline.
- State compara a Value recebida antes de display; page-numbering compara
  o valor cru. Arrays/dicts/Args/Selector/estilos levam a recursão às folhas.
- Func/Module preservados pelo MESMO produtor válido mantêm identidade,
  captura e recursos. É permitida a prova limitada de identidade retida.
  Native compara executável/variante e namespace pertinente; With conserva
  função e Args. Não usar apenas nome ou Debug.
- Função nova é uma entrada diferente para a relação conservadora; não
  é automaticamente `Unproven`. Não converter igualdade estrutural de body
  e captured Scope em identidade, pois `Value == Value` pode observá-la.
- Se todas as entradas do produtor selecionado permanecem válidas,
  conservar sua saída original também conserva callbacks que ele produziu.
  Se uma captura mudou via query/state/page, invalidar o produtor e seus
  descendentes. Isso cobre o refutador relevante de captura alterada sem
  mudança de páginas, sem igualdade global nova de Func.

Um caso construível merece distinção honesta: `state.update` pode retornar
Func nova. `introspect/from_tags.rs:63-68` efetivamente chama a callback e
armazena sua Value. Essa é estrutura fechada com identidade observável,
não Dynamic opaco. O comparador conservador pode classificá-la Different
e reavaliar; não deve classificá-la Same por Debug, nem Unknown só porque
é função. Não foi encontrado caso obrigatório de P1339 que exija corrigir
a identidade/memoização global dessas Funcs de state. O predecessor não
mediu falha desse caso e não deve transformá-lo em bloqueio demonstrado.
Se um oráculo obrigatório posterior mostrar diferença de saída/warning
nessa combinação, a testemunha reabre a causa específica; isso não
autoriza agora reparar state global ou remover o oráculo.

## `Unproven` sem semântica de produto inventada

O comparador pode conservar internamente a categoria de insuficiência
para extensões opacas. `context_reads_valid_for` não a converte em true.
Entretanto um diagnóstico genérico novo para documento válido, ou warning
vanilla de não convergência produzido só por falta de comparador, não faz
parte desta decisão. Não atribuir nova política de erro às extensões Rust
externas como requisito para fechar o fragmento CLI que não as constrói.

No contrato, opaco DELIBERADO externo continua Unknown e é identificado
como tal antes do selo; um obrigatório continua bloqueante se chegar a
Unknown. O que se retifica aqui é a classificação anterior da ameaça:
nenhuma fixture obrigatória examinada força essa opacidade externa. Não
foram apagados obrigatórios, não se declarou cobertura de plugins opcionais
nem se certificou uma categoria por fallback permissivo.

Conclusão de desenho: pode-se completar e congelar o L0 desse fragmento
com o seed e a seleção acima, retenção causal e comparação fechada. Os
gates funcionais/discriminatórios continuam necessários; as duas hipóteses
externas do predecessor, por si, não justificam nova rodada documental ou
gate humano. Não há PASS nem selo neste diagnóstico.

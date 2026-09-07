# Prompt L0 — `entities/args` — argumentos e ocorrências causais
Hash do Código: 13bc8f39

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/args.rs`
**ADRs**: ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130
**Estado P1307-R3**: `DRAFT_L0_AWAITING_ADR0127`; contrato redigido, campos/API
e compatibilidade ainda sem aprovação humana. Não materializar antes do gate.

## Medição anterior à decisão

Baseline HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, mais working tree
P1306 preservado em `00_nucleo/diagnosticos/p1307-r3-baseline.json`, SHA-256
`b50e726c5830c0f91a6875d2d0a758903bc93b0bd719f4b5357828522a999293`,
capturado em `2026-09-07T17:03:42.154788+00:00` antes da escrita L0.
`01_core/src/entities/args.rs:17-32` contém items, named e span agregado;
`compiler/eval/call_dispatch.rs:389-430,1016-1023` apaga origens e named
anteriores no spread/With. A informação ainda existe na AST antes de eval.

A matriz independente `00_nucleo/diagnosticos/p1307-r2-measurement.json`,
SHA-256 `847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`,
mede no vanilla ratificado `a51e02804` origens diferentes de Args iguais após
factory/sink e cast do primeiro `pretty:"bad"` apesar do override booleano.
Na fonte ratificada `lab/typst-original/crates/typst-library/src/foundations/args.rs:218-235`,
cada ocorrência named sofre cast; `:414-445,485-491` filter/map preservam
origens seletivamente; `:468-482` Add não tem a regra de With. A informação
é necessária para observáveis da língua, não para reproduzir layout Rust.

O L0 anterior afirmava spread adiado e nativas recebendo `&[Value]`; a fonte
vigente `call_dispatch.rs:410-419,461-468` já expande spread e entrega `&Args`.
Essas afirmações históricas são substituídas por este contrato. A igualdade
Rust atual deriva items/named/span; não se infere paridade geral com a
igualdade vanilla de `foundations/args.rs:461-465`, que ignora spans.

## Decisão e ownership

Este é o único owner de Args e ArgOccurrence. `entities/func.md` possui Func,
não redefine Args. O carrier persistente acompanha o valor; nenhum mapa
global, comparação de valores ou recuperação posterior de AST prova origem.
É escolha concreta de contrato, não alegação de impossibilidade de outro
layout. L1 permanece puro; não adicionar I/O, contexto mutável global ou crate.

## Interface pública proposta

```rust
#[derive(Debug, Clone)]
pub struct ArgOccurrence {
    pub name: Option<EcoString>,
    pub value: Value,
    pub span: Span,
    pub value_span: Span,
}

#[derive(Clone)]
pub struct Args {
    pub items: Vec<Value>,
    pub named: IndexMap<EcoString, Value, FxBuildHasher>,
    pub span: Span,
    pub occurrences: Option<Vec<ArgOccurrence>>,
}

impl Args {
    pub fn positional(items: Vec<Value>) -> Self;
    pub fn from_parts(items: Vec<Value>, named: IndexMap<EcoString, Value, FxBuildHasher>, span: Span) -> Self;
    pub fn from_occurrences(span: Span, occurrences: Vec<ArgOccurrence>) -> Self;
    pub fn occurrence_sequence(&self) -> Vec<ArgOccurrence>;
    pub fn invalidate_occurrences(&mut self);
    pub fn remove_positional(&mut self, index: usize) -> Option<Value>;
    pub fn remove_named(&mut self, name: &str) -> Option<Value>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

`name: None` significa positional; `Some` conserva o nome exato. `span`
individual ancora o argumento completo, `value_span` sua expressão-valor;
cada Span conserva seu FileId. Detached significa ausência real de âncora,
não permissão para inventar a Source da chamada final. O span agregado
continua independente dos individuais e não é usado para reconstituí-los.

### Sequência e views

- `Some(sequence)` é a sequência causal autoritativa, inclusive ordem conjunta
  e todas as ocorrências named, com os valores anteriores ainda necessários
  a casts futuros. Não é metadata puramente cosmética.
- `items` é a projeção positional na mesma ordem. `named` projeta último valor
  de cada nome, conservando a posição da primeira ocorrência na sequência.
  Valores projetados são clones dos mesmos valores avaliados, sem reavaliação.
- `from_occurrences` estabelece Some e ambas as views simultaneamente.
- `from_parts` guarda as views e span fornecidos, com None. `positional` é
  construção sintética com named vazio, span detached e None; assinatura e
  `len` Rust permanecem os vigentes (`items.len()`). `is_empty` lê ambas views.
- `occurrence_sequence` devolve clone de Some. Para None, devolve uma sequência
  sintética: posicionais primeiro, depois named na ordem da view, ambos spans
  individuais detached. Não representa a ordem lexical nem origens perdidas.
  Ao combinar síntese e origem conhecida, só o fragmento sintético é detached;
  não apagar a proveniência conhecida do outro fragmento.

### Coerência sob transformação e mutabilidade pública

`remove_positional(index)` conta somente posicionais, remove a ocorrência
exata e retorna seu valor. Índice fora do domínio retorna None sem alteração.
`remove_named(name)` remove todas as ocorrências com esse nome e retorna o
último valor; ausência retorna None sem alteração. Em Some, os dois métodos
regeneram ambas views; em None, modificam as views e conservam None. O span
agregado não muda nesses métodos. Não fazem casts nem decidem diagnósticos.

Antes de qualquer mutação direta de items/named quando occurrences é Some,
o caller é obrigado a chamar `invalidate_occurrences`, que só limpa o carrier,
conservando views/span. None já está invalidado e não exige chamada redundante.
Essa forma existe para adaptação sintética/legada explicitamente auditada;
é PROIBIDA quando descarta a origem necessária de argumentos de usuário.
Nessas rotas, usar os métodos coerentes ou transformar a sequência e chamar
`from_occurrences`. Alterar occurrences diretamente e deixar views stale
também é proibido: reconstruir o Args integral. Nenhum reader pode resolver
inconsistência escolhendo silenciosamente uma das cópias.

Campos públicos não impõem isso automaticamente no sistema de tipos Rust.
A auditoria de TODOS os writers é condição de materialização e deve
classificar cada um como síntese, transporte ou consumo. Falta de owner
autorizado é bloqueio antes do código, não justificativa para fallback.
Não validar origem por `PartialEq`, hash de Value, nome de binding ou busca
textual: valores iguais podem ter origens diferentes e NaN não é identidade.

Alterar somente Args.span, como fazem seletores privados já autorizados,
não invalida os spans individuais. Esse ajuste não pode reescrever a origem
de nenhuma ocorrência. `occurrence_sequence` não repara carrier stale.

### Clone, igualdade e observabilidade

Clone conserva valores e todos os spans; não consulta World nem executa Func.
PartialEq de Args mantém exatamente a comparação legada items/named/span,
ignorando o campo novo. Não derivar PartialEq incluindo occurrences, nem
corrigir a dívida de igualdade com vanilla nesta revisão. Debug mantém a
estrutura anterior sem exibir o carrier. A representação da linguagem é de
`compiler/eval/repr.md`, incluindo sua decisão explícita para sequência causal;
não surge por Debug. `compiler/stdlib/collections.md` possui as projeções e
callbacks de arguments; `compiler/stdlib/loading.md` possui casts/encoders.

Args não valida assinatura, não escolhe qual erro vence e não fornece
`take<T>`/`finish` tipados nesta revisão. A nativa pode consumir a sequência
com seu cursor local e selecionar arg-span ou value-span conforme a falha.
Não duplicar essa decisão no dispatcher.

## Gate e aceitação

Novo campo e tipos/métodos públicos quebram literais de construção externos:
ADR-0127 obrigatório, mesmo mantendo as views. Redigir não autoriza escrever
Rust ou testes. A migração deve substituir literais sintéticos por from_parts
ou positional e transportes por ocorrências, sem obrigar todos os consumidores
a adotar validação nova. Owners consumidores permanecem 1:1.

Após aprovação, verificar independentemente: projeções com named repetidos;
remoção ordinal e por nome; clone entre Sources; síntese sem origem inventada;
mistura de None/Some sem apagar âncoras; casts do primeiro named inválido;
Args de factory/sink; map/filter e join conforme seus owners; igualdade legada
inclusive span agregado e NaN sem usar igualdade como prova de origem.
Unknown de caso obrigatório não satisfaz o gate. Testes de invariantes devem
refutar writers que conservam Some depois de mutar views e consumidores que
descartam origens para obter um resultado aparentemente válido.

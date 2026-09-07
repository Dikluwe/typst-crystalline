# P1307-R3 — interface proposta de Args e transporte

Estado: `DRAFT_L0_AWAITING_ADR0127`. Autorização recebida somente para redigir
L0. Não autoriza Rust, headers, testes RED ou materialização. Regime executado
sem atestação de isolamento técnico; autor `/root/p1307_contract`.

Baseline pré-escrita `p1307-r3-baseline.json` SHA-256
`b50e726c5830c0f91a6875d2d0a758903bc93b0bd719f4b5357828522a999293`;
preflight SHA-256 `d333d8b298067d9e104efdad45c1241eaae0a821a7e49988dcf3c6658c1fa4d2`.
HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais working tree P1306,
capturado em `2026-09-07T17:03:42.154788+00:00`; o baseline contém bytes e
hashes de todos os L0 permitidos antes de qualquer escrita R3.

## Medição anterior à proposta

`p1307-r2-measurement.json` SHA-256
`847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`
confirma as origens f/g diferentes após Args spread e sink, e o primeiro
`pretty:"bad"` inválido apesar do override booleano posterior. Auditoria
`p1307-r2-span-audit.md` SHA-256
`adb27e4be810f841c06fb70cf611f63bb6205443e2d7d47c8485de6a6f962910`.

Leitura adicional R3, sobre o mesmo source baseline:
`lab/typst-original/crates/typst-library/src/foundations/args.rs:414-445`
filter preserva ocorrência, map preserva nome/arg-span mas destaca value-span;
`:485-491` os resultados de ambos têm span agregado detached; `:468-482`
Add elimina named LHS presente no RHS antes de concatenar. With, ao contrário,
concatena todas as ocorrências (`foundations/func.rs:372-374`). A conclusão
anterior de concatenação simples para join fica refinada aqui; o predecessor
permanece imutável. A fonte cristalina `entities/args.rs:17-32` deriva
PartialEq incluindo o span agregado; R3 não corrige essa dívida de igualdade.

## API concreta para coordenação

Owner exclusivo `entities/args.md` → `01_core/src/entities/args.rs`:

```rust
pub struct ArgOccurrence {
    pub name: Option<EcoString>,
    pub value: Value,
    pub span: Span,
    pub value_span: Span,
}
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

`from_parts`/`positional` inicializam None. `from_occurrences` gera ambas as
views a partir da sequência causal: positional em ordem; named último valor,
posição da primeira ocorrência. `occurrence_sequence` clona Some; para None,
sintetiza posicionais seguidos de named na ordem das views, com ambos spans
individuais detached. Não reconstrói origem por igualdade de valores.
`remove_positional` remove exatamente a ocorrência ordinal positional;
`remove_named` remove todas as ocorrências daquele nome e devolve o último
valor. Em Some ambos regeneram views; em None mantêm None.

Campos públicos preservados não oferecem garantia automática por tipos Rust:
antes de mutar items/named diretamente quando Some, o caller DEVE invalidar
o carrier. None já está invalidado e dispensa a chamada redundante.
Em rotas de transporte de argumentos de usuário essa perda é proibida:
usar os métodos coerentes ou construir de ocorrências. Nenhum consumidor
pode aceitar Some stale ou detectar origem comparando valores. A auditoria
de todos os writers é gate de materialização; writer fora da allowlist que
precise mudança reabre owner antes de código. Não existe validação de
assinatura, mensagem ou cast nesta entidade ou duplicada no dispatch.

`span` agregado pode ser alterado independentemente para os seletores privados
legados; não muda os spans individuais. Mutação manual de occurrences só é
permitida via reconstrução integral com `from_occurrences`. Clone preserva o
carrier; PartialEq mantém a comparação legada items/named/span, ignorando
somente o novo campo. Debug não ganha metadata nova; repr da linguagem não
é responsabilidade da entidade.

## Alterações semânticas declaradas, não escondidas em metadata

- With retém todas as ocorrências para casts posteriores dos encoders;
  `.with` em si não valida assinatura nem executa a função.
- `arguments.filter/map` percorrem a ordem conjunta, incluindo named
  repetidos; filter preserva spans individuais, map destaca só value-span;
  ambos destacam span agregado. Isto pode alterar ordem/número de callbacks.
- Join Args remove do lado esquerdo todo named presente no direito, preserva
  as demais ocorrências, concatena RHS e destaca span agregado. Não usar
  o algoritmo de With. A ordem da view named pode mudar em colisões.
- `arguments.len` passa a contar ocorrências quando disponíveis; `pos` e
  `named` continuam projeções. Args::len Rust continua contando posicionais.
- PartialEq legado e demais callbacks/nativas não adotam automaticamente
  validação de todas as ocorrências; sua mudança seria outro contrato.

Tudo acima é proposta submetida ao gate, sem aprovação implícita da mudança
pública ou dos novos observáveis. `repr(arguments)` segue a decisão explícita
do owner `compiler/eval/repr.md`: ordem causal quando Some e impressão legada
quando None; não pode mudar incidentalmente por Debug. A propagação de repr
a CBOR deve ser delimitada nesse owner e em loading, sem promessa incompatível
de payload intocado.

## Owners e fronteiras

Este autor redige somente `entities/args.md`, `entities/func.md`,
`compiler/eval/call_dispatch.md`, `compiler/eval/closures.md`,
`compiler/stdlib/collections.md`, `compiler/eval/operators/join.md`.
Func conserva sua representação/interface; limpa ownership Args duplicado e
descreve With sem prescrever a antiga fusão destrutiva. Loading escolhe casts,
precedência e duas classes de âncoras downstream. Root coordena migração
math/numbering/int e os demais owners autorizados; não reexportar tipos novos
por conveniência nem ampliar entities/mod.rs sem autorização.

Após os L0 concretos e revisão independente, solicitar confirmação humana
ADR-0127 para campos/API/compatibilidade e os observáveis enumerados, além
dos encoders/defaults. Hashes documentais não são autorização nem selo de
implementação. Sem mutantes, RED/GREEN ou certificado nesta rodada.

## Bloqueio de migração identificado durante revisão

O reviewer identificou e o coordenador reconfirmou writers produtivos fora
dos quatorze owners permitidos nesta rodada: prepend em
`01_core/src/compiler/stdlib/gradients.rs:97` e
`01_core/src/compiler/stdlib/foundations/float.rs:120,124`; consumo em
`01_core/src/compiler/eval/bindings/method_dispatch.rs:46,336,388` e
`01_core/src/compiler/eval/bindings/field_access.rs:767`.
Os L0 correspondentes são `compiler/stdlib/gradients.md`,
`compiler/stdlib/foundations/float.md`,
`compiler/eval/bindings/method_dispatch.md` e
`compiler/eval/bindings/field_access.md`. Esta nota registra a necessidade de
reabertura antes de migrá-los; não os altera e não certifica inventário
exaustivo. O campo Some não pode chegar a esses writers e sair stale.

Logo os drafts concretos permitem revisão humana do desenho, mas ainda não
constituem pacote completo materializável. A autorização para esses owners,
sua redação L0 e a classificação integral de writers continuam gates
anteriores a Rust; a aprovação do campo isolado não os dispensaria.

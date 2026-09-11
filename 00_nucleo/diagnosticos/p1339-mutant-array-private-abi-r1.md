# P1339 Array — ABI de tradução privada prospectiva

Papel `/root/p1336_tests`; preparação mecânica segregada sem atestação de
isolamento técnico. Não é implementação produtiva, expected adicional,
evidência de execução ou autorização para ciclo 2.

Fonte inspecionada: somente cópia isolada do commit
`2f42d64253547734564513a1159ee6b584c1c4b4`, materializada pelo arquivo
`/tmp/p1339-mutant-array.jrXHiv/baseline-build.tar`, SHA
`ad3eb3e338d486330d6cc2a49091be3beb83cb4f68269b06bed1560f6b28bc66`.
Inventário integral de fonte/configuração no recibo de build
`p1339-mutant-array-build-r1.json`, SHA
`b972819d26f9f1465484dc0f3cfece79ea4e5a11d8dedfa1f84345594906bad6`.
Manifesto ancestral SHA
`842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b`.

## Declarações reais relevantes

```rust
crate::entities::args::Args::from_parts(
    items: Vec<Value>,
    named: IndexMap<EcoString, Value, FxBuildHasher>,
    span: Span,
) -> Args;

crate::entities::args::Args::from_occurrences(
    span: Span,
    occurrences: Vec<ArgOccurrence>,
) -> Args;

pub struct ArgOccurrence {
    pub name: Option<EcoString>,
    pub value: Value,
    pub span: Span,
    pub value_span: Span,
}

crate::entities::span::Span::from_range(FileId, Range<usize>) -> Span;
crate::entities::span::Span::detached() -> Span;
crate::entities::file_id::FileId::from_raw(NonZeroU16) -> FileId;
crate::entities::source::Source::new(FileId, String) -> Source;
crate::entities::bytes::Bytes::new(Vec<u8>) -> Bytes;
```

`Args::from_parts` constrói `occurrences=None`; a vista sintetizada por
`occurrence_sequence()` detacha tanto `span` quanto `value_span`. Logo não
constrói sozinho os vetores U01/U03 da ponte autoral r1: eles precisam de
occurrence.span detached **e** value_span conhecido. Para aqueles quatro
vetores exatos, `from_occurrences` permite ambos os campos de forma tipada.
Um caso extra específico de `from_parts` depende de autoria explícita do
oráculo; não será inventado pelo adapter nem substituirá U01..U04.

## ABI estreita proposta do harness

```rust
fn run_array_owner_vectors(
    actual: impl FnMut(&Args, Span) -> SourceResult<Value>,
);
```

O segundo argumento é somente um span de callee disponível ao controle
isolado; nunca contém ID, expectativa ou classificação de caso. A mesma
função de harness cria Args reais e chama o binding uma vez por vetor.
As quatro expectativas pertencem à ponte autoral, não ao binding.

No controle/mutante isolado, o binding é a chamada direta ao helper real
`compiler::eval::call_dispatch::p1339_mutant_array::convert(&Args, Span)`;
o modo vem exclusivamente da configuração do processo. Não duplicar a
conversão ou validação dentro do teste. O helper é o mesmo que o CLI compilado
chama e sua identidade/fonte/patch deve ser auditada ao compilar o testbin.

No produto futuro, a ligação test-only recebe os mesmos Args e chama o owner
Array real, adaptando apenas os parâmetros adicionais do ABI nativo já
autorizado. Nenhuma nova API produtiva é pressuposta. O adapter não lê o
candidato nem fornece um stub com a resposta esperada. O verificador deve
julgar a ligação e os pins efetivos antes da medição devida.

O harness utilizará uma Source real em memória, com FileId explícito e texto
de comprimento suficiente para todos os raw ranges declarados. Não procura
offsets por conteúdo, não produz spans a partir de mensagens e não representa
origem detached como um range fictício. Construtores são inputs sintéticos
legítimos; isso não afirma que o programa público E08 produza o mesmo carrier.
E08 permanece Unknown histórico até decisão independente específica.

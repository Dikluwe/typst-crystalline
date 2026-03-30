# ⚖️ ADR-0030: Gestão eficiente de RAM é domínio de L1 — alinhamento filosófico

**Status**: `ACCEPTED`
**Data**: 2026-03-29
**Corrige (não revoga)**: ADR-0004 (justificativa sobre performance), ADR-0015 (remoção de ecow do parser)

---

## Contexto

A ADR-0004 contém a seguinte justificativa:

> "A optimização de performance do compilador... é conhecimento de
> infraestrutura, não de domínio."

Esta frase foi escrita no contexto da remoção do interner global de
`FileId` de L1 — uma decisão correcta. Mas a generalização que dela
emergiu — "optimização de performance pertence a L3" — tornou-se
uma instrução falsa que guiou decisões subsequentes na direcção errada.

A ADR-0015 removeu `ecow` do parser com base nessa generalização.
A ADR-0007 removeu `rustc_hash` com base nessa generalização.
Ambas foram posteriormente revertidas (ADR-0024, ADR-0018).

A ADR-0030 declara formalmente a posição correcta para eliminar
a ambiguidade antes que produza mais ciclos de decisão/reversão.

---

## A distinção correcta

O que pertence a L3 não é "performance" — é **I/O de sistema**.
A distinção relevante não é entre "optimizado" e "simples", mas entre
"tem efeitos colaterais no sistema operativo" e "opera exclusivamente
em CPU e RAM".

### Performance que é domínio de L1

Estas escolhas são correctas e exigidas em L1:

| Escolha | Porquê é domínio |
|---------|-----------------|
| `Arc<T>` em tipos clonados no hot path | Semântica de partilha sem cópia faz parte do modelo de dados |
| `EcoString` em `Value::Str` | Clone O(1) de strings em eval() — a performance é o comportamento correcto |
| `FxHashMap` em `Scope` | O hasher é um detalhe do tipo de dados, não infraestrutura |
| `Arc<[Content]>` em sequências grandes | Evitar cópias O(n) em percursos de layout é requisito de correctitude |
| `content_hash: u64` pré-computado em `Source` | Imutabilidade + hash O(1) — ver ADR-0031 |

Nenhum destes usos tem efeitos colaterais no sistema operativo.
São estruturas de dados de alta performance confinadas à RAM.

### O que pertence a L3 (I/O de sistema)

| Operação | Pertence a L3 porque |
|----------|---------------------|
| Ler um ficheiro do disco | Efeito colateral no sistema de ficheiros |
| Caching no sistema de ficheiros | Estado persistente fora da RAM |
| Memoização com `comemo` | Infraestrutura de invalidação de cache incremental |
| Parsing de fontes TrueType | Requer `ttf-parser`, crate de L3 |

---

## Consequências para o código existente

### ADR-0004 — código correcto, justificativa corrigida

O código produzido pela ADR-0004 (`FileId(NonZeroU16)` sem interner global,
`SyntaxText(Arc<str>)`) está correcto. Não requer alteração.

O que era errado era a generalização na justificativa. O interner global
foi removido de L1 porque era **estado global mutável** (V13) — não porque
"performance pertence a L3". `Arc<str>` em `SyntaxText` foi adicionado
precisamente porque performance de clone é domínio.

### ADR-0015 — código correcto no parser, posição filosófica corrigida

A remoção de `ecow` do **parser** em ADR-0015 mantém-se correcta: o parser
constrói strings uma vez, não as clona no hot path. `String` é suficiente.

O erro era a generalização: "logo, ecow não pertence a L1". A ADR-0024
corrigiu isso para `Value::Str` ao distinguir contextos — o parser constrói,
eval() clona repetidamente.

---

## Regra de execução

Ao decidir entre uma estrutura de dados simples e uma estrutura de alta
performance em L1, a pergunta correcta é:

**"Esta estrutura tem efeitos colaterais no sistema operativo?"**

- Sim → não pertence a L1 (mover para L3)
- Não → pertence a L1 se o domínio a justificar

A pergunta incorrecta é:

**"Esta estrutura é uma optimização de performance?"**

Performance de alocação e gestão de RAM não é "optimização" — é parte
do comportamento correcto de um compilador. Um compilador que copia
árvores O(n) quando podia partilhá-las via `Arc` não é mais puro —
é incorrectamente lento.

---

## Sobre strings e arrays

Para referência futura:

**Strings em L1**: usar `EcoString` onde o valor é clonado no hot path
de eval() ou passado entre closures. Usar `String` onde é construído
uma vez (parser, construção de mensagens de erro).

**Arrays em L1**: `Vec<Value>` é aceitável agora. Quando arrays grandes
aparecerem no hot path de eval() — documentado em DEBT.md — migrar para
`Arc<[Value]>` ou `EcoVec<Value>`. A migração é uma decisão de
correctitude, não de "optimização prematura".

---

## Referências

- ADR-0004 — justificativa corrigida (código intacto)
- ADR-0007 — revogada por ADR-0018 (exemplo do padrão errado)
- ADR-0015 — posição filosófica corrigida (código do parser intacto)
- ADR-0018 — precedente: "o critério não é 'é externo?' mas 'viola pureza?'"
- ADR-0024 — distinção correcta: parser vs eval() para ecow
- ADR-0029 — definição física de pureza

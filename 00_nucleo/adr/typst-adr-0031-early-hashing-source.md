# ⚖️ ADR-0031: Early hashing em Source — complementa ADR-0016

**Status**: `IMPLEMENTADO`
**Data**: 2026-03-29
**Complementa**: ADR-0016 (remoção de LazyHash de L1)

---

## Contexto

A ADR-0016 removeu `LazyHash<T>` de L1 com a seguinte consequência
documentada:

> "Quando `comemo` precisar de rastrear `Source` (Passo 10), a decisão
> será entre Opção C (campo de hash) ou delegar o hashing ao wrapper
> de comemo em L3."

A Opção C foi identificada na tabela de análise da ADR-0016:

> "C — Hash em construção: adicionar campo `content_hash: u64` a
> `SourceInner`. Hash correcto e puro; custo único na construção."

Mas a ADR-0016 escolheu a Opção A (remover sem substituir) e adiou
a Opção C. Com a definição formal de pureza estabelecida pela ADR-0029,
a Opção C é claramente a correcta e esta ADR formaliza-a.

---

## O problema com LazyHash

`LazyHash<T>` usa mutabilidade interior (`OnceLock` ou equivalente)
para calcular o hash na primeira invocação e cachê-lo. Isso viola
a pureza de L1 não por ter I/O de sistema, mas porque introduz
**estado observável a partir do exterior** que não é determinado
pelos campos públicos do tipo — o hash pode ser `None` ou `Some`
dependendo de quando foi acedido.

Mais precisamente: `LazyHash` quebra a propriedade de que dois valores
com os mesmos campos são indistinguíveis. O estado interno de caching
torna-os distinguíveis por timing.

---

## A solução: Early Hashing (pré-computação na construção)

O hash é calculado **uma vez, no momento da construção**, e armazenado
como campo imutável. Não há estado mutável — o hash é parte do valor
desde o momento da criação.

```rust
// Em 01_core/src/entities/source.rs

struct SourceInner {
    id:           FileId,
    text:         String,
    root:         SyntaxNode,
    content_hash: u64,   // ADR-0031 — pré-computado em Source::new()
}

impl Source {
    pub fn new(id: FileId, text: String) -> Self {
        use std::hash::{Hash, Hasher};
        use rustc_hash::FxHasher;  // ADR-0018 — autorizado em L1

        let mut hasher = FxHasher::default();
        text.hash(&mut hasher);
        let content_hash = hasher.finish();

        let root = crate::rules::parse::parse(&text);

        Self(Arc::new(SourceInner { id, text, root, content_hash }))
    }

    /// Hash do conteúdo de texto — O(1), pré-computado na construção.
    /// Usado por comemo para rastreamento incremental de invalidação.
    pub fn content_hash(&self) -> u64 {
        self.0.content_hash
    }
}

// Hash e Eq baseados no content_hash — sem recomputação
impl std::hash::Hash for Source {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.content_hash.hash(state);
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        // Identidade de FileId + igualdade de hash
        // (hash collision teórico — aceitável para uso com comemo)
        self.0.id == other.0.id && self.0.content_hash == other.0.content_hash
    }
}

impl Eq for Source {}
```

---

## Propriedades desta abordagem

| Propriedade | Estado |
|-------------|--------|
| Imutabilidade de L1 | ✓ — `content_hash` é calculado uma vez, nunca muda |
| Performance de hash | ✓ — O(1) após construção |
| Determinismo | ✓ — mesmo texto → mesmo hash, sempre |
| Zero I/O de sistema | ✓ — `FxHasher` opera em RAM |
| Zero estado global mutável | ✓ — campo de struct, não static |
| Pureza observacional | ✓ — dois `Source` com mesmo id e texto têm o mesmo hash |

Contraste com `LazyHash`: o hash de `Source` é o mesmo independentemente
de quando ou quantas vezes `content_hash()` é invocado.

---

## Custo e trade-offs

**Custo**: uma passagem de hash sobre o texto na construção de `Source`.
Para um ficheiro típico de Typst (1–100 KB), `FxHasher` sobre o texto
leva sub-microsecond. O custo é completamente dominado pelo parse
que acontece na mesma construção.

**Trade-off de colisão**: `PartialEq` baseado em hash pode ter falsos
positivos com probabilidade 2⁻⁶⁴. Para uso com `comemo` (invalidação
de cache), um falso positivo significa que um ficheiro modificado não
invalida a cache — consequência: resultado de compilação stale.
Aceitável para a fase actual; pode ser mitigado com hash de 128 bits
se necessário.

**Alternativa rejeitada**: `PartialEq` field-by-field sobre o texto
(`self.0.text == other.0.text`) é O(n). Inapropriado para o hot path
de `comemo`.

---

## Quando implementar

Esta ADR formaliza a decisão. A implementação acontece quando:

1. `comemo` precisar de rastrear `Source` (contexto do Passo 10 —
   já implementado via `eval_for_test`), ou
2. Testes de conformidade revelarem que `Source` precisa de `Hash`/`Eq`
   para funcionar correctamente com o pipeline de compilação incremental.

Se `Source` já tem `Hash`/`Eq` implementados de outra forma no código
actual, verificar se são consistentes com esta ADR antes de alterar.

---

## Extensão a outros tipos pesados

O padrão de early hashing aplica-se a qualquer tipo em L1 que:
- É clonado frequentemente no hot path
- Precisa de `Hash`/`Eq` para uso com `comemo` ou estruturas de dados
- Tem conteúdo que pode ser hashado deterministicamente na construção

Candidatos: `Module` (hash dos bindings do scope), tipos de Content
quando `comemo` rastrear resultados de layout.

Cada caso requer uma entrada em DEBT.md ou uma ADR separada — não
aplicar o padrão por conveniência sem documentação.

---

## Referências

- ADR-0016 — remoção de LazyHash; Opção C identificada mas adiada
- ADR-0018 — `rustc_hash` autorizado em L1; `FxHasher` disponível
- ADR-0029 — definição física de pureza; estado global mutável proibido
- ADR-0030 — performance de RAM é domínio, não infraestrutura
- DEBT.md — item de `Value::Array Arc<[Value]>` relacionado

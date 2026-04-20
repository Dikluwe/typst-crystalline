# Prompt L0 — `entities/ptr_eq_arc` — PtrEqArc<T>
Hash do Código: 52a69fe6

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/ptr_eq_arc.rs`
**Criado em**: 2026-04-20 (Passo 74)
**ADRs relevantes**: ADR-0029 (Arc em struct de domínio permitido), ADR-0026 (Content como enum fechado)

---

## Contexto e Objetivo

`Arc<Vec<u8>>` com `PartialEq` derivado desreferencia e compara byte a byte — O(N) onde
N é o tamanho dos dados da imagem. Para imagens grandes (JPEGs de 5 MB), esta comparação
é inaceitável em estruturas comparadas frequentemente (DEBT-26).

`PtrEqArc<T>` resolve DEBT-26: envolve `Arc<T>` e implementa `PartialEq`, `Eq`, e `Hash`
por ponteiro (endereço do bloco de controlo do Arc) em vez de por valor — O(1) constante.

**Uso principal**: campo `data: PtrEqArc<Vec<u8>>` em `Content::Image`. Substitui
`Arc<Vec<u8>>` directo para garantir que a comparação de imagens seja por identidade
(mesmo objecto de memória) e não por conteúdo.

**Segurança**: comparação por ponteiro é semanticamente correcta enquanto os dados
forem imutáveis após criação (`Vec<u8>` não é mutado depois de carregado do disco).

---

## Interface

```rust
/// Arc<T> com PartialEq e Hash por ponteiro em vez de por valor.
#[derive(Debug, Clone)]
pub struct PtrEqArc<T>(pub Arc<T>);

impl<T> PartialEq for PtrEqArc<T>;
impl<T> Eq for PtrEqArc<T>;
impl<T> std::hash::Hash for PtrEqArc<T>;
impl<T> std::ops::Deref for PtrEqArc<T> { type Target = Arc<T>; }
```

- `PartialEq`: usa `Arc::ptr_eq` — O(1)
- `Hash`: usa `Arc::as_ptr(&self.0) as usize` — consistente com `PartialEq`
- `Deref`: expõe `&Arc<T>` para acesso ao Arc interno (e deref subsequente para `&T`)
- `Clone`: clona o `Arc` interno — incrementa reference count, O(1)

---

## Critérios de Verificação

```
// Clones do mesmo Arc são iguais
p1 = PtrEqArc(arc1); p2 = PtrEqArc(Arc::clone(&arc1)) → p1 == p2

// Arcs diferentes com mesmo conteúdo são desiguais
p1 = PtrEqArc(Arc::new(vec![1,2,3])); p3 = PtrEqArc(Arc::new(vec![1,2,3])) → p1 != p3

// Deref funciona
PtrEqArc(arc).deref() → &Arc<T>

// Hash consistente com PartialEq (invariante de std::hash::Hash)
p1 == p2 → hash(p1) == hash(p2)
```

---

## Restrições

- **Apenas em L1** — não importar crates externas; usa apenas `std::sync::Arc`.
- Campo `pub` (`PtrEqArc(pub Arc<T>)`) para acesso directo via `.0` no layouter
  (`Arc::clone(&data.0)` em `Content::Image { data, .. }`).
- **Não implementar `PartialOrd` ou `Ord`** — comparação por ponteiro não tem ordenação
  semântica significativa para imagens.

# Prompt L0 — `entities/label`

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/label.rs`
**Criado em**: 2026-04-12 (Passo 59 — introspecção e referências cruzadas)
**Atualizado em**: 2026-04-12 (restauro — expandido com contrato de identidade semântica e relação com ast/markup.rs)
**ADRs relevantes**: ADR-0015 (String nativa em vez de EcoString no domínio puro L1)

---

## Contexto e Objetivo

O motor Cristalino requer um mecanismo para identificar inequivocamente
elementos na AST e no `Content` (como headings, figuras ou equações) para
permitir referências cruzadas e índices. A `Label` é o **tipo de domínio puro
(L1)** que encapsula essa identidade.

Produzida pela sintaxe `<nome>` em Typst — ex: `= Introdução <intro>`.
Usada pelo motor de introspecção (Passo 59) para resolver `@intro` em
referências cruzadas.

**Distinção crítica**: A `entities::label::Label` (este módulo) é uma
**entidade de domínio** — um identificador semântico puro de tipo `String`.
O `ast::markup::Label` (em `ast/markup.rs`) é um **nó da AST** — um wrapper
em torno de `&SyntaxNode` que expõe o texto via `.get()` removendo os `< >`.

**ADR-0015**: `String` nativa em vez de `EcoString` no domínio puro. A label
da entidade nunca contém os delimitadores `<` `>` do markup — apenas o
identificador interno.

---

## Restrições Estruturais

- Camada **L1**: zero I/O, zero estado global.
- Sem dependências externas.
- `Label(pub String)` — tuple struct com campo público.
- Deriva `Debug + Clone + PartialEq + Eq + Hash` — obrigatórios para uso como
  chave em `HashMap<Label, Content>` no motor de introspecção.
- **Não** deriva `Copy` — `String` não é `Copy`.

---

## Instrução

```rust
/// Etiqueta de conteúdo — identificador semântico atribuído a um nó.
///
/// Produzida pela sintaxe `<nome>` em Typst (ex: `= Introdução <intro>`).
/// Usada pelo motor de introspecção para resolver referências cruzadas.
///
/// O campo interno nunca contém os delimitadores `< >` do markup —
/// apenas o identificador puro (ex: "intro", não "<intro>").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);
```

---

## Critérios de Verificação

```
// Igualdade por conteúdo
Label("intro".to_string()) == Label("intro".to_string()) = true
Label("intro".to_string()) != Label("outro".to_string()) = true

// Hash determinístico — duas Labels iguais têm o mesmo hash
// (garantido pelo derive)

// Sem delimitadores < > no conteúdo interno
// (o ast::markup::Label.get() remove os < > ao ler da árvore)

// Usável como chave em HashMap
let mut map: HashMap<Label, u32> = HashMap::new();
map.insert(Label("intro".to_string()), 1);
assert_eq!(map[&Label("intro".to_string())], 1);
```

---

## Relação com outros tipos

| Tipo | Onde | Papel |
|------|------|-------|
| `entities::label::Label` | este módulo | Entidade de domínio — identificador semântico puro |
| `ast::markup::Label<'a>` | `ast/markup.rs` | Nó da AST — wrapper sobre `&SyntaxNode`; `.get()` extrai o texto sem `< >` |
| `Content` (futuro L1) | `01_core` | Pode conter `Option<Label>` como campo de introspecção |

---

## Resultado Esperado

- `01_core/src/entities/label.rs` com `struct Label(pub String)` e testes co-localizados
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/label.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-12 | Criação — Passo 59: entidade de domínio para introspecção e referências cruzadas | `label.rs` |
| 2026-04-12 | Restauro — expandido com ADR-0015, distinção de `ast::markup::Label`, critérios e relações | `label.md` |

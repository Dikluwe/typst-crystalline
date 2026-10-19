# L0 — Passo 1074: Heading Força `italic=false` em vez de Herdar — Achado #9 do P1031

**Gate**: `ADR-0127` — mudança de comportamento por defeito (heading dentro de
`#set text(style: "italic")` é afectado). **Requer confirmação do dono antes de
codificar.**

**Base**: P1031, Achado #9. Medição já pronta em `compiler/layout/heading.md`
("ACHADO ESCALADO — medido 2026-08-13"): sob `#set text(style: "italic")`,
vanilla usa `LibertinusSerif-BoldItalic` (herda o itálico); cristalino emite o
mesmo operador `/F2 15.4 Tf` com e sem o `set text` envolvente (o itálico
envolvente não tem efeito). Código real já visto nesta conversa (upload
anterior, `heading.rs`) — não é preciso pedir de novo.

---

## 1. O código exacto a mudar (já confirmado, não suposição)

`01_core/src/compiler/layout/heading.rs`, dentro de `layout()`:

```rust
let prev = layouter.style.clone();
layouter.style = TextStyle {
    bold: true,
    italic: false,   // <- esta linha força false, independente do que `prev` tinha
    size: heading_size,
    ..TextStyle::default()
};
```

**Cuidado com a armadilha do `..TextStyle::default()`**: simplesmente remover a
linha `italic: false,` **não basta** — sem um campo `italic:` explícito, o
`..TextStyle::default()` no fim da struct preenche `italic` com o valor default
(presumivelmente `false` também), não com o valor herdado de `prev`. Isso
produziria o mesmo bug por um caminho diferente (default em vez de herança
real).

## 2. Mecanismo correcto

```rust
let prev = layouter.style.clone();
layouter.style = TextStyle {
    bold: true,
    italic: prev.italic,   // herda explicitamente, não usa default nem force
    size: heading_size,
    ..TextStyle::default()
};
```

`bold: true` permanece forçado — isso é citação literal confirmada do vanilla
(`out.set(TextElem::weight, FontWeight::BOLD)`, já documentado em `heading.md`
como comportamento correcto, não o bug). Só `italic` muda de forçado para
herdado.

## 3. Verificar outros campos da mesma struct, não só `italic`

`heading.md` já identificou que a diferença `bold`=citação literal vs
`italic`=herança-quebrada é específica desses dois campos — mas a struct
`TextStyle` provavelmente tem mais campos (`size` já é deliberadamente
sobrescrito, correcto). Conferir se há algum outro campo além de `italic` que
devia herdar de `prev` e está a cair no `..TextStyle::default()` silenciosamente
— não assumir que só `italic` tem este problema sem checar a lista completa de
campos de `TextStyle`.

## 4. Medição

Reaproveitar o repro já documentado em `heading.md`
(`#set text(style: "italic")` + `= Cabecalho`), confirmar depois da correcção
que:
- O operador `Tf` do heading muda entre "com" e "sem" o `#set text(style:
  "italic")` envolvente (ao contrário de antes, onde eram idênticos).
- A fonte seleccionada corresponde à variante itálica disponível (mesmo cuidado
  de confound já registado em `heading.md` — não comparar nomes de fonte entre
  binários diferentes, comparar o cristalino consigo mesmo).

## 5. Critérios de verificação

1. Heading dentro de `#set text(style: "italic")` — operador `Tf` diferente do
   caso sem o `set text` (confirma que o itálico agora tem efeito).
2. Heading fora de qualquer `#set text(style:)` — sem alteração (guarda de
   não-regressão, itálico continua `false` por herança do default normal).
3. `bold: true` continua a aplicar-se sempre, independente de `prev.bold`
   (não regressão do comportamento já correcto).
4. Conferência do §3 — outros campos de `TextStyle` revistos, não só `italic`.
5. `crystalline-lint .` — 0 erros.
6. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- Correcção aplicada com herança real (`prev.italic`), não remoção simples da
  linha.
- §3 respondido — lista de campos de `TextStyle` conferida, não só `italic`.
- 6 critérios de verificação confirmados com medição real, reaproveitando o
  repro do `heading.md`.

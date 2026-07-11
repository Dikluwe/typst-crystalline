# Paridade Produção — P709 — Módulo `std` (acesso à stdlib não-sombreada)

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-709.md`
**Hash do commit (implementação):** a preencher no commit seguinte.
**HEAD base:** `045aa8f09` (fim de P708, detached HEAD).
**Estado:** ÂMBITO DE P709 FECHADO — `std` implementado e verificado em **dois** pontos de construção de scope (documento principal e ficheiros importados, achado a meio do passo). `cetz` avança mais fundo de novo, novo bloqueio isolado: `Value::Length` não tem nenhum campo/método (`.pt()`, `.mm()`, `.to-absolute()`, `.abs`, etc.).

---

## 1. Sonda — mecanismo vanilla confirmado, não assumido

`foundations/scope.rs:24,51-56` (vanilla): `Scopes.base: Option<&Library>`
é uma camada de fallback à parte do stack `top`/`scopes` do utilizador.
`Library::std` (`typst-library/src/lib.rs:180,231`) é
`Binding::detached(global.clone())` — clone independente da stdlib
inteira, capturado uma única vez na construção da `Library`.

**Medido, não assumido**: `#let std = "oops"; #std` → `"oops"` no
vanilla — `std` **é** sombreável como qualquer nome (o
`cannot_mutate_constant` de `get_mut` não se aplica à criação de um novo
binding via `#let`).

```
let length = 5; std.length         → length   (tipo builtin, não sombreado)
let calc = "x"; std.calc.round(3.7) → 4       (submódulo via std funciona)
```

### Arquitetura cristalina — mais simples

O cristalino não tem uma camada `base` separada do stack de scopes;
`make_stdlib()` já vive num frame que os `#let` do documento só sombreiam
por cima, nunca mutam. Bastou clonar o `Scope` de `make_stdlib()` **antes**
de o espalhar, e registá-lo como `Value::Module` sob `"std"` — o field
access `.calc`/`.length` sobre `Value::Module` já existia (P679), zero
código de dispatch novo.

---

## 2. Implementação

### Dois pontos de construção de scope (achado a meio do passo)

A implementação inicial só cobria `eval_with_full_error`
(`eval/mod.rs::run_pass`, o documento principal). Ao correr a reprodução
completa de `cetz`, o erro `"unknown variable: std"` **persistiu**
inalterado — isolei uma **segunda** construção de scope independente,
`eval_imported_file` (`rules/eval/modules.rs:43-103`), usada para **cada
ficheiro importado** (`#import "pkg"`), com o seu próprio
`make_stdlib()`/`Scopes::new` — `cetz` é avaliado inteiramente por este
caminho, não pelo do documento principal. Corrigido com o mesmo
mecanismo (clone do stdlib antes de espalhar), no mesmo ficheiro.

```rust
// eval/mod.rs (documento principal) e rules/eval/modules.rs (ficheiros
// importados) — mesmo padrão nos dois:
let stdlib = make_stdlib(&inputs);
scopes.define("std", Value::Module(Module::new("std", stdlib.clone())));
for (name, binding) in stdlib.iter() {
    scopes.define(name, binding.value().clone());
}
```

### Âmbito medido — só o scope de `make_stdlib`

`grep` exaustivo a `std.` em `cetz`: `color`, `curve`, `gradient`,
`length`, `measure`, `stroke`, `tiling` — todos vivem em `make_stdlib()`.
**Scope-out explícito**: `predefined_color_bindings()` (`red`/`blue`),
`text`, elementos do `ElementRegistry` — não incluídos em `std`, sem
consumidor medido.

### Testes (5 novos, `eval/tests.rs`)

`std.length` após sombreamento, `std.calc.round(...)` (submódulo), `std`
em si sombreável, `std` sem sombreamento idêntico ao builtin, e **`std`
disponível num ficheiro importado** (o teste que teria apanhado a segunda
construção de scope em falta, se tivesse sido escrito antes da
reprodução completa).

---

## 3. Validação — âmbito de P709 confirmado

Reexecutados os casos da sonda (release build, ambos os pontos de
construção de scope): `std.length` → `length`; `std.calc.round(3.7)` →
`4`; `#let std = "oops"; #std` → `"oops"` — todos **idênticos ao
vanilla**.

- `cargo test --workspace` → **3795 passed**, 0 failed (3790 de P708 + 5
  novos de P709); `typst-infra` inalterado (626/5).
- `crystalline-lint .` → 0 violations (sem drift novo — `modules.rs` já
  partilhava a L0 `rules/eval.md`, realinhada em P708).

---

## 4. Repetição da reprodução de P700-708 — mais progresso, novo bloqueio

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

Tempo de compilação: **~52.8s** — na mesma ordem de grandeza de P708
(~51.8s), sem regressão.

**Novo bloqueio**: `error: field access não suportado em length`.
Isolado: `canvas.typ:36` — `let length = length.to-absolute()`, chamado
logo a seguir ao ponto onde `std.length` foi usado para detectar
sombreamento (linhas 32/42 do mesmo ficheiro). Confirmado standalone:
```
#set text(size: 12pt)
#context [
  #(6pt).to-absolute()
  #(6pt + 10em).to-absolute()
]
```
→ vanilla: `6pt 126pt`, exit 0. Cristalino: mesmo erro genérico —
`Value::Length` **não tem nenhum** campo ou método implementado
(`.pt()`, `.mm()`, `.cm()`, `.inches()`, `.to-absolute()`, `.abs`) —
confirmado com `foundations/layout/length.rs:96-161` do vanilla
(`#[scope] impl Length`).

### Próximo passo sugerido (P710, não iniciado)

1. Confirmar contra o vanilla a semântica exacta de cada método —
   `.pt()`/`.mm()`/`.cm()`/`.inches()` falham se a componente `em` não for
   zero (`ensure_that_em_is_zero`); `.abs` (campo) ignora a componente
   `em`; `.to-absolute()` precisa de contexto de estilo (`Tracked<Context>`,
   tamanho de fonte) para resolver `em` para absoluto.
2. Medir o que `cetz` realmente precisa (`.to-absolute()` confirmado;
   `.pt()`/`.mm()`/`.cm()`/`.inches()`/`.abs` a confirmar por grep) antes
   de decidir o âmbito.
3. Reexecutar a reprodução deste §4 como critério de fecho.

---

## 5. Estado da cadeia P678–709

`std` fechado e verificado nos dois pontos reais de construção de scope.
`cetz` continua a avançar de forma consistente (tempos de compilação na
mesma ordem de grandeza de P708) — o bloqueio mudou de "variável
desconhecida" para "campo/método ausente num tipo primitivo" (`Length`),
uma categoria de gap mais próxima da que resolveu P703 (`rgb`)/P705
(`luma`) do que da que resolveu P708 (mecanismo de binding). Pausada
aqui, com P710 sugerido.

## 6. Critério de fecho do passo

- [x] Sonda completa, natureza de `std` confirmada (mecanismo vanilla +
      arquitetura cristalina mais simples, sem precisar de replicar o
      fallback dedicado).
- [x] Implementado e testado, incluindo sombreamento, sub-módulos, e —
      achado a meio do passo — o segundo ponto de construção de scope
      (`eval_imported_file`, o caminho real usado por `cetz`).
- [x] Sem regressão em `cargo test --workspace` (3795 passed).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — **progresso mantido**, próximo bloqueio
      identificado (`Value::Length` sem campos/métodos), não sucesso
      completo.
- [x] Relatório com resultado exacto (este ficheiro).

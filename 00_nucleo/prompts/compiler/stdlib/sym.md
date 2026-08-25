# Prompt L0 — `sym` — módulo de símbolos Unicode
Hash do Código: 9bf5af9a

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/sym.rs`, `01_core/src/compiler/eval/mod.rs`
**Origem**: Passo 471 — módulo `sym` com tabela de ~70 símbolos prioritários; registado no scope como `Value::Dict`. **P731**: passa a `Value::Module` (paridade vanilla — medido: `type(sym)` → `module`). **P766**: expansão por uso real do corpus (tilde, integral, chevron, suit, tack, space, emptyset, bracket, amp, e variantes de plus/gt/diamond).
**ADRs**: ADR-0017, ADR-0107 (paridade linguagem vs mecânica), ADR-0029 (pureza L1).

---

## 1. Contexto

O vanilla expõe `sym` como módulo acessível via `sym.arrow`, `sym.alpha`, etc. O cristalino implementa um subset prioritário como `Value::Module` no scope global.

Divergência declarada: o vanilla suporta modificadores encadeados (`sym.arrow.r.double`). O cristalino suporta `sym.arrow.r.filled` via `Symbol::with_variants` e `Symbol::modified`; símbolos sem variantes rejeitam modifiers.

## 2. Tabela

Duas categorias:

1. **Símbolos simples** (`SYM_SIMPLE`): nome → string de exactamente um
   grapheme cluster. Inclui letras gregas, operadores básicos, e variantes
   pré-definidas como `eq.not`.
2. **Grupos com variantes** (`SYM_GROUPS`): nome de grupo →
   `Symbol::with_variants`. Base e cada variante são strings de exactamente um
   grapheme cluster. Exemplos: `arrow`, `tilde`, `integral`, `chevron`, `suit`,
   `tack`, `space`, `emptyset`, `bracket`, `amp`.

**P1161:** a troca mecânica `char` → `&str` nas tabelas de `sym` acompanha o
novo contrato de `Symbol` e não muda os valores de um codepoint existentes.
Não adicionar entradas multi-codepoint a `sym` sem medição própria.

**P1140.3-A — correção de tabela:** `sqrt` não pertence ao módulo público
`sym` do vanilla ratificado. A entrada cristalina `("sqrt", '√')` é removida;
o path público correspondente é a função `math.sqrt`. O caractere continua
podendo existir como dado interno/render do radical, mas não como
`sym.sqrt`. Esta é correção de tabela de paridade, não remoção de
`MathRootElem` nem mudança de layout.

## 3. Funções

```rust
/// Procura um símbolo pelo nome (incluindo nomes compostos pré-definidos e
/// variants encadeados como "arrow.r.filled" ou "tilde.equiv").
pub fn sym_lookup(name: &str) -> Option<Symbol>;

/// Constrói o Value::Module com entradas sem ponto no scope (acessíveis via FieldAccess, P731).
pub fn build_sym_module() -> Value;
```

## 4. Registo no scope

```rust
// 01_core/src/compiler/eval/mod.rs
scope.define("sym", build_sym_module());
```

Apenas entradas sem `.` no nome ficam no scope do módulo. As compostas (`"eq.not"`, `"arrow.r.filled"`) são resolvidas via `sym_lookup`.

## 5. Eval markup

`Value::Symbol(s)` em contexto de markup →
`Content::Text(s.value.clone())`, preservando o grapheme integral.

## 6. Scope-out

- Tabela completa do vanilla (~centenas de símbolos emoji) — apenas grupos com uso identificado no corpus são implementados em P766; restantes ficam scope-out consciente.
- **P820 — depreciação ao nível de variante**: o codex `sym.txt` (vanilla 0.15.0) tem 14 tags `@deprecated`; 13 são ao nível de **variante** (`gt.tri`, `gt.tri.eq`, `gt.tri.eq.not`, `gt.tri.not`, `lt.tri` + 3, `tack.*.double` ×5) e ficam em scope-out — requerem mensagem de depreciação por variante em `SymbolVariant`, mecanismo separado. Apenas `join` é depreciação de símbolo de topo (ver §7).

## 7. P820 — `join`/`bowtie` + mecanismo de depreciação (achado #7 de P810)

Medição na fonte vanilla 0.15.0 (binário + codex `sym.txt`):

- `$join$` / `$join.r$` → vanilla **warning** `` `join` is deprecated, use `bowtie.big` instead `` (span na raiz `join`, exit 0); cristalino dava `error: unknown variable: join` (exit 1).
- `#sym.join` → mesmo warning, span no campo (@1:5); cristalino dava `module 'sym' does not contain field "join"`.
- `$bowtie$` → ⋈ (sem variante bare no codex — cai na primeira, `stroked`); `$bowtie.big$` → ⨝ (variante `stroked.big`, algoritmo de menor número de modifiers extra); `$bowtie.stroked$` → ⋈; `$bowtie.filled$` → ⧓; `#sym.bowtie.big` → ⨝. Tudo exit 0, sem warning.
- Mecanismo geral vanilla: `Binding::deprecated(Deprecation)` (`foundations/scope.rs:257-373`); a fonte de dados dos símbolos é a tag `@deprecated` do codex `sym.txt`.

Regras:

- Novos grupos em `SYM_GROUPS`: `join` (base ⨝; variantes `r` ⟖, `l` ⟕, `l.r` ⟗) e `bowtie` (base ⋈; variantes do codex `stroked`/`stroked.big`/`stroked.big.l`/`.r`/`.l.r`/`filled`/`filled.l`/`filled.r`).
- Nova tabela `SYM_DEPRECATED: &[(&str, &str)]` (nome → mensagem verbatim) + `pub fn sym_deprecation(name) -> Option<&'static str>`. Cobre apenas `join` (única depreciação de topo do codex e caso medido).
- Emissão do warning (mensagem verbatim, **sem hint**): em modo math na resolução do `MathIdent` (bare e raiz de field access — span no ident, `eval/math.rs`); em `#sym.<nome>` no field access genérico (span no campo, `eval/bindings.rs`). O símbolo **resolve** — warning, nunca erro.

Critérios de verificação (binário):

- `$join$`, `$join.r$` → warning `` `join` is deprecated, use `bowtie.big` instead ``, exit 0.
- `#sym.join` → mesmo warning (span @1:5), exit 0.
- `$bowtie$`, `$bowtie.big$`, `#sym.bowtie.big` → exit 0, sem warning.
- `$foo.bar$` → `unknown variable: foo` + 2 hints (nunca `variável desconhecida` em português).

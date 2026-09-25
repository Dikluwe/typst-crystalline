# Prompt L0 — `sym` — módulo de símbolos Unicode
Hash do Código: c48fd6c8

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/sym.rs`
**Origem**: Passo 471 — módulo `sym` com tabela de ~70 símbolos prioritários; registado no scope como `Value::Dict`. **P731**: passa a `Value::Module` (paridade vanilla — medido: `type(sym)` → `module`). **P766**: expansão por uso real do corpus. **P1283**: catálogo integral do vanilla ratificado a partir de `codex = 0.3.0`.
**ADRs**: ADR-0017, ADR-0107 (paridade linguagem vs mecânica), ADR-0029 (pureza L1), ADR-0129 (ownership 1:1).

---

## 1. Contexto

O vanilla expõe `sym` como módulo acessível via `sym.arrow`, `sym.alpha`, etc. P1283
fecha o subset histórico e passa a derivar o catálogo público completo da mesma fonte
declarativa pinada pelo vanilla ratificado.

**Medição P1283 antes da decisão (2026-08-30, vanilla `a51e02804`):** o workspace
vanilla usa `codex = 0.3.0` (`lab/typst-original/Cargo.toml:50`; checksum crates.io
`0732ab1a27b4ea05e6f9f60a5122c9924dd5123defde0d8e907f58cf643d40e6`). A fonte
`codex-0.3.0/src/modules/sym.txt:1-1340`, SHA-256
`6ee467d9939acb5c7d0a3eba30c9f640d157529cbf57df752367deb343e0fc16`, declara
297 símbolos diretos, 37 símbolos nos submódulos `gender`/`control`, 2 submódulos e
1.206 registos de valor/variante. Há 46 parents sem variante bare. Esta contagem é
inventário de paths/registos, não percentagem de paridade.

## 2. Tabela

P1283 substitui as tabelas manuais incompletas por um adaptador puro sobre
`codex::SYM`, pinado exactamente em `0.3.0`. Duas categorias semânticas permanecem:

1. **Símbolos simples** (`SYM_SIMPLE`): nome → string de exactamente um
   grapheme cluster. Inclui letras gregas, operadores básicos, e variantes
   pré-definidas como `eq.not`.
2. **Grupos com variantes**: nome de grupo → `Symbol::with_variants`. Base e cada
   variante são strings Unicode integrais, incluindo VS15/VS16 e ZWJ. Modifiers são
   conjuntos; ordem de aplicação não altera a seleção.

`codex::Def::Module` é convertido recursivamente, preservando `sym.gender.*` e
`sym.control.*`. Para parents sem bare, o valor público default é o resultado do
`ModifierSet` vazio no algoritmo do codex: maior coincidência, menos modifiers extra e
primeira variante em empate. Não fabricar uma variante vazia que não existe na fonte.

Aliases são nomes distintos e permanecem ambos no scope, com identidade completa de
base e variants: `dollar`/`pataca`, `yen`/`yuan`, `emptyset`/`nothing` e
`gradient`/`nabla`. Igualdade apenas do valor base não constitui alias; por exemplo,
`infinity` e `oo` continuam estruturalmente distintos.

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

## 6. P1283 — fechamento e divergência intencional

A tabela e todas as variantes de `codex 0.3.0` deixam de ser scope-out. Existência,
kind, valor default, `repr`, modifiers e valores de variantes são obrigação fechada.

Depreciação ao nível de variante permanece divergência intencional individualizada: a
entidade `SymbolVariant` ainda não transporta mensagem. Depreciação de binding de topo
continua observável por `sym_deprecation`. Esta divergência não autoriza omitir o símbolo
ou variante e não conta como paridade de diagnóstico.

`sym.registered` é extensão cristalina histórica, ausente do codex pinado. P1283 a
preserva sem crédito de paridade e sem remoção automática, conforme a proibição do passo.

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

## 8. P1495 — catálogo pinado e compatibilidade (materialização P1496)

### Medição anterior à decisão

O vanilla `9f2b6e8715237cb086899a42873660fe744622e8` usa `codex` git
`2e43ab90079cf9f9fa670d421c8c38233143b376` (`lab/typst-original/Cargo.toml:51`),
não `0.3.0` histórico P1283. SHA-256 de `sym.txt`: antigo
`6ee467d9939acb5c7d0a3eba30c9f640d157529cbf57df752367deb343e0fc16`, novo
`60c79540af8ebe590c80d672b55a41f337bc8d61ebd42d9be8879fce9b18f8bc`.
`sym.txt:765-793,1377-1400` declara `chess`, `keyboard`, `ordinal`:
3 módulos, 21 filhos, 19 variantes. Sondas bilaterais de `repr`
confirmam 43 paths (3 pais + 40 descendentes) por namespace vanilla; faltam os pais no
cristalino P1494. Doze famílias (`eq, equiv, gt, lat, lt, not, prec,
smt, succ, tack, tilde, trademark`) têm diferenças verificadas;
`not.inv` vale ⌙. Cinco `tack.*.double` mudaram (`CHANGELOG.md:33-40`).
O diagnóstico P1495 guarda 198 sondas e proveniência; metadados
sozinhos não provam comportamento da linguagem.

`CHANGELOG.md:42-47` remove `join{,.r,.l,.l.r}` e
`{gt,lt}.tri{,.eq,.eq.not,.not}` do vanilla novo; o cristalino P1494
ainda os aceita em `sym`, `math` e bare. O catálogo novo deprecia
`spacebar` para `keyboard.space` (`sym.txt:1374-1375`): vanilla avisa
nos três acessos, cristalino P1494 silencia.

### Decisão proprietária

P1496 pina exatamente a revisão git acima e materializa módulos, filhos,
modifiers, valores Unicode e `repr` novos, sem branch móvel. Os cinco
`tack.*.double` são correções de entradas de tabela para o novo alvo
de linguagem: embora o crate os chame breaking, não mudam path,
assinatura, carrier nem fase; seguem fluxo contínuo ADR-0127 com
L0 antes de RED→GREEN. Um novo contrato público ou valor diferente
no pin de linguagem refutaria esta classificação.

**Compatibilidade:** conservar os doze paths removidos upstream como
extensões cristalinas sem crédito de paridade. Valores estáveis:
`join` ⨝/`.r` ⟖/`.l` ⟕/`.l.r` ⟗; `gt.tri` ⊳/`.eq` ⊵/
`.eq.not` ⋭/`.not` ⋫; `lt.tri` ⊲/`.eq` ⊴/`.eq.not` ⋬/
`.not` ⋪. `build_sym_module`, `sym_lookup`, espelho `math` e bare
devem concordar; trocar apenas o crate retiraria bindings dos
módulos, deixando fallback incoerente. Mesclar só os aliases
legados, sem substituir grupos `gt`/`lt` inteiros nem perder
variantes novas. `sym.registered` continua extensão separada.

`spacebar` resolve e avisa nos três acessos com a mensagem vanilla
`` `spacebar` is deprecated, use `keyboard.space` instead ``.
`join` conserva o aviso histórico em `sym` e bare; `math.join`
permanece silencioso, como no cristalino P1494. Uniformizar o
aviso desta extensão seria novo comportamento por defeito e exige
gate ADR-0127; não é autorizado por P1495.
Warnings de variantes antigas continuam dívida P1283 de
`SymbolVariant`; não ressuscitar avisos `tri` removidos do vanilla
nem avisar os cinco `tack` des-depreciados. Emissão pertence a
`compiler/eval/math.md` e `compiler/eval/bindings/field_access.md`.

P1495 entrega L0 e sondas RED; P1496 completa lockfile e código.
Testar 43 paths por namespace, 12 famílias com valor/modifiers/`repr`,
cinco `tack`, `not.inv`, `spacebar`, aliases legados nos três acessos
e controles `registered`, `sqrt`, `emoji`. Remover qualquer alias
público no futuro exige novo L0 e confirmação ADR-0127.

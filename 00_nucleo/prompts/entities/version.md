# Prompt L0 — `Version` — sequência de componentes inteiros
Hash do Código: c0d3882e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/version.rs` (owner único; a integração
histórica com Value pertence ao Prompt proprietário de `entities/value.rs`).
**Origem**: Passo 401 — modelagem de `Value::Version`. **Corrigido em P684** — a
leitura original (semver 2.0.0 com `pre`/`build`) foi uma confusão com o SemVer
geral; o `version` do Typst é outro conceito (ver nota de origem no diagnóstico
de P684).
**ADRs**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029
(pureza L1), ADR-0108 (medir antes de decidir).

---

## 1. Contexto

O Typst `version` (documentação oficial + sonda directa em `0.15.0 (969087ec)`)
é **uma sequência arbitrária de componentes inteiros**; só os três primeiros têm
nome (`major`, `minor`, `patch`). Não há pré-lançamento nem metadados de build
como texto. Confirmado por sonda:

- `version()`, `version(1)`, `version(1, 2)`, `version(1, 2, 3, 4, 5)` → todos aceites.
- `version(1, 2, 3, pre: "alpha")` → `unexpected argument: pre`.
- `version(1, 2, 3, "alpha.1")` → `expected integer or array, found string`.
- `version(1, 2, 3) == version(1, 2, 3, 0)` → `true` (componente em falta = `0`).
- `repr(version(0, 11, 0))` → `"version(0, 11, 0)"` (repr preserva zeros à direita).

## 2. Tipo L1

```rust
// 01_core/src/entities/version.rs
#[derive(Debug, Clone)]
pub struct Version {
    pub components: Vec<u64>, // guardados como foram dados (zeros à direita preservados)
}
```

- `Clone` (não `Copy`) devido ao `Vec<u64>`. Zero I/O.
- `PartialEq`/`Eq`/`Hash` e `Ord` implementados **à mão** com semântica
  **zero-pad** (componente em falta = `0`), para reproduzir o vanilla:
  - `eq`: compara `component(i)` (0 se ausente) para `i` em `0..max(len)`.
  - `hash`: ignora zeros à direita (consistente com `eq`).
  - `cmp`: lexicográfico zero-pad sobre `0..max(len)`.
- `repr` usa `components` como dados (preserva zeros à direita).

## 3. Construtores e acesso

```rust
impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self;
    pub fn from_components(components: Vec<u64>) -> Self;
    pub fn component(&self, i: usize) -> u64; // 0 se ausente
    pub fn major(&self) -> u64;  // component(0)
    pub fn minor(&self) -> u64;  // component(1)
    pub fn patch(&self) -> u64;  // component(2)
    pub fn to_string(&self) -> String;        // "c0.c1.c2…" (vazio para version())
    pub fn from_str(s: &str) -> Option<Self>; // só inteiros separados por '.'
}
impl Default for Version { fn default() -> Self } // version() vazia
```

`from_str` rejeita string vazia, não-inteiros e negativos; **não** aceita
`pre`/`build` textuais (`"1.2.3-alpha.1"`, `"1.2.3+build.2"` → `None`).

## 4. Comparação (zero-pad, paridade vanilla)

- Igualdade directa sobre todos os componentes com zero-pad (sem qualquer
  tratamento especial de `pre`/`build`, que não existem).
- Ordem lexicográfica zero-pad: mais curto (prefixo) < mais longo;
  componente a componente; trailing zero não altera a ordem.

## 5. Variant `Value::Version`

```rust
// 01_core/src/entities/value.rs
Version(Arc<crate::entities::version::Version>),
```

- `Arc` para cheap clone (padrão P395).
- `type_name()` → `"version"`.
- `cast_version()` extrai `Arc<Version>` ou converte `Str` (parse int-only).
- `From<Version> for Value`.

## 6. Cast

- `Value::Version(v) -> Arc<Version>`: identidade.
- `Value::Str(s) -> Arc<Version>`: parse de componentes inteiros (`Version::from_str`).
- Outros tipos → `None`.

## 7. Repr

`version({componentes separados por ", "})`, tal como dados (ex.:
`version(1, 2, 3, 4, 5)`, `version(0, 11, 0)`, `version()` se vazio).

## 8. Field access

Só os três primeiros componentes têm nome: `.major`, `.minor`, `.patch` →
`component(0|1|2)` (0 se ausente). `.pre`/`.build` → `campo desconhecido em version`.

## 8a. Método `.at(index)` — P796

Medido no vanilla (`lab/typst-original/crates/typst-library/src/foundations/version.rs:109-134`,
sonda directa em 0.15.0: `#sys.version.at(0)` com `sys.version == version(0, 15, 0)` → `0`,
paridade com `major`).

```rust
impl Version {
    /// Índice negativo conta a partir do fim da lista de componentes
    /// **explícita** (`self.components.len()`, não a sequência infinita
    /// zero-pad usada por `component`/`Ord`/`Eq`). Índice positivo além do
    /// comprimento explícito devolve `0` (zero-pad, igual a `component`).
    /// Índice negativo fora de limites é erro.
    pub fn at(&self, index: i64) -> Result<i64, String>;
}
```

- Resolução: `index < 0` → `len_explícito.checked_add(index)`; se `< 0` ou overflow,
  erro. `index >= 0` (já resolvido ou originalmente não-negativo) → `component(i)`
  (0 se além do comprimento — reaproveita a semântica já existente).
- **Mensagem de erro replica o vanilla ao carácter** (ADR-0108, excepção —
  mecânica é o observável em mensagens de erro):
  `"component index out of bounds (index: {index}, len: {len})"`, onde `len` é
  `self.components.len()` (comprimento explícito, não zero-pad).
- Exposto como método de instância (`sys.version.at(0)`), não como field access —
  dispatch dedicado em `01_core/src/compiler/eval/bindings.rs`
  (`eval_version_method_value`, mesmo padrão de `eval_counter_method_value`/
  `eval_color_method`), interceptado em `closures.rs` antes do fallback
  genérico de field access. `has_readonly_method` já antecipava isto
  (`Value::Version(_), "at" => true`, P716).
- Sem argumentos nomeados (vanilla não tem `default:` em `version.at`, ao
  contrário de `array.at`/`dict.at`). Argumento posicional não-inteiro ou
  aridade errada → erro genérico existente (`expected integer, found …` /
  `version.at() requires exactly one positional argument`), sem paridade
  literal exigida (mecânica de argparsing, não observável de linguagem).

## 8b. Exibição em markup (`#sys.version`) — P796

**Achado (P786)**: `#sys.version` interpolado em markup mostrava
`version(0, 15, 0)` (o `repr()`) em vez de `0.15.0` (o `Display`/`to_string()`).
Causa: `value_to_display_content` (`01_core/src/compiler/eval/mod.rs`) não tinha
armo dedicado para `Value::Version` e caía no catch-all `other` que usa
`repr_value`.

Medido no vanilla (`foundations/value.rs::Value::display`, linha ~200):
`Self::Version(v) => TextElem::packed(eco_format!("{v}"))` — usa o `Display`
do tipo (equivalente a `Version::to_string()` no cristalino), **não** o repr.
`repr(sys.version)` continua a mostrar `version(0, 15, 0)` (repr não muda).

Correção: `value_to_display_content` ganha um armo dedicado, **antes** do
catch-all:

```rust
Value::Version(v) => {
    let text = v.to_string();
    if text.is_empty() { None } else { Some(Content::Text(text.into())) }
}
```

(`version()` vazia → `to_string()` = `""` → `None`, mesmo tratamento que os
outros armos de texto vazio nesta função.)

## 9. Constante de paridade partilhada — P796

`PARITY_VERSION: (u64, u64, u64) = (0, 15, 1)` — a versão **de paridade** com
a linguagem Typst (não a versão do crate/binário cristalino, ver decisão em
`compiler/stdlib/sys.md` §"Decisão: versão de paridade"). Definida **aqui**
(`01_core/src/entities/version.rs`, `pub const PARITY_VERSION`) como fonte
única, consumida por:

- `01_core/src/compiler/stdlib/sys.rs` (`sys.version`) — já usava este valor
  localmente antes de P796; passa a importar a constante em vez de duplicá-la.
- `02_shell/src/cli.rs` (`--version` do CLI) — P796 fecha a divergência
  registada no achado de P786 (`--version` mostrava `0.1.0`, a versão do
  crate Cargo, inconsistente com `sys.version` == `0.15.1`). Ver decisão e
  mecanismo completo em `shell/cli.md` §"Decisão — número de versão do CLI".

### 9a. Política de atualização e correção P1137

**Medição direta reproduzida em P1137** (2026-08-23; HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5`, working tree não commitada; binários e
comandos registados em `diagnosticos/paridade-matriz-p1137-baseline.md`):

```
lab/typst-original/target/release/typst --version → typst 0.15.1 (...)
target/release/typst --version                      → typst 0.15.0 (...)
```

`02_shell/src/cli.rs:84-96` consome a mesma constante `PARITY_VERSION` usada por
`compiler/stdlib/sys.rs`; a medição do CLI é portanto uma leitura independente da mesma
fonte de valor, não prova de proveniência do hash impresso. A medição anterior direta de
`#repr(sys.version)` continua a confirmar `version(0, 15, 0)` vs
`version(0, 15, 1)`.

O baseline de paridade usado nesta medição histórica era upstream/main `a51e02804`
(ratificado 2026-08-11), cujo `sys.version` reportava `(0, 15, 1)`. A constante daqui
reportava `(0, 15, 0)` — e já estava atrás antes do sync, porque o baseline anterior era
a **tag 0.15.1**.

`sys.version` é **superfície de linguagem**: qualquer documento a pode imprimir, comparar
(`sys.version >= version(0, 15, 1)`) ou usar para ramificar. Logo isto é paridade no sentido
de ADR-0107, não mecânica — e é uma divergência real, não uma escolha registada.

**Decisão P1137:** `PARITY_VERSION` representa exatamente a versão de linguagem declarada
pelo vanilla ratificado em `upstream/main a51e02804`: `(0, 15, 1)`. O cristalino segue o
Typst ratificado mais novo deste projeto; `(0, 15, 0)` era esquecimento, não modo de
compatibilidade. A constante **não acompanha automaticamente** `upstream/main`, uma tag ou
a versão mais recente da rede: só muda num passo explícito de re-sync que substitua o hash
pinado e volte a medir `sys.version`. Isso preserva a reprodutibilidade do oráculo.

**Ressincronização P1355 (2026-09-12):** o alvo então corrente passa a upstream/main
`586e1bd43`, árvore `2847920eeaa98d26a41dc6fb0f9c02334b9d593b`. A sonda direta do
novo binário ratificado devolve `version(0, 15, 1)`, portanto `PARITY_VERSION` permanece
inalterada. Esta troca atualiza a autoridade do oráculo; não reescreve as medições
históricas de `a51e02804` nem concede crédito automático às novas superfícies.

Impacto público deliberado da correção P1137:

- `sys.version` e `repr(sys.version)` passam de `0.15.0` para `0.15.1`;
- comparações como `sys.version >= version(0, 15, 1)` passam de `false` para `true`;
- `--version`, por consumir a mesma constante, passa a anunciar `0.15.1` após rebuild;
- nenhum campo, assinatura ou tipo público muda.

Aceitação no nível da linguagem: teste RED fixa o valor antigo como divergente contra o
oráculo e GREEN exige `sys.version == version(0, 15, 1)`, sua `repr` e sua exibição em
markup. O CLI verifica adicionalmente a propagação da fonte única. O que refutaria esta
decisão seria o binário ratificado `a51e02804` devolver valor diferente em sonda direta de
`sys.version`; a string nominal de outro binário/tag não a refuta.

**Ressincronização P1489 (2026-09-24):** a medição direta do binário vanilla
construído de upstream/main `9f2b6e8715237cb086899a42873660fe744622e8`
(árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, SHA-256 do binário
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`)
devolve `"version(0, 15, 1)"` para `typst eval 'sys.version' --format json`.
O cristalino HEAD `9e3a267f5` também devolve `"version(0, 15, 1)"` no mesmo
comando. Logo `PARITY_VERSION` permanece `(0, 15, 1)`; não há mudança
de contrato nem necessidade de gate ADR-0127. A medição anterior de P1355
continua histórica, não prova automaticamente outros comportamentos do novo pin.

## 10. Scope-out

- Nenhum item pendente conhecido após P796 (`.at()` e exibição em markup
  fechados; ver §8a/§8b).


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — identidade pública do tipo `version`

O nome global `version` é o valor `Value::Type(Type::Version)`, chamável
pelo dispatcher estático e delegado ao construtor L1 já existente. Assim,
`type(version) == type` e `repr(type(version)) == "type"`, enquanto valores
construídos continuam com `type_name() == "version"`. A representação interna
da entidade e suas operações não mudam.

## P1339 — causa semântica compartilhada por chamada estática e ligada

### Medição anterior à decisão

No baseline HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`,
`01_core/src/entities/version.rs:103–120` já conserva o índice original para
erro, resolve negativos pelo comprimento explícito e devolve zero quando
positivo está além da lista. A fonte ratificada `a51e02804`,
`lab/typst-original/crates/typst-library/src/foundations/version.rs:109–134`,
documenta a mesma regra. A medição histórica dos casos `version-static-*` e
`version-bound-*` registra
proveniência integral, diff/stat vazio na medição, horários e saídas: negativos
fora da lista dão `component index out of bounds (index: ..., len: ...)`;
positivos além da lista dão zero, inclusive sobre versão vazia.

### Decisão proprietária e sucessão

`Version::at(&self, index: i64) -> Result<i64, String>` continua a única
causa da consulta. A descoberta `version.at` e o parser compartilhado com
`.at()` pertencem ao consumer `compiler/stdlib/primitives_constructors/version.rs`.
Este adendo sucede as descrições históricas de dispatch em bindings/closures
e, exclusivamente para `at`, revoga a dispensa literal de erros de argparsing
do §8a: mensagens, hints, spans e prioridade são observáveis da linguagem,
devem coincidir com a referência. A entidade recebe o índice já validado;
não recebe Args/AST, contexto, World ou spans de chamada.

Positivo usa extensão por zero. Negativo usa a lista explícita, conservando
zeros finais, e nunca a extensão infinita. `i64::MIN` deve falhar sem overflow
ou panic e imprimir o índice original. Não alterar a assinatura, o carrier
de componentes, major/minor/patch, repr, display, igualdade/hash/ordem,
constructor ou `PARITY_VERSION`. As referências antigas a versões nominais
acima são registro histórico; o alvo P1339 é somente `a51e02804`.

Presença, consulta e diagnóstico são linguagem; representação e chamadas
internas são mecânica. A documentação fundamenta a intenção; a medição
confirma comportamento. Inferência de reutilização suficiente é refutada se
uma entrada válida de `version.at` exigir alterar a regra de `Version::at`.
Não declarar que §10 fecha paridade geral: P1339 fecha apenas a rota `at`
no fragmento contratado e mantém os outros membros como controles, inclusive
a divergência de campos nomeados ausentes registrada em A.2.

## Default derivado com valor preservado — 2026-10-13

### Medição anterior à decisão

`01_core/src/entities/version.rs:134` contém a implementação manual de Default sinalizada pelo Clippy. O corpo existente retorna a mesma variante ou inicializa cada campo pelo seu default. O tipo não é genérico, portanto a derivação não acrescenta bounds a parâmetros públicos. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, spans, UTC, status e diffstat estão em `00_nucleo/diagnosticos/expressoes-defaults-ci-2026-10-13/before.json.gz`. É evidência de implementação, não nova medição de paridade ou desempenho.

### Decisão e aceitação

Substituir somente a implementação manual de Default por derive(Default), conservando o trait já implementado e seu valor. Version mantém components vazio. Preservar PARITY_VERSION e as implementações manuais de PartialEq/Eq/Ord/Hash com zero-pad; somente Default passa a ser derivado. Os exemplos anteriores de impl Default explicitam o valor exigido; a derivação equivalente passa a ser a forma desta implementação.

Não alterar assinatura pública, campos, variantes, comportamento padrão do produto ou fase. Correção mecânica interna em fluxo contínuo ADR-0127. Preservar testes e expectativas; validar todos os targets do workspace, suíte core, registro de testes, build, rustdoc, Clippy e linhagem.


## Nomes de domínio e traits Rust — 2026-10-16

### Medição anterior à decisão

Version.from_str aparece no inventário Clippy de 13 avisos. A assinatura e o corpo vigentes foram lidos antes desta decisão; snapshot 00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/before.json.gz conserva fontes/L0 completos, hashes globais, HEAD, working tree não commitado, UTC e diffstat. O contrato de retorno difere do trait sugerido. É comparação de interfaces Rust, sem nova medição de paridade ou benefício de leitura por IA.

### Responsabilidade exclusiva e aceitação

Preservar from_str(&str) -> Option<Self>, trim/parse u64, rejeições e componentes explícitos. Nenhum FromStr/Err novo, normalização de zeros ou mudança de PARITY_VERSION. from_str recebe expectativa nominal de should_implement_trait. A migração de to_string para Display é proposta pública separada e não está autorizada por este adendo.

Usar somente #[expect(clippy::should_implement_trait, reason="S004: parser de componentes Option preservado; FromStr exige Result e um contrato de erro distinto")] sobre o método nomeado. Não suprimir módulo/impl/crate, elevar limiar ou estender a exceção a Styles::from_iter. Implementar o trait proposto aqui exigiria contrato novo ou semântica diferente; a exceção preserva o contrato existente. Fluxo contínuo ADR-0127 para metadados/documentação, sem API, default ou fase novos. L0 primeiro e resselo restrito; RED do lint e GREEN incluindo unfulfilled_lint_expectations como erro; preservar corpos e usar os testes funcionais existentes sem alterar esperados. Validar workspace/build/Clippy/doc/fmt/linhagem sob 12 GiB, swap zero e um job.


## Traits Rust de formatação/construção — proposta 2026-10-16

### Medição anterior à decisão

O inventário anterior tem should_implement_trait em Styles.from_iter e inherent_to_string nos três tipos. As quatro implementações e os L0 vigentes foram lidos; fontes/hashes/HEAD/working tree/UTC/diffstat estão no base.json.gz deste pacote. Os dois parsers Option e os dois next de domínio foram classificados separadamente; não são alvo de conversão para traits. A sonda duration-before-probe e a correção interna anterior documentam a faixa i128 sem copiar o defeito. É mudança de contrato Rust, não mudança de linguagem Typst ou nova paridade medida.

### Contrato proposto

Implementar std::fmt::Display para Version e retirar o método inerente público to_string. Renomear o corpo atual para helper privado canonical_string, Display escreve essa string com f.write_str e ToString vem do blanket padrão. Preservar componentes explícitos, trailing zeros, vazio e ordem; não tocar em PARITY_VERSION, igualdade/hash/ordem, at, parsers, casts ou repr Typst. A precisão/largura de Formatter não redefine sua string canônica. FromStr não é adicionado.

O contrato só materializa após confirmação humana ADR-0127 §2.1/2.4: existem três novos impls Display, um FromIterator e migração de três métodos inerentes públicos para o blanket ToString. Código genérico pode passar a depender desses traits; rollback pode quebrar esse código. A implementação não deve adicionar FromStr, novas variantes/campos, flags, defaults, fase, dependências ou supressões. Os comentários/exemplos antigos de to_string passam a significar a interface ToString, somente após ratificação.

### Aceitação

T1: usos genéricos Display+ToString e collect::<Styles> devem falhar antes por trait ausente e passar depois. T2: Decimal exibe 1.2300 sem perder zeros, preserva negativo/zero e mantém rejeições Option; Version exibe vazio e 1.2.0 preservando componentes. T3: Duration mantém textos corrigidos de MIN/MAX/MIN+1, zero e limites negativos, via format! e ToString. T4: collect vazio é vazio e Weight(400),Weight(700) produz weight Some(700), com demais campos inalterados; adapter inerente mantém chamadas existentes. T5: suíte existente de linguagem/casts/repr/show/CLI, workspace completo, build, Clippy/expectativas, Rustdoc, formato e linhagem, sob 12 GiB, swap zero e um job, sem relaxar esperados. Sem publicar release.

## Ratificação humana — 2026-10-16

Proposta aprovada pelo humano nesta sessão, manifesto SHA-256 `d3fba43d799461d294ad562c37649ba9c2e4da2a45d7b7099953624ced28e7e8`. O contrato acima passa a vigente; a aprovação inclui materialização e testes T1–T5, sem release. Registro em `00_nucleo/diagnosticos/clippy-enums-2026-10-16/approval-receipt.json`.

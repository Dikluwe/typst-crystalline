# Prompt L0 — `Decimal` — precisão fixa decimal
Hash do Código: bc96717c

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/decimal.rs`, `01_core/src/entities/value.rs`
**Origem**: Passo 399 — modelagem de `Value::Decimal` (S); primeiro tipo S puro após abertura do portão ADR-0017.
**ADRs**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (operações aritméticas scope-out).

---

## 1. Contexto

O Typst vanilla expõe `decimal` como tipo de vírgula fixa em base 10 (função `decimal("...")`) — **não** de precisão arbitrária; ver a citação em §2. Em cristalino, o variant `Value::Decimal` estava ausente por causa do enum fechado (ADR-0017). Com o portão aberto em P395 e `Bytes` modelado em P398, modela-se agora `Decimal` como tipo puro.

## 2. Tipo L1

```rust
// 01_core/src/entities/decimal.rs
pub use rust_decimal::Decimal as InnerDecimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal(pub InnerDecimal);
```

- `rust_decimal::Decimal` é puro-Rust, `Copy` (128-bit stack value), zero alloc.
- Precisão fixa 28 dígitos — suficiente para paridade linguagem (ADR-0107).

> **Fonte de paridade (P1031)** — doc comment `#[ty]` do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/foundations/decimal.rs`, publicado em
> `typst.app/docs/reference/foundations/decimal/`:
>
> - **Tipo e propósito** — `decimal.rs:15-18`: *"A fixed-point decimal number type. This type
>   should be used for precise arithmetic operations on numbers represented in base 10. A
>   typical use case is representing currency."*
> - **Construtor por string** — `decimal.rs:26-30`: *"To create a decimal number, use the
>   `{decimal(string)}` constructor, such as in `{decimal("3.141592653")}` _(note the double
>   quotes)._ This constructor preserves all given fractional digits, provided they are
>   representable as per the limits specified below (otherwise, an error is raised)."*
>   Confirma a forma de superfície `decimal("...")` do contexto acima.
> - **Limites** — `decimal.rs:72-79`: *"A `decimal` number has a limit of **28 to 29**
>   significant base-10 digits. […] The maximum and minimum `decimal` numbers have a value of
>   `{79228162514264337593543950335}` and `{-79228162514264337593543950335}` respectively. In
>   contrast with `float`, this type does not support infinity or NaN, so overflowing or
>   underflowing operations will raise an error."*
>
> **Precisão da redacção acima**: *"precisão de arbitrária"* no contexto e *"precisão fixa
> 28 dígitos"* aqui são ambas aproximações. A linguagem diz **fixed-point**, com **28 a 29**
> dígitos significativos (não exactamente 28) e um máximo nomeado, que é exactamente
> `2^96 − 1` — o mesmo limite de `rust_decimal`. A escolha de `rust_decimal` está, portanto,
> alinhada com a linguagem por construção, e não só "suficiente". Corrigido o enquadramento;
> os números permanecem os do vanilla.
- `bigdecimal` (precisão arbitrária com heap) é scope-out; `rust_decimal` é a opção pragmática.

## 3. Construtores e acesso

```rust
impl Decimal {
    pub fn new(num: i64, scale: u32) -> Self { Self(InnerDecimal::new(num, scale)) }
    pub fn from_str(s: &str) -> Option<Self> { InnerDecimal::from_str(s).ok().map(Self) }
    pub fn from_f64(f: f64) -> Option<Self> { InnerDecimal::from_f64(f).map(Self) }
    pub fn from_i64(i: i64) -> Self { Self(InnerDecimal::from(i)) }
    pub fn to_string(&self) -> String { self.0.to_string() }
}

impl Default for Decimal {
    fn default() -> Self { Self(InnerDecimal::default()) }
}
```

## 4. Variant `Value::Decimal`

```rust
// 01_core/src/entities/value.rs
Decimal(crate::entities::decimal::Decimal),
```

Atualizações necessárias:
- `type_name()` → `"decimal"`.
- `PartialEq` via derive (a struct já implementa).
- `Hash` via derive/Debug formatting (padrão existente).
- `cast_decimal()` para extrair `&Decimal`.
- `From<Decimal> for Value`.

## 5. Cast

- `Value::Decimal(d) -> Decimal`: identidade.
- `Value::Int(i) -> Decimal`: `Decimal::from_i64(i)` (preciso).
- `Value::Float(f) -> Decimal`: `Decimal::from_f64(f)` (com perda documentada; `None` para NaN/Inf).
- `Value::Str(s) -> Decimal`: `Decimal::from_str(s)` (fallible).
- Outros tipos → erro de tipo.

## 6. Repr

Quando existir `native_repr`, deve devolver a string literal decimal pura (`"123.45"`), sem notação científica. `rust_decimal::Decimal::to_string()` já produz esse formato.

## 7. Scope-out

- `native_decimal(...)` stdlib — passo futuro (P400 ou agregado).
- Operações aritméticas (`+`, `-`, `*`, `/`, `%`, `pow`) — scope-out ADR-0054 graded.
- Comparações (`<`, `>`, `<=`, `>=`) — `PartialOrd` existe no tipo; consumer futuro.
- Literal suffixo decimal — não existe no vanilla.

## 8. Testes

- Construção `Decimal::new`, `from_str`, `from_i64`, `from_f64`.
- Igualdade (`1.0 == 1.00`), `Copy`, `Default`.
- `Value::Decimal` discriminação, `type_name`, `cast_decimal`, `From`.
- Cast a partir de `Int`, `Float`, `Str` (válido e inválido).


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — identidade pública do tipo `decimal`

O nome global `decimal` é o valor `Value::Type(Type::Decimal)`, chamável
pelo dispatcher estático e delegado ao construtor L1 já existente. Assim,
`type(decimal) == type` e `repr(type(decimal)) == "type"`, enquanto valores
construídos continuam com `type_name() == "decimal"`. A representação interna
da entidade e suas operações não mudam.

## Default derivado com valor preservado — 2026-10-13

### Medição anterior à decisão

`01_core/src/entities/decimal.rs:127` contém a implementação manual de Default sinalizada pelo Clippy. O corpo existente retorna a mesma variante ou inicializa cada campo pelo seu default. O tipo não é genérico, portanto a derivação não acrescenta bounds a parâmetros públicos. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, spans, UTC, status e diffstat estão em `00_nucleo/diagnosticos/expressoes-defaults-ci-2026-10-13/before.json.gz`. É evidência de implementação, não nova medição de paridade ou desempenho.

### Decisão e aceitação

Substituir somente a implementação manual de Default por derive(Default), conservando o trait já implementado e seu valor. Decimal continua delegando somente a InnerDecimal::default. Não alterar mantissa, escala, métodos numéricos ou traits restantes. Os exemplos anteriores de impl Default explicitam o valor exigido; a derivação equivalente passa a ser a forma desta implementação.

Não alterar assinatura pública, campos, variantes, comportamento padrão do produto ou fase. Correção mecânica interna em fluxo contínuo ADR-0127. Preservar testes e expectativas; validar todos os targets do workspace, suíte core, registro de testes, build, rustdoc, Clippy e linhagem.


## Nomes de domínio e traits Rust — 2026-10-16

### Medição anterior à decisão

Decimal.from_str aparece no inventário Clippy de 13 avisos. A assinatura e o corpo vigentes foram lidos antes desta decisão; snapshot 00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/before.json.gz conserva fontes/L0 completos, hashes globais, HEAD, working tree não commitado, UTC e diffstat. O contrato de retorno difere do trait sugerido. É comparação de interfaces Rust, sem nova medição de paridade ou benefício de leitura por IA.

### Responsabilidade exclusiva e aceitação

Preservar from_str(&str) -> Option<Self> e delegação ao parser atual; nenhum FromStr/Err novo ou mudança de rejeições. O consumidor escolhe diagnóstico de linguagem; não fabricar um erro Rust só para cumprir um nome. from_str recebe expectativa nominal de should_implement_trait. A migração de to_string para Display é proposta pública separada e não está autorizada por este adendo.

Usar somente #[expect(clippy::should_implement_trait, reason="S003: parser de domínio Option preservado; FromStr exige Result e um contrato de erro distinto")] sobre o método nomeado. Não suprimir módulo/impl/crate, elevar limiar ou estender a exceção a Styles::from_iter. Implementar o trait proposto aqui exigiria contrato novo ou semântica diferente; a exceção preserva o contrato existente. Fluxo contínuo ADR-0127 para metadados/documentação, sem API, default ou fase novos. L0 primeiro e resselo restrito; RED do lint e GREEN incluindo unfulfilled_lint_expectations como erro; preservar corpos e usar os testes funcionais existentes sem alterar esperados. Validar workspace/build/Clippy/doc/fmt/linhagem sob 12 GiB, swap zero e um job.


## Traits Rust de formatação/construção — proposta 2026-10-16

### Medição anterior à decisão

O inventário anterior tem should_implement_trait em Styles.from_iter e inherent_to_string nos três tipos. As quatro implementações e os L0 vigentes foram lidos; fontes/hashes/HEAD/working tree/UTC/diffstat estão no base.json.gz deste pacote. Os dois parsers Option e os dois next de domínio foram classificados separadamente; não são alvo de conversão para traits. A sonda duration-before-probe e a correção interna anterior documentam a faixa i128 sem copiar o defeito. É mudança de contrato Rust, não mudança de linguagem Typst ou nova paridade medida.

### Contrato proposto

Implementar std::fmt::Display para Decimal, delegando à implementação Display de InnerDecimal. Retirar somente o método inerente público to_string; chamadas usuais e UFCS ToString usam o blanket ToString. Preservar texto decimal, escala/zeros e sinais no formato padrão, sem notação científica nova. FromStr não é adicionado: from_str continua Option e sua expectativa nominal permanece. Nenhum método aritmético/round/cast/Default ou campo muda.

O contrato só materializa após confirmação humana ADR-0127 §2.1/2.4: existem três novos impls Display, um FromIterator e migração de três métodos inerentes públicos para o blanket ToString. Código genérico pode passar a depender desses traits; rollback pode quebrar esse código. A implementação não deve adicionar FromStr, novas variantes/campos, flags, defaults, fase, dependências ou supressões. Os comentários/exemplos antigos de to_string passam a significar a interface ToString, somente após ratificação.

### Aceitação

T1: usos genéricos Display+ToString e collect::<Styles> devem falhar antes por trait ausente e passar depois. T2: Decimal exibe 1.2300 sem perder zeros, preserva negativo/zero e mantém rejeições Option; Version exibe vazio e 1.2.0 preservando componentes. T3: Duration mantém textos corrigidos de MIN/MAX/MIN+1, zero e limites negativos, via format! e ToString. T4: collect vazio é vazio e Weight(400),Weight(700) produz weight Some(700), com demais campos inalterados; adapter inerente mantém chamadas existentes. T5: suíte existente de linguagem/casts/repr/show/CLI, workspace completo, build, Clippy/expectativas, Rustdoc, formato e linhagem, sob 12 GiB, swap zero e um job, sem relaxar esperados. Sem publicar release.

## Ratificação humana — 2026-10-16

Proposta aprovada pelo humano nesta sessão, manifesto SHA-256 `d3fba43d799461d294ad562c37649ba9c2e4da2a45d7b7099953624ced28e7e8`. O contrato acima passa a vigente; a aprovação inclui materialização e testes T1–T5, sem release. Registro em `00_nucleo/diagnosticos/clippy-enums-2026-10-16/approval-receipt.json`.

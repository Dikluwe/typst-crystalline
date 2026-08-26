# Prompt L0 — `stdlib/foundations/int` — superfície pública de `int`
Hash do Código: 5317a0e5

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/int.rs`
**Prompt pai**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: P1147
**Baseline**: vanilla ratificado `a51e02804`
**ADRs**: ADR-0107, ADR-0108, ADR-0127

## Medição anterior à decisão

O scope literal de `a51e02804:foundations/int.rs:68-428` contém `min`, `max`,
`signum`, seis operações bitwise e `from-bytes`/`to-bytes`. Sondas nos dois
binários ratificados coincidiram para as formas estática e de instância,
signed two's complement, shifts, constructor float/decimal e conversões de
bytes. `int.bit-and` é sentinela, não scope completo.

## Contrato da linguagem

- `min`/`max` continuam constantes `i64::MIN/MAX`.
- `signum(self)` retorna -1, 0 ou 1.
- `bit-not`, `bit-and`, `bit-or`, `bit-xor` operam nos 64 bits signed.
- `bit-lshift(self, shift: u32)` usa shift verificado; shift >= 64 erra
  `the result is too large`. Bits que saem dos 64 são descartados.
- `bit-rshift(self, shift: u32, logical: false)` satura em 0/-1 para shifts
  excessivos; `logical: true` reinterpreta como unsigned e satura em 0.
- `from-bytes(bytes, endian: "little", signed: true)` aceita no máximo oito
  bytes e faz extensão de sinal.
- `to-bytes(self, endian: "little", size: 8)` trunca ou estende para o size
  pedido e retorna `Bytes`.
- Funções com `self` existem nas formas estática e de instância; `from-bytes`
  é somente field estático.

O constructor permanece em `cast.rs` e aceita int, bool, float, decimal e
string. Float/decimal truncam em direção a zero; overflow erra. `base` só vale
para strings e deve estar entre 2 e 36.

## Estrutura

`int_type_field` é match fechado dos onze fields. As nativas são puras e
operam sobre tipos existentes; não criar wrapper, trait, registry ou despacho
dinâmico. O dispatch de instância intercepta somente os oito métodos com
`self`: `signum`, seis bitwise e `to-bytes`, sintetizando o receiver como
primeiro positional e delegando à mesma nativa estática.

Não há mudança de contrato público Rust, default do produto ou fase do
pipeline: L0 primeiro + RED→GREEN em fluxo contínuo (ADR-0127).

## Verificação

Cobrir descoberta, duas formas, min/max, negativos, min/max, shifts 0/63/64,
logical, endian, signed, sizes 0/8/9, roundtrip, casts e erros. Finalizar com
sondas ratificadas, workspace build/check/fmt, `git diff --check` e
`crystalline-lint .` sem violations.

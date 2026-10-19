# L0 — Signed time interval
Hash do Código: df4f6262

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/duration.rs`

## Evidence before specification

Historical upstream `e0e8ca4d` explicitly permits positive and negative spans
at `crates/typst-library/src/foundations/duration.rs:9-12`; constructor evidence
at `duration.rs:38-42` distinguishes an interval from a datetime and names weeks,
days, hours, minutes and seconds. The signed observable is language semantics;
the i128 representation is mechanics. These are not current-pin measurements.

## Owned public domain

`Duration` has public `nanos: i128`, deriving
Debug/Clone/Copy/PartialEq/Eq/PartialOrd/Ord/Hash. Negative intervals are
representable, comparable and printable. Default is ZERO.
Preserve ZERO, SECOND, MINUTE, HOUR and DAY, in nanoseconds: 0, 1_000_000_000,
60_000_000_000, 3_600_000_000_000 and 86_400_000_000_000 respectively.
`from_nanos(i128)` and `from_seconds/from_minutes/from_hours/from_days(i64)`
construct signed intervals. `as_seconds/as_minutes/as_hours/as_days(&self)`
return i64 whole-unit totals; `is_zero` tests zero. Existing domain signatures
remain intact. Neg maps to a signed Duration; no allocation or I/O is required
for storage. Constructor overflow handling outside the previously declared
real-value scope remains unresolved, not newly guaranteed by this cleanup.
The domain `to_string` preserves compact canonical formatting such as
`3d2h30m15.500s`, its negative form and `0s`; it is not language repr.

## Exclusive ownership

Value transport, casts and binary arithmetic are owned by
`00_nucleo/prompts/entities/value.md`; public type identity and dispatch by
eval owners; named duration repr by `00_nucleo/prompts/compiler/eval/repr.md`.
The previous cast/arithmetic/field clauses are handoff constraints retained in
the diagnostic, not requirements re-owned by this entity. String parsing remains
outside the declared domain scope. Preserve negative-duration semantics and
public signatures/defaults without introducing a new datetime or clock policy.


## Formatação em toda a faixa de nanos — 2026-10-16

### Medição anterior à decisão

Sonda isolada inclui o consumer vigente sem editá-lo e compila com rustc 1.92 em debug e -O. duration-before-probe.json em lab/results/construction/diagnostics/clippy-traits-formatacao-2026-10-16 registra i128::MIN: panic por overflow em debug e --55s em release; MIN+1/MAX, zero e -1s são controles. before.json.gz conserva fontes/L0, hashes, HEAD, working tree não commitado, UTC e diffstat; probe-result.json registra teto/RAM e comandos. duration.rs:103 usa self.nanos.abs(), cuja magnitude positiva não cabe em i128 para MIN. É defeito na formatação de um valor já representável por from_nanos e pelo campo público, não proposta de saturação ou de novo domínio público. O valor esperado de MIN vem da decomposição inteira exata de 2^127 em dias/horas/minutos/segundos/nanos, não de f64 ou do comportamento defeituoso em release.

### Responsabilidade exclusiva e aceitação

Na formatação atual, usar unsigned_abs para obter magnitude u128, com divisores positivos das mesmas unidades convertidos a u128. Conservar decomposição, ordem das partes, zeros, remoção de zeros fracionários e prefixo de um único sinal. MIN deve produzir -1969226660422097589487121d13h41m55.884105728s; MAX produz a forma positiva terminada em 727s; MIN+1 é o negativo de MAX. Preservar -1ns, -1s, limites de unidades, zero e todos os exemplos anteriores. Não mudar constructors, campos, as_*, Neg, política de overflow aritmético ou repr de linguagem. Não copiar esse defeito de outro compilador.

Correção de fórmula interna em fluxo contínuo ADR-0127, sem API, default ou fase novos. L0 primeiro, teste de extremos RED real antes do código e GREEN depois; confirmar também sonda debug/release e controles existentes. Nenhuma supressão de inherent_to_string nesta etapa: Display é proposta pública separada. Resselo restrito, suíte workspace/build/Clippy/doc/fmt/linhagem e RAM máxima 12 GiB, swap zero e um job. Não afirmar ganho de performance ou paridade global a partir desta correção local.


## Traits Rust de formatação/construção — proposta 2026-10-16

### Medição anterior à decisão

O inventário anterior tem should_implement_trait em Styles.from_iter e inherent_to_string nos três tipos. As quatro implementações e os L0 vigentes foram lidos; fontes/hashes/HEAD/working tree/UTC/diffstat estão no base.json.gz deste pacote. Os dois parsers Option e os dois next de domínio foram classificados separadamente; não são alvo de conversão para traits. A sonda duration-before-probe e a correção interna anterior documentam a faixa i128 sem copiar o defeito. É mudança de contrato Rust, não mudança de linguagem Typst ou nova paridade medida.

### Contrato proposto

Implementar std::fmt::Display para Duration e retirar o método inerente público to_string. Renomear sua implementação corrigida para helper privado canonical_string, preservando exatamente o algoritmo u128 e o corpo atual; Display escreve essa string com f.write_str. ToString vem do blanket padrão. Preservar MIN/MAX/zero, sinais, frações e unidades; não alterar repr Typst, Neg, overflow aritmético ou as_*. A precisão/largura de Formatter não redefine a string canônica deste tipo.

O contrato só materializa após confirmação humana ADR-0127 §2.1/2.4: existem três novos impls Display, um FromIterator e migração de três métodos inerentes públicos para o blanket ToString. Código genérico pode passar a depender desses traits; rollback pode quebrar esse código. A implementação não deve adicionar FromStr, novas variantes/campos, flags, defaults, fase, dependências ou supressões. Os comentários/exemplos antigos de to_string passam a significar a interface ToString, somente após ratificação.

### Aceitação

T1: usos genéricos Display+ToString e collect::<Styles> devem falhar antes por trait ausente e passar depois. T2: Decimal exibe 1.2300 sem perder zeros, preserva negativo/zero e mantém rejeições Option; Version exibe vazio e 1.2.0 preservando componentes. T3: Duration mantém textos corrigidos de MIN/MAX/MIN+1, zero e limites negativos, via format! e ToString. T4: collect vazio é vazio e Weight(400),Weight(700) produz weight Some(700), com demais campos inalterados; adapter inerente mantém chamadas existentes. T5: suíte existente de linguagem/casts/repr/show/CLI, workspace completo, build, Clippy/expectativas, Rustdoc, formato e linhagem, sob 12 GiB, swap zero e um job, sem relaxar esperados. Sem publicar release.

## Ratificação humana — 2026-10-16

Proposta aprovada pelo humano nesta sessão, manifesto SHA-256 `d3fba43d799461d294ad562c37649ba9c2e4da2a45d7b7099953624ced28e7e8`. O contrato acima passa a vigente; a aprovação inclui materialização e testes T1–T5, sem release. Registro em `lab/results/construction/diagnostics/clippy-enums-2026-10-16/approval-receipt.json`.

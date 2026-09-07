# P1303 — recibo da implementação mínima P6

## Veredito do papel

`P1303_IMPLEMENTATION_GREEN`.

O papel **IMPLEMENTADOR** materializou somente a troca de âncora autorizada no
ramo especial dos três fields `pdf.*` bloqueados por `a11y-extras`. O regime foi
o protocolo completo de materialização segregada, **executado sem atestação de
isolamento técnico**: filesystem e contexto de coordenação eram compartilhados;
hashes, allowlist, ordem causal e diff literal identificam a execução, mas não
provam isolamento.

Este recibo não é o veredito P7/P8, não executa mutantes e não certifica
equivalência funcional geral.

## Identidade, capacidades e entradas congeladas

- executor: sessão Codex no papel exclusivo `IMPLEMENTADOR` P6;
- ambiente: `/repos/Antigravity/typst-crystalline`;
- HEAD: `5b4a0d0438a535c54fdb5e74b28903c1313f5bc2`;
- captura final anterior à escrita deste recibo:
  `2026-09-04T01:05:40,351080474-03:00`;
- leitura exercida: `AGENTS.md`, skill `tekt-materializacao-segregada` e suas
  duas referências, passo P1303 pelo path exato, ADR-0107/0108/0127, owner L0,
  contrato, oracle, plano adversarial, recibo do gate L0, recibo RED e o
  consumer produtivo;
- escrita exercida: somente
  `01_core/src/compiler/eval/bindings/field_access.rs` e este recibo;
- testes candidatos: executados, mas não editados;
- não executados: `crystalline-lint --fix-hashes`, ataques P7, staging ou
  commit.

Hashes conferidos imediatamente antes da implementação e novamente depois dos
gates:

| Input protegido | SHA-256 conferido |
|---|---|
| `00_nucleo/materialization/typst-passo-1303.md` | `f1db5c02b7e6af5ef461900c213e928a8d16bbed156d8eba7ff5c6b32c584fe9` |
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `92aa897909abfb6095ab59191614b0fd87c0d648e8e593ed8f0702176c2a542e` |
| `00_nucleo/diagnosticos/p1303-contract.md` | `837cb4bd1f38428d93cdadf9216c0182c69e3d8bfa8eb489ed52a710e2bbdf8a` |
| `00_nucleo/diagnosticos/p1303-oracle-receipt.md` | `f23960b7c5a59f0586fda5378588d52de7546ff8551a5c6787c322af6f652289` |
| `00_nucleo/diagnosticos/p1303-adversarial-plan.md` | `ebaaad08d475a53fc2dc6fe41ea69c901069f302633b7e41ba0f5293031057b8` |
| `00_nucleo/diagnosticos/p1303-l0-gate-receipt.md` | `98ae55dcc62a8b1d29e2ab7133997e06c3b6587354b5851d21cca70e657dbe15` |
| `00_nucleo/diagnosticos/p1303-red-tests-receipt.md` | `d1d20e0510f328b1ee505c3344287d70e8d28f56ed2e4608943118f871806e4a` |
| `01_core/src/compiler/eval/tests.rs` | `ab57d8c30dd54e1f8d638fcad4ab52e3c2c4bf9c4b3f674816806fe5ba8824b0` |

O hash do teste permaneceu idêntico ao handoff congelado. Nenhum input
protegido mudou durante P6.

## Implementação e diff literal

| Artefato | SHA-256 |
|---|---|
| `field_access.rs` pré-implementação | `6ce0009a9a25c786b935128dd3e5cc40689ef8a0942b0bd089ee627ec14cee58` |
| `field_access.rs` pós-implementação | `6b82a38b7c953ebe9dcbe96857dd9cb689aaaf69e95df153afa5de668ef89486` |

Diff literal produzido por P6, relativo ao estado de handoff:

```diff
--- a/01_core/src/compiler/eval/bindings/field_access.rs
+++ b/01_core/src/compiler/eval/bindings/field_access.rs
@@
             return Err(vec![SourceDiagnostic::error(
-                access.span(),
+                access.field().span(),
                 format!(
```

O diff do mesmo arquivo contra o HEAD também contém a alteração preexistente de
`@prompt-hash adcb180b` para `73f15bd4`, produzida e registrada pelo papel L0
antes de P6. Essa alteração não foi escrita pelo IMPLEMENTADOR. A contribuição
P6 é exclusivamente o hunk acima.

## RED → GREEN

RED recebido e congelado em
`p1303-red-tests-receipt.md` (`d1d20e05…`):

```text
test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 5431 filtered out; finished in 0.59s
```

A única falha agregava exatamente seis divergências de span:
`default` e `html` × `data-cell`, `header-cell`, `table-summary`, observando
início `10` em lugar de `14`. Mensagem, dois hints ordenados, cardinalidade e os
quatro testes de controle já coincidiam.

Comando GREEN executado após o patch:

```bash
cargo test -p typst-core p1303 -- --test-threads=1
```

Exit `0`. Trecho terminal integral relevante do output:

```text
warning: `typst-core` (lib test) generated 675 warnings (run `cargo fix --lib -p typst-core --tests` to apply 115 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.03s
     Running unittests src/lib.rs (target/debug/deps/typst_core-d51858d7cd5964e0)

running 5 tests
test compiler::eval::tests::tests::p1303_negativos_pdf_exatos_nos_perfis_sem_a11y ... ok
test compiler::eval::tests::tests::p1303_ordem_repeticao_e_estado_completo ... ok
test compiler::eval::tests::tests::p1303_positivos_pdf_exatos_nos_perfis_com_a11y ... ok
test compiler::eval::tests::tests::p1303_sentinelas_de_span_module_e_nao_module ... ok
test compiler::eval::tests::tests::p1303_sentinelas_pdf_ungated_e_html_disabled ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 5431 filtered out; finished in 0.53s
```

Os `675` warnings são preexistentes e não impediram compilação nem execução dos
cinco testes. O gate exigido terminou GREEN `5/5`.

## Checks adicionais

Comando:

```bash
cargo fmt --all -- --check
```

Exit `0`; output vazio.

Comando:

```bash
git diff --check
```

Exit `0`; output vazio.

## Containment e limites

Na captura anterior à criação deste recibo, `git diff HEAD --stat` registrou:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  80 +++++
 00_nucleo/prompts/compiler/eval/tests.md           | 100 +++++-
 01_core/src/compiler/eval/bindings/field_access.rs |   4 +-
 01_core/src/compiler/eval/tests.rs                 | 382 ++++++++++++++++++++-
 4 files changed, 562 insertions(+), 4 deletions(-)
```

`git diff --cached --stat` produziu output vazio. As mudanças fora do hunk P6
já pertenciam aos papéis anteriores e foram preservadas. O IMPLEMENTADOR não
moveu o gate, não alterou catálogo, mensagem, hints, APIs, `Module`, `Scope`,
`Features`, stdlib, L0, testes, oracle, contrato ou plano adversarial.

A alegação deste recibo limita-se ao candidato mínimo e ao GREEN dos cinco
testes P1303. Não cobre mutation score, matriz bilateral final, suites P1300/
P1301, workspace completo, lint arquitetural, PDF exportado, acessibilidade ou
equivalência funcional geral. Esses gates e o veredito pertencem aos papéis
P7/P8.

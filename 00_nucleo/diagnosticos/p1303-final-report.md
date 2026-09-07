# P1303 — relatório final independente P8

## Veredito

`P1303_CERTIFIED`

O P1303 está certificado exclusivamente para os spans dos três fields
feature-gated `pdf.data-cell`, `pdf.header-cell` e `pdf.table-summary`, nos
observáveis e sentinelas de `C-P1303-v1`. Esta alegação não é equivalência
funcional geral do compilador, do namespace `pdf`, do PDF exportado nem da
acessibilidade.

O protocolo completo de materialização segregada foi **executado sem
atestação de isolamento técnico**. O filesystem e o contexto de coordenação
eram compartilhados; hashes, bytes, allowlists, ordem causal, outputs e
restaurações identificam a execução, mas não provam isolamento de capacidades.

## Cadeia detached

Os três artefatos finais foram criados na ordem causal manifesto → certificado
→ relatório, sem ciclos de auto-hash:

| Artefato predecessor | SHA-256 | Bytes |
|---|---|---:|
| `00_nucleo/diagnosticos/p1303-manifest.json` | `1ec694b8e9247fdd6eda821e2ab8a66b925d8e5b2386277d2402f7108618923a` | 12980 |
| `00_nucleo/diagnosticos/p1303-certificate.json` | `77d209164fa3e2c9af5d5df69ee8ad7d72db8a5fc829e7469a827efcbc46fb90` | 8425 |

O manifesto não contém auto-hash; o certificado pina hash e tamanho do
manifesto. O certificado não pina a si próprio nem este relatório. Este
relatório pina os dois predecessores e sua identidade própria é publicada
externamente após o fechamento.

## Auditoria da obrigação, contrato e causalidade

O baseline é o commit
`5b4a0d0438a535c54fdb5e74b28903c1313f5bc2`. P0 registrou árvore limpa salvo o
próprio passo P1303, sem diff tracked ou staged. O vanilla ratificado permaneceu
`a51e02804`, pelo binário `/usr/local/bin/typst` de SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` e
51851160 bytes.

A medição pré-patch reproduziu 48 runs e 24 comparações em ordem normal e
invertida. Por ordem, `default` e `html` tiveram três
`DIFFERENT_DIAGNOSTIC`; `a11y` e `html+a11y`, três `MATCH_VALUE`. Foram 0
`Unknown`/`EXECUTION_UNKNOWN` e 0 divergências de repetição. A única diferença
negativa foi a âncora `10..end` no cristalino contra `14..end` no vanilla;
mensagem, dois hints ordenados, severidade e cardinalidades já coincidiam.

`C-P1303-v1` foi congelado antes do RED. O plano adversarial nominou os 14
mutantes e seus assassinos. Os owners L0
`compiler/eval/bindings/field_access.md` e `compiler/eval/tests.md` receberam
medição antes da decisão, inferência refutável, classificação
`ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY`, regra field-only e proibições antes de
qualquer código candidato. O resselo foi feito nessa fase e o dry-run devolveu
`Nothing to fix`.

O RED permanente teve 4 testes de controle verdes e uma falha agregada
contendo exatamente os seis spans previstos (`3 fields × 2 perfis`), sem
divergência de mensagem, hints ou cardinalidade. A implementação produtiva é o
patch mínimo de uma linha no ramo especial:

```diff
-                access.span(),
+                access.field().span(),
```

Não houve mudança de catálogo, API, entidade, trait, assinatura, default,
feature flag, fase, CLI, exportador, wiring, lab ou `stdlib/pdf.rs`. O diff
tracked final está contido exatamente nos quatro paths autorizados: dois owners
L0, o consumer produtivo e seu consumer test-only.

## Poder discriminatório e restauração

O ledger registra 14 mutantes aplicáveis, 14 execuções válidas, 14 mortos,
0 Unknown, 0 falhas de compilação e `mutation_score = 1.0`.

M10-A compilou, mas não materializou a exposição sem `a11y-extras` porque o
interceptor de field access permaneceu ativo. Sua classificação é
`INVALID_MUTANT_EXECUTION`, com crédito zero e fora do numerador; o denominador
permaneceu 14. O addendum R1 definiu M10-B como uma única ativação sincronizada
de dois hunks. M10-B foi válido, expôs os seis sucessos negativos inesperados,
foi morto pelo assassino congelado e foi restaurado. O único crédito nominal de
M10 vem dessa execução válida, nunca de M10-A.

Todos os mutantes foram restaurados por patch inverso. Os hashes candidatos
finais conferem:

| Path | SHA-256 | Bytes |
|---|---|---:|
| `01_core/src/compiler/eval/bindings/field_access.rs` | `6b82a38b7c953ebe9dcbe96857dd9cb689aaaf69e95df153afa5de668ef89486` | 35879 |
| `01_core/src/compiler/eval/tests.rs` | `ab57d8c30dd54e1f8d638fcad4ab52e3c2c4bf9c4b3f674816806fe5ba8824b0` | 732366 |
| `01_core/src/compiler/stdlib/pdf.rs` | `b1d7b325eace8f6d819a16c6ff7c135f1ca42781bacaec71888045dbd977ce4d` | 17833 |

`pdf.rs` está ausente do diff tracked final. Contrato, oracle, planos, L0,
testes e receipts conferem com os hashes congelados no manifesto e no
certificado.

## Gates finais

O recibo P7 preserva os outputs integrais e comprova execução na ordem exigida:

1. `cargo fmt --all -- --check`;
2. `cargo test -p typst-core p1303 -- --test-threads=1`;
3. `cargo test -p typst-core p1301 -- --test-threads=1`;
4. `cargo test -p typst-core p1300 -- --test-threads=1`;
5. `cargo test -p typst-core`;
6. `cargo test --workspace`;
7. `cargo build`;
8. `crystalline-lint .`;
9. `crystalline-lint --fail-on warning --checks v3,v4,v5,v13,v14,v15,v26 .`;
10. `crystalline-lint --fix-hashes --dry-run .`;
11. `git diff --check`.

Resultado: 11/11 exits zero, strict lint sem violations e dry-run `Nothing to
fix`. O P8 repetiu sobre os hashes atuais `git diff --check`,
`cargo fmt --all -- --check`, o focal P1303 (5 passed, 0 failed) e o dry-run do
linter (`Nothing to fix`); os quatro passaram.

A matriz bilateral final, revalidada diretamente a partir dos runs e não apenas
do veredito do runner, contém 48 runs e 24 comparações. Em cada uma das ordens:

| Perfil | Resultado |
|---|---:|
| `default` | 3 `MATCH_DIAGNOSTIC` |
| `html` | 3 `MATCH_DIAGNOSTIC` |
| `a11y` | 3 `MATCH_VALUE` |
| `html+a11y` | 3 `MATCH_VALUE` |

Totais impeditivos: `DIFFERENT_DIAGNOSTIC = 0`, `EXECUTION_UNKNOWN = 0`,
`Unknown = 0` e divergências de repetição = 0. Os 24 checks contratuais
field×perfil×ordem passaram.

## Inventário residual ajustado

Após o fechamento dos três spans, o universo do ledger P1299 permanece com 160
paths:

| Classe residual | Paths |
|---|---:|
| `MISSING_LANGUAGE_MEMBER` | 106 |
| `WRONG_PUBLIC_KIND_OR_IDENTITY` | 11 |
| `INTENTIONAL_PRODUCT_EXTENSION` | 42 |
| `EXPECTED_FEATURE_DISABLED` | 1 |
| **Total** | **160** |

`repr(std)` fica fora desse ledger e deve entrar como item separado de
identidade pública no próximo rebaseline; não deve ser mascarado por
diagnósticos de field ausente.

## Próximo passo recomendado

Recomenda-se um rebaseline incremental do HEAD certificado antes de selecionar
o próximo lote. `json.encode`, `toml.encode` e `yaml.encode` constituem apenas
uma hipótese de seleção por coesão de owner; não são autorização para editar L0,
código, testes ou contrato. A matriz fresca pode selecionar outro lote.

Nenhum staging ou commit foi realizado.

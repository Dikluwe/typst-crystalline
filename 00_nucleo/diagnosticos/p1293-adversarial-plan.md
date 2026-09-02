# P1293 — plano adversarial final e plano de correção segregada

**Papel:** `atacante_p1293`, sequência causal 6  
**Estado:** campanha executada; fechamento rejeitado até correções e nova
verificação independente  
**Regime:** protocolo Tekt completo, segregado por capacidades e artefatos,
sem isolamento técnico de leitura  
**HEAD:** `7dd25ff0e222b6c7c640d6bc7957b98f94227507` (`Tekt`)  
**Working tree:** não commitada; baseline fresco P1293 tinha `git diff HEAD`
vazio e somente os três artefatos iniciais P1293 não rastreados.

Este papel leu integralmente o passo explicitamente autorizado
`00_nucleo/materialization/typst-passo-1293.md`, as ADRs 0107, 0108, 0127,
0128 e 0129, os L0s proprietários relevantes e os recibos/manifesto/selo.
Não leu qualquer outro ficheiro de `materialization/` ou `context/`. Não
alterou produto, L0, Núcleo, contrato, oráculo, manifesto ou selo.

## Hipóteses atacadas

1. A aprovação local A/B/C/D implica fechamento integrado.
2. O `Group` lógico introduzido pelo lote B preserva tanto o render quanto
   todos os observadores históricos.
3. O oráculo protegido atual ainda é inteiramente derivado do seu único L0.
4. Toda escrita rastreada P1293 pertence a alguma allowlist serial.
5. O default HTML continua cristalino e o perfil vanilla é somente explícito.
6. `MathAttachSlot` mantém três estados realmente distintos.
7. Os dez nomes D mudam sem alterar scope keys, ponteiros e aliases flat.
8. Os mutantes registrados continuam sem sobreviventes e sem `Unknown`.
9. A violação operacional `rg` pode ser tratada como se não tivesse ocorrido.

## Campanha e resultado

### Protegidos e focais

- o oráculo P1293 passou `11/11`; A `1/1`, B `2/2`, C `5/5` e D `2/2`
  também passaram em ordem direta e reversa na campanha atual;
- `typst-core p1293_`: `50/50`; P1105: `6/6`; `typst-infra p1293_`:
  `6/6`;
- P1289: `1/1`, P1290 core: `7/7`, P1291 core: `30/30` e P1291 wiring:
  `5/5`;
- o registro congelado contém 31 IDs e o gate recebido registra 31/31
  rejeitados em cada ordem, score 1.0, zero sobreviventes e zero `Unknown`;
- não foi encontrado sobrevivente funcional contra o fragmento protegido
  atual. O gate é uma prova de poder do oráculo, não substitui os gates globais.

### Bloqueio global de infra e causalidade

Em `2026-09-02T16:38:34-03:00`, o comando

```text
RUSTFLAGS='-Awarnings' cargo test -p typst-infra --lib -- --nocapture
```

terminou em `2026-09-02T16:38:40-03:00`, exit `101`, com `910 passed / 8
failed`; log `/tmp/p1293-adversary-infra-lib.log`, SHA-256
`5c330283e23b98a36a332e459d88072a00e800a59fc093d1ef13a54a5ad327eb`.
Falharam:

- P1132e em `03_infra/src/integration_tests.rs:4650`;
- P1132g em `:4695`;
- P1132k em `:4737`;
- P1132n em `:4957`;
- P1132o em `:5008`;
- P1132p em `:5045`;
- P1133b em `:4795`;
- P1136 em `:4892`.

Os oito casos usam `frame_items_recursive` (`integration_tests.rs:34-51`). O
helper desce por `Semantic`, `Group` e `Link`, mas devolve os filhos sem compor
`Group.pos`/`matrix`. O lote B, no owner
`01_core/src/compiler/layout/equation.rs:454-485`, moveu os filhos de Formula
para coordenadas locais e colocou-os num `Group` identidade em `frame_pos`.
Isto é exatamente a mecânica requerida pelo L0
`00_nucleo/prompts/compiler/layout/equation.md`: o render absoluto não muda e
a extensão lógica passa a sobreviver ao terminal flush. Os números falhos são
coordenadas locais tratadas como globais; em P1132o o filtro `y < 80` também
inclui cinco `+` locais que antes ficavam fora da janela global.

Classificação ADR-0107: os oito REDs demonstram um observador de teste obsoleto,
não uma regressão visual por si só. Reverter o `Group` ou alterar os valores
esperados inverteria a causalidade. O reparo correto compõe a posição e a
transformação ancestral e preserva as expectativas globais atuais. O L0
`infra/integration_tests.md:16-28,40-52` já legitima helpers privados que
atravessam frames e verificam layout; não é necessário reabrir L0 produtivo.
Se a composição correta ainda divergir, então há regressão real e o owner
produtivo exato deve ser reaberto antes de código.

### Regressão protegida P1292

Em `2026-09-02T16:42:04-03:00`, o comando

```text
RUSTFLAGS='-Awarnings' cargo test -q -p typst-wiring --test p1292_contract
```

terminou em `2026-09-02T16:42:11-03:00`, exit `101`, `10 passed / 1 failed`.
Falhou `p1292_b_plain_mixed_baselines_and_following_line_advance` em
`04_wiring/tests/p1292_contract.rs:422`: duas baselines em vez de uma.

O SVG atual de `A $x$ B` possui grupo externo
`translate(10.395 2.651)` e matriz interna do glifo com `y=4.862`; a baseline
absoluta continua `2.651 + 4.862 = 7.513`, igual aos glifos A/B. O parser
`svg_glyph_baselines` (`p1292_contract.rs:144-165`) recolhe apenas o sexto
operando da matriz interna e ignora a translação ancestral. É o mesmo efeito
causal do `Group` do lote B, agora através do exporter SVG.

O observável P1292 permanece correto e a mecânica de transporte ficou obsoleta.
Como o consumer protegido P1292 terá de mudar e o seu L0 vigente não descreve
essa prova de baseline mista nem composição de transformações, a correção deve
reabrir primeiro `00_nucleo/prompts/wiring/tests/p1292_contract.md`, limitar a
alteração ao parser de transporte, preservar `7.513` e todas as expectativas,
resselar o header e emitir uma emenda de preservação P1292. Não há novo gate
humano ADR-0127: é correção test-only de transporte, sem API/default/fase.

### Formatação e allowlist

Em `2026-09-02T16:38:23-03:00`, `cargo fmt --all -- --check` terminou às
`16:38:25-03:00`, exit `1`, em 15 arquivos:

```text
01_core/src/compiler/math/layout/accent.rs
01_core/src/compiler/math/layout/cancel.rs
01_core/src/compiler/math/layout/cases.rs
01_core/src/compiler/math/layout/frac.rs
01_core/src/compiler/math/layout/matrix.rs
01_core/src/compiler/math/layout/mod.rs
01_core/src/compiler/math/layout/root.rs
01_core/src/compiler/math/layout/spacing.rs
01_core/src/compiler/math/layout/tests.rs
01_core/src/compiler/math/layout/underover.rs
01_core/src/compiler/math/layout/vec.rs
01_core/src/compiler/stdlib/structural/math.rs
01_core/src/entities/elements/math_attach.rs
03_infra/src/font_metrics.rs
03_infra/src/shaper.rs
```

Todos surgiram depois do baseline P1293 com diff rastreado vazio e pertencem
causalmente à janela do lote B. Sete contêm mudanças B autorizadas e apenas
precisam do rustfmt atual: `mod.rs`, `spacing.rs`, `tests.rs`,
`structural/math.rs`, `math_attach.rs`, `font_metrics.rs`, `shaper.rs`.

Os outros oito têm somente uma troca de ordem de imports contrária ao rustfmt:
`accent.rs`, `cancel.rs`, `cases.rs`, `frac.rs`, `matrix.rs`, `root.rs`,
`underover.rs`, `vec.rs`. Sete nem aparecem no manifesto/selo; `frac.rs`
aparece somente como congelado/fora de escopo. São escritas fora da allowlist,
independentemente de não mudarem semântica. Devem ser restauradas ao preimage
HEAD, o que coincide com o resultado do rustfmt vigente. Nenhum L0 é reaberto
por limpeza mecânica; é obrigatório um selo corretivo com allowlist exata.

Também existe `./-`, não rastreado, 7.735 bytes, mtime
`2026-09-01T19:15:45.047110808-03:00`, SHA-256
`414e2d883b7561996d3986f61c9e63ff210862f0e4cfb15f773b45973da36967`.
É um SVG com viewBox `46.797666666666665 × 24.057`, exatamente o vetor binom
display B-P07, não presente no baseline e fora de qualquer allowlist de saída.
Deve ser removido e a limpeza registrada; a autoria exata não é demonstrável
pelos recibos preservados.

### Linhagem do oráculo P1293

V5, V7, V15 e V26 passam com a sintaxe atual do linter, mas existe drift
semântico que essas checks não detectam:

- `00_nucleo/prompts/wiring/tests/p1293_contract.md:4-7` ainda declara gate C
  pendente;
- `:265-268` congela 29 mutações;
- `:317-320` afirma que o oráculo continua no hash antigo e proíbe a atualização;
- o consumer atual tem C-P10 e MC13/MC14 (`04_wiring/tests/p1293_contract.rs:
  917-947,1147-1148`), 31 IDs e SHA-256 `ed3e0b57...`;
- `p1293-contract-receipt.md` ainda se apresenta como reaberto após o RED B e
  não é uma síntese causal A-D vigente;
- o receipt discriminatório atual declara em `capabilities.integrated_write`
  o receipt anterior `p1293-preseal-discrimination-c-export-facade-receipt.json`,
  não o próprio artefato P08.

Logo C-P10/MC13/MC14 foram acrescentados ao único consumer antes de serem
legitimados pelo L0 proprietário. O L0 e o receipt canônico precisam ser
reabertos apenas para registrar o contrato já autorizado e protegido, nunca
para adaptar expectativa ao candidato. Como `infra/export/mod.md` e a confirmação
C já exigem a fachada direta, isto não introduz contrato público novo e não
requer novo gate humano; qualquer obrigação adicional exigiria nova classificação
ADR-0127 e paragem. A mudança do L0/header/receipt invalida os hashes protegidos:
é necessário RED/preimage causal preservado, discriminação fresca 31/31 nas duas
ordens e selo substituto antes de novo veredito.

### HTML, tri-state e nomes D

Não foi encontrada refutação substantiva nestas frentes:

- CLI e APIs antigas usam `Crystalline` por default; `Vanilla` é escolha
  explícita e só altera escaping HTML. Feature e target continuam eixos
  independentes;
- `MathAttachSlot::{Omitted, ExplicitNone, Present(Content)}` existe e é
  propagado; somente `Present` cria caixa e o `repr` distingue ausência,
  `none` e `[]`;
- os dez nomes namespaced são curtos, as dez chaves e ponteiros continuam
  correspondentes e os seis aliases flat históricos mantêm underscore;
- A/B/C/D passam no oráculo P1293 e o registro discriminatório não contém
  sobrevivente conhecido.

### Violação operacional `rg`

O `rg` sobre `00_nucleo` que atravessou `materialization/` é uma
`PROCESS_VIOLATION_CONFIRMED_NOT_ABSOLVED`. O oráculo protegido fica fora da
raiz reportada e não há evidência de influência semântica; também não existem
comando/transcript completos que provem não exposição. Isso não refuta sozinho
o produto C, mas impede qualquer declaração de isolamento técnico ou de
cumprimento operacional sem ressalva. O certificado futuro deve carregar o
incidente literalmente; a única claim admissível é a desta página.

## Sequência obrigatória de correção

1. Uma autoridade separada invalida/supersede o handoff final ativo
   `post_lot_d_approval_final_handoff`; ele é read-only e não concede correção.
2. O autor de contrato corrige primeiro o L0 do oráculo P1293 para estado,
   C-P10, MC13/MC14 e denominador 31; emite receipt canônico substituto. Não
   muda expectativas. Sem gate humano novo salvo ampliação substantiva.
3. O autor P1292 corrige primeiro o L0 do oráculo P1292 para a obrigação já
   existente de baseline mista e transporte que compõe transforms; ressela o
   header. Sem gate humano novo.
4. Um testador segregado altera somente:
   `03_infra/src/integration_tests.rs` para coordenadas absolutas compostas e
   `04_wiring/tests/p1292_contract.rs` para parser SVG transform-aware. Mantém
   todos os valores esperados. Qualquer divergência após composição reabre o
   owner produtivo causal e para.
5. Sob allowlist corretiva exata, rodar rustfmt nos 15 arquivos, restaurando os
   oito import-only ao preimage, e remover `./-`. Nenhuma correção semântica é
   autorizada por este plano.
6. Resselar headers alterados, reexecutar V3/V4/V5/V7/V13/V14/V15/V26,
   hash dry-run e diff-check; refazer a discriminação 31/31 em ambas as ordens
   com receipt autoconsistente e novo selo.
7. Reabrir o checkpoint de verificação do lote B e a preservação P1292; não é
   necessário reverter A/C/D se os hashes permanecerem iguais, mas todos os
   três devem ser revalidados depois do novo selo.
8. Exigir verdes `cargo test -p typst-infra --lib`, P1292 `11/11`, protected
   P1293 direto/reverso, workspace, release build, fmt e lint. Os comandos
   documentados `--fail-on-warning=V*` não existem no CLI atual; usar e registrar
   `--checks v* --fail-on warning`, ou corrigir documentalmente a invocação.
9. Só então gerar os inventários `p1293-surface-default.json` e
   `p1293-surface-html.json`, recibos de preservação/verificação, relatório e
   certificado final. Estes artefatos ainda não existem no estado auditado.

## Veredito do adversário

`REJECTED_FOR_FINALIZATION_CORRECTIONS_REQUIRED`.

O produto A/B/C/D permanece candidato e não há aprovação adversarial. O lote B
deve reabrir sua verificação/preservação; os L0s de oráculo P1293 e P1292, os
respectivos headers e a cadeia de selo devem reabrir. O L0 produtivo de
`equation.rs` não deve ser reaberto nem revertido com a evidência atual. O
passo não pode fechar com infra `8` RED, P1292 `1` RED, fmt `15` RED, nove
escritas/saídas fora de allowlist, owner do oráculo P1293 defasado e artefatos
finais ausentes.

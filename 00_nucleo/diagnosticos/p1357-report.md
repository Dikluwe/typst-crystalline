# P1357 — verificação independente final, revisão focal 2

## Veredito

**CONTINUE_WITH_LIMITATIONS.** A revisão 2 fecha as lacunas instrumentais dos inventários: há 32 execuções, quatro hashes determinísticos idênticos e crosswalk integral de 8.724 linhas. Também acrescenta ao S07 proveniência completa do `qpdf` e um observador vertical separado. Permanecem três limitações focais: S02 top-level não observa o setting efetivo; o comparador vertical S07 é confundido por textos diferentes; e S11 continua `Unknown(PARSER_UNSUPPORTED)` fail-closed. Essas limitações definem continuidade e não promovem ausência de observador a `MATCH`. Execução **executado sem atestação de isolamento**.

## Proveniência congelada

Executor `/root/verificador_p1356`; UTC `2026-09-12T19:49:14Z`; HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`; tree `c3c2fa834e6ec318690ddb88e181277ff2e25b27`. Conferiram por SHA-256: passo `fd4046587ebaedc5dec0714e288b769453f77023a69e9fdc98c58c441ac9a031`, oráculos `39992f97f74eff4593596a9996d4f5b78ecc7977ddd78e7abd3d9047fec9e604`, mutantes `697ee83f87d9ed749e142a2b27a33853a79c26a60afcd8ed1febebcf83c1856f`, harness candidato `f59cc94163c5be064f0bd9eea950e9c51d405e7cac0803fc8682b6532e9dec1b`, gate R2 `4a03fe7f419583bb91bd06ddb13df5d65969d2e7aaa32819048d328af171f3e5`, inventários R2 `2b1b6aad9cce0de4b3fb508037e03ad60188086ed36c0492e5d4ac92cce36f8b` e sentinelas R2 `a385b85a79a9e6349d365210e0ef0b5f6d08114ac5dc77af4633d21bd0b3c960`.

Os binários medidos permanecem vanilla ratificado `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f` e cristalino `8d9bee2a51d3a309c7459904e619972c608a311f6531c3956c1c635ad2e0bba4`. `git diff --name-only -- 01_core 02_shell 03_infra 04_wiring` retornou vazio: zero alterações em L1–L4.

Estado não commitado registrado antes da atualização dos quatro outputs:

```text
 M 00_nucleo/materialization/typst-passo-1356.md
 M lab/surface-inventory/src/main.rs
 M lab/typst-original/crates/p1140-inventory/src/main.rs
?? 00_nucleo/diagnosticos/p1356-contract.json
?? 00_nucleo/diagnosticos/p1356-delta-attribution.json
?? 00_nucleo/diagnosticos/p1356-harness-seal.json
?? 00_nucleo/diagnosticos/p1356-harness.py
?? 00_nucleo/diagnosticos/p1356-inventory-summary.json
?? 00_nucleo/diagnosticos/p1356-manifest.json
?? 00_nucleo/diagnosticos/p1356-matrix-summary.json
?? 00_nucleo/diagnosticos/p1356-mutants.json
?? 00_nucleo/diagnosticos/p1356-oracles.json
?? 00_nucleo/diagnosticos/p1356-parity-report.md
?? 00_nucleo/diagnosticos/p1357-harness.py
?? 00_nucleo/diagnosticos/p1357-inventory-summary.json
?? 00_nucleo/diagnosticos/p1357-mutants.json
?? 00_nucleo/diagnosticos/p1357-oracles.json
?? 00_nucleo/diagnosticos/p1357-parity-backlog.json
?? 00_nucleo/diagnosticos/p1357-report.md
?? 00_nucleo/diagnosticos/p1357-sentinels-summary.json
?? 00_nucleo/materialization/typst-passo-1357.md
```

## Gate do harness

A reexecução independente foi byte-idêntica ao recibo R2. Foram rejeitados 9/9 mutantes válidos, `mutation_score=1.0`, com quatro hashes iguais a `5cd99a6d78105cd165d9b10e62791e7c55c01c5fc68e41e30efbfde1304e01c7`. Esse score sela somente o gate abstrato do harness; não elimina as limitações observacionais nem equivale a fechamento de paridade.

## Inventários: correção confirmada

O recibo v2 contém quatro fases de oito execuções: `canonical-1`, `canonical-2`, `reverse-1` e `reverse-2`. Cada combinação produto/perfil ocorre quatro vezes; todas as 32 execuções têm exit 0 e `reason_code=null`. A dependência vanilla→cristalino é preservada dentro de cada perfil, enquanto a ordem dos perfis é invertida nas duas fases reverse.

Os oito envelopes têm identidades bilaterais e vetores de feature exatos. Contagens vanilla/cristalino: default 2021/2008; html 2251/2079; a11y-extras 2027/2011; html+a11y 2257/2082. A recomputação canônica dos envelopes produziu `c59de2cc6af95bb610a05ed7e57eb7d75a623834f81eb759baba72a8d6157ece`, igual aos quatro hashes reportados.

O crosswalk presença-only tem 8.724 pares únicos `(profile,path)`. Recomputar a classificação a partir dos bits de presença reproduziu: default 1966/55/42; html 2037/214/42; a11y-extras 1969/58/42; html+a11y 2040/217/42, na ordem `BOTH_PRESENT/VANILLA_ONLY/CRYSTALLINE_ONLY`. As somas reconciliam todos os oito `route_count`. O cruzamento com as listas P1356 revelou ainda três paths vanilla-only nos perfis a11y, `format.pdf.data-cell`, `format.pdf.header-cell` e `format.pdf.table-summary`; foram acrescentados a `TARGET_DRIFT` sem owner nem item de implementação. Em custo, a revisão passa de uma rodada bilateral para quatro, isto é, 32 invocações de enumerador; o ganho é evidência real de repetição e inversão, não duplicação declarativa.

## Sentinelas: 69 células e quatro hashes

Há 69 células: 60 `DIFFERENCE`, 8 `MATCH` e 1 `Unknown`. As 68 células executáveis, em duas ordens e duas repetições, recomputam quatro vezes `bb006d00fcaf814914a3c0952b2e2c38c3a2a6703a67663ed2aada080eb5192f`.

S04 confirma quatro matches de `luma(128)` e vinte diferenças nas demais formas. S10 produz 24 diferenças; S02 produz 12 diferenças e quatro `MATCH` de compilação top-level. Estes quatro últimos não encerram a obrigação semântica: o modo `compile` normaliza stdout vazio como valor e não mede o setting efetivo do documento.

### S07

A correção de proveniência é efetiva. Todas as quatro células S07 têm morfologia completa via `/usr/bin/qpdf --json`, exit 0, versão 11.9.0 e binário SHA-256 `10fc302c4ca9860f24b8d2cb7f8a4cc454ba59d4a91e7a8e40f6b2c229486df7`. O vanilla contém `/Span`; o cristalino não, portanto a diferença morfológica é bilateral e discriminatória.

O observável vertical também está separado e provenançado: `/usr/bin/pdftotext -bbox-layout`, exit 0, versão 24.02.0, SHA-256 `0fb98ea179e19154a90202608c164f2a319b79f16576fa6534b2d601033565e7`. As quatro linhas lone/control estão completas. Porém, o critério atual compara hashes de payloads que contêm textos diferentes (`Lone` contra `First`/`Second`); a desigualdade é inevitável sem depender da expansão vertical. Além disso, altura de página e posições Y do texto são efetivamente iguais e não há marcador posterior ou bbox do bloco que exponha os 100%. A instrumentação existe, mas o delta vertical ainda não é discriminatório.

### S11

S11 permanece corretamente fail-closed: `status=Unknown`, `complete=false`, `reason_code=PARSER_UNSUPPORTED`, `boundary_vector=null`, símbolo cristalino ausente e `clock_fallback_used=false`. Não há promoção a sucesso.

## Fila com ownership comprovado

A fila preserva as classes `PRODUCT_GAP`, `TARGET_DRIFT`, `HARNESS_LIMITATION` e `MATCH`. Apenas dois grupos entram em implementação, ambos com Prompt L0 existente e relação owner→consumer comprovada por header `@prompt` exato:

- `color.md` → `01_core/src/compiler/stdlib/foundations/color.rs`, 20 células S04;
- `shell/cli.md` → `02_shell/src/cli.rs`, 32 células S10/S02 CLI.

Os L0s devem ser atualizados antes de RED/código. Os 145 paths/492 células retidos do P1356 continuam listados integralmente. A lista de target drift preserva os 124 paths/494 células do P1356 e acrescenta os três paths/6 células de presença descobertos nos novos perfis, totalizando 127 paths, sempre sem implementação automática. S02 nested e a morfologia S07 também ficam fora da fila de código porque não foi comprovado um owner L0 único; nomes de pastas ou símbolos não serviram como atribuição.

## Gates proporcionais

- `python3 -m py_compile 00_nucleo/diagnosticos/p1357-harness.py`: exit 0;
- reexecução do gate e `cmp` com `/tmp/p1357-gate-r2.json`: exit 0;
- recomputação independente de envelopes, 32 execuções, crosswalk, 69 células, S07 e S11: todas as asserções passaram;
- `cargo fmt --all -- --check`: exit 0;
- testes do enumerador cristalino: 2/2;
- testes do enumerador vanilla: 2/2;
- `crystalline-lint .`: exit 0, somente warnings/infos existentes;
- `git diff --check`: executado após a escrita final.

## Continuidade

O próximo ensaio deve tornar o eixo vertical S07 discriminatório com fontes textualmente equivalentes e um marcador pós-bloco ou bbox do próprio bloco; deve também observar o setting efetivo no S02 top-level e procurar um adaptador binding-free para o vetor exato S11. O inventário e a morfologia S07 já fornecem evidência suficiente para continuidade sem repetir as correções validadas nesta revisão.

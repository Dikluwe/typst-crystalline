# L0 — math/layout/matrix
Hash do Código: 0431ed0b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/matrix.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/matrix.rs:21` locates `pub(super) fn layout_matrix(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Matrix carrier, alignment and augmentation

Consume complete MathMatrixElem and preserve rows/cells, optional delimiter
sides, alignment, augment, relative gaps and presence metadata.
Derive effective fields by explicit > math.mat StyleChain > defaults;
resolve gaps/stroke against outer style, while cells use shared denominator
style. Default row gap is 0.2em, column gap 0.5em and automatic augment
thickness 0.05em; these are language defaults, not measured PDF adjustments.

Without ampersands, honor start/left/center/right/end alignment. With
MathAlignPoint, split cells late into alternating right/left subcolumns.
Transfer natural class spacing at ampersand boundaries into the left cell's
width, not a normal comma-separated column gap. Preserve unequal split
counts, row order and empty cell morphology.
Physical direction limitations are not a new definition of start/end.

Use shared grid floors, row baselines and 1.1 delimiter target. Axis-center
only grid content before merging already-centered delimiters, with no
redundant horizontal padding. Final ascent/descent includes delimiters;
horizontal composition preserves all component baselines.

Augment accepts multiple horizontal/vertical offsets, preserves duplicates
and order, normalizes negative offsets from the end and accepts borders 0/n.
Horizontal offsets validate against row count, vertical against column count;
out-of-dimension offsets are diagnostic, not silently clamped or ignored.
Interior strokes lie at the corresponding gap midpoint; borders use the
full box boundary. Auto stroke uses Square cap; explicit/custom folded stroke
preserves thickness, paint and cap. The compatibility wrapper retains its
established Butt behavior and does not redefine public mat defaults.
Do not bake raster/PDF behavior into this owner.

Acceptance checks axis-specific relative gaps, asymmetric alignment, every
offset including negatives/borders/duplicates, invalid dimensions, automatic/
custom/folded strokes, optional delimiters and the complete Content::math_matrix
route. Existing consumer deviations remain pending diagnostics, not exceptions
to these contracts.

## Fluxo de controle e verificações executáveis — 2026-10-13

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. 01_core/src/compiler/math/layout/matrix.rs:543-549 contém o trecho medido. O inventário Clippy e a leitura dos owners distinguem código sem efeito, dados inválidos deliberados e verificações vazias. Fontes, L0, UTC, status, diffstat e comandos estão em `lab/results/construction/diagnostics/controle-fluxo-ci-2026-10-13/before.json.gz` e nas evidências do mesmo diretório. Não é nova medição de paridade nem inferência sobre intenção upstream.

### Decisão e aceitação

Substituir o teste local que apenas afirma true por um caso produtivo de matriz 2×2 sem delimitadores, com textos de comprimentos distintos, alinhamento à esquerda e gaps absolutos explícitos. Verificar preservação dos quatro textos em ordem de linhas/colunas, baselines compartilhadas por linha, aumento de coluna com gap e aumento da separação entre linhas com row-gap. As expectativas são relações geométricas do contrato, não valores copiados da implementação candidata. O teste deve detectar saída vazia ou perda/reordenação de células.

Preservar APIs públicas, comportamento padrão, fase e expectativas de linguagem existentes. Revalidar a suíte apropriada, Clippy, build e linhagem. Correção interna em fluxo contínuo ADR-0127.

## Helpers e armazenamento sem uso — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/math/layout/matrix.rs` nas linhas 21, 377, 378 contém os itens sinalizados no inventário Clippy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, referências, UTC, status e diffstat em `lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`. A pesquisa de chamadas e os caminhos produtivos precedem esta classificação; não é nova medição de paridade nem prova de desempenho.

### Decisão e aceitação

Compilar o helper interno `layout_matrix` somente sob `#[cfg(test)]`, conservando assinatura e corpo para os testes existentes. O despacho produtivo usa layout_matrix_elem. Preservar no wrapper de teste os defaults e o cap Butt históricos, sem transferi-los ao carrier produtivo. Remover apenas o armazenamento final dos campos privados GridGeometry.row_ascent e row_descent, nunca lidos. Conservar os vetores locais e todas as fórmulas de baseline/height em measure.

Preservar as rotas produtivas, nomes e corpos dos testes existentes, APIs externas, defaults e fase. Alteração interna em fluxo contínuo ADR-0127. Revalidar build e check de todos os targets do workspace, suíte core, inventário de testes, Clippy e linhagem; não eliminar expectativas para passar o gate.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C120`: `01_core/src/compiler/math/layout/matrix.rs:22`, `layout_matrix`, 8 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C120` — `layout_matrix`: Exceção pontual justificada

Compatibilidade exclusivamente cfg(test): o L0 manda conservar assinatura/corpo e defaults históricos do wrapper, enquanto produção usa layout_matrix_elem com carrier completo. Não refatorar a entrada produtiva por um aviso do helper de teste.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C120: Compatibilidade exclusivamente cfg(test): o L0 manda conservar assinatura/corpo e defaults históricos do wrapper, enquanto produção usa layout_matrix_elem com carrier completo")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### Limite da exceção

Autorizar somente `#[expect(clippy::too_many_arguments, reason = "...")]` no item
nominal acima, com o motivo arquitetural indicado. Não aplicar no módulo, impl,
crate ou configuração global; não elevar o limiar nem abranger outro lint.
Nos casos com remoção de argumento morto, a exceção residual depende dessa correção.
Se a função deixar de exceder o limiar, retirar/revisar a expectativa; não silenciar
`unfulfilled_lint_expectations`. Esta permissão específica supera proibições
genéricas de supressão em seções anteriores somente para este lint e os itens
nominais justificados aqui. As demais obrigações de correção permanecem.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C120. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.


## Exaustividade nominal e origem matemática — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `lab/results/construction/diagnostics/tekt-v16-math-layout-2026-10-17/before.json.gz` (snapshot inicial preservado em initial-ten-owners.json.gz). AST/spans em contexts.json/all-contexts.json/wildcard-inventory.json, parser pinado do CI. A suíte anterior passou no mesmo código de entrada, revalidado por hashes.

source-probe-before.json mede limits inline com largura 7.2 pt versus 12.912 pt ao acrescentar Source. source-family-before.json preserva a primeira matriz de 12 pares (11 diferenças). source-family-supplement.json e sua análise preservam 23 pares com métricas atribuíveis (19 diferenças e 4 controles iguais): limites, kerns, Op.text, acentos/dotless, suportes, alinhamento de matriz, espaçamento, sequências, stretch e classe. source-lines-before.json mede a mesma equação com 2 linhas versus 1 e última baseline 16.2 versus 0 sob Source, apesar dos mesmos extents. São observáveis de geometria, identidade extraível e metadados da equação, não igualdade de bytes/pointers/Rust. A transparência já é obrigatória no núcleo content-source-origin; a inspeção mostra os helpers classificando o wrapper em vez do body. Não se infere intenção histórica ou paridade global a partir desses números. O erro de contagem do analisador complementar (24 em vez de 23 pares) foi corrigido usando o stdout original válido, sem refazer a medição nem contar o erro como RED funcional.

### Prescrição exclusiva

Normalizar cell.source_body() na classificação de split_cell_on_align_point e item.source_body() na detecção do MathAlignPoint. Cada parte derivada conserva sua origem própria ou recebe a origem comprovada da célula pelo helper existente, nunca span fabricado. Fallback conserva cell.clone() inteiro. edge_node inspeciona o body Source por empréstimo para achar a primeira/última folha; assinaturas, células originais, plano, alinhamento, gaps e augment permanecem.

Trocar os fallbacks enumerados pelo complemento nominal atual dos braços **incondicionais**, sem alterar corpo/guard/ordem; variantes guardadas continuam no complemento para quando a condição falha. A projeção Option de attach é a única exceção estrutural: None retorna por ?, Some passa ao match nominal de Content normalizado, sem padrão profundo.

- **L16** — `01_core/src/compiler/math/layout/matrix.rs:533`, `split_cell_on_align_point`, Content: excluir somente `MathSequence, MathAlignPoint`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

### Aceitação e limites

L0 antes de código. Correção interna/paridade ADR-0127 em fluxo contínuo: sem campo/assinatura pública, trait, dependência, flag/default novo, compatibilidade ou fase nova. Grupos V19 compartilham o corpo anterior explicitamente justificado por caso; as notas permanecem visíveis e não provam cobertura de execução por alternativa. Não adicionar allow/expect, elevar limiar, capturar @ ou esconder um match em if-let/matches!. Somente a transparência e as reestruturações rasas aqui nomeadas alteram o código funcional; não editar fórmulas/constantes ou relaxar expectativas anteriores. RED funcional das regressões Source e RED estrutural dos avisos/sondas antes do reparo. Após, regressões GREEN, zero V16 dos locais emitidos, revisão futura dos 36 locais (incluindo a nova projeção de Content sob Option) por sentinelas Content/Value/MathStyleKind/FrameItem em workspace descartável com cfg(test) compilado. Os scrutinees/funções atuais e a composição ganham só as alterações descritas; revalidar ast/replay, assinaturas e demais owners.

Gates: suíte workspace completa, build, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito/reversa, SARIF diferencial completo e diff-check. V16 esperado 170→152; notas V19 novas somente dos grupos prescritos e do observador novo; V20 sem crescimento. As demais violações continuam pendentes. Fontes congeladas durante os gates; MemoryMax=12 GiB, swap zero, um job Cargo, comandos pesados sequenciais. Sem commit/push/release ou alteração do pin CI. Não alegar ganho de RAM, leitura de IA ou paridade integral por encerrar o lint.

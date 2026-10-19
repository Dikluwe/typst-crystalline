# L0 — compiler/stdlib/tests/shapes
Hash do Código: 0a06eda0

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/shapes.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own only the shapes regression tests and their local fixtures/assertions. Use the private shared support fixtures. Preserve arguments, diagnostic messages, spans and language-level observations. Expectations must be independent of candidate output. Do not implement namespace behavior or transfer production contracts into this test consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/core/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Adaptar apenas construção dos argumentos Value Stroke pelo From existente; conservar comparação da geometria Stroke própria dos shapes e todas as expectativas.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.

Ratificação Memória 5: o dono confirmou o manifesto SHA-256
6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237
com “confirmo” em 2026-10-08. O gate ADR-0127 deste pacote está satisfeito;
suas cláusulas propostas passam a vigorar nesta materialização.

## Contornos com Paint e dicionário uniforme — 2026-10-14

### Medição anterior à decisão

before/results.json.gz no diagnóstico stroke-paint-2026-10-14 conserva HEAD,
working tree, diffstat, UTC, fontes/owners, comandos e hashes dos dois binários.
layout/stroke.rs:53-63 limita named paint a Color; shapes/paint.rs:45-64 entrega
os demais paints ao caster legado que só aceita Length/Color/Stroke;
shapes/rect.rs:40-43 descarta todos os Dict. O controle uniforme renderiza 1pt
preto em Tekt; named Gradient/Tiling/Auto e circle Gradient são rejeitados.
O binário vanilla pinado 9f2b6e871 aceita esses valores. Dict vazio desenha
contorno preto em Tekt e nenhum contorno no vanilla. A fonte pinada stroke.rs:
236-243 e sides.rs:238-258 distingue Paint smart, dict vazio e dict uniforme.
A medição de square(size:...) falha por dimensão no Tekt e não prova cast de
stroke; suas dimensões e defaults permanecem abertos, sem mudar este contrato.

### Responsabilidade e aceitação

Adicionar RED→GREEN para paints Gradient/Tiling diretos, dicionários uniformes
ricos em rect/square, dicionário vazio sem fallback e dicionário uniforme inválido.
Verificar Paint, espessura, cap e ausência real; manter controles de omissão e
none existentes. Não usar os dicionários por-lado como evidência de fechamento.

Correção interna de paridade ADR-0127 em fluxo contínuo: L0 primeiro, RED→GREEN,
resselo restrito, build, suíte e linhagem. Aceitação preserva significado da
pintura e sua forma disponível; igualdade binária com vanilla não é requisito.
Recolher PDF/raster e erros, conservando as divergências não corrigidas.

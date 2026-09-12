# Prompt L0 — `wiring/tests/p1292_contract` — oráculos black-box
Hash do Código: e208b1f7

**Camada:** L4 — teste de integração
**Ficheiro alvo:** `04_wiring/tests/p1292_contract.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129
**Vanilla ratificado:** `a51e02804`

## Responsabilidade

O consumer executa o binário como caixa-preta. Usa diretórios temporários
isolados, propaga stdout/stderr em falha, não lê a implementação e nunca converte
`Unknown` em sucesso.

JSON, SVG, PDF e bbox são transportes mecânicos. Os observáveis são identidade
pública, morfologia, dimensões, posições, presença e ordem.

## Lote A — superfície preservada

Verifica callbacks, `cross`, background, spans, ausência de documento
provisório e suas formas públicas. Mudanças no harness não autorizam alterações
nessas expectativas.

## Lote B — identidade e geometria SVG

`math.underline == underline` é avaliado diretamente em JSON e deve resultar
no boolean `false`, nunca na string `"false"`.

Fixtures geométricas usam página automática. O parser extrai dimensões da raiz
SVG e regras visíveis, sem comparar bytes completos, IDs de glyph ou ordem de
`defs`. A baseline de um glyph é global: o observador percorre grupos
ancestrais e acumula `translate(...)` e `matrix(...)` na ordem afim do
documento antes de comparar a coordenada.

A composição é genérica; não seleciona por caractere, fonte, número de fixture
ou constante sentinela.

## Lote C — `measure` contextual

Cada caso materializa o resultado de `measure` como shape de cor única dentro
de `context`, compila para SVG e extrai as dimensões do shape. Não usa
`query`, metadata ou root como substituto da execução contextual.

Região finita e `height:auto` são casos separados. Delimitadores, alinhamento,
repr, presença/default e erros permanecem cobertos. O controle contextual deve
emitir o shape antes de se avaliar a feature.

## Lote D — `place.flush` multipágina

O teste compila PDF e observa página, bbox e ordem de tokens textuais únicos.
Ausência de `pdftotext` é erro explícito. Tokens de observação não participam
do fluxo e devem aparecer uma única vez.

O contrato cobre prefixo/sufixo, controle sem flush, floats top/bottom, nesting
no `Layouter` ativo e no-op sem floats. Não compara bytes PDF, timestamps,
compressão ou ordem interna de operadores.

## Aceitação

- transformações SVG ancestrais são aplicadas uma única vez;
- controles provam os transportes JSON, SVG e PDF antes das features;
- expectativas semânticas não são adaptadas ao candidato;
- temporários são removidos;
- o prompt legitima somente `04_wiring/tests/p1292_contract.rs`.

# Prompt L0 — `infra/integration_tests` — suíte E2E L3
Hash do Código: a2a64470

**Camada:** L3, somente `#[cfg(test)]`
**Ficheiro proprietário:** `03_infra/src/integration_tests.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Responsabilidade

A suíte exercita o pipeline real por fronteiras que mocks de L1 não cobrem:
`SystemWorld`, filesystem temporário, eval, introspecção, layout e export. Ela
legitima somente o harness e os testes do consumer; módulos produtivos mantêm
seus próprios owners.

Helpers test-only criam e removem diretórios temporários, avaliam fontes,
atravessam frames e compilam documentos. I/O e world concreto são permitidos
neste consumer L3 e não se transferem para L1.

## Observáveis

Testes podem verificar valores de eval, morfologia e layout, warnings,
diagnósticos e estruturas observáveis dos formatos exportados. Igualdade de
bytes ou estrutura interna só é contrato quando ela própria é o observável do
formato.

## Coordenadas globais de `FrameItem`

Helpers que comparam geometria carregam a transformação afim acumulada do
ancestral até o item:

- ao entrar em `Group`, compõem a translação de `Group.pos`, a
  `Group.matrix` e o transform ancestral na mesma ordem do exporter;
- grupos aninhados compõem transitivamente;
- pontos de `Text`, `TextShaped`, `Glyph`, `Image` e `Shape`, além dos
  extremos de `Line`, são projetados uma única vez no referencial global;
- `Semantic` é envelope transparente e `Link` preserva sua convenção.

Travessias que apenas contam ou classificam itens podem observar variantes
locais. Toda asserção geométrica usa coordenadas globais. É proibido compensar
o transporte alterando expectativas, fixtures ou layout produtivo.

## Aceitação

- o módulo só compila sob `cfg(test)`;
- helpers permanecem privados ao harness;
- temporários são removidos;
- testes passam sem mudar contratos produtivos para acomodar mecânica do
  observador.

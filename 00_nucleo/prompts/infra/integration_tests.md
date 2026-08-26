# Prompt L0 — `infra/integration_tests` — suíte E2E L3

Hash do Código: 841aa746

**Camada:** L3, somente `#[cfg(test)]`
**Ficheiro proprietário:** `03_infra/src/integration_tests.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Responsabilidade

Esta suíte exercita o pipeline real por fronteiras que mocks de L1 não cobrem:
`SystemWorld`, filesystem temporário, eval, introspecção, layout e export. Seus
testes integram módulos produtivos já especificados por seus próprios owners; este
L0 legitima apenas o harness e os testes no consumer proprietário.

Helpers test-only constroem e removem diretórios temporários, criam um
`SystemWorld`, avaliam fontes, atravessam frames e compilam documentos até os
artefatos de export. Filesystem, relógio e world concreto são permitidos aqui por
se tratar de consumer L3 test-only; essa permissão não se transfere para L1.

## Observáveis e paridade

Os testes podem verificar valores produzidos por eval, morfologia e layout,
warnings e mensagens de erro quando são observáveis, além da validade e das
estruturas observáveis dos formatos exportados. Segundo ADR-0107/0108, igualdade
acidental de bytes, estrutura interna ou passos do algoritmo não vira contrato,
exceto quando bytes ou estrutura são precisamente o observável do formato sob
teste. Toda medição decisória registra proveniência reproduzível.

## Proveniência P844

P844 acrescentou os testes `p844_a1_...` a `p844_a8_...` e os helpers
`p844_expand_plain_text`/`p844_expand_errors`, seguindo P506/P821. O lote cobriu
query→content, seletor por função de elemento, `state.at`, `state.final`,
`counter.final`, `counter.at(Location)`, repr de array em `#context`,
`counter.display` com numbering do `#set` e pattern real, além da sonda de
`#context` entre headings via expansão e reintrospecção. Essa proveniência não
pretende enumerar ou esgotar a suíte vigente.

## Aceitação estrutural

- o módulo é compilado somente sob `cfg(test)` pela raiz da crate;
- helpers permanecem privados ao harness;
- os testes selecionados da suíte compilam e passam sem alterar contratos
  produtivos para acomodar mecânica de teste.

## Fora de escopo

Não pertencem a este owner os contratos dos módulos produtivos exercitados, unit
tests internos desses módulos, fixtures externas ou comportamento novo do produto.
Cada correção funcional descoberta pela suíte exige seu próprio L0 e classificação
ADR-0127 antes de alterar produção.

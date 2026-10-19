# Gate concreto — warnings CLI, 2026-10-06

Status atual: confirmado pelo humano (confirmo), aplicado e materializado.
Proposta original e hashes dos candidatos abaixo preservados como histórico;
recibo: warnings-cli-aprovacao-2026-10-06.json.
Validação e refinamento medido da ordem HTML: ../diagnosticos/warnings-cli-execucao-2026-10-06.md.

A coleta `00_nucleo/diagnosticos/plano-debitos-2026-10-06/before.json.gz`
comprova diferenças de ordem e hint. A ADR-0045, adendo P1139, §8 conserva
expressamente warnings → errors. O pin atual drena errors → warnings.
Substituir essa ordem requer decisão nova; não tratar a política anterior como
bug sem decisão. A aprovação geral do plano autoriza preparar este pacote,
não constitui aprovação de uma assinatura pública ainda não apresentada.

A proposta substitui a ordem nos comandos compile/query pelo comportamento
medido e adiciona em L2:

```rust
pub fn query_deprecation_hint(intent: &QueryIntent) -> String;
```

A função produz a sugestão específica de eval a partir de selector/field/one e
input, sem I/O. O wiring compõe/draina, sem receber algoritmo de apresentação.
Query stdin usa a API for_stdin já existente para preservar sua Source.

Os quatro L0 candidatos completos, seus owners exclusivos e hashes base/
candidato estão em `warnings-cli-2026-10-06/manifest.json`. São cópias para
revisão, não L0 vigente nem consumers adicionais. Na aplicação substituir as
cláusulas conflitantes do contrato antigo pelas ratificadas, conservando
história nos diagnósticos e mantendo os outros gates pendentes intactos.

Confirmar o pacote autoriza: atualizar o adendo §8 da ADR-0045 para erros antes
dos avisos nos comandos finitos medidos; guardar L0s ratificados; testes
RED→GREEN; materializar a função pública e a composição; selar apenas owners
alterados e revalidar. Não autoriza API de watch, nova fase matemática,
NodeKind novo, unsafe ou ativação de propostas de PDF/bundle pendentes.

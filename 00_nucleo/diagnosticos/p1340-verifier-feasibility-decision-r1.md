# P1340 — decisão de viabilidade e primeiro gate afetado

**Veredito: PAUSA_ADR0127_POLITICA_DE_INCONCLUSIVIDADE.** Não é exigência de API nova, não é falha de compilação e não autoriza implementar um erro de produto arbitrário. Nenhum selo/GO produtivo emitido.

Regime: executado sem atestação de isolamento. Verificador `/root/p1311_review`, sem escrita de produção, L0, contrato ou oráculos. Antecedente: `p1340-baseline.json` SHA-256 `0869202e774bd7b76278365aac7292d45a1c930be42cf83270845c985cb5000a`; HEAD `2f42d64253547734564513a1159ee6b584c1c4b4` mais working tree congelada. Esta decisão resulta de inspeção estática, sem novo build, teste ou matriz.

## O que a auditoria adversarial resolve

`p1340-adversary-audit-r1.md` oferece uma via plausível pela API existente: validar a tentativa contra o snapshot efetivamente lido, com seus recursos preservados. O código de state/query clona carriers armazenados; CounterFold produz Locations/inteiros; portanto a hipótese anterior de state recriar a closure no replay não sustenta exigir uma API nova. A mesma folha opaca nunca se compara como Same. Autorreflexividade positiva pode certificar a precondição de fechamento do registro, condicionada a replay completo/determinístico e preservação dos recursos.

Isso não certifica automaticamente toda a implementação: o adversário publicou A01–A08 como causas prospectivas, não mutantes executados. Não foram contados como rejeitados. A discriminação P1339 continua evidência histórica, sem demonstrar sozinha esses novos ataques à coordenação.

## Lacuna que permanece

O L0 novo `infra/pipeline/context_stabilization.md` conserva literalmente as cláusulas anteriores. Determina que incompletude não certifica o documento, identidade ambígua invalida a tentativa e que limite interno/observabilidade insuficiente não pode receber warning vanilla de não convergência. Ao mesmo tempo, sua cláusula final determina D5/I4 no teto sem erro final, enquanto erro final impede exportação. Não define qual resultado produtivo devolver quando o corpo não errou, mas sua estabilidade permanece não comprovável. `SourceResult<PagedDocument>` não fornece variante inconclusiva.

`compiler/eval.md` permite que o próprio mecanismo de validação devolva `Err(SourceDiagnostic)`; não determina que toda autorreflexividade falsa seja essa falha. A implementação atual devolve `Ok(false)` para Different e Unproven, e diagnosis pode devolver vetor vazio. Não é lícito inventar causalidade ou erro do corpo a partir disso. A existência de um canal Err não escolhe a política pública do orquestrador.

Logo, autorreflexividade pode demonstrar o ramo positivo, mas não elimina a necessidade de uma decisão para seu ramo falso. Considerá-la somente precondição dos testes e excluir a opacidade na aplicação produtiva reduziria F09/A01 depois do congelamento. Exportar D5 a despeito dela violaria a não certificação; acrescentar warning linguístico seria igualmente incorreto. Ausência de fixture CLI simples não prova impossibilidade de entrada pela extensão Rust nem elimina a obrigação congelada.

## Menor escolha a submeter ao dono

Definir somente o resultado observável quando a estabilização não pode ser comprovada por opacidade/incompletude/identidade ambígua, distinto da não convergência linguística comprovada:

- Opção mínima proposta para decisão, ainda não autorizada: impedir exportação usando o canal existente `Err(SourceDiagnostic)`, com diagnóstico próprio de incapacidade de comprovação; fixar mensagem, âncora, momento, preservação de erros anteriores e destino dos sinks no L0/contrato antes de código. Isso pode dispensar enum ou assinatura pública nova.
- Se o dono não quiser esse erro de produto, precisa escolher expressamente outra política observável compatível ou reduzir legitimamente o domínio reivindicado. O verificador não escolhe exportação, warning, loop ilimitado ou silêncio como fallback.

O primeiro gate afetado é a intenção/política terminal do novo owner L0, antes do selo e da implementação P1340. Após a escolha: L0 primeiro; sucessor contratual estreito preservando predecessores; focais de autorreflexividade fechada, mudança comprovada sem detalhe e opacidade mista A01/A02; discriminação real pertinente; então selo e implementação. Nenhum reset de orçamento nem repetição de corpora globais é exigido por esta constatação. A transferência literal, baseline e RED local permanecem evidências válidas do que mediram; não constituem autorização da política faltante.

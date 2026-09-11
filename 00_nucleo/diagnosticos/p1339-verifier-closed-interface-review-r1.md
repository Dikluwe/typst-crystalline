# P1339 — auditoria da interface prospectiva closed_state

Verificador `/root/p1311_review`; executado sem atestação de isolamento.
Entrada: `p1339-mutant-closed-state-interface.md`, SHA-256
`7ff05129bd9213ffbe1a9c72b403262559834c782bb9f10928ea7d20bfd02f8d`.
Autoridade estreita: `p1339-closed-harness-authority.json`, SHA-256
`08d0be7f19894c111130d0c791df6804cd1e7d89d2b8d6f418702be88af2125d`.

A interface foi lida integralmente. Os entrypoints `eval_expr`, `Engine` e
as duas entradas de introspecção foram confrontados com a fonte baseline
indicada e correspondem às assinaturas. A distinção entre walk puro e runtime
é necessária: pós-processadores não podem ser simulados por um snapshot vazio.
A preservação do Result do corpo separadamente da validação também é necessária.

Aceitável como proposta mecânica, **insuficiente para congelar cobertura**:

- Os três métodos públicos previstos observam seleção, validade e diagnósticos.
  Não distinguem por si mesmos as variantes privadas Different e Unproven.
- F06 precisa binding test-only separado para a operação privada real usada
  pela validação. O adapter não pode implementar relação própria ou converter
  arbitrariamente `false` em Unknown. Inputs/asserts vêm do autor independente.
- O inventário de Value/Content/Args/Selector e metadados causais deve ser
  exaustivo; cada variante e campo exigido precisa teste concreto ou testemunha
  estrutural admitida pelo contrato, nunca wildcard que devolva Same.
- F07/F08 exigem observações da identidade/geração retida, invalidação dos
  descendentes e sequência real de tentativas. Resultado booleano ou contador
  implementado no próprio fixture não é testemunha desse ciclo.
- Sources, spans, recursos, bindings compartilhados, snapshots e overlays
  precisam de definições e predicados fechados antes do selo; não podem ser
  escolhidos após observar o candidato.

Nenhum harness ou teste interno foi executado por este recibo; zero crédito
pré-selo e nenhum GO. O contrato sucessor e os artefatos completos ainda
precisam ser pinados e auditados. Os F devidos ao candidato só são satisfeitos
por compilação/execução real final, sem mock, fallback ou alteração semântica
dos templates depois do selo.

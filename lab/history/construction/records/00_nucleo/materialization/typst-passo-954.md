# Passo 954 — localizar a origem da decisão "verboso primeiro, compacto depois, acessibilidade em eixo separado" e verificar colisão de ADR

**Precede este passo**: decisão mencionada pelo dono como "nova descoberta, ordem de prioridade que
acabamos de fechar" — modo verboso primeiro (auditoria), modo compacto depois (validado pelo
decalque contra o verboso), acessibilidade como eixo separado. **Antes de escrever qualquer ADR
nova, confirmar onde exatamente esta decisão foi tomada e se já não colide com algo existente.**

---

## Fase A — localizar a origem exata da decisão

1. Procurar nos relatórios de P950-953 (e, se necessário, nos prompts que os precederam) o ponto
   exato onde esta ordem de prioridade foi decidida ou implicada — não presumir que foi um passo
   só; pode ter emergido da combinação de decisões em vários pontos (por exemplo, P953 decidindo
   manter `BDC`/`EMC` como scope-out separado, mais alguma decisão sobre ordem de verificação em
   P944-949).
2. Se a decisão não estiver registada em nenhum relatório de passo com essa formulação exata:
   confirmar se foi uma decisão tomada directamente na conversa com o dono, fora do ciclo normal
   de passo — nesse caso, isso também é informação relevante para a spec/ADR (registar a
   proveniência real, não inventar um passo de origem que não existe).
3. Documentar a citação exata (ou a paráfrase mais próxima) e o contexto em que apareceu, antes de
   escrever a ADR — a ADR deve referenciar a origem real, mesmo padrão de proveniência já exigido
   em toda esta frente.

## Fase B — verificar colisão com ADRs existentes

1. Varredura de `00_nucleo/adr/` por conteúdo relacionado a: verificação/attestation (`L11`,
   `ADR-0119` disciplina de verificação), ordem de operações em passos de investigação (`ADR-0114`
   sonda antes da spec, `ADR-0117` sonda A.0), e qualquer ADR já existente sobre modo verboso vs
   compacto de exportação/verificação — não presumir que não existe nada parecido.
2. Se encontrar algo relacionado mas não idêntico: confirmar se a decisão nova é uma extensão de
   uma ADR existente (nesse caso, talvez não precise de número novo, só uma seção nova na ADR
   existente) ou é genuinamente uma decisão nova e distinta.
3. Confirmar o próximo número de ADR livre por varredura real do directório — não assumir, mesmo
   erro já cometido antes nesta conversa (`ADR-0112` colidiu, corrigido em P910).

## Fase C — decisão final

1. Com a origem confirmada (Fase A) e a ausência de colisão confirmada (Fase B): decidir se a
   decisão merece ADR nova, extensão de ADR existente, ou registo mais leve (nota em L0, sem ADR).
2. Se for ADR nova: escrever seguindo o formato já estabelecido neste projecto (contexto, decisão,
   alternativas consideradas, consequências), citando a origem real confirmada na Fase A.

## Resultado esperado

- Origem exata da decisão confirmada e citada, não presumida.
- Veredicto de colisão com ADRs existentes, com número de ADR confirmado por varredura real se
  for o caso de precisar de um novo.
- ADR (ou extensão, ou nota) escrita só depois das duas confirmações acima, não antes.

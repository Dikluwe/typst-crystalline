# Passo 975 — módulo oráculo de paridade de operador (flag `--oracle-report`, gera PDF + relatório)

**Precede este passo**: pedido do dono — um módulo oráculo para paridade de **operador** (não de
geometria/posição, isso já é o `math_oracle.rs` de P969), activado por flag, que gera o PDF normal
e, junto, um relatório de paridade. Desenho abaixo, porque o pedido veio sem forma definida — a
motivação real é evitar repetir o que aconteceu com o achado de P968 (74% do texto com `Tr 2`
indevido): uma regressão de vocabulário de operador que só foi descoberta por auditoria externa
manual, contando operadores um a um no content stream. Este módulo deveria detectar esse tipo de
coisa **automaticamente**, sem depender de auditoria externa.

**Pré-condição de árvore**: `git status`. Confirmar P974 presente.

---

## Fase A — desenhar o oráculo de operador (gate obrigatório — nova superfície de CLI/flag)

1. **Distinguir de P969**: o `math_oracle.rs` verifica fórmulas de *posição* em teste, nunca
   compilado em produção. Este módulo é sobre *vocabulário de operador* do content stream
   (`Tm`/`Td`, `Tr`, `cs`/`scn`, `q`/`cm`/`Q`, `BDC`/`EMC`), roda **em produção**, activado por
   flag, e produz artefacto visível ao utilizador (o relatório).
2. **Desenhar os checks concretos**, cada um com a expectativa clara (o que o modo verboso, por
   ser o padrão desde P956, deve produzir sempre):
   - Proporção de blocos `Tr 2` (contorno/faux-bold) — sinalizar se muito acima da proporção real
     de conteúdo marcado como negrito no documento (o check que teria apanhado P968 sozinho, sem
     esperar por auditoria externa).
   - Consistência `Tm`/`Td` com o modo activo (verboso ⇒ 100% `Tm`; compacto ⇒ 100% `Td`) —
     sinalizar qualquer mistura inesperada.
   - Presença de `cs`/`scn` por bloco de texto no modo verboso (deve ser 100%, exceto onde a cor
     realmente não muda — decidir o critério exacto).
   - Balanceamento `q`/`cm`/`Q` (todo `q` tem `Q` correspondente, sem aninhamento quebrado).
   - Contagem de `BDC`/`EMC` (deve ser 0, dado o scope-out actual de `ADR-0126`) — sinalizar se
     deixar de ser 0 sem uma decisão explícita de promover acessibilidade.
   - Confirmar com o dono se há mais checks a incluir, ou se este conjunto inicial basta para a
     primeira versão.
3. **Decidir o formato do relatório**: ficheiro separado ao lado do PDF (`nome.oracle.json` ou
   `.txt`), ou impresso no stdout/stderr durante a compilação — confirmar preferência.
4. **Decidir o nome e comportamento da flag** (`--oracle-report`, ou nome melhor) — confirmar se
   gera sempre relatório quando activa, ou se só sinaliza quando encontra algo fora do esperado
   (silencioso se tudo estiver correcto).
5. **Confirmar onde este código vive** — provavelmente `03_infra/src/export/` (lê o content stream
   já emitido) ou um módulo novo `03_infra/src/oracle/` — decidir sem misturar com o exportador em
   si (o oráculo lê o que foi escrito, não decide o que escrever).
6. Editar L0s, sincronizar hashes, **parar para confirmação do dono antes da Fase B** — flag nova
   de CLI é superfície pública, per `ADR-0127`.

## Fase B — Implementação (protocolo de dois agentes de P898 — nova funcionalidade de produção,
não só teste)

1. Agente A escreve testes cobrindo: documento sem negrito genuíno (deve dar 0% `Tr 2`, guarda
   directa do bug de P968 — se este check já existisse, teria apanhado sozinho); documento com
   negrito genuíno (proporção correcta); modo compacto (100% `Td`, sem sinalizar isso como
   problema); `q`/`cm`/`Q` desbalanceado forçado artificialmente (para confirmar que o check
   detecta).
2. Agente B implementa os checks desenhados na Fase A.
3. Revisão do orquestrador — rodar o oráculo contra o documento de 30 secções actual e confirmar
   que não sinaliza nada (já que P958-974 corrigiram tudo que a auditoria externa achou) — se
   sinalizar algo, investigar antes de fechar o passo.
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Confirmar que a flag funciona end-to-end: `typst compile --oracle-report doc.typ` produz o PDF
   normal mais o relatório, sem a flag o comportamento é inalterado (custo zero quando desactivada).
2. Documentar a flag (`--help`, L0 de CLI).
3. Benchmark: confirmar que ter a flag disponível (mas desligada) não custa nada ao caminho normal;
   medir o custo de ativá-la separadamente (é esperado ter algum custo, já que lê o content stream
   de novo — não precisa de ser zero, só não pode ser o caminho por defeito).

## Resultado esperado

- Módulo oráculo de operador, activado por flag, produzindo PDF + relatório de paridade.
- Checks cobrindo pelo menos os cinco itens da Fase A.2, com pelo menos um teste que reproduz e
  captura o padrão do bug real de P968.
- Rodado contra o documento de 30 secções actual, confirmando zero achados (estado já corrigido).
- Custo zero quando a flag está desligada.

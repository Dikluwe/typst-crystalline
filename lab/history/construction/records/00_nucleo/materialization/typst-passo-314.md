# Tarefa P314 — As duas ADRs do modelo de elemento + conserto do imposto de hash (L0 fino)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P314 (confirmar que está livre: nenhum `typst-passo-314*`
nem diagnóstico/ADR o consumiu; se consumido, usar o próximo e registrar).
**Tipo**: materialização de **processo e linhagem** — duas ADRs novas + fatiar
prompts L0 grossos + re-apontar headers de linhagem. **Zero mudança de lógica
em qualquer `.rs`** (o diff de código permitido é exclusivamente as linhas
`@prompt` / `@prompt-hash`).
**Fonte de decisão**: `00_nucleo/diagnosticos/diagnostico-modelo-elemento-passo-313.md`
(§2.1 o imposto de hash medido; §5 recomendação primária; §6 o texto do
princípio) + decisão do dono do projeto (2026-06-10): **recomendação primária
aceita, com o conserto do imposto de hash puxado para antes do D**.
**Pastas restritas**: `00_nucleo/materialization/` e `00_nucleo/context/` não
autorizadas.

---

## O que este passo entrega (e o que não)

Entrega: (1) ADR "Atomicidade para agentes"; (2) ADR "Modelo de elemento:
D incremental agora, F como destino"; (3) os prompts L0 finos substituindo
`rules/stdlib.md` e `rules/math/layout.md`; (4) headers re-apontados e hashes
regenerados; (5) a medição antes/depois do imposto.

NÃO entrega: o trait `Element` ou qualquer código do D (é o P316); o conserto
das 3 violações V9 pré-existentes em `03_infra` (registradas no mapa de
migração; fora de escopo — não introduzir novas é o critério aqui); fatiar
outros prompts grossos não medidos (se a varredura revelar candidatos com
acoplamento parecido, **listar no relatório**, não fatiar).

---

## FASE A — Redação (a IA redige; nada de código ainda)

### A.1 — ADR "Atomicidade para agentes"

Número: o próximo livre em `00_nucleo/adr/` (o diagnóstico 313 cita
ADR-0102/0103 como recentes; confirmar no diretório). Conteúdo: o texto do
§6 do diagnóstico 313 como base — as três cláusulas (hubs concentradores são
anti-padrão de manutenção por IA; acoplamento por linhagem deve ser fino;
verificação mecânica não pode depender de o agente lembrar) e o corolário de
sequência (medir custo-por-elemento como métrica de saúde). Pode refinar a
redação; não pode enfraquecer as cláusulas. Citar: o requisito do dono
(transcrito no §0 do 313), as medições que o motivaram (29 ficheiros do
P311b, 18 de puro hash; `content.rs` 5782 linhas / 77 variantes), e a falha
F4 do diagnóstico de bloqueio como o precedente da cláusula 3.

### A.2 — ADR "Modelo de elemento: D agora, F como destino"

Número: o seguinte ao A.1. Conteúdo mínimo:

- **Decisão**: adotar o candidato **D** (enum fino com delegação por módulo —
  variante `Nome(Arc<nome::Nome>)`, lógica em `entities/elements/`, matches
  do hub viram dispatchers) de forma **incremental por lotes**, absorvendo
  `ElementPayload` no trait (fecha a tripla-definição do locatável, §1.2 do
  313). Declarar **F** (propriedades reificadas / PropMap) como **destino**,
  a ser executado **junto com o DEBT da StyleChain (sucessor 99.E)** — porque
  A ≡ F (verificado no §4 do 313: `Style`/`StyleDelta` hardcodam 10
  propriedades à mão; a PropMap serve elemento e estilo de uma vez).
- **Trava gravada**: F não começa sem repor a verificação mecânica que o
  compilador deixa de dar (teste-que-varre-a-tabela ou regra nova do
  crystalline-lint) — erro-de-compilação não vira erro-de-runtime silencioso.
- **Compatibilidade gravada**: a forma do `impl Element` do D deve nascer
  compatível com virar descritor do F (os módulos do D são o continente que
  F preenche — §4 do 313).
- **Descartes com razão**: E (geração por macro) — viola a ADR do A.1
  (código gerado é opaco para agentes; resíduo `__ComemoCall` medido);
  F-direto-agora — risco e custo L+ sem a StyleChain agendada.
- **Sequência**: P314 (este) → P316+ (D por lotes; primeiro lote decidido no
  arranque do P316) → F com 99.E.
- **Relação com ADR-0026**: esta ADR **complementa** (o enum fechado
  permanece; muda a morada da lógica por variante), não revoga. Dizer isso
  explicitamente para o leitor futuro.
- Adicionar uma linha de referência cruzada no DEBT da StyleChain
  (`debt-stylechain-*.md` / 99.E): "ver ADR-XXXX — F resolve este DEBT por
  construção; sequência gravada lá".

### A.3 — Os prompts L0 finos (a partição, sem spec nova)

Alvos medidos (§2.1 do 313): `00_nucleo/prompts/engine/stdlib.md` (linhagem de
~10 ficheiros `stdlib/*.rs`) e `00_nucleo/prompts/engine/math/layout.md`
(~8 ficheiros `math/layout/*.rs`) — confirmar os paths reais no repositório.

Regras da partição:

1. **Content-preserving**: a união dos prompts novos = o prompt velho. Nada de
   especificação inventada, nada de especificação perdida. Se uma regra do
   prompt velho é partilhada por vários elementos, ela vai para um
   `_comum.md` da área (`stdlib/_comum.md`, `math/layout/_comum.md`), citado
   pelos prompts finos — não duplicada em cada um.
2. **Um `.rs` → um prompt**: cada ficheiro de código alvo passa a apontar
   para exatamente um prompt fino (o do seu elemento/função). Ficheiros
   genuinamente partilhados (ex.: `mod.rs` de registo) apontam para o
   `_comum.md` da área.
3. **O prompt velho não some silenciosamente**: vira um índice de uma página
   listando os finos que o substituem ("fatiado em P314, ver ADR do A.1"),
   para qualquer linhagem antiga encontrada depois ter trilha.
4. Produzir a **tabela de mapeamento** ficheiro `.rs` → prompt novo (vai no
   relatório e numa seção do índice do item 3).

### A.4 — CHECKPOINT obrigatório (protocolo de nucleação, passo 2)

**Parar aqui.** Apresentar ao humano: as duas ADRs, os prompts finos, o
índice e a tabela de mapeamento. **Só prosseguir para a Fase B quando o
humano confirmar que guardou os L0 e está de acordo** — é a Trava
Arquitetural do `CLAUDE.md` (a IA redige o L0 e para).

---

## FASE B — Aplicação da linhagem (após confirmação humana)

1. Re-apontar a linha `@prompt` dos `.rs` alvo conforme a tabela de
   mapeamento do A.3. **Nenhuma outra linha de código muda.**
2. `crystalline-lint --fix-hashes .` para regenerar os `@prompt-hash` (a
   solução documentada da V5).
3. Validação:
   - `cargo build` — verde.
   - `cargo test --workspace` — mesma contagem de antes (registrar; nenhum
     teste novo nem removido é esperado).
   - `crystalline-lint .` — **nenhuma violação nova**; as 3 V9 pré-existentes
     de `03_infra` continuam (registrar que continuam, fora de escopo).
   - `git diff --stat` dos `.rs`: **apenas** linhas `@prompt`/`@prompt-hash`.
     Qualquer outra mudança em `.rs` é bug deste passo — reverter.

4. **A medição que fecha o passo** (o critério de aceitação do conserto):
   simular a edição que o P311 fez — tocar **um** prompt fino de elemento
   (mudança trivial, ex.: espaço em comentário), rodar `--fix-hashes`, contar
   quantos `.rs` re-hasheiam. **Esperado: ≤ 2** (o ficheiro do elemento; +1
   se `_comum.md` for tocado, o que esta simulação não faz). Reverter a
   mudança trivial depois. Registrar: **antes 18 ficheiros → depois N**.

---

## Relatório (arquivo do passo + resumo no chat)

Arquivo `typst-passo-314.md` na convenção do repositório, e no chat:

- Números das duas ADRs criadas.
- A tabela de mapeamento (quantos prompts finos por área).
- A medição antes/depois do imposto de hash (18 → N).
- Validação: build/testes/lint (com a nota das 3 V9 pré-existentes).
- Candidatos a fatiamento futuro encontrados e **não** fatiados (se houver).
- `git status` final.

## Restrições finais

- A Fase B não começa sem a confirmação humana do checkpoint A.4.
- Zero mudança de lógica; zero conserto oportunista (nem as V9, nem refactor
  de passagem).
- Toda contagem com o comando registrado; nada estimado de memória.

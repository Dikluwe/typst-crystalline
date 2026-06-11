# Tarefa P317 — Zerar as V9 + Lote 2 do modelo D (família math) + o modelo de lote reutilizável

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P317 (confirmar livre; registrar se divergir).
**Tipo**: (Pre) conserto de violações do linter em L3 + (Lote 2) continuação
da migração D (ADR-0105), comportamento idêntico + (Modelo) gravar a receita
de lote como artefato para os lotes 3+.
**Fontes**: ADR-0104/0105, relatórios P314/P316, mapa de migração (§ evidências
— as 3 V9), `CLAUDE.md` (tabela do linter; Trava Arquitetural).
**Pastas restritas**: `00_nucleo/materialization/` e `00_nucleo/context/` não
autorizadas.
**Commits**: um para a Pre-1 ("Passo 317 — zera V9"), um para o lote
("Passo 317 — lote 2 math"). Lotes isoláveis no histórico (lição do 313).

---

## Pre-1 — Zerar as 3 V9 (o critério primário volta a ser binário)

As violações (mapa de migração): `03_infra/src/font_metrics.rs:10` e
`03_infra/src/layout.rs:7` importam **subdiretórios internos de L1** em vez
das portas `[l1_ports]`.

1. **Diagnosticar antes de mexer**: para cada import ofensor, verificar se o
   símbolo já é exposto por uma porta declarada (`[l1_ports]`).
   - **Se sim**: o conserto é só trocar o path do import — mudança local em
     L3, sem L0 novo (os L0 de `03_infra` não especificam paths de import;
     confirmar e registrar).
   - **Se não** (a porta precisa crescer para expor o símbolo): isso toca a
     superfície de L1 → **redigir a atualização do L0 da porta, PARAR no
     checkpoint, e só aplicar após confirmação humana** (Trava
     Arquitetural). Não contornar re-exportando por dentro de L3.
2. Validar: `cargo build` + `cargo test --workspace` verdes;
   `crystalline-lint .` → **ZERO violations totais** (0 V7 desde P316, 0 V9
   agora, 0 drift). A partir deste commit, "zero violations" volta a valer
   sem asterisco — e **qualquer violação em relatório futuro é regressão**,
   não herança.

## Pre-2 — A tabela de largura de uso (o dimensionador de todos os lotes)

Para **cada uma das ~74 variantes restantes** do `Content`: medir a largura
de uso — nº de sites de construção/match fora de `content.rs`
(`grep -rn 'Content::Nome' --include='*.rs' | grep -v content.rs | wc -l`,
ou método equivalente registrado). Emitir a tabela completa (variante ×
largura × locatável s/n × família) **no relatório**. Esta tabela é o
dimensionador dos lotes 2..N — o achado do P316 (custo dominante ∝ largura,
mensurável antes) operacionalizado.

---

## Lote 2 — Família math restante (decisão do dono: opção b do P316)

**Composição**: todas as variantes `Math*` ainda não migradas (enumerar do
enum; esperadas: `MathOp`, `MathAccent`, `MathCancel`, `MathUnderover`,
`MathFrac`, `MathAttach`, `MathRoot` e as demais `Math*` que o enum tiver,
exceto `MathStyled`, já migrada no P316). **Ordem de migração: largura de uso
crescente** (Pre-2) — a mais barata primeiro, a mais larga por último.

Executar pelo **MODELO DE LOTE** abaixo, com `LOTE = {a lista enumerada}`,
`N_LOTE = 2`. Notas específicas desta família:

- Os prompts L0 de math/layout já são finos (P314); os de elemento
  (`entities/elements/math_*.md`) são novos.
- Se alguma `Math*` for locatável, segue o caminho do Heading (P316 §2.4);
  esperado: nenhuma é — confirmar e registrar.
- O handler `MathStyled` em `_comum.md` (P314) é precedente de forma para os
  novos.

---

## MODELO DE LOTE (artefato reutilizável — gravar como ficheiro no repo)

Além de executá-lo para o Lote 2, **gravar** este modelo em
`00_nucleo/modelo-lote-migracao-d.md` (entregável do passo), com os
parâmetros marcados, para os lotes 3+ serem instanciados pela própria sessão
do typst sem retrabalho. Conteúdo do modelo:

> **Parâmetros**: `N_LOTE`, `LOTE` (lista de variantes), notas da família.
>
> **Fase A — L0 (redigir e PARAR)**
> 1. Um prompt fino por variante: `entities/elements/<nome>.md`
>    (content-preserving do que existir; spec nova só do desenho do trait).
> 2. Atualizar `entities/content.md` (variantes → `Nome(Arc<NomeElem>)`;
>    braços → dispatch).
> 3. `_comum.md` de `elements/` só muda se o trait mudar — e o trait **não
>    muda em lote** (mudança de trait = passo próprio, não lote).
> 4. **CHECKPOINT humano** (Trava Arquitetural): decisões, L0s, plano de
>    toque (a lista de sites do grep). Prosseguir só com confirmação.
>
> **Fase B — implementação (ordem: largura de uso crescente)**
> 1. Testes unitários do `impl Element` de cada variante primeiro (falham).
> 2. Migrar variante a variante: struct `NomeElem` no módulo próprio;
>    variante `Nome(Arc<NomeElem>)`; construtor ergonómico; os 6 matches do
>    hub viram dispatch; sites de construção/match atualizados.
> 3. Locatável segue o precedente Heading (`element_kind`/`to_payload`);
>    estado misto dos enums permanece (esvaziam lote a lote).
> 4. Linhagem: headers + `crystalline-lint --fix-hashes .`.
>
> **Validação (critérios fixos)**
> - `cargo build` e `cargo test --workspace` verdes; contagem cresce só
>   pelos testes unitários novos; **nenhuma asserção existente alterada**
>   (sintaxe de construção pode mudar; asserção não).
> - `crystalline-lint .` → **ZERO violations** (qualquer uma = regressão do
>   lote; parar e corrigir antes de seguir).
> - Ressalva conhecida da stack (`recursao_infinita…` com stack default)
>   pode reaparecer; registrar, não é regressão.
>
> **Medições (fecham o lote — métrica ADR-0104)**
> - `content.rs`: linhas antes/depois (esperado: **encolhe** — o setup foi
>   pago no P316; lote que não encolher o hub exige explicação no
>   relatório).
> - Custo-por-variante: ficheiros/linhas fora do módulo próprio, **comparado
>   à previsão da tabela de largura** — o lote também valida o preditor.
> - Parte atómica: linhas dos módulos novos.
>
> **Relatório** (`typst-passo-<N>.md` + resumo): decisões, medições vs
> previsão, proposta do lote seguinte **derivada da tabela de largura**
> (decisão humana), fora-de-escopo confirmado.
>
> **Regras permanentes**: trait não muda em lote; quebra de desenho → parar
> e voltar ao L0; conserto oportunista proibido; toda medição com comando.

---

## Relatório do P317 (`typst-passo-317.md` + resumo no chat)

- Pre-1: o caminho do conserto (porta já expunha? L0 precisou crescer?) e o
  **ZERO violations** confirmado.
- Pre-2: a tabela de largura completa (74 variantes).
- Lote 2: medições vs previsão; `content.rs` antes/depois; contagem da suíte.
- O modelo gravado (`00_nucleo/modelo-lote-migracao-d.md`).
- Proposta do Lote 3 derivada da tabela (decisão humana).
- `git log --oneline` dos commits; `git status` limpo.

## Fora de escopo

- Lotes 3+ (instanciados pelo modelo, decisão por lote).
- F / PropMap / StyleChain (DEBT 99.E).
- DEBT-57 (specs ausentes — registrado, não escrito).
- Fatiar `rules/eval.md` / `rules/parse.md` / `rules/layout.md` (só se
  morderem: se o lote editar um deles, fatiar primeiro pela receita do P314,
  com `git rm` do velho).
- Mudanças no trait `Element` (passo próprio se precisar).

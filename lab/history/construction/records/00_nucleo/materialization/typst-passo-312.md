# Tarefa: gerar a v1 do Mapa de Migração vanilla ↔ cristalino (tabela de correspondência)

**Repositório de trabalho**: typst-crystalline (raiz).
**Ferramenta externa**: o binário da lente (`tekt-cargo-dsm/target/release/lente`,
nível de item — prompt 0078 da lente; ajustar o caminho se o repositório vizinho
estiver noutro lugar).
**Entregáveis** (os únicos arquivos novos permitidos):
1. `00_nucleo/mapa-migracao-vanilla-cristalino.md` — o documento.
2. `lab/mapa-migracao/gerar.py` — o gerador da parte mecânica (regenerável).
3. `lab/mapa-migracao/README.md` — uma página: como regenerar.

**Nenhum arquivo existente é modificado** (a tabela de cobertura atual fica
intocada; este mapa convive com ela). Exceção operacional única: o symlink
temporário do passo 1, removido ao final, inclusive em caso de falha.

**Pastas restritas**: `00_nucleo/materialization/` e `00_nucleo/context/` **não
estão autorizadas nesta tarefa**. Onde as fontes não restritas não bastarem para
classificar um módulo, o status é `pendente-de-confirmação` — declarar é a
resposta certa, adivinhar não.

---

## Propósito

O projeto precisa decidir o que fazer (P312) e se a refatoração está valendo.
Para isso falta um documento que mostre, **por crate e por módulo**, o que do
vanilla já existe no cristalino, onde está, e o que falta — separando o que é
mecânico (regenerável pela lente em segundos, nunca editado à mão) do que é
declarado (curado por humano/sessão, sempre com fonte). A discordância entre as
duas colunas é o detector automático de deriva documental (a falha F4 do
diagnóstico de bloqueio): nenhuma sessão futura precisa *lembrar* de propagar
nada — a contradição aparece sozinha na regeneração.

---

## Passo 1 — Gerar o JSON da comparação

```bash
ls lab/typst-original/Cargo.toml*          # se só houver .original:
ln -s Cargo.toml.original lab/typst-original/Cargo.toml
<caminho>/lente --comparar --antes lab/typst-original --depois . > /tmp/comparar-typst-itens.json
rm lab/typst-original/Cargo.toml
```

Portão: pareados = 1474 e sem-par ≈ 10910/1203 (laudo 0078 da lente). Se
divergir muito, parar e reportar. **Inspecionar a estrutura real do JSON antes
de escrever o gerador** — não assumir nomes de campo (a seção de itens tem
`pareados` com `de`/`para`/`kind`/`nome`/`trait`, `ambiguos`, `sem_par_antes`,
`sem_par_depois`; conferir os nomes exatos e os campos disponíveis em cada
lista, em particular se `trait` e `kind` existem nas listas de sem-par).

## Passo 2 — O gerador da parte mecânica (`lab/mapa-migracao/gerar.py`)

Script descartável (Arena), python3 puro, que lê o JSON e emite a tabela
mecânica em markdown. Regras operacionais:

**Módulo de um item** (derivado do path):
1. Remover segmentos de resolução `<...>` (ex.: `Abs::<Add>::Output`).
2. Remover o último segmento (o nome do item).
3. Enquanto o último segmento restante começar com maiúscula (tipos pai,
   convenção Rust: módulos são snake_case, tipos são CamelCase), remover
   também. O que sobra é o módulo.
4. Registrar a regra no README e no cabeçalho do documento.

**Boilerplate separado do real**: item é boilerplate se o campo `trait` está
preenchido (folha de impl-de-trait — a definição da medição 0077 da lente). Se
alguma lista do JSON não carregar `trait`, usar o fallback declarado: `fn` cujo
pai imediato é tipo (CamelCase) e cujo nome ∈ {fmt, clone, clone_from, hash,
eq, ne, cmp, partial_cmp, default, from, try_from, into, deref, deref_mut,
drop, as_ref, borrow} — e **registrar qual definição foi usada e onde**.

**Por módulo do vanilla**, emitir:

| coluna | conteúdo |
|---|---|
| módulo | path do módulo no vanilla |
| itens (reais+bp) | total, separando reais e boilerplate |
| pareados (reais) | quantos itens reais têm par no cristalino |
| sem-par (reais) | idem sem par |
| % | pareados reais / itens reais |
| destino dominante | o módulo do cristalino que mais recebe os pareados deste módulo (e a fração: "entities::ast (92%)") |
| sugestão mecânica | `migrado` (≥90% reais pareados) · `parcial` (entre) · `não-iniciado` (0 pareado real) · `só-boilerplate` (0 itens reais) |

E o mesmo no sentido inverso, mais curto: **módulos do cristalino** com itens
sem-par-depois (o que o cristalino tem de novo, por módulo) — sem coluna
declarada, é só censo.

**Rollup por crate**: módulos por sugestão (`typst_library: N módulos — X
migrados, Y parciais, Z não-iniciados, W só-boilerplate`), itens reais
pareados/sem-par por crate.

**Determinismo**: ordenar tudo (por crate, por path); duas execuções, mesma
saída. Os thresholds (90%) ficam como constantes nomeadas no topo do script,
com comentário de que são sugestão mecânica, não veredito.

## Passo 3 — A coluna declarada (pré-preenchida com fonte, nunca sem fonte)

Para cada módulo do vanilla da tabela, adicionar a coluna **status declarado**
com o vocabulário fechado:

`fechado` · `fechado-consolidado` (implementado com forma diferente de
propósito — ex.: Content/ADR-0026) · `parcial` · `pendente` ·
`fora-de-escopo` · `pendente-de-confirmação`

Fontes permitidas para a pré-marcação (cada marcação **cita o arquivo**):
`CLAUDE.md` (tabela de ADRs e convenções), `00_nucleo/adr/`,
`typst-cobertura-vanilla-vs-cristalino.md` (mesmo com a deriva conhecida até
P283 — citar com a ressalva), `00_nucleo/DEBT.md`, `00_nucleo/diagnosticos/`.
Sem fonte suficiente → `pendente-de-confirmação`, sem exceção.

A coluna declarada vive **no documento**, não no gerador — o gerador emite a
parte mecânica; o documento junta as duas. O README explica a regra de
manutenção: a mecânica regenera e substitui; a declarada só muda citando o
passo/ADR que mudou o estado.

## Passo 4 — Discordâncias (o detector de deriva)

Seção própria do documento listando todo módulo onde mecânica e declarada
conflitam, em dois sentidos:

- declarado `fechado`/`fechado-consolidado` com mecânica `não-iniciado` e sem
  nota de consolidação → ou a declaração está errada, ou falta a nota;
- declarado `pendente`/`parcial` com mecânica `migrado` → o trabalho aconteceu
  e ninguém declarou (a classe exata da deriva P284–P311b).

Cada entrada: módulo, os dois status, e o que confirmar.

## Passo 5 — Evidências para o veredito (seção final, sem veredito)

Reunir, com fonte, os números que a decisão "está valendo / o que fazer"
precisa — **sem responder por ela**:

- Estrutural: maior SCC 203 (vanilla) → 15 (cristalino); `crystalline-lint .`
  zero violations (rodar e citar a saída).
- Funcional: a cobertura declarada da tabela atual (com a ressalva da deriva)
  e a contagem de testes (rodar `cargo test --workspace` se couber no tempo;
  senão citar a última contagem documentada, com fonte).
- O "falta honesto": módulos `não-iniciados` reais (mecânica) que **também**
  estão `pendente`/`ausente` na declarada — contagem e lista por crate.
- O passivo: resumo de uma linha por item do `DEBT.md`.
- As ressalvas permanentes: itens reais do `typst-macros` ausentes do censo
  (falha de extração conhecida da lente); itens transformados de kind
  (Content) não pareiam por definição da chave.

## Passo 6 — Relatório de execução

Além dos três arquivos, responder no chat com: o rollup por crate, o tamanho
da seção de discordâncias (quantos módulos), os 10 maiores módulos
`não-iniciados` reais, o tempo de geração, e o `git status` (só os 3 arquivos
novos + nada mais; symlink removido).

---

## Restrições finais

- Nenhuma instrução histórica executada; nenhum código de produto tocado
  (o script em `lab/` é Arena, fora das camadas L1–L4 e da trava de L0).
- Honestidade acima de completude: `pendente-de-confirmação` é resposta
  válida; uma coluna declarada inventada destruiria o propósito do documento.
- Se a lente falhar ou o JSON divergir do portão: desfazer o symlink e
  reportar, sem improvisar.

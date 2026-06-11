# lab/mapa-migracao — gerador da parte mecânica do Mapa de Migração

Arena (`lab/`), fora das camadas L1–L4 e da trava de L0. Descartável e
regenerável. Produz a **parte mecânica** de
`00_nucleo/mapa-migracao-vanilla-cristalino.md`.

## O que isto é (e o que não é)

O mapa de migração tem **duas colunas independentes** por módulo:

- **Mecânica** — o que a lente (`lente --comparar`, laudo 0078) mede por
  pareamento de path entre `lab/typst-original` (vanilla) e a raiz
  (cristalino). É isto que `gerar.py` emite. **Nunca editada à mão.**
- **Declarada** — o estado afirmado pelos documentos do projeto, sempre
  com fonte citada. **Vive no documento, não aqui.** `gerar.py` não a
  conhece.

A discordância entre as duas é o detector de deriva documental (falha F4
do `diagnostico-bloqueio-processo-2026-06-09.md`).

## Como regenerar a parte mecânica

```bash
# 1. Gerar o JSON da lente (symlink temporário; Cargo.toml só existe
#    como .original na quarentena).
ln -s Cargo.toml.original lab/typst-original/Cargo.toml
<caminho>/lente --comparar --antes lab/typst-original --depois . \
    > /tmp/comparar-typst-itens.json
rm lab/typst-original/Cargo.toml          # remover SEMPRE, mesmo em falha

# 2. Emitir a tabela mecânica.
python3 lab/mapa-migracao/gerar.py /tmp/comparar-typst-itens.json
```

Portão esperado (laudo 0078): pareados = 1474; sem-par ≈ 10910 (antes) /
1203 (depois). Se divergir muito, parar e reportar — não improvisar.

## Regras de derivação (implementadas em `gerar.py`)

**Módulo de um item**, a partir do path:

1. Remover segmentos de resolução `<...>` (ex.: `Abs::<Add>::Output`).
   Verificado no JSON: todo segmento de resolução começa por `<` ao
   separar por `::`, logo basta descartar esses segmentos.
2. Remover o último segmento (o nome do item).
3. Enquanto o último segmento restante começar por **maiúscula** (tipos
   pai — em Rust módulos são `snake_case`, tipos são `CamelCase`),
   removê-lo. O que sobra é o módulo (mínimo: o crate).

**Boilerplate vs real**: um item é boilerplate se o campo `trait` está
preenchido (folha de impl-de-trait — definição primária da medição 0077
da lente). Verificado: **todas** as listas do JSON (`pareados`,
`sem_par_antes`, `sem_par_depois`, `ambiguos`) carregam `trait`, logo a
**definição primária** foi usada e o **fallback** (fn de nome canónico
`fmt`/`clone`/`from`/… cujo pai é um tipo) **não foi necessário**.

**Sugestão mecânica** (constante `MIGRADO_PCT = 0.90` no topo do script;
é sugestão, não veredito humano):

- `migrado` — ≥ 90 % dos itens reais pareados
- `parcial` — entre 0 e 90 %
- `não-iniciado` — 0 itens reais pareados
- `só-boilerplate` — 0 itens reais (só folhas de trait)

A saída é **determinística**: tudo ordenado por crate e por path; duas
execuções produzem a mesma saída byte-a-byte.

## Manutenção do documento (regra de ouro)

- A **parte mecânica** do documento (rollup, tabela por módulo nas colunas
  mecânicas, censo inverso, portão) **regenera e substitui** via este
  script. Nunca se edita à mão.
- A **coluna declarada + fonte** só muda **citando o passo/ADR** que mudou
  o estado. Uma marcação sem fonte é um bug do documento — o valor certo
  é `pendente-de-confirmação`.

Quando o conjunto de módulos muda (a lente repareia), reconciliar as
células declaradas à mão: a mecânica é a verdade da esquerda, a declarada
é curada à direita, e a §4 do documento (discordâncias) recalcula-se ao
cruzar as duas.

## Limitação conhecida da lente (não é bug do mapa)

A lente pareia por **nome de símbolo** normalizado na raiz do crate. O
cristalino **reescreveu** `typst_library` em `typst_core` com tipos
**renomeados** (Elem structs → variants de enum). Logo uma feature
migrada-e-renomeada nunca pareia e aparece como `não-iniciado` mecânico.
Por isso o sinal real de para-onde-foi é a coluna **destino dominante**, e
o cruzamento útil para reescrita é a §4.1 do documento (declarado fechado
vs mecânica não-iniciado), não a §4.2.

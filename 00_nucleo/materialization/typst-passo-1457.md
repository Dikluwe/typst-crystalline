# Passo 1457 — eliminar o classificador hardcoded de escalares YAML

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado:** EXECUÇÃO AUTORIZADA — correção interna de paridade  
**Data da medição:** 2026-09-18T22:17:12-03:00  
**Commit medido:** `b2eba024821ed5f10a4667d6179da3ea2cebff8d`  
**Árvore medida:** `6d60b5f0c09425ed8595a54670bc822589eeff80`, limpa  
**Regime:** executado sem atestação de isolamento

## 1. Próxima unidade

Depois de P1456 remover a reescrita ad hoc de page ranges, a mesma ponte
`BibEntry → YAML → hayagriva` ainda contém `needs_yaml_quotes`: uma lista
manual de prefixos, separadores e quatro spellings que tenta prever quando o
parser YAML interpretará texto como outro tipo.

O classificador é incompleto por construção. A correção não deve ampliar a
lista; deve eliminar a decisão e serializar toda entrada textual como string.

## 2. Medição anterior à decisão

- L0 identificador administrativo anterior:
  [dado administrativo removido];
- consumer identificador administrativo:
  [dado administrativo removido];
- vanilla ratificado identificador administrativo:
  [dado administrativo removido];
- cristalino release identificador administrativo:
  [dado administrativo removido].

Na fixture pública IEEE com `title = {True}`, o vanilla compila a
bibliografia. O cristalino falha antes do CSL:
`true-title.title: invalid type: boolean true, expected a formattable string`.
O valor saiu sem aspas porque a tabela só reconhece `true` em minúsculas,
embora o parser YAML reconheça também `True`. A mesma classe inclui `NULL`,
`.nan`, indicadores, escapes e controles omitidos pelo catálogo.

## 3. Obrigação

1. Serializar cada campo textual da ponte incondicionalmente como string JSON,
   representação aceita como escalar quoted pelo YAML.
2. Preservar exatamente o valor lógico de vazio, texto comum, spellings
   tipados pelo YAML, Unicode, aspas, barra invertida, newline e tab.
3. Remover `needs_yaml_quotes`; não criar tabela sucessora, branches por
   spelling/campo, escaping manual nem pós-processamento CSL.
4. Preservar P1456, parsing BibTeX, tipos de entrada, autores, style, locale e
   renderização dos valores comuns.

## 4. Prova e ataques

Congelar contrato, oráculos e mutantes antes do candidato. O RED deve provar
que `True` não faz round-trip textual. Positivos cobrem vazio, plain text,
`True`, `NULL`, `.nan`, indicador `?`, aspas, barra invertida, newline, tab e
Unicode. Ataques retêm ou ampliam a tabela, omitem escapes, aplicam a solução
a apenas um campo, corrompem Unicode, aceitam um só binário ou promovem
`Unknown`. Exigir score `1.0`, repetição em três ordens e zero sobreviventes.

## 5. Gate ADR-0127

Trata-se de correção interna de representação e paridade, sem mudança de API,
default, fase ou compatibilidade documentada. O L0 foi atualizado primeiro e
o fluxo segue sem parada conforme ADR-0127.

## 6. Fecho

Executar RED→GREEN, campanha bilateral pública, mutantes, owner tests,
`cargo test --workspace`, `cargo build`, `cargo fmt --all -- --check`,
`crystalline-lint .` e `git diff --check`. Produto e certificado ficam em
commits separados.

## 7. Execução e resultado

**Estado medido:** working tree não commitado sobre
`b2eba024821ed5f10a4667d6179da3ea2cebff8d`, em
`2026-09-18T22:27:17-03:00`. O recorte altera L0, consumer e a permissão
interna já existente de `serde_json` no `crystalline.toml`; adiciona este
passo e sete artefactos `p1457-*` em `00_nucleo/diagnosticos/`.

O RED focal produziu as duas falhas congeladas: `plain text` saiu sem aspas e
`True` foi interpretado pelo YAML como booleano. O candidato removeu
`needs_yaml_quotes` e delegou todo escalar textual a
`serde_json::to_string`, função pura da crate já presente em L1 por ADR-0111
e já usada no módulo `loading`. O allowlist foi tornado explícito para
`to_string`; nenhuma dependência nova entrou no grafo.

A antiga sentinela P644, que dependia de U+0000 cru para fabricar YAML
inválido, foi alinhada ao novo L0: o controle é serializado como `\u0000` e
faz round-trip textual. A propagação de erros YAML externos de P644 não foi
removida.

Resultados reproduzidos:

- `cargo test -q -p typst-core p1457_ -- --nocapture`: 3/3;
- `cargo test -q -p typst-core compiler::layout::bib_csl::tests -- --nocapture`:
  36/36;
- campanha IEEE pública, nas ordens forward, reverse e forward-repeat:
  vanilla e cristalino compilaram e extraíram
  `[1] A. Alpha, “True,” Journal, 2024.` em todas as execuções;
- `python3 [artefato protocolar removido]`: 12/12 mutantes
  mortos, score `1.0`, zero sobreviventes e zero promoção de `Unknown`;
- a busca por `needs_yaml_quotes` e pelo escaping com `s.replace` no consumer
  ficou vazia;
- `cargo build --release --bin typst`, `cargo build`,
  `cargo fmt --all -- --check`, `crystalline-lint .` e `git diff --check`:
  sucesso, com zero violations;
- `cargo test --workspace`: 5915/5923 testes passaram. Permanecem as mesmas
  oito falhas históricas fora de `bib_csl`: P1292 `math_vec`, quatro casos
  P981/P1132R de delimitadores `math lr`, `cases` com align point e duas
  contagens HTML P1423/P1168.

O resselo atualizou somente os campos mecânicos de linhagem depois do
candidato. O preseal preserva o L0 semântico anterior ao candidato
(`342400dd...`); o L0 final tem identificador administrativo
[dado administrativo removido],
o consumer final
[dado administrativo removido],
o allowlist final
[dado administrativo removido]
e o binário release
[dado administrativo removido].
Não se alega isolamento entre papéis: executado sem atestação de
isolamento.

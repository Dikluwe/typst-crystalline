# Passo 1460 — eliminar o hardcode de heading no serializer L2

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado:** IMPLEMENTADO — certificação segregada no commit subsequente

**Data da medição:** 2026-09-19

**Commit medido:** `7f7a7ab1f9afc3e92ed29800408d9508525a6094`

**Árvore medida:** `a06219a55c83ff0eb53148acebe393ae1b02a08a`, limpa antes da edição L0

**Regime:** executado sem atestação de isolamento

## 1. Próxima unidade

P1459 centralizou as traduções de heading em L1, mas deixou a dívida
explicitamente seguinte: `02_shell/src/cli.rs::content_to_semantic` ainda
decide o fallback raw com `Content::text("Section")`. Acrescentar uma
constante ou tabela L2 apenas deslocaria o hardcode; a unidade correta é
delegar ao owner canónico P1459.

## 2. Medição anterior à decisão

Medição em `2026-09-19T03:02:37Z`, no commit e árvore acima, antes do
candidato e com apenas os três L0 P1460 já editados no working tree:

- consumer L2 identificador administrativo:
  [dado administrativo removido];
- owner L1 identificador administrativo:
  [dado administrativo removido];
- hub L1 identificador administrativo:
  [dado administrativo removido];
- binário cristalino release identificador administrativo:
  [dado administrativo removido];
- binário vanilla ratificado identificador administrativo:
  [dado administrativo removido].

Para `heading[First]`, o cristalino produziu 211 bytes JSON compacto, 282
bytes JSON pretty e 185 bytes YAML, com hashes respectivamente
[dado administrativo removido],
[dado administrativo removido]
e [dado administrativo removido].
O vanilla produziu a forma menor de 57 bytes, hash
[dado administrativo removido].
Logo a referência do output P1460 é o baseline cristalino congelado; a forma
raw vanilla delimita o que este passo não pode alegar.

## 3. Obrigação e arquitetura

1. Tornar pública somente a função existente
   `heading_supplement_for_lang(Option<&Lang>) -> &'static str`.
2. Manter o módulo físico e `NAMES` privados; o hub faz um reexport público
   estreito, sem wrapper, constante ou segunda tabela.
3. L2 constrói o mesmo `Content::text` com
   `heading_supplement_for_lang(None)`; `None` expressa ausência real de
   língua causal no `Content` raw.
4. Preservar byte a byte JSON/YAML, pretty, fields, ordem, newline,
   `--field`, `--one`, erro raw e o snapshot causal `LocatedContent(Some)`.

O literal inglês que permanece dentro do owner é o dado canónico do fallback,
exercitado juntamente com as 45 traduções; não é uma decisão duplicada no
consumer. Recriá-lo fora do owner, mesmo por constante, é proibido.

## 4. Gate ADR-0127

O reexport cria API pública aditiva e, portanto, exigia confirmação. O fecho
P1459 avisou expressamente esse gate; o pedido subsequente do dono para
escrever e implementar o próximo passo, reiterado em 2026-09-19, confirma
esta ampliação mínima. Não muda default, fase, output ou assinatura existente.

## 5. Provas e limites congelados

Contrato, oráculos e ataques ficam selados antes do candidato. O primeiro RED
deve provar que L2 ainda não alcança o owner pelo path público. O GREEN deve
cobrir API cross-crate, transcript raw exato, field/one e snapshot alemão.
Quatro mutações compiladas atacam null, omissão, owner errado e sobrescrita do
snapshot; travas estáticas rejeitam duplicação, locale diferente de `None` e
API excessiva. Score exigido: 1,0; Unknown não recebe crédito.

Ficam fora: paridade raw geral, remoção de fields sintéticos, constructor raw
com supplement explícito e a divergência show-set/pré-show. Esses itens não
podem ser rebatizados como concluídos por P1460.

## 6. Fecho previsto

Executar RED→GREEN, ataques em forward/reverse/forward-repeat, testes P1459,
workspace, build, fmt, `crystalline-lint .` e `git diff --check`. O produto e
a certificação ficam em commits separados; a certificação declara
explicitamente a ausência de atestação de isolamento.

## 7. Execução RED→GREEN

O preseal foi congelado antes do candidato em
relatório consolidado da limpeza.
Contrato, oráculos e catálogo adversarial permaneceram imutáveis depois do
selo. A primeira execução RED também encontrou uma ambiguidade de tipo na
fixture de snapshot; ela foi corrigida ainda antes de código produtivo. O RED
válido então falhou somente com `E0603`: L2 não conseguia importar a função
privada do owner.

O candidato promoveu a função existente para `pub`, promoveu o reexport do
hub para `pub use` e substituiu o literal L2 por
`Content::text(heading_supplement_for_lang(None))`. O módulo físico e a tabela
continuam privados. `crystalline-lint --fix-hashes .` ressellou os três pares
L0↔consumer; não foi criado Núcleo nem wrapper novo.

Depois de `rustfmt`, o runner estático precisou aceitar a vírgula final
mecânica em `None,`. Esse ajuste de instrumento não alterou contrato nem
oráculo; a campanha compilada foi repetida integralmente sobre os bytes
formatados finais.

## 8. Provas finais

Estado medido em `2026-09-19T03:20:50Z`: working tree não commitado derivado
do commit e árvore baseline acima, com os ficheiros P1460 listados no
manifesto. Identidades finais dos consumers:

Proveniência revalidada em `2026-09-19T03:24:21Z`: `git diff HEAD --stat`
continha exatamente os três L0 e os três consumers, 6 ficheiros tracked,
224 inserções e 11 remoções. `git status --short` acrescentava 11 novos:
este passo e `p1460-{attack-runner.py,contract.json,manifest.json,
mutants.json,mutation-results.json,oracle-results.json,oracles.json,
postseal-amendment.json,preseal.json,static-attack-results.json}` em
`00_nucleo/diagnosticos/`. Nenhum outro ficheiro estava alterado.

- owner L1:
  [dado administrativo removido];
- hub L1:
  [dado administrativo removido];
- CLI L2:
  [dado administrativo removido];
- binário cristalino release:
  [dado administrativo removido];
- binário vanilla ratificado:
  [dado administrativo removido].

Resultados:

- testes focais P1460: 2 aprovados, 0 falhas;
- regressões P1459: 3 aprovadas, 0 falhas;
- oráculos públicos: 10/10 em cada ordem forward, reverse e
  forward-repeat, 30/30 no total, mais YAML-pretty, erro raw e forma vanilla
  menor como três controles aprovados;
- mutação compilada manual: 4 mutantes válidos, todos apanhados nas três
  ordens, 12/12 execuções, score 1,0; o arquivo candidato voltou ao mesmo
  identificador administrativo depois de cada campanha;
- ataques estáticos/de modelo: 16/16 rejeitados, score discriminatório 1,0;
  esse número não é apresentado como mutation testing compilado;
- `cargo test --workspace --quiet`: o test binary de core conservou
  exatamente o baseline histórico, 5919 aprovados e 8 falhas conhecidas
  (P1292, quatro P981/P1132R, align point e duas contagens HTML), sem falha
  P1460;
- `cargo build --workspace`, `cargo fmt --all -- --check` e
  `git diff --check`: sucesso;
- `crystalline-lint .`: exit 0, zero violations, inclusive V5/V15/V26;
- todos os JSON P1460 são válidos.

A varredura de `Content::text("Section")` em Rust produtivo deixou zero
decisões de suplemento de heading fora do owner. A única ocorrência textual
exata remanescente fora dele está numa fixture `#[cfg(test)]` de repr e não
decide suplemento. As ocorrências `Section` no próprio owner são a tabela e o
fallback canónicos, não uma duplicação de consumer.

## 9. Veredito e limites honestos

**VERIFIED no recorte P1460:** o fallback raw de heading em L2 já não contém
nem desloca um hardcode linguístico; consome diretamente o owner L1 público
mínimo com ausência causal explícita e preserva todos os bytes contratados.

Não é alegada paridade raw geral: o vanilla ratificado continua emitindo uma
forma menor. Também não são alegadas correção de show-set/pré-show, remoção de
outros defaults sintéticos de Content nem eliminação global de toda string
literal do repositório. Essas seriam unidades semânticas próprias, e algumas
exigem novo gate ADR-0127.

Toda a execução e revisão deste passo ocorreu **sem atestação de isolamento**.

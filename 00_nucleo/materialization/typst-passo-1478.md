# Passo 1478 — PDF/A-4f sem anexos: `/EmbeddedFiles` vazio e revalidação integral de PDF/A

**Estado:** GREEN — NÃO SELADO (código funcional completo, 16/16 veraPDF,
regressão zero; bloqueado por achado `[PARSE]` herdado de P1477 em
`tests.rs:11146`, não agravado por este passo — ver §10)

**Data da auditoria:** 2026-09-22T13:59:10-03:00

**Baseline de redação:** `67a8768c8bf39b019528362484eeee591d4212a1`,
tree `f4fb1d2dddad9144e6dccf08f88b320eb1bf7954`, working tree limpo.

**Vanilla ratificado:**
`586e1bd43fae6c9a973218163d3165c53ab8d16d`
(`/usr/local/bin/typst`, SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`).

**Binário cristalino medido:** `target/release/typst`, recompilado sobre o
baseline acima, SHA-256
`455494d18734ed7f63c305ee5cc72a1560815b07fe672cb7f09dcc999c430727`.

## 0. Origem e retificação obrigatória

P1476 mediu A4f/A4e **com anexo** e fechou corretamente esse subconjunto.
P1477 fechou A1a/A1b e concluiu, de forma mais ampla que a evidência, que a
família PDF/A estava completa. A auditoria posterior deste passo testou
explicitamente A4f/A4e **sem anexo** e encontrou uma lacuna residual só em
A4f.

Este passo não apaga silenciosamente o registo histórico. O L0 deve ganhar
uma retificação P1478 que diga:

- a afirmação de conformidade de P1476 permanece válida para A4f/A4e com
  anexo;
- a afirmação final de P1477 não cobria A4f sem anexo;
- A4f sem anexo só fecha quando o catálogo contém a name tree
  `/EmbeddedFiles` vazia exigida pelo perfil.

## 1. Objetivo

Fazer um documento compilado com `--pdf-standard a-4f` e zero anexos passar
integralmente no veraPDF 1.30.2, sem alterar nenhum outro perfil nem os
caminhos com anexos.

O resultado estrutural necessário é:

```text
/Names <<
  /EmbeddedFiles << /Names [] >>
>>
```

A representação por objetos indiretos é permitida. Igualdade de IDs, bytes,
ordem dos objetos ou forma inline do vanilla não é critério (ADR-0107). O
observável é a presença da chave `/EmbeddedFiles` apontando para uma name
tree válida cujo array `/Names` é vazio.

Depois do GREEN focal, revalidar toda a família PDF/A:

1. Sem anexos: A1a, A1b, A2a, A2b, A2u, A3a, A3b, A3u, A4, A4f e A4e.
2. Com um anexo `relationship: "supplement"`: A3a, A3b, A3u, A4f e A4e.

São 16 combinações de conformidade. Cada contagem só pode fechar o passo com
commit, hora, hashes dos binários e estado exato da árvore registados.

## 2. Medição anterior à decisão

### 2.1 Fixture bilateral

A medição usou uma fixture sem anexos, SHA-256
`93b3d9d54a454cf7d3f86ca780f3d30244759125ea4cb90e95ae300142f547d1`:

```typst
#set document(title: "PDF/UA parity audit", author: "typst-crystalline")
#set page(width: 12cm, height: 8cm)
#set text(lang: "en")

= Accessible heading

This is a short tagged paragraph with *emphasis*.
```

O nome histórico da fixture é irrelevante; para P1478 ela só exerce texto,
heading, metadata e ausência de `pdf.attach`.

### 2.2 Resultado veraPDF 1.30.2

| Perfil sem anexo | Implementação | `isCompliant` | regras aprovadas | regras falhadas | checks falhados |
|---|---|---:|---:|---:|---:|
| A4f | vanilla ratificado | true | 109 | 0 | 0 |
| A4f | cristalino P1477 | false | 108 | 1 | 1 |
| A4e | vanilla ratificado | true | 109 | 0 | 0 |
| A4e | cristalino P1477 | true | 109 | 0 | 0 |

Única falha cristalina A4f:

```text
ISO 19005-4:2020, 6.9, test 5
A PDF/A-4f conforming file shall contain an EmbeddedFiles key in the
name dictionary of the document catalog dictionary
```

O que refutaria a classificação de causa única: após inserir a name tree
vazia, veraPDF revelar qualquer segunda regra falhada na mesma fixture.

### 2.3 Forma observada no vanilla

`qpdf --qdf --object-streams=disable` sobre o PDF A4f vanilla sem anexos:

```text
/Names <<
  /EmbeddedFiles <<
    /Names [
    ]
  >>
>>
```

No A4e vanilla sem anexos, `/EmbeddedFiles` fica ausente. No cristalino A4f
pré-P1478, o catálogo já pode conter `/Names` por causa de destinos nomeados,
mas esse dicionário não contém `/EmbeddedFiles`. Portanto o reparo precisa
**fundir** a entrada na `/Names` existente, nunca substituir `/Dests`.

### 2.4 Evidência da fonte pinada e causa cristalina

Fonte efetivamente usada pelo vanilla ratificado (`krilla`, checkout local
`7772dbe`):

- `configure/validate.rs:269-273`: `requires_embedded_files_when_empty` é
  delegado ao archival;
- `configure/validate.rs:859-871`: somente `A4F => true`; A1/A2/A3/A4/A4E
  são `false`;
- `chunk_container.rs:294-331`: escreve `/EmbeddedFiles` quando há anexos
  **ou** quando `requires_embedded_files_when_empty()` é verdadeiro; o loop
  sobre zero ficheiros produz `/Names []`.

No cristalino, `03_infra/src/export/builder.rs:2511-2514` retorna
imediatamente quando `doc.attachments.is_empty()`. Todo o código posterior já
sabe:

- construir `<< /Names [...] >>`;
- alocar o objeto da name tree;
- inserir `/EmbeddedFiles` numa `/Names` já existente;
- criar `/Names` no catálogo quando ainda não existe;
- omitir `/AF` quando `filespec_ids` está vazio.

Logo a correção mínima é remover o early-return **somente para A4f**. Não é
necessário criar outro encoder, alterar entidades ou tocar no parser.

## 3. Auditoria L0 e sequência obrigatória

Owner produtivo único:

- `00_nucleo/prompts/infra/export/builder.md`
- `03_infra/src/export/builder.rs`

Antes de qualquer RED/código:

1. Acrescentar ao L0 uma secção `P1478 — A4f requer /EmbeddedFiles vazio` com
   a medição, o contrato e a retificação de §0.
2. Acrescentar P1478 ao histórico de revisões.
3. Não modificar `entities/document_format_options.md`, `pdf_attach`, CLI,
   pipeline ou wiring: os tipos e o transporte de `A4f` já estão corretos.
4. Como esta é uma correção interna de paridade, seguir fluxo contínuo da
   ADR-0127: L0 primeiro, RED depois, implementação e resselo, sem paragem
   humana intermediária.

O V5 já presente no baseline entre este L0 e `builder.rs` precisa ser
absorvido pelo resselo final de P1478; não declarar o passo fechado mantendo
o drift.

## 4. Contrato de implementação

### 4.1 Gate distinto do suporte geral a anexos

Adicionar um predicado explícito equivalente a:

```rust
matches!(self.standard.archival, Some(PdfArchival::A4f))
```

Este predicado significa **"requer `/EmbeddedFiles` mesmo vazio"**, não
"suporta anexos". Não reutilizar `archival_supports_embedded_files`, pois ele
também inclui A3a/A3b/A3u/A4e e criaria árvores vazias onde o vanilla não as
emite.

### 4.2 Early-return e reaproveitamento do caminho existente

Em `emit_attachments`:

```text
se attachments está vazio E o archival não é A4f → retornar como hoje
caso contrário → executar o caminho existente
```

Com A4f vazio, o acumulador atual naturalmente produz
`<< /Names [] >>`; `filespec_ids` permanece vazio e, portanto, `/AF` continua
ausente. A rotina existente de merge deve inserir `/EmbeddedFiles` na
`/Names` já usada por `/Dests`, se houver.

Evitar redesenho maior. Um helper privado pode ser extraído somente se os
testes mostrarem duplicação real; não criar novo módulo, nova entidade ou
novo tipo público.

### 4.3 Invariantes

- A4f sem anexo: `/EmbeddedFiles` presente, `/Names []`, zero Filespec, zero
  EmbeddedFile stream, zero `/AF`.
- A4f com anexos: comportamento P1476 preservado, incluindo
  `/AFRelationship`, `/AF` e entradas lexicograficamente ordenadas.
- A4e sem anexo: `/EmbeddedFiles` ausente.
- A1/A2/A3/A4 sem anexos e PDF comum: bytes estruturais dessa área
  inalterados; não criar `/EmbeddedFiles` vazio.
- Se `/Names /Dests` já existe, preservar `/Dests` e acrescentar
  `/EmbeddedFiles`; nunca substituir o dicionário.
- Os três caminhos do builder (Helvetica, CIDFont, multifont) chamam a mesma
  rotina e observam o mesmo gate.

## 5. RED primeiro

Não acrescentar mais testes a `03_infra/src/export/tests.rs`: o baseline já
tem um erro `[PARSE]` do `crystalline-lint` a partir de `tests.rs:11146`,
associado ao tamanho acumulado desse ficheiro. O RED de P1478 deve ficar num
`#[cfg(test)] mod p1478_pdfa4f_empty_embedded_files_tests` dentro de
`builder.rs`, sob o mesmo owner L0, até a sanitização dedicada do agregador.

Testes mínimos:

1. `A4f` + documento sem attachment → catálogo resolve `/Names`, o dicionário
   resolve `/EmbeddedFiles`, e a name tree contém `/Names []`.
2. Mesmo caso → ausência de `/AF`, `/Type /Filespec` e
   `/Type /EmbeddedFile`.
3. `A4e` sem attachment → `/EmbeddedFiles` ausente.
4. Tabela negativa para `None`, A1a/A1b, A2a/A2b/A2u, A3a/A3b/A3u e A4:
   nenhum ganha árvore vazia.
5. A4f com um attachment → uma entrada não vazia e regressão integral dos
   testes P1476.
6. A4f sem attachment, mas com destino nomeado → `/Dests` e
   `/EmbeddedFiles` coexistem sob a mesma `/Names` do catálogo.

Confirmar RED no baseline: somente os casos positivos A4f-vazio devem
falhar. Guardas negativas e casos com attachment já devem passar.

## 6. GREEN e validação externa

### 6.1 Testes Rust

Executar, no mínimo:

```text
cargo test -p typst-infra --lib p1478 -- --nocapture
cargo test -p typst-infra --lib p1475 -- --nocapture
cargo test -p typst-infra --lib p1476 -- --nocapture
cargo test -p typst-infra --lib p1477 -- --nocapture
cargo test -p typst-infra --lib
cargo test -p typst-core --lib
cargo test -p typst-shell --lib
cargo test -p typst-wiring
```

Qualquer falha pré-existente precisa ser reproduzida no baseline exato e
registada; não a classificar apenas por memória de passos anteriores.

### 6.2 Matriz veraPDF obrigatória

Recompilar `target/release/typst`. Para cada perfil, compilar a mesma fixture
e validar com o flavour explícito do veraPDF 1.30.2:

```text
A1a A1b A2a A2b A2u A3a A3b A3u A4 A4f A4e — sem attachment
A3a A3b A3u A4f A4e                           — com attachment
```

Meta: `isCompliant=true`, `failedRules=0`, `failedChecks=0` nas 16 linhas.
Não aceitar detecção automática de flavour como prova; usar `-f 1a`, `-f
1b`, ..., `-f 4f`, `-f 4e` explicitamente.

Também executar `qpdf --check` e inspeção QDF focal de A4f/A4e sem anexo.

## 7. Gates de selagem

```text
cargo build --workspace
cargo fmt --check
cargo clippy -p typst-infra --all-targets
crystalline-lint .
crystalline-lint --checks v5,v15,v26 .
git diff --check
```

Todos precisam terminar sem erro e V5/V15/V26 precisam ficar em zero. O erro
`[PARSE]` herdado em `tests.rs:11146` é um bloqueador conhecido do gate
literal: P1478 não o piora porque não toca esse ficheiro, mas também não pode
ser marcado `MATERIALIZADO` enquanto `crystalline-lint .` continuar falhando.
Se a sanitização do agregador ainda não tiver sido executada, fechar o código
funcional como GREEN e deixar o passo explicitamente **NÃO SELADO**, nunca
converter a falha em exceção silenciosa.

Depois do código GREEN, executar `crystalline-lint --fix-hashes .` somente
com V15/V26 válidos, rever o diff gerado e repetir os dois comandos de lint.

## 8. Não-objetivos

- PDF/UA-1 — permanece frente separada.
- `color.spot.tint`, gate de `asset` e adjudicação das 42 extras.
- Emitir `/AF []` sem attachments — proibido por este contrato; o vanilla
  A4f vazio exige `/EmbeddedFiles`, não um array `/AF` vazio.
- Árvore vazia para A4e/A3 — medição e fonte pinada dizem explicitamente que
  não é exigida.
- Igualdade de bytes, object IDs, forma inline ou ordem física dos objetos.
- Alterar a forma de anexos P1286/P1476, MIME, `/Params`, datas ou nomes.

## 9. Critério de fechamento

P1478 fecha somente quando, no mesmo estado commitável:

1. A4f sem anexo passa veraPDF integralmente.
2. As outras 15 combinações da matriz continuam com zero falhas.
3. RED→GREEN focal e regressões P1475-P1477 estão registados.
4. O L0 contém a retificação histórica, o contrato P1478 e hash resselado.
5. `crystalline-lint .` e V5/V15/V26 estão em zero.
6. O relatório final contém commit, tree, hora, `git diff HEAD --stat` se a
   árvore não estiver commitada, hashes dos dois binários e contagens
   completas do veraPDF.

## 10. Resultado (2026-09-22) — GREEN, NÃO SELADO

Ver `00_nucleo/prompts/infra/export/builder.md` §P1478 "Execução" para
o relato completo (achado de integração, RED→GREEN, gates, matriz
veraPDF). Resumo dos seis itens do critério de fechamento (§9):

1. **Satisfeito.** A4f sem anexo: `isCompliant=true, failedRules=0`.
2. **Satisfeito.** Matriz completa 16/16 conforme (11 sem anexo + 5 com
   anexo), incluindo A4e sem anexo continuar `/EmbeddedFiles` ausente
   (regressão negativa confirmada).
3. **Satisfeito.** 6 testes novos (`p1478_pdfa4f_empty_embedded_files_
   tests`, dentro de `builder.rs` per §5), RED→GREEN registado; P1475
   (11/11), P1476 (9/9), P1477 (11/11) e `p307b` (9/9) sem regressão.
4. **Satisfeito.** `infra/export/builder.md` §P1478 contém a
   retificação de §0, o contrato, e o hash foi resselado
   (`crystalline-lint --fix-hashes .`, aplicado com sucesso apesar do
   achado [PARSE] — ver item 5).
5. **Não satisfeito literalmente, per exceção explícita do próprio
   passo.** `crystalline-lint .` (default) e `--checks v5,v15,v26`
   continuam bloqueados pelo achado `[PARSE]` em `tests.rs:11146`
   (herdado de P1477, tamanho acumulado do ficheiro — P1478 não toca
   `tests.rs`, não agrava o achado). V5/V15/V26 = 0 confirmados
   **manualmente** na saída completa do run default (nenhuma linha
   `[V5]`/`[V15]`/`[V26]` além do drift já resselado) — não é uma
   confirmação automatizada pela própria ferramenta.
6. **Satisfeito** — proveniência abaixo.

### Proveniência da medição final

- Commit no momento do fecho: `23d827664` (commit L0-only anterior a
  este passo); árvore não commitada neste ponto —
  `git status --short`: `M typst-passo-1478.md`, `M builder.md`,
  `M builder.rs` (3 ficheiros, únicos tocados por este passo).
- Hora: `2026-09-22T14:45:40-03:00`.
- Binário cristalino release recompilado: SHA-256
  `183e0f3181511bf98fca7cc27026a0cb12154d95a925f224f67116608fc6cd4a`.
- Binário vanilla ratificado: SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`
  (inalterado, mesmo de sempre nesta sessão).
- Fixture sem anexo: SHA-256
  `93b3d9d54a454cf7d3f86ca780f3d30244759125ea4cb90e95ae300142f547d1`
  (idêntica à do passo, confirmada por `sha256sum` antes da medição).
- veraPDF: 16/16 combinações `isCompliant=true, failedRules=0`
  (tabela completa em `builder.md` §P1478).

### Achado de integração registado (não previsto pelo passo)

`pdf_indirect_ref_after` (helper de `emit_attachments`) tinha um bug
pré-existente, exposto pelo caso 6 do §5 (`/Dests`+`/EmbeddedFiles`
coexistindo): a comparação de token `"R"` falhava quando colada sem
espaço a um delimitador PDF (`"R>>"`, caso em que a chave procurada é
a última antes do fecho do dicionário). Corrigido — ver `builder.md`
§P1478 para o detalhe completo.

### Não selado — o que falta

A sanitização dedicada do agregador de testes (`03_infra/src/export/
tests.rs`, achado `[PARSE]` do `crystalline-lint`, origem em P1477,
tamanho acumulado do ficheiro) é um passo próprio, fora do escopo
deste. Até essa sanitização, `crystalline-lint .` continuará a
reportar exit 1 nesta árvore — não é uma regressão de P1478 nem motivo
para reverter o código funcional GREEN deste passo.


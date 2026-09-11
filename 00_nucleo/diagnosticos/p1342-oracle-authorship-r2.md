# P1342 — revisão focal R2 dos oráculos

Regime: **executado sem atestacao de isolamento**. Papel: autor independente
de oráculos, sem leitura de candidato futuro P1342. Veredito:
**ORACLE_REVISED_NOT_VERIFIED_NOT_SEALED**.

R1 permanece intacto. Esta revisão responde somente aos bloqueios B01–B05 e
aos negativos válidos A01–A21 do parecer adversarial R1. Não é pré-selo,
autorização de implementação, teste A/B, certificado nem conclusão sobre o
produto.

## Divisão composite

O checker R2 julga duas entradas separadas:

1. o DTO candidato, composto por `normal`, `repeat` e `reverse`, cada qual com
   challenge, snapshot cru, recibos de append, células, projeção e classificação;
2. um envelope de evidência de source/runtime fechado, cujo SHA-256 é fornecido
   ao `judge` fora do DTO.

Assim, nenhum membro do DTO pode declarar cobertura, alcance de fixture,
posição de hook, writer único, freeze cru, challenge emitido pelo driver,
Location ou snapshots observados. O futuro testador/verificador deve produzir e
pinar esse envelope. A evidência construída pelo runner do corpus é apenas um
fixture do formato: não certifica source nem runtime de candidato algum.

Os pins externos exatos são os bytes do contrato R2
`62d656241ecd96cd3d984025e59990d607a0efd75c3c326503c7ec07a3486c4d`,
manifesto R2
`3b522cf1b0f288fe1e472f21f775089541835ede754250a14a3c2d17c17e9d48`,
fixture `98159f5a...` e freeze L0 `2fb962c3...`. Manifesto fornecido que não seja
estruturalmente idêntico aos bytes pinados é `Violated` antes de qualquer DTO.

## Hooks, challenge e identidades

O manifesto contém 19 linhas estáticas. Para cada célula de origem aplicam-se
exatamente 18: H00D e H00S são duas linhas distintas e mutuamente exclusivas;
uma célula recebe uma, nunca ambas. O envelope externo fecha todas as 19
inspeções de source e exatamente 18 contagens por célula, incluindo zeros
exigidos pelo `attempt-kind` e por `R`.

Cada run recebe 32 bytes e usa seu SHA-256 como namespace. Identidades de run,
ledger, célula, carriers, Funcs, SyntaxNode, Content, Location, snapshots, Dicts
e raw object contêm o commitment e são locais. Os conjuntos concretos de IDs de
`normal`, `repeat` e `reverse` devem ser disjuntos. Os três challenges no corpus
são vetores distintos de oráculo; frescura imprevisível no candidato requer o
recibo do driver no envelope externo.

A06 não é descartado: substituir o namespace derivado por IDs fixos produz
`Violated`. A estabilidade de três execuções com IDs iguais não dá crédito.

## Raw, recibos e projeção

Cada evento fechado é canonizado no subconjunto RFC 8785 sem floats. O recibo
de append é:

```text
SHA-256(
  "P1342-APPEND-V2\0" || challenge32 || u64be(seq) ||
  previous_receipt_or_32_zero_bytes || utf8(hook) || event_digest
)
```

Há um recibo por evento, `hook_hit` real por hook/célula, `seq` global contíguo
e predecessor criptográfico. Antes da projeção, H16 congela o raw; seu digest é
registrado externamente antes e depois. A projeção precisa ser imagem read-only
das células cruas e conservar ambos os digests.

A07 também permanece válido: reencadear ordinais na projeção depois do freeze,
mesmo produzindo uma sequência visualmente coerente, diverge do raw e dos
recibos e é `Violated`. Se um futuro candidato reconstruir também o raw antes do
freeze, o envelope pinado precisa demonstrar writer único, hooks produtivos e
inspeção direta; JSON sozinho nunca basta.

## Fecho B01–B05

- B01: manifesto exato, enums/metadados fechados, 19 rows pinadas e evidência
  externa por símbolo, branch Rust, `cfg(p1339_observation)`, decoys e alcance.
- B02: `session_carrier` em toda a célula e `occurrence_carrier` em todos os
  passos da criação ao replay-exit; endpoints With, Func/SyntaxNode, Location e
  snapshots pre/post são contínuos e confrontados com observação externa.
- B03: schemas fechados de raw, recibo, célula, evento, refs/data, projeção e
  evidência; evento extra também é validado; `R` vem dos hits H12A externos.
- B04: challenge emitido pelo driver, IDs challenge-bound e conjuntos disjuntos;
  raw object, Location e snapshots aparecem no envelope independente.
- B05: pins/source, raw/challenge/recibos, schema/tipos e causalidade são
  avaliados nesta ordem; só depois a opacidade deliberada pode gerar `Unknown`.

Os ranges continuam os reais: context-body `131..177`, occurrence
`144..164`, dict-prebound `155..162`, syntax-body `64..121` e dict-witness
`82..97`. `syntax-body` é `SyntaxNode`/`CodeBlock`, nunca `Func`. Os Dicts são
listas ordenadas de pares e `bool` não satisfaz inteiro.

## Corpus fechado e resultado da calibração

O corpus tem exatamente 23 casos:

- `P01-inspectable-composite` → `Preserved`;
- `P02-opaque-composite` → `Unknown`, com somente o witness opaco;
- A01–A21 → `Violated`, todos marcados como negativos válidos.

Os casos cobrem, sem expansão, dead branch, `cfg(test)` decoy, metadados/enum,
IDs fixos, rechain pós-hoc, carriers em todos os elos, Location, snapshot post,
eventos extras/duplicados, bool, span trocado, attempt-kind/`R` e precedência de
`Unknown`.

A primeira execução R2 rejeitou 20/21; A15 ainda passava porque a observação
externa do snapshot post não era comparada ao endpoint do raw. Uma revisão focal
adicionou essa igualdade. A segunda execução rejeitou 21/21, sem regressão dos
dois controles: score local de calibração `1.0`. Isso consome uma revisão focal,
dentro do budget; não é o score independente de pré-selo.

`normal`, `repeat` e `reverse` percorrem os mesmos 23 IDs (reverse em ordem
inversa), cada caso materializando três runs/challenges. A classificação por ID
é idêntica nas três ordens.

## Limite

Não se fecha lifecycle/profile P1340, NT01–NT06, retenção, descarte,
invalidação, política terminal, equivalência geral ou aceitação de candidato.
O próximo poder de decisão pertence ao adversário/verificador independente.

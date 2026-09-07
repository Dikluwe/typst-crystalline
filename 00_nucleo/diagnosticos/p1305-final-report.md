# P1305 — a distinção entre global e import não cabe somente em repr

## Resultado

**Implementação interrompida antes do patch.** O controle exigido pelo próprio
P1305 refutou a hipótese de que os doze caminhos poderiam ser corrigidos
somente no formatter. Não houve alteração de L0, produto ou testes.
Veredito independente: `P1305_BLOCKED`, causa
`INSUFFICIENT_CARRIER_FOR_REPR_ONLY_SCOPE`. O verificador reproduziu os
dois binários no perfil padrão, conferiu os canais integrais, a cópia de
baseline e a ausência de alterações produtivas.

Não é a elisão dos arrays que impede o avanço. O problema é representar o
global como `<module global>` sem também renomear um arquivo comum chamado
`std.typ`. A coorte não foi reduzida silenciosamente para implementar só arrays.

## Contraprova reproduzida

O fixture `reexport/std.typ` contém exatamente:

```typst
#import std: *
```

O documento principal importa esse arquivo com alias, preserva um alias do
global real e observa ambos. No perfil padrão, em ordem normal e invertida:

| Objeto observado | Vanilla ratificado | Cristalino baseline |
|---|---|---|
| Global `std` e seu alias | `<module global>` | `module(std)` |
| Import ordinário `std.typ`, com `x = 7` | `<module std>` | `module(std)` |
| Import `std.typ` que reexporta `std: *` e seu alias | `<module std>` | `module(std)` |
| Import ordinário `global.typ` | `<module global>` | `module(global)` |
| Import ordinário `map.typ` | `<module map>` | `module(map)` |

Os kinds são `module`; os lookups ordinários retornam 7, 8 e 9 e
`calc.abs(-7)` retorna 7 tanto no global quanto no reexport. Os arquivos,
comandos, valores completos e canais brutos estão em
`p1305-pre-measurement.json`; não são expectativas derivadas do formatter.

A inspeção independente de fonte explica a colisão:

- `eval/mod.rs:324,562` e `eval/modules.rs:66` constroem o global com
  `Module::new("std", stdlib.clone())`.
- `eval/modules.rs:215-218` copia os bindings do wildcard em ordem;
  `:113-114` constrói o import com o nome do arquivo e seu scope exportado.
- `entities/module.rs:32-44,62-70` não guarda procedência. Conteúdo,
  introspecção, metadados e estilos ficam nos mesmos defaults nesses dois
  caminhos; o import descarta seu conteúdo. Scope/Binding também não
  preservam origem.
- `repr_value(&Value)` não recebe uma referência canônica ao global.
  Endereços/identidade Arc ou uma lista de builtins não recuperam a origem
  perdida; o reexport transporta os próprios valores da stdlib.

Assim, trocar apenas o nome `std` por `global`, ou reconhecer a stdlib pelos
bindings, não resolve o contrato. A regra nominal existente nos diagnósticos
não prova que essa transformação seja válida para a repr.

## Proveniência e limites da medição

HEAD: `8eb41b769eb840c7ab1063f981f98fdd4047952b`, working tree com
o candidato P1303 certificado ainda não commitado. A única diferença desse
HEAD para o HEAD histórico do P1304 é o documento P1305. O manifesto
`p1305-manifest.json` preserva o status completo, arquivos e pins, capturados
em `2026-09-07T13:12:34.238628+00:00`; o diff produtivo é:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  80 +++++
 00_nucleo/prompts/compiler/eval/tests.md           | 100 +++++-
 01_core/src/compiler/eval/bindings/field_access.rs |   4 +-
 01_core/src/compiler/eval/tests.rs                 | 382 ++++++++++++++++++++-
 4 files changed, 562 insertions(+), 4 deletions(-)
```

Essas mudanças são anteriores ao P1305 e permaneceram intactas. O diff
binário tem SHA-256 `fca8f14b7fcad814ca954d8933c05a6e2644683c9a4529d9f169bcfb5d1c2a49`.

O binário RAM P1304 já não existia. O baseline foi reconstruído offline em
cópia isolada `/tmp/p1305-baseline.rxgh2e`, com target exclusivo
`/dev/shm/p1305-target.3j4tck2g`. A cópia foi reconferida contra os 527
arquivos de fontes/configuração considerados pelo build; nenhum diferiu.
O comando e a duração estão em `p1305-build.log`, iniciado em
`2026-09-07T13:14:08.697778+00:00`, exit 0. Binários:

- cristalino reconstruído: SHA-256
  `4a4e1bd46c053dd19bfcae2230537c9478fbfb2ff26a94dfd87d9f29fee2835d`;
- vanilla ratificado `a51e02804`, `/usr/local/bin/typst`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Houve uma revisão focal do fixture: `str(type(...))` não era suportado no
cristalino e foi substituído por `repr(type(...))`. A primeira tentativa,
seus fixtures e saídas foram preservados dentro da medição; não contam como RED.
A tentativa vigente registra 16 processos: os quatro do perfil padrão
(dois binários, duas ordens) são comparações válidas. Nos demais perfis,
as seis execuções cristalinas foram rejeitadas pelo parser CLI porque query
não aceita `--features`; os respectivos resultados vanilla não tornam esses
pares comparáveis. Não há alegação de confirmação bilateral nos quatro perfis.
Warnings de depreciação de query foram preservados e não são prova de paridade.

Um contraexemplo válido no perfil padrão basta para refutar a suficiência
universal do escopo. Não se converteu falha de ferramenta em defeito de
Module, `Unknown` em PASS, nem erro de fixture em RED semântico.

## O que precisa de decisão

**Proposta para reabrir o escopo, ainda não implementada:** investigar o uso
do nome público `global` nos três pontos existentes de construção, sob os
owners `compiler/eval.md` e `compiler/eval/modules.md`, além de `repr.md`.
Os imports ordinários preservariam seus próprios nomes. Antes de codificar,
medir efeitos em bare imports de Module, aliases, diagnósticos e nas três
rotas de avaliação, e atualizar os L0 correspondentes.

Isso é uma hipótese menor que criar um novo campo de identidade. A evidência
não obriga a criar entidade/API; a classificação ADR-0127 concreta depende
da revisão dessa hipótese. A parada atual decorre da fronteira explícita do
P1305: ele proíbe mudar a construção do std e manda parar se repr sozinho
não puder distinguir os objetos. A skill de materialização segregada reforça
essa parada por informação insuficiente no contrato, sem emitir selo.

Nenhuma etapa posterior foi apresentada como concluída: L0-first, selo,
RED→GREEN, mutantes, 627 probes finais, workspace tests e gates de produto
não foram executados. Os doze caminhos continuam abertos, assim como as
dívidas auxiliares já registradas pelo P1304. Não houve staging ou commit.

Regime desta auditoria: `executado sem atestação de isolamento técnico`.
Autor de contrato e verificador trabalharam separadamente; o filesystem
compartilhado impede alegar isolamento técnico.

## Evidência final ligada, sem ciclo

| Artefato em `00_nucleo/diagnosticos/` | SHA-256 |
|---|---|
| `p1305-manifest.json` | `783cf02f9832e7a06ce0e53a1bc356ee5ca13eb3f225fc506649c023cc665b3b` |
| `p1305-pre-measurement.json` | `e2b3ef86b7893815cc700f09c95701f47485f4f24fff3f26571b087adffcf834` |
| `p1305-contract.md` | `bd7dfc46dbcd93087b7abd6c67bd64938c0162344cfb38606a4163083dcab038` |
| `p1305-verification-receipt.json` | `bcf6dd2e4092266ca6c294551aeeeeaddc8832c7b8c7d795da96f5ab57dd2c87` |
| `p1305-certificate.json` | `008be227d322a764f56b368f42291b5fe4a057b3cc41f67ff4fc5fc2776442b4` |

O certificado antecede este fechamento e não pina o relatório. Não há selo
de implementação nem veredito de paridade: a conclusão certificada é o
impedimento do escopo atual, com um contraexemplo suficiente e reproduzível.

# P1287 — receipt de verificação final segregada

**Estado:** `VEREDICTO_GLOBAL_CONFIRMADO_COM_LIMITACAO_DE_ATESTACAO`  
**Veredito confirmado:** **PARIDADE PARCIAL**  
**Papel:** Verificador segregado `/root/verificador_p1287`  
**Janela de verificação:** `2026-08-30T23:05:39-03:00`–`2026-08-30T23:10:20-03:00`  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`  
**Árvore:** working tree não commitida e compartilhada  
**Atestação:** executado sem atestação de isolamento forte do host.

## Autoridade e não interferência

O verificador leu integralmente a skill `tekt-materializacao-segregada`, as
duas referências diretamente exigidas, `AGENTS.md`, somente o passo P1287
explicitamente autorizado e os artefatos P1287. Não alterou contrato,
baseline, oráculos, testes, fixtures, adaptador, código produtivo, Prompt L0 ou
relatório. A única escrita persistente deste papel é este receipt.

## Integridade da cadeia protegida

| Artefato | SHA-256 verificado |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| passo P1287 | `eb0622dd0fe195dfb4ed0dcdae079d0bc8c0162837531358a3598b7c9f0e32a8` |
| receipt de baseline | `3bbba47235e4865bb3c9e788bb709215dd0a53297ee587d4334a01ca42126cd6` |
| manifesto | `5d4cc9a6180300c8402be4a91b30104db08874f8540c9bc8af5b895a9fdf725b` |
| receipt de contrato | `4683d23a5a489e258931ea3389be809c7b4e5b143c4f7a66ec7210c4dbd4f7a5` |
| runner global | `745a4c470704de54f9a550d220a3092bfa8452e3b02cb7010846758eab36bd37` |
| testes do runner | `3ac4daa6ff8cb2c9d8396ec4288b737c8496d0473c5d5d901165574af241c535` |
| baseline de oráculo | `ff09c8146b73684b2e746c631a6103817b6df7070aa8248e2d88dbca34749488` |
| receipt de oráculo | `b8ba1fa8d3d7bf1a305deee400ed811b5e034888ec4aeb3797392a95ea97e109` |
| plano adversarial | `af92a768ab0acdab681192c0525c01a96f56b21942915b10f8b3dc204925db3f` |
| receipt adversarial | `0255970c6f858d69ae733f490c645c394a230f006c7ddee3419d9a65d081f8b0` |
| adaptador candidato | `152fe7ae7f65dcd015a24951e33cd682bda8590609dd20ef3fcc7ad2c8dc7679` |
| testes do adaptador | `dabfe005488bbca84373ebaa93b3b3edd64ad93b75ff0e03d228007cafb2b490` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| `target/release/typst` | `acc52526e1c1cfde21c4583857330a6f64540f46caf1b7fa89c666da6deee64a` |
| relatório final verificado | `7ca2da4a24bcc04e588e28090010062d68c7b32290dda8a02da966da647e9d76` |

`python3 lab/parity/matrix/p1287_global.py --self-test` passou e verificou
manifesto, suíte, fixtures e baseline congelados. A ordem causal documentada é
baseline → contrato → oráculos → campanha adversarial → execução candidata;
os hashes protegidos continuaram iguais depois dos gates finais.

## Reexecução do contrato bilateral

O comando

```text
python3 lab/parity/matrix/p1287_global.py \
  --binary lab/parity/matrix/p1287_candidate_adapter.py
```

terminou com exit `1` e reproduziu `13 Violated` e `10 Unknown`, sem converter
incapacidade em sucesso. Foram ainda reproduzidas diretamente quatro
testemunhas de linguagem: o vanilla aceitou C05, C07, C08 e E11; o candidato
rejeitou, respectivamente, alinhamento de tabela por array, `link(label)`,
`image(format:)` e o carrier HTML válido. Uma única testemunha dessas já
satisfaz a regra global negativa do manifesto; as demais e os `Unknown`
continuam listados no relatório.

A identidade por bytes do baseline está pinada, mas a associação independente
desses bytes ao commit upstream declarado permanece `Unknown`, como já
congelado. Isso limita qualquer alegação positiva geral, sem apagar diferenças
reproduzidas contra o executável pinado.

## Exportadores, focal e inventários

- A matriz focal foi reexecutada com exit `0`: `17 MATCH`, `2 DIFF` esperados
  e `19/19` expectativas satisfeitas. O JSON desta repetição tem SHA-256
  `a139984975d75058b8af81deecc1146de6b67505a7f2a710799a28f2c5c7ac26`;
  runner e manifesto focal mantiveram, respectivamente,
  `25e025b80d9c19fb1bce2e3d3c6f7caa9d0a680c63a17e9d6e5c6e27e0da8d5b`
  e `9cdcb75527d5c0f2ae5c69cb2d7238c0c558c5fc27f65ef149b8e98c3fee241e`.
- O PNG focal foi recalculado em `1191 × 1684`: exatamente `11` pixels
  distintos, delta máximo de canal `1` e média absoluta por canal
  `4.113392007754118e-06`, nas mesmas coordenadas e pares RGBA do relatório.
  Como não existe máscara pré-candidata congelada, a igualdade RGBA exata
  continua falhando e nenhuma tolerância é promovida.
- No PDF focal, `qpdf --check` passou nos dois lados; página A4, texto
  `Hello, parity.`, embedding, subset, Unicode, encoding `Identity-H` e tagging
  coincidiram. A diferença observada foi o nome nominal
  `LibertinusSerif-Regular-Identity-H` versus
  `LibertinusSerif-Regular`. No carrier E02 amplo, o candidato voltou a gerar
  avisos xref de offset zero para objetos `26`–`125`, e `qpdf` terminou com
  exit `3`; o `Unknown` conservador do runner é reproduzível.
- Os hashes dos inventários temporários conferem com o relatório: padrão
  `a9233711b83be0af44464271dfda9479412569fe791a585a7e75c9e4aa1489b8`
  e HTML
  `68efbabc8cc19c3b0bbcd52ab752ad55561b0544727d9e51b483d612fc846b7b`.
  As contagens foram reextraídas como `1552/71/386/45` e
  `1553/122/449/45` para `MATCH/MISSING_MEMBER/UNVERIFIED_METADATA/EXTRA_BINDING`,
  com os demais estados zerados. Os probes pinados reproduzem `111` casos
  (`72` iguais) e `115` (`77` iguais). Nada disso foi convertido em percentual
  de linguagem.

## Campanha adversarial e limite de atestação

O resultado temporário ainda estava presente em
`/tmp/p1287-adversary.0MLDpN/campaign-result.json` e o seu SHA-256
`1c27f94bcd4d6ba2c843ad6e80d16be91584490cc0c4ec85ae7992642ffa88e6`
confere com o receipt. Ele contém `12` mutantes válidos, `12` mortos e `12`
restaurações, com os hashes injetados e restaurados iguais aos registrados; os
artefatos protegidos também permaneceram intactos.

Limitação: o driver efêmero da campanha e a sua invocação exata não foram
persistidos; somente o resultado canônico temporário, o plano e o receipt
estão disponíveis. Por isso este verificador corrobora o resultado registrado,
mas não emite certificado de replay independente do gate de mutação. Essa
limitação reduz a força de atestação do protocolo completo, sem alterar o
veredito global, que foi confirmado por reexecução bilateral direta.

## Gates finais reexecutados

| Gate | Resultado do verificador |
|---|---|
| `cargo test --workspace` | exit `0`; suíte e doctests passaram |
| sete módulos Python de runner/inventário | `90` testes, zero falhas |
| runner e adaptador P1287 | `10` testes, zero falhas |
| `run_p1286_oracles.py --binary target/debug/typst` | exit `0`; `58 Preserved`, `2 BaselineOnly`, `1 NotApplicable`, zero `Unknown`/`Violated` |
| corpus bilateral P1287 | exit `1` deliberado; diferenças classificadas |
| matriz focal | exit `0`; `19/19` expectativas satisfeitas |
| `cargo build --workspace --bin typst` | exit `0` |
| `cargo fmt --all -- --check` | exit `0` |
| `crystalline-lint .` | exit `0`; warnings V16/V19/V20 emitidos |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | exit `0`; `No violations found` |
| `git diff --check` | exit `0` |

No snapshot anterior à criação deste receipt, `git diff HEAD --stat` produziu
`124 files changed, 647273 insertions(+), 1789 deletions(-)`, SHA-256
`8b22c5e3e6a3427dde83ddce564dfa45986b4d49f341a92be0619104f4ec1d5c`;
`git status --short` teve SHA-256
`d11505c6b4d492eb3471661eff297486a5457f09b239fe1738c5e97b1fcad5c7`.
Esses valores descrevem a árvore compartilhada e não autoria isolada.

## Veredito do verificador

O relatório aplica corretamente a regra global do contrato: há diferenças de
linguagem bilaterais, reproduzíveis e com binários pinados, além de resíduos
explicitamente preservados como `Unknown`. Confirmo, portanto, o veredito
**PARIDADE PARCIAL**, limitado ao baseline por bytes e aos observáveis
registrados, sem equivalência funcional geral e sem atestação de isolamento
forte.

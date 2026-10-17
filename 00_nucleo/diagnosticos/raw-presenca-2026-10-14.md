# Raw: presença lexical aprovada e materializada — 2026-10-14

A mudança aprovada está implementada: omissão de `block`/`lang` permanece distinta
de `block: false` e `lang: none`. O contrato Rust acrescenta `RawElem.set_fields: u8`,
`RAW_SET_BLOCK = 1` e `RAW_SET_LANG = 2`; `Content::raw` conserva a assinatura.
Construções externas por struct literal precisam informar o novo campo, conforme
avisado na proposta. RawLine, fase, render e casts legados não foram alterados.

## Proveniência e autorização

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. A lista exata de alterações
`git diff HEAD --stat`, status, UTC, fontes e SHA-256 está em before-source.json.gz,
red-state-final.json.gz, candidate-final.json.gz e workspace-pre.json.gz. Não houve
commit, push ou publicação. Fontes mantiveram os mesmos hashes durante validação
completa. Suíte: 2026-10-14T09:55:52.486231+00:00 → 2026-10-14T10:01:53.724652+00:00.

Manifesto aprovado `b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45`; resposta humana
`Aprovar a proposta`, registrada em materialization/raw-presenca-2026-10-14/approval-receipt.json.
O manifesto e os candidatos permanecem byte a byte idênticos aos aprovados. A cópia
canônica atualiza a ratificação e posiciona os metadados no formato exigido pelo
linter. Encoding/CLI tinham avançado após a proposta; suas correções foram preservadas.

Oráculo: vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489,
SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
`before/results.json.gz` e `after/results.json.gz` registram argv, saídas, fonte,
UTC, HEAD/estado e SHA-256 de cada binário. Não se usa a versão textual como prova.

## Medição e decisão

Na fonte inicial, RawElem raw.rs:21–25 só conservava valores; native_raw
markup.rs:78–136 perdia presença. O novo getter retorna somente valores conhecidos;
Content delega, has/at/fields distinguem Set/Unset/Undeclared, igualdade e observação
incluem presença, e CLI/encoders usam a projeção comum. A falha era do modelo de
código e seus consumidores; a especificação aprovada agora legitima a correção.
Não é uma classificação estatística das falhas do projeto inteiro.

O corpus bilateral contém 70 comparações de caso/formato,
incluindo controles. Coincidências semânticas: 26 antes,
66 depois. As quatro diferenças restantes são dois casos
legados em dois formatos: raw() aceita texto ausente e block:none é aceito como
omissão no Tekt; o vanilla rejeita. Estão fora deste gate, sem crédito de paridade.
JSON/YAML são comparados como valores; a ordem text/block/lang tem teste próprio.
Erros comparam mensagem causal; saídas integrais permanecem disponíveis.

As cinco amostras PDF (inline, bloco, markup, Bool/Auto e show) têm pixels idênticos
antes/depois e passaram qpdf; hashes de pixels, fontes e textos estão em comparison.json
mais render-before/render-after. Isso comprova preservação nesses controles, não
paridade universal de render. A amostra markup também foi inspecionada visualmente.

## Testes, linhagem e limites

- Dez testes focados passaram: métodos/campos, aliases/Source, igualdade,
  JSON/YAML, diagnósticos, markup/Bool/Auto, cópias/mapas, construtor Rust,
  observação e CLI/query com snapshot presente/ausente.
- Seis regressões unitárias foram observadas em RED antes da implementação;
  os encoders também têm RED bilateral. Falhas de fixture foram preservadas:
  Location::from_raw é privado em L2; decode não existe em json/yaml. Foram corrigidas
  para APIs reais, sem modificar esperados para acomodar o código.
- Suíte completa: **8495 passaram, 0 falharam,
  7 ignorados**. A tentativa anterior terminou com status 143 durante
  compilação, causa não estabelecida; não foi contada como teste aprovado/reprovado.
- Build offline/locked, rustdoc com -D warnings, all-target check, formato,
  diff check e linhagem V1/V5/V15/V26 estrita passaram, assim como reverse lineage.
- Núcleo raw-field-presence: pin `f29e26ff07053f940cfe4ab5358fc9a6ac8d4d689528e169b5f8567492f52f46`; nove owners 1:1
  atualizados, resselo restrito. Os bloqueios iniciais de posicionamento de
  metadados foram corrigidos e conservados nos logs.
- Clippy permanece com 181 avisos no inventário `--cap-lints warn`;
  isso **não** aprova o gate CI `-D warnings`. Linter completo: zero erros,
  382 warnings e 1627 notes.
  Uma nova nota V19 aponta o match text/block/lang com três alternativas;
  os três campos são cobertos pelos testes. Não houve aumento de warnings.

CI remoto/release, demais lacunas de produção e transporte de tiling permanecem
abertos. Este lote não certifica o projeto inteiro para produção.

## Evidências preservadas

[evidence.json.gz](raw-presenca-2026-10-14/evidence.json.gz) contém 87
entradas com caminho, SHA-256 e bytes em base64, relidas e verificadas. Inclui o
pacote aprovado, recibo, estados, patch, comandos, logs, medições, imagens e scripts
históricos; scripts de mutação não devem ser reaplicados à árvore já corrigida.

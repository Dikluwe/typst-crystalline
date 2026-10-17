# Transporte de tiling: contrato aprovado e tipos puros — T1a — 2026-10-14

A aprovação T1–T3 foi registrada e os tipos `FramePaint`, `TilingFrame` e
`FrameStroke` foram implementados em entities/frame_paint.rs, com módulo
público na fachada. Este lote é **T1a, incompleto**: ainda não troca os campos
de FrameItem nem muda o layout/export. Não fecha os defeitos de tiling nem
certifica produção. T1b conclui a migração de produtores/consumidores; T2
implementa os recursos PDF/SVG/PNG; T3 conclui as integrações geométricas.
A aprovação dessas etapas já existe; não requer nova pergunta sobre o mesmo
contrato. T4 e publicação não fazem parte da aprovação.

## Autorização e proveniência

Resposta humana “Aprovar a proposta” ao manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`.
Manifesto, contrato e todos os candidatos foram conferidos antes da edição;
nenhum arquivo base do pacote havia mudado. O pacote permanece imutável;
`materialization/tiling-transporte-2026-10-14/approval-receipt.json` registra
a ratificação. A aprovação de Raw é distinta e já estava materializada.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before-source,
red-state, candidate-final, frame-consumers e workspace-pre conservam UTC,
status, `git diff HEAD --stat`, fontes e hashes. A suíte executou de
2026-10-14T12:29:42.082256+00:00 a 2026-10-14T12:35:54.855679+00:00, com hashes dos fontes inalterados.
Nenhum commit, push, PR, publicação ou release foi realizado.

## Medição e implementação

A medição anterior do contrato aprovado mostrou a repetição limitada a 10000
células, fallback sólido e ausência de transporte de recursos. Ela continua
válida para seu estado registrado; não foi repetida nem usada como prova de
melhora visual neste lote. Esta rodada só estabelece a representação pura.

`FramePaint` mantém Solid/Gradient integrais e uma célula preparada em Arc.
`TilingFrame` conserva Frame, spacing assinado, offset resolvido, ângulo e
Self/Parent. `FrameStroke` conserva todos os campos geométricos e presença;
seu Default coincide com Stroke::default. A unidade não executa layout,
I/O ou render. Não existe conversão total de Paint declarativo nem igualdade
de linguagem nova. Validação de finitude/pitch e materialização são obrigações
dos produtores em T1b/T2, não garantia de valores arbitrários construídos por
struct literal no carrier público. A ausência de dados declarativos em toda
a árvore do Frame também depende da migração de Shape em T1b; não está
certificada pela mera existência destes tipos.

Quatro testes unitários verificam cor/gradiente integral, compartilhamento da
célula e placement, defaults e retenção da geometria/presença do contorno.
O RED foi erro de compilação por ausência dos tipos aprovados; não é RED de
render. Depois, os quatro passaram. Dois doctests compile_fail protegem a
fronteira contra From<Paint> implícito e PartialEq do carrier.
O compartilhamento Arc é evidência de custo de transporte constante por clone,
não definição de paridade de linguagem nem medição de desempenho do render.

## Validação e linhagem

- Suíte completa: **8521 passaram, 0 falharam,
  7 ignorados**, processo concluído com status zero.
- Build offline/locked, rustdoc com -D warnings, all-target check, fmt,
  diff check, V1/V5/V15/V26 estritos e reverse lineage passaram.
- Clippy inventariado com --cap-lints warn: 181 avisos,
  sem aumento. Isso não aprova o gate CI -D warnings. Linter completo:
  zero erros, 382 warnings,
  1629 notes, sem aumento.
- Núcleo aprovado tiling-frame-transport, pin
  `4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee`,
  instalado e pinado pelos dois owners. Resselo limitado a frame_paint/mod.
  V26 bloqueou um pin duplicado durante a instalação; corrigido e evidência
  preservada, sem contornar o gate.

O inventário lexical frame-consumers contém caminhos, linhas e hashes dos
usos de Shape/Group, inclusive testes e padrões. Não é contagem de falhas nem
de arquivos que necessariamente precisarão mudar. Recursos de fontes/imagens,
shaping, padrões aninhados e metadados precisam de auditoria própria em T2;
FrameVisitor genérico não deve transformar decoração em texto da página.

## Continuidade e limites

T1b: atualizar owners antes de migrar Shape.fill/stroke e Group.paint_root;
preparar fill e stroke no layout, sem conversão que esconda tiling em cor.
T2: PatternType1/PDF, pattern/SVG e textura/PNG, recursos recursivos,
clipping/fill-rule e sobreposição/densidade. T3: controles de Parent e
geometria em Box/Block/Place/Rotate/Scale, preservando divergências Tekt
comprovadamente corretas. Construção externa de Shape/Group ainda não mudou
neste sublote; a quebra aprovada ocorre na migração T1b.

A investigação block(outset) foi deixada pendente para priorizar a nova
aprovação; nesta retomada houve apenas leitura de seus fontes. Raw continua
com sua aprovação/implementação anteriores. CI remoto/release e demais
correções de produção continuam abertos. Não existe classificação estatística
completa que permita atribuir a maioria dos defeitos a código ou prompts.

## Evidência preservada

[evidence.json.gz](tiling-carrier-2026-10-14/evidence.json.gz) conserva
47 entradas com caminho/SHA-256/base64, relidas e verificadas.
SHA-256 do arquivo: `2d353b8e920f12ba18ff00a150369ca8c4d52bb4e08b7374bafac5484495d8ed`.
Inclui pacote aprovado, recibo, fontes antes/RED/depois, patch, comandos,
logs e scripts. Scripts de mutação são históricos e não devem ser reaplicados.

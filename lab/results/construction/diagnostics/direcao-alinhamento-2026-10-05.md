# Direção de parágrafo e direção de trechos — 2026-10-05

O layout deixou de escolher a âncora de cada linha pela direção do primeiro item. Dentro do subconjunto textual certificado, mantém a direção compartilhada do parágrafo e conserva os overrides de direção nos TextStyle dos trechos. Foram corrigidos os controles de linha isolada com direção oposta, estilo integral, restauração por parbreak e última linha pendente. Ordenação bidi de scripts misturados, corpo de Block e align explícito continuam como limites separados, medidos e abertos.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. [initial.json.gz](direcao-alinhamento-2026-10-05/initial.json.gz) registra UTC, lista exata `git diff HEAD --stat`, status, todas as fontes produtivas/L0 herdadas, treze fixtures iniciais e comandos/caixas/textos bilaterais. [scope-before.json.gz](direcao-alinhamento-2026-10-05/scope-before.json.gz) inclui fontes iniciais dos sete owners e consumers afetados, recuperando de HEAD os que estavam limpos. [execution.json.gz](direcao-alinhamento-2026-10-05/execution.json.gz) arquiva fontes finais, hashes, logs e horários; [review.diff](direcao-alinhamento-2026-10-05/review.diff) isola esta etapa. Nenhuma alteração herdada fora desse escopo foi modificada.

Baseline SHA-256 `a0caf9bba4692f764b4cb3af3a30555e5b83995f59372775a3bf258419ec1f20`, cópia em `/tmp/tekt-direcao-alinhamento/baseline-typst`. Candidato final `64e35e6b7e69881b44c287daac6bc0d3bb668993f4019e97e1002909edcc8e27`, receita `cargo build --release`, árvore final no recibo. Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, árvore/receita ratificadas no P1489. CLI com `SOURCE_DATE_EPOCH=0`, `RAYON_NUM_THREADS=1`. Validação final iniciada em 2026-10-05 22:54:48 UTC; demais horários estão nos recibos. Não houve medição de desempenho nesta correção.

## Medição antes da decisão

Em página de 220pt com margem de 20pt, um parágrafo RTL contendo `مرحبا`, quebra forçada, `#text(dir: ltr)[ABC 123]`, quebra e `عالم` colocava ABC em x=20 no Tekt e x=152,949220 no vanilla. O inverso LTR com linha árabe colocava a linha árabe em x=174,567870 no Tekt e x=20 no vanilla. Já `#text(dir: ltr)` sobre o parágrafo inteiro, sob exterior RTL, coincidia à esquerda. Logo, trocar indiscriminadamente a direção do trecho pela direção exterior também seria incorreto.

Fonte local: `cursor.rs:156–162` e `line_commit.rs:210–216` no baseline consultam o primeiro Text/TextShaped da linha. Fonte vanilla: `flow/collect.rs:129–136` obtém `StyleChain::trunk_from_pairs`, e `inline/mod.rs:189,232` resolve direção e alignment compartilhados antes de construir as linhas. O observável da língua é a âncora e a direção de cada trecho; a estrutura de StyleChain, estratégia de preflight e campos privados são mecânica interna.

[paridade-producao-p576.md](paridade-producao-p576.md) documenta a introdução de alinhamento RTL baseado no estilo de itens da linha. Os controles históricos eram RTL/LTR simples e documento misto separado por vspace; não estabelecem paridade atual de linhas opostas no mesmo parágrafo. Isso liga a responsabilidade à implementação histórica, sem presumir intenção por inércia ou reconstituir a árvore exata daquela medição.

## Implementação e escopo

L0 primeiro, fluxo contínuo ADR-0127: correção interna/de paridade, sem entidade, assinatura pública, flag ou fase nova. Os campos novos são privados de Layouter. Nenhum `unsafe` foi acrescentado.

- `paragraph.rs` possui o resolver puro e a consulta de direção para fechamento. Certifica Text, Empty, Space/SpaceFromNewline, Linebreak, Sequence, Styled e Strong/Emph antes da emissão. Um escopo integral fornece seu estilo; siblings independentes usam o escopo exterior, não um consenso dos valores de direção de seus runs. Conteúdo desconhecido conserva fallback. Par explícito usa sua cadeia exterior.
- `sequence.rs` resolve segmentos separados por Parbreak no percurso incremental. Escopos aninhados herdam o contexto ativo; a cauda ainda aberta conserva a direção quando o escopo retorna. Um wrapper exterior recusado não apaga a direção certificada pelo filho.
- `cursor.rs` e `line_commit.rs` consultam esse owner para alinhamento/justificação. O flush limpa somente a direção pendente; o contexto ativo permanece nas linhas seguintes do wrap. Equações de largura, gaps, tracking, baseline e paginação não mudaram.
- `mod.rs` armazena dois Option<Dir> privados; `checkpoint.rs` captura/restaura ambos. `sub_frame.rs` isola os slots no contexto independente e os restaura junto da linha pai. Nenhum metadado novo foi transportado em FrameItem ou ao L3.

### Refutações durante a implementação

O teste manual inicial reproduziu o defeito e falhou; [red-state.json.gz](direcao-alinhamento-2026-10-05/red-state.json.gz). A primeira comparação CLI refutou o primeiro preflight: Space de newline após `#set` impedia reconhecer o estilo integral e deslocava até a primeira linha RTL. [first-candidate.json.gz](direcao-alinhamento-2026-10-05/first-candidate.json.gz) conserva a regressão. [tree-state.json.gz](direcao-alinhamento-2026-10-05/tree-state.json.gz) conserva a sonda da fonte realmente avaliada e seu código de teste. O resolver foi corrigido para não tratar Empty/carriers de espaço fraco como produtores substantivos; sua emissão/largura não é removida. Texto literal de whitespace continua texto.

A segunda comparação mostrou wraps intermediários corrigidos e última linha ainda errada: um wrapper exterior sem prova fazia pending=None após o filho guardar a direção. A retenção agora só é sobrescrita por direção Some; flush continua dono do reset. Há teste fonte→layout com diretiva de página e último run de direção oposta. Os testes adicionais exercitam restauração causal por checkpoint e isolamento/restauração do subframe. A tentativa inicial do teste de subframe usou a API inline com width preenchido e falhou no guard de uso; foi corrigida para a API geométrica apropriada, sem mudar esse guard produtivo.

## Validação final e limites

[verify.py](direcao-alinhamento-2026-10-05/verify.py) e [verification.json.gz](direcao-alinhamento-2026-10-05/verification.json.gz) registram quinze fontes, três binários e resultados completos. O verificador exige coincidência de texto e caixas nos nove controles fechados, não apenas imprime seu resultado. Texto normalizado remove whitespace e controles bidi U+202A–U+202E, sem reordenar caracteres. Caixas são associadas por palavra e baseline, com tolerância 0,05pt e ocorrência física mais próxima para repetidas; os casos fechados têm palavras identificáveis, sem usar a ambiguidade das repetições para certificar wraps.

| Controle | Resultado final contra vanilla |
|---|---|
| Run LTR sobre parágrafo inteiro sob RTL | texto e caixas coincidem |
| Parágrafo RTL com linha LTR intermediária | texto e caixas coincidem; ABC x=152,949220 |
| Parágrafos separados por linha em branco | texto e caixas coincidem |
| Duas linhas dentro do mesmo escopo LTR | texto e caixas coincidem |
| Run RTL sobre parágrafo inteiro sob LTR | texto e caixas coincidem |
| Parágrafo LTR com linha árabe intermediária | texto e caixas coincidem; árabe x=20 |
| Par explícito RTL com corpo LTR | texto e caixas coincidem |
| Última linha LTR em parágrafo RTL | texto e caixas coincidem |
| Última linha RTL em parágrafo LTR | texto e caixas coincidem |

Maior delta de caixa nesses nove controles: aproximadamente 0,01560pt. Não se exige igualdade de bytes PDF.

Seis controles permanecem abertos:

1. **RTL com trechos misturados na mesma linha e dois wraps mistos.** A âncora passou a ser conservada no wrap: no RTL a borda direita de todas as linhas chega a aproximadamente 200pt, e no LTR todas começam em 20pt. Mas ordem de texto/caixas de palavras ainda diverge. Nas últimas linhas, o span da borda oposta ainda difere cerca de um espaço (RTL min x=24,273930 contra27,770508; LTR max x=151,280400 contra147,772948). A âncora corrigida não fecha o espaçamento ou a ordem visual. O L3 ainda decide base pela linha/estilos de itens, sem receber este contexto de parágrafo. Próximo passo: separar essa perda de contexto das fronteiras de espaços e medir antes de escolher transporte ou fórmula.
2. **Block RTL com corpo inteiramente LTR.** ABC continua em x=20, contra152,949220 no vanilla. O Block não participa da certificação textual exterior; sua política de corpo independente precisa de owner próprio. Não contornar por reconhecimento de fixture.
3. **Align explícito left/right sob RTL.** Ambos conservam divergência de posição e baseline. Ler placement e medir precedência entre alignment explícito e default por direção; não corrigir só uma margem com constante de fixture.

No baseline, os seis controles já divergiam. A correção da âncora altera as posições em linhas mistas e melhora as bordas correspondentes, mas não afirma que cada posição desses casos ficou correta ou que suas saídas ficaram idênticas ao baseline. Não certificar paridade geral de direção mista.

## Checks e linhagem

`SOURCE_DATE_EPOCH=0 cargo test -p typst-core --lib`: **6519 passed, 5 ignored**. L3: **1160 passed**, zero falhas. Build debug/release e diff-check passaram. V15/V26 limpos antes do resselo. Resselo limitado aos sete owners em [scoped-seals.json](direcao-alinhamento-2026-10-05/scoped-seals.json).

Linter completo: **zero erros, 533 warnings, 1572 notes**. A comparação por multiset de regra/arquivo/mensagem não encontrou novos findings produtivos. Há um V5 adicional na cópia histórica `00_nucleo/diagnosticos/font-demand-geometry-study/before/line_commit.rs`, que ainda aponta ao L0 vigente agora atualizado; essa fonte arquivada não foi ressellada nem alterada. Alguns findings produtivos anteriores desapareceram com a delegação. Não afirmar zero violations global ou apagar a evidência histórica para limpar o contador. Nenhum V5/V6/V15/V26 ficou nos sete consumers tocados; Coverage::from_ranges permanece pendente/intacto.

Manter a separação de direção e âncora no domínio: RED→GREEN e nove controles fonte→PDF certificados, rollback e regiões cobertos. Próxima investigação prioritária é a direção-base que não chega ao bidi por linha; Block e precedência de Align são frentes delimitadas, não um único erro chamado genericamente RTL.

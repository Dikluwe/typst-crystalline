# Auditoria bilateral: composição de notas de rodapé

Estado medido: `results.json` registra hora UTC, HEAD, hashes dos binários, SHA-256 das sondas, páginas e texto extraído. `git-diff-HEAD-stat.txt` e `git-status-porcelain-v1.z` fixam a árvore de trabalho não commitada. O binário vanilla é o build pinado `9f2b6e8715237cb086899a42873660fe744622e8`. A medição é do binário cristalino em `target/release/typst` com SHA-256 registrado, não de uma recompilação implícita da árvore corrente.

Comandos, para cada `CASO` em `fit`, `defer`, `oversize` e cada `BIN`:

```sh
BIN compile CASO.typ CASO-BIN.pdf
pdfinfo CASO-BIN.pdf
pdftotext -layout CASO-BIN.pdf -
```

| Sonda | Vanilla pinado | Tekt medido | Observação no nível da linguagem |
| --- | --- | --- | --- |
| `fit.typ` | 1 página; chamada e corpo presentes | 1 página; chamada e corpo presentes | Controle positivo. A grafia do marcador do corpo difere; não foi objeto desta decisão. |
| `defer.typ` | 2 páginas; chamada e corpo na página 2 | 2 páginas; chamada na página 1, corpo na página 2 | A relação chamada/corpo perde a co-localização no Tekt. |
| `oversize.typ` | 2 páginas; linhas 1–14 e 15–21 do corpo distribuídas | 1 página; `pdftotext` encontra apenas linhas 1–11; aviso de possível truncamento/sobreposição | O conteúdo final não fica recuperável pela extração do PDF e a nota não continua. |

A diferença de `defer` é explicada pela ordem atual: `layout/footnote.rs:47-51` emite a chamada inline e enfileira o corpo; `footnote_flush.rs:150-169` decide adiar apenas o corpo, no fechamento da página. Isto é inferência causal a partir do código e da sonda, refutável por um percurso que migre a chamada depois desse ponto. A diferença de `oversize` decorre do sub-frame de altura ilimitada (`footnote_flush.rs:140-149`) e do `force_emit` (`:153-166`); a extensão fora da página não é fragmentada. O vanilla pinado usa região limitada, migração de origem quando o primeiro fragmento não cabe e fila de fragmentos restantes (`lab/typst-original/crates/typst-layout/src/flow/compose.rs:670-731`). A intenção do vanilla não é inferida apenas pelo resultado; essa é a mecânica observada na fonte local.

O L0 vigente `prompts/compiler/footnote_overflow_columns.md` permite explicitamente emissão forçada com aviso e não promete continuação; `prompts/compiler/layout/footnote.md` define emissão inline seguida de fila. Portanto, a correção completa exige rever esses contratos e a coordenação de paginação, antes de escrever L1. A proposta é separar três comportamentos sob responsabilidade das unidades existentes: (1) reserva/decisão da primeira região junto à chamada; (2) migração da chamada com o conteúdo de origem quando o primeiro fragmento não cabe; (3) continuação do corpo por fragmentos com progresso garantido, preservando ordem e identidade. O wire escolhe a política de região disponível, sem mover lógica de render para entidades nem substituir o despacho estático (ADR-0109). A aceitação será por presença, ordem e páginas observáveis; não por igualdade de bytes do PDF ou pela mesma estrutura interna do vanilla (ADR-0107).

Este diagnóstico não modifica o L0 nem o código. A correção de PDF/UA bundle é uma frente separada já concluída; arquivos compartilhados não foram alterados nesta auditoria.

# Auditoria — composição de comportamentos versus construção de percursos

**Estado medido:** 2026-09-30T02:43:07Z, HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitada. O `git-diff-stat.txt` e `git-status.txt` preservam o estado completo da medição. Binário cristalino `target/release/typst` SHA-256 `f10c23135e444dc94c446e8d3522b8f34ed1d4c7c89a90718e33c24ed15932de`; entradas, comandos, códigos de saída e SHA do artefato estão em `probe.json`. Auditoria de leitura paralela de L1, L3/L4 e L0/ADRs; nenhum código de produto foi alterado.

## Critério

Há **composição de comportamentos** quando unidades mantêm contratos próprios, uma política no owner correto escolhe ou combina essas unidades por condições observáveis e todas as rotas preservam as mesmas garantias. Separar um arquivo grande em módulos menores melhora localidade, mas sozinho não comprova essa propriedade. A escolha pode ser estática e exaustiva: a ADR-0109 (linhas 28–44, 80–85) rejeita despacho dinâmico em nome de atomização. O wiring L4 compõe L1–L3 e targets; política de layout permanece no L1. A ADR-0129 (linhas 54–93) compartilha claims por Núcleos, não código ou ownership produtivo.

| Frente | Estado observado | Evidência e consequência |
| --- | --- | --- |
| Parágrafo | Composição parcial certificada | `01_core/src/compiler/layout/paragraph_plan.rs:13-30,70-113` escolhe Simple/Optimized, hifenização ou seleção comum após `prepare`; `:43-68` e `paragraph_prepare.rs:126-137` recusam contextos ainda não certificados. O L0 `paragraph_plan.md:18-24,123-125` declara o fallback. Tracking uniforme foi medido; tracking misto com hifenização e largura variável ainda pedem sonda bilateral. |
| Pipeline PDF dentro de bundle | **Defeito confirmado de composição**, prioridade imediata | `03_infra/src/pipeline/pdf.rs:404-426` verifica título/alt PDF/UA-1 e anexos duplicados antes de exportar. `bundle.rs:463-513` resolve opções, mas pula essas verificações e chama o exporter. A sonda abaixo prova pelo menos a omissão do título. O PDF integrado não preserva o contrato do target isolado. |
| Fontes PDF | Construção duplicada, risco ainda não medido | `pdf.rs:428-541` escolhe fallback, single-font, multi-font e caso de fonte variável; `bundle.rs:479-512` repete resolução/gate e exporta sempre pela chamada multifont. Não se afirma diferença linguística ou desempenho sem comparação por classe de fonte. |
| Nota longa | Limite assumido, não defeito demonstrado | `01_core/src/compiler/layout/footnote_flush.rs:120-170` mede bodies inteiros, encaixa greedy e força o primeiro oversized com aviso. `footnote_overflow_columns.md:34-76` permite explicitamente truncamento/sobreposição nesse escape. Continuação/splitting do vanilla exigiria novo L0 e medição prévia. |
| Matemática extensível | Composição já existente | `01_core/src/compiler/math/layout/stretchy.rs:65-77,155-163` escolhe variante, assembly ou base conforme capacidade da fonte. O operador Large em `math/layout/mod.rs:1712-1742` parece ter percurso mais estreito, mas ainda é só candidato de sonda; seu owner é `_comum.md`, não `op.md`. |
| Bibliografia | Composição parcial | `01_core/src/compiler/layout/bibliography.rs:31-66` usa conteúdo CSL em cache ou render local de compatibilidade. `layout/bibliography.md:18-26` não declara equivalência entre rotas; medir falta de cache antes de propor unificação. |
| Wiring por formato | Composição legítima e explícita | `04_wiring/src/compile.rs:304-444` usa `match` exaustivo por target. O tamanho do `match` não é violação: `wiring/compile.md:32-36` exige essa seleção e a ADR-0109 preserva exaustividade. |

## Sonda causal PDF/UA-1

`single.typ` e `bundle.typ` contêm o mesmo corpo `#set format.pdf(standard: "ua-1")` seguido de `Hello`, sem `document.title`. Os comandos exatos estão em `probe.json`. O PDF isolado (`--format pdf`) sai com código 1 e `PDF/UA-1 error: missing document title`. O bundle (`--format bundle --features bundle`) sai com código 0 e publica `one.pdf`; seu XMP contém `<pdfuaid:part>1</pdfuaid:part>` sem `<dc:title>`. O resultado prova que a rota bundle ignora a validação de título do owner PDF. Não é prova de conformidade PDF/UA completa nem medição contra vanilla. Os possíveis desvios de alt/anexos são inferidos da fonte e precisam de testes próprios.

`--pdf-standard ua-1` na CLI do bundle mostrou outro comportamento: `compile.rs:155` só forma o override para `OutputFormat::Pdf`, embora `bundle.rs:71,468` aceite o campo. Este ponto precisa ser classificado contra o L0 da CLI antes de ser chamado de defeito; a sonda decisiva acima usa a propriedade documental dentro de cada target.

## Ordem de trabalho proposta

1. Tornar explícito no L0 de bundle que cada PDF interno herda as validações PDF-only do owner `pdf.md`; medir também alt e anexos duplicados. Uma implementação posterior deve compartilhar a validação no owner PDF e fazer bundle delegar a ele, preservando mensagens e ordem, antes de exportar qualquer arquivo do bundle. Como a rota passaria de sucesso para erro em entradas PDF/UA inválidas, classificar pelo gate ADR-0127 antes de escrever código.
2. Comparar a escolha de fontes de PDF isolado e bundle para zero, uma, várias e variável com eixos não-default, usando o mesmo documento e opções. Só então decidir se a seleção pode ser compartilhada sem alterar comportamento.
3. Medir uma fatia recusada do parágrafo (tracking misto com hifenização ou largura variável) e uma nota longa contra o vanilla pinado. Manter seletor estático no L1 e fallback até certificação completa.

A classificação estrutural não serve como prova de paridade linguística. Cada ampliação exige medição `file:line`, entradas equivalentes e resultado observável antes de atualizar o L0, conforme ADR-0108.

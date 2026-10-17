# Prioridade2 — contrato concreto para ratificação

Estado: preparação concluída; nenhum código produtivo foi escrito. Este pacote de processo não é L0. Três candidatos L0 completos acompanham o manifesto, com owners1:1 previstos.

## Medição anterior

Experimento de05/10 com ttf-parser0.25.1: extração/normalização isoladas53.19→10.85ms, equivalência nas1112 faces daquele catálogo e sete testes sintéticos. Proveniência integral em `00_nucleo/diagnosticos/cmap-intervalos-2026-10-05/`. Auditoria atual `00_nucleo/diagnosticos/coverage-ranges-preparacao-2026-10-07/audit.json`: consumer Coverage byte-idêntico à medição, extrator fonts.rs byte-idêntico ao mesmo HEAD. O release atual é R1,0a2410ec5b4a075417943d25a4d24ac860da6ff0adf4845707d4f18dc9cc7add. Não há ganho da CLI produtiva atribuído a intervalos; C2-R1 já corrigido não altera este algoritmo.

## Única extensão pública

Adicionar em Coverage, sob `00_nucleo/prompts/entities/font_book.md`:

```rust
pub fn from_ranges(
    ranges: impl IntoIterator<Item = std::ops::Range<u32>>,
) -> Self;
```

Intervalos semiabertos; vazios/invertidos não acrescentam membros. União exata aceita ordem arbitrária, sobreposição, adjacência e repetição. Não aproximar blocos Unicode nem filtrar pontos por glyphID. Preservar from_codepoints, contains, iter e sua representação/contratos; L1 recebe apenas inteiros neutros, sem parser/bytes/I/O.

## Materialização privada

Instalar o owner único do filho `03_infra/src/fonts/coverage.rs` em `00_nucleo/prompts/infra/fonts/coverage.md`; parent fonts.rs conserva seu facade privado e delega. Extrair intervalos4/12/13 pela elegibilidade tipada; preservar callback de outros formatos, rollback de tentativa nativa e fallback inteiro da Face quando o endpoint não puder ser representado. Catálogo, descoberta, cache, TTC/mmap/shaping e prioridade3/leitura seletiva ficam com seus contratos vigentes.

## Ordem após ratificação

1. Instalar exatamente os três candidatos como L0 canônicos antes de código; confirmar hashes, ownership/grafo, preservar snapshot atual.
2. Testes L1 contra set oracle; scaffold temporário vazio permite RED runtime do método novo; implementar união e GREEN. Não chamar falha de compilação ou cronômetro de RED semântico.
3. Testes sintéticos de adapter L3 e controles de faces/fallback; implementar bytes/checks e delegação privada, revalidar legado.
4. Resselo restrito dos três pares. Builds/suites/lint, seleção/fallback e controles reais de PDF/linguagem. Benchmark antes/depois do catálogo/CLI com input, fontes e saída equivalentes; medir RSS e separar tempo warm/new process.
5. Atualizar plano/relatório com proveniência, limitações e resultado real, mesmo se o ganho for pequeno ou refutado. Sem commit/reset ou hash global.

## Gate

ADR-0127 §2 exige confirmação antes de escrever código que amplia contrato público. O pedido “Continue com a2” autoriza esta frente e sua preparação; este documento concretiza a assinatura e os owners que precisam ser ratificados. A confirmação deste pacote autoriza implementar e validar em fluxo contínuo, sem pedir de novo pelos detalhes privados já especificados.

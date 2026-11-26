# Proposta de pacote: PNG de várias páginas

**26 de novembro de 2026 · aprovado pelo dono em 26/11/2026 (“Aprovo A–D”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) §1 (contrato de CLI)**

O problema apareceu ao medir o [pacote de cabeçalhos e rodapés](grid-headers-package-2026-11-26.pt-BR.md): a comparação por PNG via apenas uma página do cristalino em todo teste com várias páginas.

## O problema

`typst compile` com saída PNG grava só a primeira página. O nome do arquivo não é tratado como modelo: `out-{p}.png` vira literalmente `out-{p}.png`. `--pages` é ignorado, e um documento de várias páginas sai para `out.png` ou para a saída padrão sem erro. O SVG já segue a linguagem desde 22/10/2026.

## Medição

HEAD `3cdb73482`, árvore com a alteração de outra autoria em `layout/decorations` (`git diff HEAD --stat`: 5 arquivos, nenhum do CLI). Cristalino debug `d963afd2606cfbb07f80c024874e5f2c174bea63f7f256c87796ec21439b5715`, compilado do mesmo código Rust que `3cdb73482`, com a mesma alteração de outra autoria. Vanilla `lab/typst-original/target/release/typst`. Sondas de 11:20 a 11:44 de Brasília, com um documento de 3 páginas (`m.typ`), um de 1 página (`s.typ`) e um de 12 páginas (`d.typ`). As entradas estão em `lab/results/png-pages-2026-11-26/`, local e não versionado.

| Saída | Vanilla | Cristalino |
|---|---|---|
| `m-{p}.png`, 3 páginas | `m-1.png`, `m-2.png`, `m-3.png` | `m-{p}.png`, só a página 1 |
| `d-{0p}.png`, 12 páginas | `d-01.png` … `d-12.png` | `d-{0p}.png` |
| `m-{p}-of-{t}.png` | `m-1-of-3.png` … | `m-{p}-of-{t}.png` |
| `m-{n}.png` | `m-1.png` … | `m-{n}.png` |
| `m.png`, 3 páginas | erro `cannot export multiple images without a page number template ({p}, {0p}) in the output path`, código 1 | `m.png`, só a página 1, código 0 |
| `-`, 3 páginas | erro `cannot export multiple images to stdout`, código 1 | página 1 na saída, código 0 |
| `--pages 2-3` com `m-{p}.png` | `m-2.png`, `m-3.png` | `m-{p}.png`, página 1 |
| `s-{p}.png`, 1 página | `s-1.png` | `s-{p}.png` |
| `s.png`, 1 página | `s.png` | `s.png` |
| `m-{p}.svg` | `m-1.svg` … | `m-1.svg` … (já igual) |

## Causa

- `compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override` (`03_infra/src/pipeline/raster.rs:94`) exporta `doc.pages.first()`. O L0 de raster manda exportar só a primeira página (`00_nucleo/prompts/infra/pipeline/raster.md:29`).
- O braço `OutputFormat::Png` de `04_wiring/src/compile.rs:532` recebe bytes de uma imagem e não passa `--pages` nem o caminho de saída. O SVG usa `compile_to_svg_output_files` (`03_infra/src/pipeline/svg.rs:170-230`), com a regra de modelo ratificada no L0 de SVG (`svg.md:126-170`).

## Escopo proposto

| Parte | Mudança |
|---|---|
| A | nova função de PNG com a mesma forma de `compile_to_svg_output_files`: compila uma vez, renderiza cada página selecionada e devolve os pares (caminho, bytes) |
| B | a regra de modelo do SVG vale para PNG: `{p}`, `{0p}`, `{n}` e `{t}`; erro sem modelo com várias páginas; erro com várias páginas para `-`; seleção vazia não grava nada |
| C | `--pages` passa a valer para PNG |
| D | `compile` e `watch` gravam e publicam a coleção de PNG pelo mesmo caminho da coleção de SVG |

As funções antigas de primeira página ficam, com a assinatura de hoje, para quem as chama fora do CLI.

## Compatibilidade e contrato

É mudança de CLI, contrato protegido (ADR-0127 §1):

- `out.png` com várias páginas passa de sucesso a erro, com a mensagem do vanilla.
- Saída padrão com várias páginas também passa a erro.
- Um caminho com `{p}` deixa de gerar um arquivo com o nome literal.
- Documentos de uma página com nome sem modelo não mudam.

Não há flag, padrão, fase nem dependência nova. PPI, fontes e renderização de cada página ficam como estão.

## Fora

- Saída PNG do subcomando `query` e de outros formatos.
- Conferir a imagem de cada página contra o vanilla: isso é paridade de renderização, medida pela suíte oficial.

## Aceitação

1. L0 de raster, compile e watch primeiro, com selos e linhagem limpos.
2. RED no CLI: cada linha da tabela de medição igual ao vanilla em nomes de arquivo, número de arquivos, código de saída e mensagem.
3. O PNG da primeira página igual, em bytes, ao que o cristalino grava hoje; as outras páginas renderizadas pela mesma função.
4. A suíte oficial medida de novo por PNG, que passa a dar o mesmo número de páginas que a medição por PDF.
5. Suíte completa, lint e linhagem reversa, com build em RAM.

## Decisão pedida

Aprovar A–D. A alternativa é aprovar só A–C para `compile`, deixando o `watch` com a primeira página até um pacote próprio.

## Execução

**26 de novembro de 2026.** HEAD `3cdb73482`, árvore não commitada com A–D e a alteração de outra autoria (`git diff HEAD --stat`: 23 arquivos, contando as duas propostas novas de diagnóstico). Cristalino debug `7dad35291baa98c16643a278580944d2a1104e24d724276cf54c33716a32d525`, compilado em RAM. Suíte do projeto 10.992 aprovados, 1 falha intermitente, 7 ignorados; doc-tests 5; lint sem violações; linhagem reversa 1.034 consumidores, 0 erros.

**Como ficou.**

- A e B: `compile_to_png_output_files` compila uma vez e renderiza cada página selecionada. A regra de modelo do SVG foi extraída para `page_template` e `page_destination` e vale para os dois formatos.
- C: `--pages` vale para PNG.
- D: o `watch` grava cada página num arquivo temporário próprio e publica cada um no nome final. Antes, o `watch` com `out-{p}.svg` falhava com `failed to replace out-{p}.svg atomically`; agora grava `out-1.svg` … como o vanilla, e o mesmo vale para PNG. `ArmedWatch` ganhou `publish_files` e `abandon_files`.

Cada linha da tabela de medição sai igual ao vanilla em nomes de arquivo, número de arquivos, código de saída e mensagem (`04_wiring/tests/format_png.rs`). O PNG da primeira página tem os bytes da função antiga de primeira página (`03_infra/src/pipeline/raster.rs`).

**Suíte oficial por PNG.** De 11:55:58 a 11:58:41 (`png-after.json`, SHA-256 `a786e89994c2b7cf321367e927f7cff311d1468af79fdd0f0b65fd1269a09089`). Em todos os 1.941 testes que geram imagem nos dois compiladores, o número de páginas do cristalino é o mesmo da medição por PDF. Os testes em que o vanilla tem várias páginas e o cristalino aparecia com uma caíram de 211 para 37; os 37 restantes têm uma página só também pelo PDF.

**Testes ajustados ao contrato aprovado.** `armed_watch_contract.rs` fixa a superfície pública de `ArmedWatch` com os dois métodos novos. `format_svg.rs` comparava a primeira página de um documento de várias páginas gravado como `main.png`, o que agora é o erro do vanilla; passa a gravar `main-{p}.png` e comparar `main-1.png`.

**Intermitência anterior.** Dois testes de bytes de PDF com gradiente (`p269_pdf_bytes_cluster_3_variants_pos_focal_reproduzivel` e `p270_4_pdf_bytes_conic_cmyk_reproduziveis`) falharam cada um uma vez na suíte completa, um deles numa worktree sem esta mudança nem a alteração de outra autoria. Os dois passam isolados em cinco rodadas e no `typst-infra` inteiro. A exportação de gradiente em PDF não é sempre determinística sob carga; a causa fica para investigar.

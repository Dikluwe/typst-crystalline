# L0 — infra/export/tiling
Hash do Código: b99cdfdd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee
- 00_nucleo/prompts/_nuclei/tiling-placement.toml sha256:9c88c9c3bde8b005623fa70b5213400655a9a741b1a140b6b76c7225fc16edb4

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/tiling.rs`

## Medição anterior à decisão

`tiling-nativo-2026-10-14/before.json.gz` conserva HEAD/UTC/working tree,
diffstat e fontes. O layout/tiling.rs legado só cancela translação do shape;
SVG/svg.rs ignora paint_root e paints.rs aceita apenas size+Color. O contrato
aprovado no manifesto d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d
exige transformação afim completa, T(offset)*R e raiz local. A fonte pinada
9f2b6e871 em typst-library/src/visualize/tiling.rs:375–377 compõe translate
com rotate; typst-pdf/src/paint.rs:171–173 aplica correct_transform antes do
placement; typst-svg/src/lib.rs:346–358 distingue referencial self/parent.
As sondas de Parent e large-offset já constam do contrato aprovado. Isso
legitima geometria observável, não exige copiar a estrutura do upstream.

## Responsabilidade

Possuir helpers privados compartilhados pelos exporters: inversa afim finita,
validação de dimensões/pitch/placement e transformação da célula para as
coordenadas do frame que contém a Shape. Não executar layout, carregar
recursos, decidir estilo nem emitir formato neste módulo.

Self compõe T(shape.pos)*T(offset)*R(angle). Parent compõe a inversa completa
de local_to_root com T(offset)*R(angle); shape.pos já pertence ao caminho da
Shape no frame, não se subtrai duas vezes. Local_to_root é identidade em
Page/célula e em Group paint_root; grupos técnicos concatenam T(pos)*matrix.
Normalizar a translação final em coordenadas da própria base da grade:
inverter a parte linear, reduzir por pitch em cada eixo e reconstruir.
Assim diferenças por períodos inteiros preservam a fase sob rotação,
escala, reflexão e shear, incluindo deslocamentos grandes.

Rejeitar matriz singular/não finita e célula/pitch não positivos/não finitos
antes de qualquer conversão inteira ou repetição. Retornar Option privado;
o caller não pode tratar None como cor sólida. Na saída SVG inválida, manter
marcador explícito de transformação não representável e paint none, sem
promover o caso a render correto. Casos de domínio inválido normalmente são
diagnóstico no preparador L1; Group singular e frames Rust externos exigem
este guard adicional no consumer.

## Aceitação

Testes numéricos analíticos discriminam Self/Parent, composição completa,
normalização periódica no referencial correto, pivô da rotação e guards.
Testes fonte→targets devem validar a geometria visual; helpers verdes não
substituem T1c/T2/T3. Conservar defaults/APIs públicas e paridade de língua,
sem fallback silencioso, geração de N clones ou comparação por bytes como
critério universal. Validar suíte/build/doc/fmt/linhagem e refutadores do
contrato completo antes de certificar tiling de produção.

## Percurso privado de recursos preparado

Medição: images/scan.rs:57,139,273, fonts.rs:80 e font_binding.rs:124 usam o visitor semântico, que em layout_types.rs:1370 visita Shape sem entrar em paint. A troca de transporte ocultaria recursos antes presentes em Group.

Decisão: este owner também oferece walker privado exclusivo de recursos visuais, delegando folhas ao visitor e descendo Group/Link/Semantic/TextReplacement e fill/stroke TilingFrame. Não chama callbacks de semântica/link de célula; não altera FrameVisitor público nem navegação/MCIDs. Coleta de recursos sem I/O aqui; decoders e fontes permanecem nos owners atuais.

## Higiene da migração nativa

Medição: verified-v3/followup-clippy-inventory.json e followup-architecture.log, com candidate-final e workspace-pre próprios, registram quatro f32::hypot proibidos, um argumento adicional, dois itens produtivos depois de módulos de teste e dois wildcards neutros novos.

Decisão interna em fluxo contínuo (ADR-0127): Posicionar helpers produtivos antes do módulo de testes, sem alteração de comportamento. O walker privado de recursos enumera explicitamente todas as folhas FrameItem, tornando futuras variantes erro de compilação. Não elevar limites nem adicionar allow. Validar suíte e renders antes/depois, além de formato, doc e linhagem.

## Composição de período e alpha sólido — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-overlap-2026-10-14/before.json.gz` preserva fontes, owners, HEAD, UTC e working tree. `tiling-nativo-2026-10-14/final-renders` e `remaining-analysis.md` registram cell8×8/pitch4×4 alpha50%: PDF vermelho opaco; SVG só uma contribuição; PNG quatro contribuições (255,15,15). `pdf_tiling.rs:184` emite BBox=size, `svg/paints.rs:148` usa overflow visible; `stream/shape.rs:155` e `stream/text.rs:275` descartam alpha. São defeitos observáveis de composição; o vanilla também corta overlap e não serve de requisito para esse defeito. Hipótese refutável: compor explicitamente um período preserva controles sem overlap e quatro contribuições coincidem com fonte-over analítica.

### Responsabilidade desta unidade

Fornecer iterador privado de contribuições que intersectam um período [0,pitch), na ordem y crescente e x crescente. Em cada eixo índices de 1-ceil(size/pitch) até zero inclusivos, translação índice*pitch. A célula já é recortada a size; o período é recortado a pitch. Sem vetor de cópias proporcional ao viewport. Dimensões/pitch finitos positivos são precondição do transporte; checar contagem representável antes de conversão. A integração de diagnóstico de limites extremos segue aberta, sem truncar repetições ou alegar sucesso.

### Aceitação

Correção interna ADR-0127 dentro do T1–T3 já aprovado, sem nova API/fase/flag/default. L0 primeiro, RED→GREEN, composição analítica (incluindo ordem com cores distintas), controles simples/nested/rotated/text/image e extração/seleção de páginas. Build, suíte workspace, formato, Rustdoc e linhagem. Alpha variável de gradientes e qualidade PNG seguem incompletos; não certificar pacote de produção com esses itens abertos.

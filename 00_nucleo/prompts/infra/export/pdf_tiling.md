# L0 — infra/export/pdf_tiling
Hash do Código: 780d8ec5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/pdf_tiling.rs`

## Medição anterior à decisão

O snapshot tiling-nativo-2026-10-14/before.json.gz registra fontes/UTC/HEAD/working tree. stream/shape.rs:156/204 converte Tiling a cor; builder.rs:277 reserva somente gradientes. O manifesto aprovado d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d exige célula preparada nativa. O pin vanilla 9f2b6e871 em typst-pdf/src/paint.rs:161–195 prepara stream de célula com raiz local e transformação corrigida.

## Decisão e contrato privado

Registrar padrões por ocorrência emprestada de Shape e identidade do Arc de célula, sem serializar endereços. IDs e nomes são locais determinísticos. Recorrer Groups com matriz local e raiz lógica paint_root; Link/Semantic apenas contém itens visuais. Cada célula aninhada tem canvas próprio e raiz identidade. O grafo de frames é finito e não contém referências de retorno mutáveis.

Emitir PatternType 1 PaintType 1 TilingType 1, BBox [0 0 cell.width cell.height], XStep/YStep = size + spacing positivos, recursos compartilhados, e stream vetorial da célula sem layout L3. A matriz converte coordenadas Y-up da célula para default user space do stream dono: flip(owner.height) * local_to_owner * placement * flip(cell.height). O helper afim de tiling decide Self/Parent e normaliza períodos. Rejeitar placement inválido explicitamente, sem sólido substituto.

O stream da célula usa itens emprestados, sem clonar ocorrências nem invalidar chaves de gradientes. Desabilitar tagging/MCIDs/UA1 durante a emissão e restaurar contexto antes da página; nenhuma anotação/destino é criado a partir da célula. A célula é recortada por BBox, inclusive overflow de conteúdo. Overlap de spacing negativo fica no padrão PDF nativo. Compartilhar fontes/imagens/spot/gradientes/patterns aninhados, sem ciclos em Resources: cada pattern deve referenciar apenas seus descendentes de tiling. Recursos de página incluem todos os padrões usados na página, inclusive descendentes.

## Aceitação

T1–T3 aprovado; este módulo não autoriza publicação. Conferir estrutura PDF/qpdf, limites/célula/spacing/offset/rotação/Parent afim/contorno/densidade, texto e imagens aninhados e isolamento semântico. Render e diagnósticos são a aceitação de linguagem; IDs e bytes não certificam paridade. Build e linhagem completos devem ficar verdes.

A montagem retorna objetos (ID, bytes) ao builder proprietário, que os armazena pela API privada existente.

## Refinamento medido do referencial da célula

A sonda native-v1/dense-open-tekt.pdf tem Matrix [1 0 0 1 0 139] e retângulo Y-up; pdftoppm a 144 DPI desloca as linhas e funde colunas de 0.5pt. O pin vanilla usa Matrix [1 0 0 -1 20 120] com caminho Y-down. A geometria contínua equivalente não produz a mesma cobertura na grade de raster do padrão. Emitir itens no espaço local Y-down, com matriz to_pdf * placement, e células aninhadas iniciam to_pdf identidade. Isto corrige o observável de cobertura, sem impor igualdade de bytes. Gradientes da célula usam o mesmo referencial Y-down.

## Reuso entre páginas e seleção

Inspeção: Patterns guarda nomes em um HashMap global e Pattern.page no primeiro encontro; Arc de célula é compartilhado entre páginas, então o segundo caminho pode reutilizar um descendente marcado na página excluída. Refutador: páginas clonadas com tiling aninhado e seleção somente da segunda. O registro de nomes deve ser separado por página, e cada descendente deve ser emitido com os recursos de sua página selecionada. Não depender da emissão de uma página excluída.

## Higiene da migração nativa

Medição: verified-v3/followup-clippy-inventory.json e followup-architecture.log, com candidate-final e workspace-pre próprios, registram quatro f32::hypot proibidos, um argumento adicional, dois itens produtivos depois de módulos de teste e dois wildcards neutros novos.

Decisão interna em fluxo contínuo (ADR-0127): A varredura enumera explicitamente as folhas FrameItem que não contêm paints, sem wildcard neutro; futuras variantes exigem decisão explícita. Não elevar limites nem adicionar allow. Validar suíte e renders antes/depois, além de formato, doc e linhagem.

## Composição de período e alpha sólido — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-overlap-2026-10-14/before.json.gz` preserva fontes, owners, HEAD, UTC e working tree. `tiling-nativo-2026-10-14/final-renders` e `remaining-analysis.md` registram cell8×8/pitch4×4 alpha50%: PDF vermelho opaco; SVG só uma contribuição; PNG quatro contribuições (255,15,15). `pdf_tiling.rs:184` emite BBox=size, `svg/paints.rs:148` usa overflow visible; `stream/shape.rs:155` e `stream/text.rs:275` descartam alpha. São defeitos observáveis de composição; o vanilla também corta overlap e não serve de requisito para esse defeito. Hipótese refutável: compor explicitamente um período preserva controles sem overlap e quatro contribuições coincidem com fonte-over analítica.

### Responsabilidade desta unidade

Substituir a dependência de overlap nativo por Form XObject da célula, BBox=size e corpo vetorial emitido uma vez. Reservar dois IDs por ocorrência (pattern e form); PatternType1 tem BBox=pitch, XStep/YStep=pitch e referências traduzidas ao Form para cada contribuição do helper. Recursos do Form conservam fontes/imagens/ExtGState/gradientes e apenas descendentes de tiling; recursos do Pattern referenciam somente o Form. Célula Y-down, matriz de placement e isolamento semântico permanecem. Não inserir grupo de transparência isolado que mude a composição de folhas. O default user space de padrões aninhados deve permanecer o da célula; testar antes de promover.

### Aceitação

Correção interna ADR-0127 dentro do T1–T3 já aprovado, sem nova API/fase/flag/default. L0 primeiro, RED→GREEN, composição analítica (incluindo ordem com cores distintas), controles simples/nested/rotated/text/image e extração/seleção de páginas. Build, suíte workspace, formato, Rustdoc e linhagem. Alpha variável de gradientes e qualidade PNG seguem incompletos; não certificar pacote de produção com esses itens abertos.

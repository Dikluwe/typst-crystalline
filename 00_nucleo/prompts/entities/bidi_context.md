# L0 — Contexto lógico imutável de bidi
Hash do Código: 18708382

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8

**Status:** contrato ratificado pelo humano; consumer materializado. Aprovação imutável em `lab/history/construction/records/00_nucleo/materialization/bidi-contexto-aprovacao-2026-10-05.json`.
**Layer:** L1.
**Exclusive consumer:** `01_core/src/entities/bidi_context.rs`.

## Medição anterior à decisão

Medição anterior à decisão: `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/initial.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree integral, UTC, fontes e binários; treze entradas CLI com vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. rtl_auto_mixed põe ABC em x=124.020510 no Tekt contra152.949220 no vanilla; ltr_numeric põe árabe à esquerda e inverte 123/456, contra números à esquerda no vanilla. `layout_bidi.rs:73–82` usa primeiro caractere forte da linha e `:227–240` usa qualquer style.dir RTL como nível padrão; `shaper/bidi.rs:34` resolve cada string novamente com base automática. O contexto privado de paragraph.rs não chega a esses consumers. Vanilla `inline/mod.rs:189,232` resolve direção compartilhada; `inline/collect.rs:156–178` conserva embedding explícito por TextElem com direção diferente da base. `probe.json.gz` registra receita, rustc, hashes da biblioteca unicode-bidi 0.3.18 e sonda independente: dentro de ABC 123 456 789 مرحبا, linha 456 789 é um run LTR; a mesma string isolada com a mesma base RTL tem runs 789, espaço, 456. Isso refuta o transporte somente de Dir e a resolução por linha/palavra isolada. Não é prova de geometria de um candidato produtivo. Semântica/sintaxe/morfologia são obrigações; biblioteca, ranges, Arc e estratégia de cache são mecânica interna (ADR-0107).

## Decisão ratificada e limites

Representar a entrada lógica completa de um parágrafo textual certificado e a origem de um fragmento nela. Não representar coordenadas como identidade, não inserir controles virtuais no Content/texto copiável, não sobrescrever a direção dos trechos nem guardar o contexto de parágrafo em TextStyle. Esta unidade possui somente dados/validação puros, sem algoritmo Unicode, fontes, regiões, I/O, relógio, globals mutáveis ou despacho dinâmico. Arc em campos é permitido por ADR-0029/0030; nenhum unsafe.

A avaliação de Unicode bidi continua em L3, na ordem existente layout→bidi→shaping. L1 fornece dados lógicos; não transferir o algoritmo de bidi para L1, nem adicionar dependência externa à whitelist neste contrato. Não certificar a linguagem inteira, objetos, matemática, links, regiões compostas, hifenização/smallcaps ou transformação cujo texto não tenha correspondência exata. Fallback sem contexto permanece explícito, não deve inventar uma origem por busca de texto ou por comparação de estilos.

## Contrato público ratificado

```rust
pub enum ParagraphBidiSegment {
    Text { text: EcoString, dir: Dir },
    Space { origin: TextSeparatorOrigin },
    Linebreak,
}

pub struct ParagraphBidiInput { /* campos privados */ }
pub struct BidiSpan { /* campos privados */ }

impl ParagraphBidiInput {
    pub fn new(base_dir: Dir, segments: Vec<ParagraphBidiSegment>) -> Option<Self>;
    pub fn base_dir(&self) -> Dir;
    pub fn segments(&self) -> &[ParagraphBidiSegment];
}

impl BidiSpan {
    pub fn new(input: Arc<ParagraphBidiInput>, segment: usize,
               range: Range<usize>) -> Option<Self>;
    pub fn input(&self) -> &Arc<ParagraphBidiInput>;
    pub fn segment(&self) -> usize;
    pub fn range(&self) -> Range<usize>;
    pub fn subspan(&self, relative: Range<usize>) -> Option<Self>;
}
```

Usar tipos canônicos L1 `Dir`, `EcoString`, `TextSeparatorOrigin` e stdlib pura `Arc`/`Range`. Tipos devem implementar Clone e Debug; não exigir Hash, comparação por endereço ou identidade global como contrato da língua. Clone de BidiSpan conserva Arc/índice/range; a entrada é imutável, com base e segmentos privados. Não disponibilizar setters.

### Validação e identidade

- `new` de input aceita somente LTR/RTL, segmentos não vazios e pelo menos um Text não vazio; retorna None para vertical ou ausência de texto substantivo. Não normalizar, reordenar ou recortar caracteres em Text. Espaço é U+0020 de origem já fornecida, não inferido de gaps. Linebreak é fronteira explícita da fonte; wrap suave não cria um segmento adicional.
- `BidiSpan::new` exige índice existente, start≤end, bounds dentro do texto original do segmento e fronteiras UTF-8 válidas. Em Space, o domínio de bytes é o U+0020 e conserva origem; Linebreak não aceita BidiSpan de texto. Não aceitar índice/range por aproximação. None indica entrada inválida, sem panic ou mutação.
- `subspan` interpreta os bounds relativamente ao começo do span atual, faz soma verificada, exige contenção no span atual e valida novamente UTF-8. Mantém input/segment. Não encontrar a subparte por igualdade de string: palavras repetidas são ocorrências distintas.
- O produtor deve garantir que o texto da folha é exatamente a fatia referida; após split usar subspans correspondentes. Reordenação física, translação, clone, envelopes e rollback não alteram origem lógica. Um merge somente conserva um span único quando os intervalos são contíguos no mesmo segmento/input; caso contrário deve conservar fragmentos e spans separados.
- A referência tem lifetime de RAM, não de World/filesystem. Não reter entre compilações/watch nesta etapa. Não comparar Eq Rust ou bytes PDF como aceitação de linguagem.

## Responsabilidades dos futuros consumers

O owner de paragraph prepara o segmento inteiro certificado antes da emissão, incluindo direção-base e os trechos de direção efetiva, espaços reais e quebras explícitas. A emissão de Text/Space/fragmentos fornece ranges originais determinísticos sem matching posterior por string e conserva offsets UTF-8. O estado de layout/checkpoints/subframes guarda cursores separados de dados imutáveis; rejeita certificação de paths que não consiga mapear exatamente. Acabar o escopo exterior não apaga o contexto aceito da cauda.

L3 constrói sua entrada Unicode auxiliar a partir destes segmentos, aplicando embeddings virtuais para Text cuja direção difere da base e conservando mapas para bytes originais. Esses caracteres auxiliares nunca entram no texto fonte, nos glifos ou no ToUnicode. Resolver o parágrafo completo; consultar ranges efetivos das linhas com as regras de fim de linha, e ranges de shaping dos fragmentos, sem redetectar base pela string fragmentada. A representação auxiliar e a biblioteca continuam pertencendo aos owners L3, não a esta entidade.

Caches de bidi, se necessários, são locais à compilação e reutilizam a mesma entrada imutável; não associar a origem por y/x ou guardar self-reference/unsafe para manter strings e BidiInfo. Pode haver um registro proprietário de strings e views emprestadas com lifetimes Rust normais em variáveis separadas. Não exigir novo método de FontMetrics nesta rodada; qualquer necessidade medida de ampliação de trait público requer L0 próprio e novo gate ADR-0127. O contexto não entra nas chaves de fontes por meio de TextStyle.

## Aceitação obrigatória após confirmação

RED→GREEN de validação UTF-8/bounds/overflow, clone/subspan e duas palavras iguais com ranges distintos. RED→GREEN do contexto completo versus resolução isolada de 456 789; campos bidi Some somente em certificação inteira, None nos caminhos fora do escopo.

Sondas fonte→PDF contra pin ratificado: bases opostas com primeiro run invertido, overrides explícitos, números/neutral, soft wrap e linebreak, escopos integrais/parbreak, restauração/checkpoints e controles antes certificados (inclusive sufixos colados). Conferir texto/codepoints, caixas agrupadas por palavras/baselines, origem dos espaços e ausência de controles auxiliares na extração. Medir os spans da última linha e discriminar falhas de espaço das de ordem bidi; não fechar por igualdade do bbox total.

Migrar cada producer/pattern afetado sob seu L0 proprietário antes de escrever código. Preenchimento None em produtores sem certificação conserva comportamento e não amplia escopo por acidente. Auditoria do cache de frames: se uma entrada carregada depende de contexto exterior que não está provado igual, invalidar ou não reutilizar; não certificar a origem de um frame só porque seu texto/geometria coincide.

Build debug/release, testes apropriados L1/L3 e V15/V26 limpos antes do resselo restrito. Publicação/re-sync vanilla/novos defaults não fazem parte desta aprovação.

